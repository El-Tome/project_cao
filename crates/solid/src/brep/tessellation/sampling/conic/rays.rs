//! The rays through every sample of every conic, for each wall about its
//! wall's axis, and the angles of every ray those walls take in contact.

use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::DVec3;

use super::of_edge;
use crate::brep::conic::{Conic, Round};
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::tessellation::contact::{Contact, Wall, Zones};
use crate::brep::tessellation::sampling::abreast::{Abreast, heights};
use crate::brep::tessellation::sampling::cone::Grid;
use crate::brep::topology::{Body, Edge, SurfaceId};

/// The rays through every sample of every conic, and through its ends, for
/// each wall about its wall's axis: what their circles, and the curve a
/// cylinder meets a perpendicular wall along, take for the conic, as the
/// walls of a meet take its samples. A conic halved between the rays of its
/// wall where a chord would leave it, beside a circle, a meet or another
/// conic sampled on other angles about the same axis, has its chords cross
/// theirs where the two touch or meet at a sharp angle, parting as the
/// square of the distance from there: on the same rays, each strip between
/// two holds both curves' chords in their order.
pub(in crate::brep::tessellation::sampling) fn rays_along(
    body: &Body,
    walls: &[Wall],
    zones: &Zones,
    grids: &BTreeMap<SurfaceId, Grid>,
    contacts: &BTreeMap<SurfaceId, Contact>,
    fineness: (f64, f64),
) -> BTreeMap<SurfaceId, Vec<DVec3>> {
    let eps = body.scale().eps();
    let alone = Abreast::default();
    let sampled: Vec<(&Edge, &Conic, Vec<DVec3>)> = body
        .edge_ids()
        .filter_map(|id| {
            let edge = body.edge(id);
            let Curve::Conic(conic) = body.curve(edge.curve) else {
                return None;
            };
            let rays = rays_about(conic, walls, contacts, eps);
            let ends = edge
                .ends
                .into_iter()
                .flatten()
                .map(|end| body.vertex(end).point);
            let points = of_edge(body, edge, conic, (zones, grids, &alone), &rays, fineness)
                .into_iter()
                .chain(ends)
                .collect();
            Some((edge, conic, points))
        })
        .collect();
    let mut along: BTreeMap<SurfaceId, Vec<DVec3>> = BTreeMap::new();
    for (edge, conic, points) in &sampled {
        let (origin, axis) = about(&conic.round);
        let beside = sampled
            .iter()
            .filter(|(other, theirs, _)| leaving_together(body, [edge, other], [conic, theirs]))
            .flat_map(|(_, _, theirs)| theirs.iter());
        let ways: Vec<DVec3> = points
            .iter()
            .chain(beside)
            .filter_map(|point| {
                let from = *point - origin;
                (from - axis * axis.dot(from)).try_normalize()
            })
            .collect();
        for (wall, _) in walls
            .iter()
            .filter(|(_, wall)| coaxial(wall, origin, axis, eps))
        {
            along.entry(*wall).or_default().extend(ways.iter().copied());
        }
    }
    along
}

/// The sine under which two curves leaving a vertex are read as leaving it
/// together.
const TOGETHER: f64 = 1e-3;

/// The drift, in radians, under which an elliptic rim and another leaving a
/// vertex together keep their own samples: a tenth of a step of the
/// coarsest grid, sixteen steps a turn. The rims of an elliptic shaft and a
/// bore of its minor radius were left uncut up to 0.025 at that grid, and
/// at finer ones only under it, whatever the lean, the height or the size.
pub(in crate::brep::tessellation) const ADRIFT: f64 = TAU / 160.0;

/// Whether two conics of one plane, cut on walls about axes across each
/// other, leave a vertex they share together: where the two walls touch at
/// a node, a plane through it cuts both along curves tangent there, parting
/// as the square of the distance from it. Sampled each on its own wall's
/// angles, their chords cross near the node, and the face between them, a
/// horn, cannot be cut (5550117749); each then takes the other's samples as
/// its wall's rays. Not on two parallel walls: touching along a line, they
/// are sampled on the rays they share already, which another wall's would
/// put out of order (5551609285). Nor on an elliptic wall and another whose
/// axis leans off it by so little that a sample handed across drifts round
/// by less than [`ADRIFT`] from one rim to the next: read off the other
/// wall's axis at another height it lands that drift off its own, nothing
/// gathers the two into one ray as two cylinders' are
/// (`contact/leaning.rs`), and handed to and fro pass after pass the rim's
/// samples part from the elliptic rim's, whose chords cross them beside
/// the tip (354000000000100).
fn leaving_together(body: &Body, [one, other]: [&Edge; 2], [first, second]: [&Conic; 2]) -> bool {
    let (Some(ends), Some(theirs)) = (one.ends, other.ends) else {
        return false;
    };
    let [(_, one_axis), (_, other_axis)] = [first, second].map(|conic| about(&conic.round));
    let sine = one_axis.cross(other_axis).length();
    if sine <= Scale::RELATIVE
        || first.plane.normal.cross(second.plane.normal).length() > Scale::RELATIVE
    {
        return false;
    }
    let elliptic = [first, second]
        .iter()
        .any(|conic| matches!(conic.round, Round::Elliptic(_)));
    let tangent = |conic: &Conic, edge: &Edge, at: DVec3| {
        let [from, to] = [edge.from, edge.to].map(|t| conic.point(t).distance(at));
        conic
            .derivative(if from <= to { edge.from } else { edge.to })
            .normalize()
    };
    ends.iter().filter(|end| theirs.contains(end)).any(|end| {
        let at = first.point(if ends[0] == *end { one.from } else { one.to });
        tangent(first, one, at)
            .cross(tangent(second, other, at))
            .length()
            <= TOGETHER
            && !(elliptic && drift(body, [&first.round, &second.round], at, sine) < ADRIFT)
    })
}

/// How far round, in radians, a sample handed from one of two walls to the
/// other drifts between their rims: how far their axes, leaning by `sine`,
/// part over the heights the faces of either span, against the nearer of
/// the two axes to `at`, where the rims leave together.
pub(in crate::brep::tessellation) fn drift(
    body: &Body,
    rounds: [&Round; 2],
    at: DVec3,
    sine: f64,
) -> f64 {
    let (_, axis) = about(rounds[0]);
    let span = rounds
        .iter()
        .filter_map(|round| {
            let surface = match round {
                Round::Cylinder(cylinder) => Surface::Cylinder(*cylinder),
                Round::Cone(cone) => Surface::Cone(*cone),
                Round::Elliptic(wall) => Surface::Elliptic(*wall),
            };
            let rank = body.surfaces.iter().position(|held| *held == surface)?;
            heights(SurfaceId(rank as u32), axis, body)
        })
        .reduce(|[low, high], [from, to]| [low.min(from), high.max(to)]);
    let Some([low, high]) = span else {
        return f64::INFINITY;
    };
    let nearest = rounds
        .map(|round| {
            let (origin, axis) = about(round);
            let from = at - origin;
            (from - axis * axis.dot(from)).length()
        })
        .into_iter()
        .fold(f64::INFINITY, f64::min);
    (high - low) * sine / nearest
}

/// The angles, about the conic's wall from its own `u`, of every ray the
/// walls about the same axis take in `contacts`.
pub(in crate::brep::tessellation::sampling) fn rays_about(
    conic: &Conic,
    walls: &[Wall],
    contacts: &BTreeMap<SurfaceId, Contact>,
    eps: f64,
) -> Vec<f64> {
    let (origin, axis) = about(&conic.round);
    let (u, v, scaled) = match conic.round {
        Round::Cylinder(cylinder) => (cylinder.u, cylinder.v, None),
        Round::Cone(cone) => (cone.u, cone.v, None),
        Round::Elliptic(wall) => (wall.u, wall.v, Some((wall.major, wall.minor))),
    };
    let own = match conic.round {
        Round::Elliptic(wall) => Some(wall),
        Round::Cylinder(_) | Round::Cone(_) => None,
    };
    walls
        .iter()
        .filter(|(_, wall)| coaxial(wall, origin, axis, eps))
        .filter_map(|(id, _)| contacts.get(id))
        .chain(
            contacts
                .values()
                .filter(|contact| own.is_some() && contact.oval == own),
        )
        .flat_map(|contact| contact.rays.iter())
        .map(|way| match scaled {
            None => way.dot(v).atan2(way.dot(u)),
            Some((major, minor)) => (way.dot(v) / minor).atan2(way.dot(u) / major),
        })
        .collect()
}

fn about(round: &Round) -> (DVec3, DVec3) {
    match round {
        Round::Cylinder(cylinder) => (cylinder.origin, cylinder.axis),
        Round::Cone(cone) => (cone.origin, cone.axis),
        Round::Elliptic(wall) => (wall.origin, wall.axis),
    }
}

/// Whether a wall turns about the line through `origin` along `axis`.
fn coaxial(wall: &Cylinder, origin: DVec3, axis: DVec3, eps: f64) -> bool {
    let from = wall.origin - origin;
    wall.axis.cross(axis).length() <= Scale::RELATIVE
        && (from - axis * axis.dot(from)).length() <= eps
}

#[cfg(test)]
mod tests;
