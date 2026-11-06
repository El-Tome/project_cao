//! The lines two walls leaning a hair off parallel cross along, where the
//! two share their rays (#706).
//!
//! Such a line runs a hair round either wall from one end to the other: no
//! further than a ray drifts from one wall's axis to the other's. A grid
//! angle or a ray crossing it between its ends puts a sample a hair from
//! the line's own on each wall, and the faces either side, each laid out on
//! its own wall, order those places apart and are cut apart. It is sampled
//! at its ends alone; nothing is sampled in the wedge beside it
//! (`wedge.rs`); and the two faces either side are cut alike there
//! (`folds/lines.rs`). Every one of the three reads it here.

use std::f64::consts::{PI, TAU};

use crate::brep::curve::{Curve, Meet, Wall};
use crate::brep::tessellation::contact::Zones;
use crate::brep::topology::{Body, Edge, EdgeId};

/// The edges of `body` that are such lines.
pub(super) fn lines(body: &Body, zones: &Zones) -> Vec<EdgeId> {
    body.edge_ids()
        .filter(|id| {
            let edge = body.edge(*id);
            matches!(body.curve(edge.curve), Curve::Meet(meet) if is_line(meet, edge, zones))
        })
        .collect()
}

/// Whether `edge` along `meet` is such a line: two cylinders that share
/// their rays meet along it, between two ends, and on neither wall does
/// its angle turn back or run round further than the pair's drift.
pub(super) fn is_line(meet: &Meet, edge: &Edge, zones: &Zones) -> bool {
    let (Wall::Cylinder(first), Wall::Cylinder(second), Some(_), Some(period)) =
        (meet.first, meet.second, edge.ends, meet.period())
    else {
        return false;
    };
    let Some(drift) = zones
        .sharing()
        .find(|(pair, _)| *pair == [first, second] || *pair == [second, first])
        .map(|(_, drift)| drift)
    else {
        return false;
    };
    let (low, high) = (edge.from.min(edge.to), edge.from.max(edge.to));
    let ends = [low, high].map(|t| meet.point(t));
    [(first, true), (second, false)]
        .into_iter()
        .all(|(wall, on_first)| {
            let [from, to] = ends.map(|point| wall.parameters(point).x);
            let round = ((to - from + PI).rem_euclid(TAU) - PI).abs();
            round <= drift
                && meet.turns(on_first).into_iter().all(|t| {
                    let first = ((low - t) / period).ceil() as i64;
                    let last = ((high - t) / period).floor() as i64;
                    (first..=last)
                        .map(|turn| t + period * turn as f64)
                        .all(|t| t <= low || t >= high)
                })
        })
}
