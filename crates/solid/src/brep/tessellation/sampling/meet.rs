//! Where an edge along the curve two perpendicular walls meet is sampled.
//!
//! At every parameter where it crosses a grid angle of either wall, so
//! that on both it has a point at every grid angle as a circle has; wherever
//! it turns back in the angle of either, so that between two of its points it
//! runs one way on both; at its ends. At the angle of every ray the circles
//! of either cylinder take, so that it stands on the same rays as a wall a
//! hair inside its own, whose chords would otherwise stand shallower than
//! its own between two of its points. Then, between two of those, wherever
//! the chord would stand further from the curve than the tolerance. A line
//! two walls sharing their rays cross along is sampled at its ends alone
//! (`crossing.rs`).
//!
//! An elliptic wall (#571) is divided as a circle of its major half-axis,
//! as its rims are, its grid an eccentric angle's; it takes the angles its
//! rims take besides their grid (`elliptic.rs`) where a cylinder takes its
//! rays.

use std::f64::consts::TAU;

use std::collections::BTreeMap;

use glam::{DVec2, DVec3};

use super::chords::{halved, straight};
use super::cone::{Grid, Spun};
use super::elliptic::{on_the_wall, reached_by_planes};
use super::ends::grazed;
use super::{divisions_in, on_a_plane, taken, touching_planes};
use crate::brep::Scale;
use crate::brep::curve::{Meet, Wall};
use crate::brep::surface::Cone;
use crate::brep::tessellation::contact::{self, Contact, Zones};
use crate::brep::topology::{Body, Edge, EdgeId, SurfaceId};

mod outside;
mod twin;

/// Why a parameter is a sample, the stronger reason first: of two places
/// closer than the kernel's tolerance, the one kept is the stronger. Either
/// end takes every place within a hair ([`Scale::HAIR`]) of it: two a few
/// tolerances off it at a stationary angle, out of order, would zigzag
/// (122431270514586). Only an end, and never the other end: two walls a hair
/// apart have grid angles a few tolerances apart along the meet, and each
/// needs its own.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Reason {
    End,
    Turn,
    Grid,
    Ray,
}

/// The points of an edge along a meet between its ends, in the way the edge
/// runs: none for a component the pair does not have. A whole loop's points
/// start at its start and do not come back to it. Each wall's grid is cut
/// into `steps` steps a turn — a cone's its own grid's, which the faces on
/// it are laid out by. `rays` are the angles the sections of its first wall
/// and of its second take besides their grid; a place on the grid or on a
/// ray is kept only where `clear` says so — and not on the ray through an
/// end where the angle runs one way from that end, which crosses it at the
/// end alone: such a place is the end read with rounding (354000000005870).
/// An edge standing within the kernel's tolerance of the chord between its
/// ends — a meet run along a line two walls touch along — is a line to the
/// kernel, sampled at its ends alone as a line is (354000000008476).
fn on_meet_by(
    meet: &Meet,
    edge: &Edge,
    tolerance: f64,
    eps: f64,
    rays: [&[f64]; 2],
    steps: [usize; 2],
    clear: &dyn Fn(DVec3) -> bool,
) -> Vec<DVec3> {
    let Some(period) = meet.period() else {
        return Vec::new();
    };
    let whole = edge.ends.is_none();
    let (low, high) = if whole {
        (edge.from, edge.from + period)
    } else {
        (edge.from.min(edge.to), edge.from.max(edge.to))
    };
    let within = |t: f64, low: f64, high: f64| {
        let first = ((low - t) / period).ceil() as i64;
        let last = ((high - t) / period).floor() as i64;
        (first..=last).map(move |turn| t + period * turn as f64)
    };
    let mut marks = vec![(low, Reason::End), (high, Reason::End)];
    for ((on_first, taken), steps) in [true, false].into_iter().zip(rays).zip(steps) {
        let turns = meet.turns(on_first);
        let grid = (0..steps)
            .flat_map(|step| meet.at_angle(on_first, TAU * step as f64 / steps as f64))
            .map(|t| (t, Reason::Grid));
        let marked = turns.iter().map(|t| (*t, Reason::Turn)).chain(grid);
        marks.extend(marked.flat_map(|(t, reason)| within(t, low, high).map(move |t| (t, reason))));
        let wall = if on_first { meet.first } else { meet.second };
        for angle in taken {
            let ended =
                [low, high].map(|end| !whole && on_ray(&wall, *angle, meet.point(end), eps));
            let by_an_end = |t: f64| {
                [low, high].into_iter().zip(ended).any(|(end, on)| {
                    let (from, to) = (t.min(end), t.max(end));
                    on && turns
                        .iter()
                        .all(|turn| within(*turn, from, to).all(|t| t <= from || t >= to))
                })
            };
            let places = meet.at_angle(on_first, *angle).into_iter();
            let places = places.flat_map(|t| within(t, low, high));
            marks.extend(places.filter(|t| !by_an_end(*t)).map(|t| (t, Reason::Ray)));
        }
    }
    marks.sort_by(|one, other| one.0.total_cmp(&other.0).then(one.1.cmp(&other.1)));

    let reach = |one: Reason, other: Reason| {
        if (one == Reason::End) != (other == Reason::End) {
            eps * Scale::HAIR
        } else {
            eps
        }
    };
    let mut kept: Vec<(f64, Reason, DVec3)> = Vec::with_capacity(marks.len());
    for (t, reason) in marks {
        let point = meet.point(t);
        if reason >= Reason::Grid && !clear(point) {
            continue;
        }
        while reason == Reason::End
            && let Some(last) = kept.last()
            && last.1 != Reason::End
            && (last.2 - point).length() <= reach(last.1, reason)
        {
            kept.pop();
        }
        match kept.last_mut() {
            Some(last) if (last.2 - point).length() <= reach(last.1, reason) => {
                if reason < last.1 {
                    *last = (t, reason, point);
                }
            }
            _ => kept.push((t, reason, point)),
        }
    }
    let point = |t: f64| meet.point(t);
    if !whole && kept.len() > 2 && straight(&point, low, high, eps) {
        kept = vec![kept[0], kept[kept.len() - 1]];
    }
    if kept.len() < 2 {
        return Vec::new();
    }

    let mut places = vec![kept[0].0];
    for pair in kept.windows(2) {
        halved(&point, pair[0].0, pair[1].0, tolerance, &mut places);
        places.push(pair[1].0);
    }
    let inner = if whole {
        &places[..places.len() - 1]
    } else {
        &places[1..places.len() - 1]
    };
    let mut points: Vec<DVec3> = inner.iter().map(|t| meet.point(*t)).collect();
    if edge.to < edge.from {
        points.reverse();
    }
    points
}

/// The points of an edge along a meet between its ends, as [`on_meet_by`]
/// places them: on a cylinder of the body, on the rays its circles take in
/// `contacts`, and none where it all but lies on a wall facing it, nor on a
/// plane touching it but off the line they touch along, as a circle takes
/// no ray there; on an elliptic wall, at the angles its rims take besides
/// their grid; and between two places whose chord passes through the
/// triangles of a wall an end lies on ([`outside`]).
pub(super) fn of_edge(
    body: &Body,
    edge: EdgeId,
    meet: &Meet,
    around: Around,
    grids: &BTreeMap<SurfaceId, Grid>,
    fineness: (f64, f64),
) -> Vec<DVec3> {
    let points = alone(body, edge, meet, around, grids, fineness);
    let walls = outside::pierced(body, edge, meet, around, fineness, &points);
    let points = outside::kept_outside(body, edge, meet, &walls, fineness.0, points);
    twin::shared(body, edge, meet, around, grids, fineness, points)
}

/// The walls of the body, the pairs of them face to face, and what each
/// takes besides its grid.
type Around<'a> = (
    &'a [contact::Wall],
    &'a Zones<'a>,
    &'a BTreeMap<SurfaceId, Contact>,
);

/// [`of_edge`], the edge sampled on its own.
fn alone(
    body: &Body,
    id: EdgeId,
    meet: &Meet,
    (walls, zones, contacts): Around,
    grids: &BTreeMap<SurfaceId, Grid>,
    (tolerance, span): (f64, f64),
) -> Vec<DVec3> {
    let edge = body.edge(id);
    let own = contact::held(body, id, meet);
    if super::crossing::is_line(meet, own, edge, zones) {
        return Vec::new();
    }
    let eps = body.scale().eps();
    let carried = [meet.first, meet.second];
    let grid = |cone: &Cone| {
        grids
            .values()
            .find(|grid| matches!(grid.spun, Spun::Cone(spun) if spun == *cone))
    };
    let steps = carried.map(|wall| match wall {
        Wall::Cone(cone) => grid(&cone).map_or_else(
            || divisions_in(size(&wall, meet), tolerance, span),
            |grid| grid.steps,
        ),
        _ => divisions_in(size(&wall, meet), tolerance, span),
    });
    let rays = [0, 1].map(|rank| match carried[rank] {
        Wall::Cylinder(cylinder) => taken(&own[rank], &cylinder, walls, contacts),
        Wall::Elliptic(oval) => {
            let steps = divisions_in(oval.major, tolerance, span);
            let beneath = contacts
                .values()
                .filter(|contact| contact.oval == Some(oval))
                .flat_map(|contact| contact.rays.iter())
                .map(|way| (way.dot(oval.v) / oval.minor).atan2(way.dot(oval.u) / oval.major));
            on_the_wall(body, &oval, steps)
                .into_iter()
                .chain(reached_by_planes(body, &oval))
                .chain(beneath)
                .collect()
        }
        Wall::Cone(cone) => grid(&cone).map_or_else(Vec::new, |grid| grid.vertex_angles().to_vec()),
    });
    let planes = carried.map(|wall| match wall {
        Wall::Cylinder(cylinder) => {
            touching_planes(body, cylinder.origin, cylinder.axis, cylinder.radius)
        }
        Wall::Elliptic(_) | Wall::Cone(_) => Vec::new(),
    });
    let grazed = grazing_ends(
        body,
        edge,
        &own,
        [0, 1].map(|rank| (size(&carried[rank], meet), steps[rank])),
    );
    let clear = |point: DVec3| {
        !grazed(point, eps * contact::APART)
            && own
                .iter()
                .zip(&planes)
                .zip(own.iter().rev())
                .all(|((wall, planes), partner)| match wall {
                    Wall::Cylinder(cylinder) => {
                        !zones.crowded_off_the_meet(cylinder, partner, point)
                            && !planes.iter().any(|plane| {
                                on_a_plane(plane, cylinder.origin, cylinder.axis, point, eps)
                            })
                    }
                    Wall::Elliptic(_) | Wall::Cone(_) => true,
                })
    };
    on_meet_by(
        meet,
        edge,
        tolerance,
        eps,
        [&rays[0], &rays[1]],
        steps,
        &clear,
    )
}

/// Whether the ray at `angle` round `wall` stands within `eps` of the ray
/// through `end`, on the wall: the two are one ray to the wall's own
/// sections.
fn on_ray(wall: &Wall, angle: f64, end: DVec3, eps: f64) -> bool {
    let [at, there] = match wall {
        Wall::Cylinder(cylinder) => {
            let own = cylinder.parameters(end);
            [
                cylinder.point(DVec2::new(angle, own.y)),
                cylinder.point(own),
            ]
        }
        Wall::Elliptic(oval) => {
            let own = oval.parameters(end);
            [oval.point(DVec2::new(angle, own.y)), oval.point(own)]
        }
        Wall::Cone(_) => return false,
    };
    at.distance(there) <= eps
}

/// What a wall is divided as: a cylinder its radius, an elliptic wall the
/// circle of its major half-axis, a cone the widest the curve reaches on it.
pub(super) fn size(wall: &Wall, meet: &Meet) -> f64 {
    match wall {
        Wall::Cylinder(cylinder) => cylinder.radius,
        Wall::Elliptic(oval) => oval.major,
        Wall::Cone(cone) => (0..64)
            .map(|step| cone.off_axis(meet.point(TAU * step as f64 / 64.0)))
            .fold(0.0, f64::max),
    }
}

/// Whether a place of an edge along the curve two walls meet stands within
/// `room` of a surface one of its ends lies on and the curve does not, a
/// step of the finer grid or less from that end: the curve grazing a cap
/// there, as a circle grazes the plane of an arc it meets, a sample a hair
/// from the end would lie on the cap's own arc ending at the same vertex.
/// An elliptic wall's step is read where its grid steps shortest, at the
/// ends of its major axis.
fn grazing_ends<'a>(
    body: &'a Body,
    edge: &'a Edge,
    walls: &[Wall; 2],
    grids: [(f64, usize); 2],
) -> impl Fn(DVec3, f64) -> bool + 'a {
    let step = walls
        .iter()
        .zip(grids)
        .map(|(wall, (size, steps))| {
            let shortest = match wall {
                Wall::Cylinder(cylinder) => cylinder.radius,
                Wall::Elliptic(oval) => oval.minor,
                Wall::Cone(_) => size,
            };
            TAU * shortest / steps as f64
        })
        .fold(f64::INFINITY, f64::min);
    grazed(body, edge, &walls.map(|wall| wall.surface()), step)
}

/// [`on_meet_by`], each wall's grid cut for its own size on a part spanning
/// `span`.
#[cfg(test)]
pub(super) fn on_meet(
    meet: &Meet,
    edge: &Edge,
    (tolerance, span): (f64, f64),
    eps: f64,
    rays: [&[f64]; 2],
    clear: &dyn Fn(DVec3) -> bool,
) -> Vec<DVec3> {
    let steps =
        [meet.first, meet.second].map(|wall| divisions_in(size(&wall, meet), tolerance, span));
    on_meet_by(meet, edge, tolerance, eps, rays, steps, clear)
}
