//! Where an edge along the curve two perpendicular cylinders meet is sampled.
//!
//! At every parameter where it crosses a grid angle of either cylinder, so
//! that on both it has a point at every grid angle as a circle has; wherever
//! it turns back in the angle of either, so that between two of its points it
//! runs one way on both; at its ends. At the angle of every ray the circles
//! of either cylinder take, so that it stands on the same rays as a wall a
//! hair inside its own, whose chords would otherwise stand shallower than
//! its own between two of its points. Then, between two of those, wherever
//! the chord would stand further from the curve than the tolerance.

use std::f64::consts::TAU;

use glam::DVec3;

use super::chords::halved;
use super::divisions;
use super::ends::grazed;
use crate::brep::curve::Meet;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::{Body, Edge};

/// Why a parameter is a sample, the stronger reason first: of two places
/// closer than the kernel's tolerance, the one kept is the stronger.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Reason {
    End,
    Turn,
    Grid,
    Ray,
}

/// The points of an edge along a meet between its ends, in the way the edge
/// runs: none for a component the pair does not have. A whole loop's points
/// start at its start and do not come back to it. `rays` are the angles the
/// circles of its first cylinder and of its second take besides their grid;
/// a place on the grid or on a ray is kept only where `clear` says so.
pub(in crate::brep::tessellation) fn on_meet(
    meet: &Meet,
    edge: &Edge,
    tolerance: f64,
    eps: f64,
    rays: [&[f64]; 2],
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
    let mut marks = vec![(low, Reason::End), (high, Reason::End)];
    for (on_first, taken) in [true, false].into_iter().zip(rays) {
        let radius = if on_first {
            meet.first.radius
        } else {
            meet.second.radius
        };
        let steps = divisions(radius, tolerance);
        let turns = meet.turns(on_first).into_iter().map(|t| (t, Reason::Turn));
        let grid = (0..steps)
            .flat_map(|step| meet.at_angle(on_first, TAU * step as f64 / steps as f64))
            .map(|t| (t, Reason::Grid));
        let through = taken
            .iter()
            .flat_map(|angle| meet.at_angle(on_first, *angle))
            .map(|t| (t, Reason::Ray));
        for (t, reason) in turns.chain(grid).chain(through) {
            let first = ((low - t) / period).ceil() as i64;
            let last = ((high - t) / period).floor() as i64;
            marks.extend((first..=last).map(|turn| (t + period * turn as f64, reason)));
        }
    }
    marks.sort_by(|one, other| one.0.total_cmp(&other.0).then(one.1.cmp(&other.1)));

    let mut kept: Vec<(f64, Reason, DVec3)> = Vec::with_capacity(marks.len());
    for (t, reason) in marks {
        let point = meet.point(t);
        if reason >= Reason::Grid && !clear(point) {
            continue;
        }
        match kept.last_mut() {
            Some(last) if (last.2 - point).length() <= eps => {
                if reason < last.1 {
                    *last = (t, reason, point);
                }
            }
            _ => kept.push((t, reason, point)),
        }
    }
    if kept.len() < 2 {
        return Vec::new();
    }

    let point = |t: f64| meet.point(t);
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

/// Whether a place of an edge along the curve two cylinders meet stands
/// within `room` of a surface one of its ends lies on and the curve does
/// not, a step of the finer grid or less from that end: the curve grazing a
/// cap there, as a circle grazes the plane of an arc it meets, a sample a
/// hair from the end would lie on the cap's own arc ending at the same
/// vertex.
pub(in crate::brep::tessellation) fn grazing_ends<'a>(
    body: &'a Body,
    edge: &'a Edge,
    walls: &[Cylinder; 2],
    tolerance: f64,
) -> impl Fn(DVec3, f64) -> bool + 'a {
    let step = walls
        .iter()
        .map(|wall| TAU * wall.radius / divisions(wall.radius, tolerance) as f64)
        .fold(f64::INFINITY, f64::min);
    grazed(body, edge, &walls.map(Surface::Cylinder), step)
}
