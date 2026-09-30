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

use super::divisions;
use crate::brep::curve::Meet;
use crate::brep::topology::Edge;

/// How many times a stretch is halved at most: past this, a chord is shorter
/// than the rounding of the parameter along a curve of any reach.
const HALVINGS: u32 = 40;

/// How many equal parts a stretch is cut into to measure its chord from the
/// curve: the places between them are where the curve is measured.
const PROBES: usize = 8;

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
/// circles of its first cylinder and of its second take besides their grid.
pub(in crate::brep::tessellation) fn on_meet(
    meet: &Meet,
    edge: &Edge,
    tolerance: f64,
    eps: f64,
    rays: [&[f64]; 2],
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

    let mut places = vec![kept[0].0];
    for pair in kept.windows(2) {
        halved(meet, pair[0].0, pair[1].0, tolerance, HALVINGS, &mut places);
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

/// The parameters strictly between `from` and `to` a stretch is cut at, in
/// order, so that no chord between two of them stands further than
/// `tolerance` from the curve.
fn halved(meet: &Meet, from: f64, to: f64, tolerance: f64, left: u32, places: &mut Vec<f64>) {
    if left == 0 || straight(meet, from, to, tolerance) {
        return;
    }
    let middle = (from + to) / 2.0;
    halved(meet, from, middle, tolerance, left - 1, places);
    places.push(middle);
    halved(meet, middle, to, tolerance, left - 1, places);
}

/// Whether the chord of a stretch stands within `tolerance` of the curve,
/// measured on the curve itself at places spread along the stretch.
fn straight(meet: &Meet, from: f64, to: f64, tolerance: f64) -> bool {
    let (start, end) = (meet.point(from), meet.point(to));
    (1..PROBES).all(|part| {
        let at = from + (to - from) * part as f64 / PROBES as f64;
        from_chord(meet.point(at), start, end) <= tolerance
    })
}

/// How far `point` stands from the segment between `start` and `end`.
fn from_chord(point: DVec3, start: DVec3, end: DVec3) -> f64 {
    let along = end - start;
    let length = along.length_squared();
    let share = if length > 0.0 {
        ((point - start).dot(along) / length).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (point - (start + along * share)).length()
}
