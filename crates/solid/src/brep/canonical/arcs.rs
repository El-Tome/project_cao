//! Decision 6: two arcs between the same two corners, on one surface, that
//! stay within the tolerance of each other along their whole length are one
//! arc. What is measured here is how far they part: exactly for lines and
//! circles, whose distance from each other turns only where a closed form
//! says, and on dense samples along the curve two cylinders meet along.

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::curves::distance;
use crate::brep::curve::Curve;

/// How many stretches an arc with no closed form is cut into to be measured.
const SAMPLES: usize = 64;

/// The largest distance a point of either stretch stands from the other's:
/// of `one` from `from` to `to`, of `other` over `along`. An end is
/// measured against the other's curve, the corner both stretches end at;
/// a point between, against the other's curve where it stands beside its
/// stretch and against the nearer end of it otherwise: two halves of two
/// circles a hair apart, one each side, run between the same two corners
/// within a hair of each other's circle all along.
pub(in crate::brep) fn parting(
    one: &Curve,
    [from, to]: [f64; 2],
    other: &Curve,
    along: [f64; 2],
) -> f64 {
    reach(one, [from, to], other, along).max(reach(other, along, one, [from, to]))
}

/// How far the stretch of `one` from `from` to `to` stands from the stretch
/// of `other` over `along` at its farthest.
fn reach(one: &Curve, [from, to]: [f64; 2], other: &Curve, along: [f64; 2]) -> f64 {
    turning(one, from, to, other)
        .into_iter()
        .chain([(from + to) / 2.0])
        .map(|at| {
            let point = one.point(at);
            if at == from || at == to {
                distance(other, point)
            } else {
                beside(other, along, point)
            }
        })
        .fold(0.0, f64::max)
}

/// How far a point stands from a stretch of a curve: from the curve where
/// its foot falls within the stretch, from the nearer end otherwise.
fn beside(curve: &Curve, [from, to]: [f64; 2], point: DVec3) -> f64 {
    let foot = curve.parameter(point);
    let foot = match curve.period() {
        Some(period) => foot - period * ((foot - from) / period).floor(),
        None => foot,
    };
    if foot >= from && foot <= to {
        distance(curve, point)
    } else {
        [from, to]
            .map(|end| curve.point(end).distance(point))
            .into_iter()
            .fold(f64::INFINITY, f64::min)
    }
}

/// The parameters of `one` between `from` and `to` where its distance from
/// `other` may be largest. From a line, a point of a line runs away linearly
/// and only the ends count. From a line in its plane, a point of a circle
/// runs away as a cosine of its angle, turning where the radius stands
/// square to the line; from a circle in its plane, where the radius runs
/// through the other centre. Anything else is sampled.
fn turning(one: &Curve, from: f64, to: f64, other: &Curve) -> Vec<f64> {
    let mut found = vec![from, to];
    let phase = match (one, other) {
        (Curve::Line(_), Curve::Line(_)) => return found,
        (Curve::Circle(circle), Curve::Line(line)) => {
            let square = circle.axis.cross(line.direction);
            Some(square.dot(circle.v).atan2(square.dot(circle.u)))
        }
        (Curve::Circle(circle), Curve::Circle(far)) => {
            let between = far.center - circle.center;
            Some(between.dot(circle.v).atan2(between.dot(circle.u)))
        }
        _ => None,
    };
    match phase {
        Some(phase) => {
            for turn in [phase, phase + PI] {
                let mut at = turn + TAU * ((from - turn) / TAU).ceil();
                while at <= to {
                    found.push(at);
                    at += TAU;
                }
            }
        }
        None => {
            found.extend((1..SAMPLES).map(|step| from + (to - from) * step as f64 / SAMPLES as f64))
        }
    }
    found
}
