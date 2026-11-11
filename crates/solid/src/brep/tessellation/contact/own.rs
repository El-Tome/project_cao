//! The rays the circles of a wall take on their own: through its vertices,
//! and through the curves it meets a perpendicular wall along. An elliptic
//! wall's rims take the eccentric angles of the same points (#571).

use std::collections::BTreeMap;
use std::f64::consts::PI;

use glam::DVec3;

use super::super::sampling::{ADRIFT, divisions_in, drift};
use super::{Contact, Wall, bears};
use crate::brep::conic::Round;
use crate::brep::curve::{self, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::{Body, EdgeId, SurfaceId};

/// For each wall, the vertices lying on it; the rays through those and
/// through the vertices standing just inside it, which it keeps whatever
/// the room of a wall close to it; and the rays through the second alone.
#[derive(Default)]
pub(super) struct Through {
    pub(super) anchors: BTreeMap<SurfaceId, Vec<DVec3>>,
    pub(super) pinned: BTreeMap<SurfaceId, Vec<DVec3>>,
    pub(super) beneath: BTreeMap<SurfaceId, Vec<DVec3>>,
}

/// What the circles of two perpendicular cylinders take for the curve they
/// meet along: the rays through every sample of it and through its ends.
///
/// Where one wall is cut from the other, each lies over the other along the
/// curve, and they part slowly: as the square of the distance from a node
/// where they touch, or from a waist where they all but touch, and along
/// the curve wherever it runs down a wall's ruling, at the tip of a window.
/// A triangle of the large wall fanned from a sample of its circle a grid
/// step away sags there below the small wall, and at a fine tolerance the
/// curve's own samples are much closer than a step of the grid. Sampled along
/// these rays, each wall is cut into strips between two samples of the curve,
/// the large one's square to them and the small one's, between its two mirror
/// lobes, across them: within a strip each sags as the square of its width,
/// and the two stay ordered.
///
/// But a third wall crossing two walls whose axes lean a hair off each
/// other takes no ray from either meet: it would carry each sample of one
/// meet onto the other, read off the other wall's axis at the meet's
/// height and so a drift round on its rims, and back pass after pass —
/// the rims of the two walls part where they leave a cap together, and
/// their chords cross beside the tip (352000000009685).
pub(super) fn along_meets(
    body: &Body,
    meets: &[Vec<DVec3>],
    own: &mut BTreeMap<SurfaceId, Vec<DVec3>>,
) {
    for id in body.edge_ids() {
        let edge = body.edge(id);
        let Curve::Meet(meet) = body.curve(edge.curve) else {
            continue;
        };
        let ends = edge
            .ends
            .into_iter()
            .flatten()
            .map(|end| body.vertex(end).point);
        let through: Vec<DVec3> = meets[id.0 as usize].iter().copied().chain(ends).collect();
        let walls = super::held(body, id, meet);
        for (side, (wall, held)) in [meet.first, meet.second].into_iter().zip(walls).enumerate() {
            let at = meet.point(edge.from);
            if relays(body, id, [held, walls[1 - side]], at) {
                continue;
            }
            let rays: Vec<DVec3> = match wall {
                curve::Wall::Cylinder(cylinder) => through
                    .iter()
                    .map(|point| flat_from(&cylinder, *point).normalize())
                    .collect(),
                curve::Wall::Elliptic(oval) => {
                    through.iter().map(|point| *point - oval.origin).collect()
                }
                curve::Wall::Cone(_) => continue,
            };
            for surface in (0..body.surfaces.len() as u32).map(SurfaceId) {
                if *body.surface(surface) == held.surface() {
                    own.entry(surface).or_default().extend(rays.iter().copied());
                }
            }
        }
    }
}

/// Whether `third`, meeting `wall` along `edge`, also meets another wall
/// leaning off `wall` by a drift under [`ADRIFT`], read at `at`.
fn relays(body: &Body, edge: EdgeId, [third, wall]: [curve::Wall; 2], at: DVec3) -> bool {
    let along = axis(&wall);
    body.edge_ids().filter(|other| *other != edge).any(|other| {
        let Curve::Meet(theirs) = body.curve(body.edge(other).curve) else {
            return false;
        };
        let walls = super::held(body, other, theirs);
        let Some(side) = walls.iter().position(|held| *held == third) else {
            return false;
        };
        let partner = walls[1 - side];
        let sine = along.cross(axis(&partner)).length();
        partner != wall
            && sine > Scale::RELATIVE
            && drift(body, [&round(wall), &round(partner)], at, sine) < ADRIFT
    })
}

fn axis(wall: &curve::Wall) -> DVec3 {
    match wall {
        curve::Wall::Cylinder(cylinder) => cylinder.axis,
        curve::Wall::Elliptic(oval) => oval.axis,
        curve::Wall::Cone(cone) => cone.axis,
    }
}

fn round(wall: curve::Wall) -> Round {
    match wall {
        curve::Wall::Cylinder(cylinder) => Round::Cylinder(cylinder),
        curve::Wall::Elliptic(oval) => Round::Elliptic(oval),
        curve::Wall::Cone(cone) => Round::Cone(cone),
    }
}

/// What the circles of every cylinder take for the vertices lying on it: the
/// rays through them. A ruling or a seam leaves its wall at a vertex off the
/// grid, and a strip of the wall beside it must meet a sample on every rim at
/// that angle, or its triangle reaches from the vertex to the other rim's next
/// step — lying flat where the wall is a hair high, over the face beside it.
///
/// So too for a vertex standing inside the wall closer than twice a chord's
/// sag — the corner of a pocket a hair inside it: the wall's chords sag
/// towards the axis, and a chord across that angle would pass inside the
/// corner. Sampled there, the wall stands on its own surface at the corner.
/// So too for each point `inside` names for the wall: a sample of another
/// curve standing there.
pub(super) fn through_vertices(
    body: &Body,
    walls: &[Wall],
    inside: &BTreeMap<SurfaceId, Vec<DVec3>>,
    (tolerance, span): (f64, f64),
    own: &mut BTreeMap<SurfaceId, Vec<DVec3>>,
) -> Through {
    let mut through = Through::default();
    for wall in walls {
        let (id, cylinder) = wall;
        let sag = cylinder.radius
            * (1.0 - (PI / divisions_in(cylinder.radius, tolerance, span) as f64).cos());
        let vertices = body
            .vertex_ids()
            .map(|id| body.vertex(id))
            .filter_map(|vertex| {
                let on = bears(body, vertex, wall);
                let inside = cylinder.distance(vertex.point);
                (on || (-2.0 * sag..0.0).contains(&inside)).then_some((vertex.point, on))
            });
        let samples = inside
            .get(id)
            .into_iter()
            .flatten()
            .map(|point| (*point, false));
        for (point, on) in vertices.chain(samples) {
            let flat = flat_from(cylinder, point);
            if flat.length() > 0.0 {
                own.entry(*id).or_default().push(flat.normalize());
                through
                    .pinned
                    .entry(*id)
                    .or_default()
                    .push(flat.normalize());
                if on {
                    through.anchors.entry(*id).or_default().push(point);
                } else {
                    through
                        .beneath
                        .entry(*id)
                        .or_default()
                        .push(flat.normalize());
                }
            }
        }
    }
    through
}

/// The way from a cylinder's axis to `point`, square to the axis.
pub(super) fn flat_from(cylinder: &Cylinder, point: DVec3) -> DVec3 {
    let from = point - cylinder.origin;
    from - cylinder.axis * cylinder.axis.dot(from)
}

/// Marks the contact of each elliptic wall of the body as its own (#571),
/// with the rays through the samples of other curves standing just inside
/// it: its rims and the curves it meets another wall along take a sample
/// at each, as a cylinder's circles take a ray.
pub(super) fn ovals(
    body: &Body,
    inside: &BTreeMap<SurfaceId, Vec<DVec3>>,
    contacts: &mut BTreeMap<SurfaceId, Contact>,
) {
    for (id, points) in inside {
        if let Some(Surface::Elliptic(oval)) = body.surfaces.get(id.0 as usize) {
            let contact = contacts.entry(*id).or_default();
            contact
                .rays
                .extend(points.iter().map(|point| *point - oval.origin));
        }
    }
    for (id, contact) in contacts.iter_mut() {
        if let Some(Surface::Elliptic(oval)) = body.surfaces.get(id.0 as usize) {
            contact.oval = Some(*oval);
        }
    }
}
