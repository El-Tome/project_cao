//! The curve two walls meet along, which no STEP entity holds, as a cubic
//! B-spline: a Hermite piece between each pair of points of the kernel's
//! closed form, its derivatives the kernel's, the knots the kernel's own
//! parameter, halved until every piece stands within a quarter of the
//! kernel's widest tolerance of the closed form.

use glam::DVec3;

use super::super::instances::{Instances, flag, real};
use crate::brep::{Curve, Scale};

const SAMPLES: usize = 32;
const MOST_PIECES: usize = 1 << 14;

pub fn written(instances: &mut Instances, curve: &Curve, from: f64, to: f64) -> usize {
    let (low, high) = (from.min(to), from.max(to));
    let closed = curve
        .period()
        .is_some_and(|period| (high - low - period).abs() <= period * Scale::RELATIVE);
    let tolerance = Scale::HAIR * Scale::of(reach(curve, low, high)).eps() / 4.0;
    let mut pieces = 4;
    let mut poles = hermite(curve, low, high, pieces);
    while pieces < MOST_PIECES && strays(curve, low, high, &poles) > tolerance {
        pieces *= 2;
        poles = hermite(curve, low, high, pieces);
    }
    if closed {
        let last = poles.len() - 1;
        poles[last] = poles[0];
    }
    let poles: Vec<String> = poles
        .into_iter()
        .map(|pole| format!("#{}", instances.point(pole)))
        .collect();
    let knots: Vec<String> = (0..=pieces)
        .map(|index| real(knot(low, high, pieces, index)))
        .collect();
    let multiplicities: Vec<&str> = (0..=pieces)
        .map(|index| {
            if index == 0 || index == pieces {
                "4"
            } else {
                "3"
            }
        })
        .collect();
    instances.add(format!(
        "B_SPLINE_CURVE_WITH_KNOTS('',3,({}),.UNSPECIFIED.,{},.F.,({}),({}),.UNSPECIFIED.)",
        poles.join(","),
        flag(closed),
        multiplicities.join(","),
        knots.join(",")
    ))
}

fn knot(low: f64, high: f64, pieces: usize, index: usize) -> f64 {
    if index == pieces {
        high
    } else {
        low + (high - low) * index as f64 / pieces as f64
    }
}

fn reach(curve: &Curve, low: f64, high: f64) -> f64 {
    (0..=SAMPLES)
        .map(|index| curve.point(low + (high - low) * index as f64 / SAMPLES as f64))
        .map(|point| point.abs().max_element())
        .fold(0.0, f64::max)
}

fn hermite(curve: &Curve, low: f64, high: f64, pieces: usize) -> Vec<DVec3> {
    let mut poles = vec![curve.point(low)];
    for index in 0..pieces {
        let (start, end) = (
            knot(low, high, pieces, index),
            knot(low, high, pieces, index + 1),
        );
        let third = (end - start) / 3.0;
        let last = curve.point(end);
        poles.push(poles[poles.len() - 1] + curve.derivative(start) * third);
        poles.push(last - curve.derivative(end) * third);
        poles.push(last);
    }
    poles
}

fn strays(curve: &Curve, low: f64, high: f64, poles: &[DVec3]) -> f64 {
    let pieces = (poles.len() - 1) / 3;
    let mut worst: f64 = 0.0;
    for index in 0..pieces {
        let (start, end) = (
            knot(low, high, pieces, index),
            knot(low, high, pieces, index + 1),
        );
        let [a, b, c, d] = [0, 1, 2, 3].map(|pole| poles[3 * index + pole]);
        for sample in 1..SAMPLES {
            let s = sample as f64 / SAMPLES as f64;
            let r = 1.0 - s;
            let on =
                a * (r * r * r) + b * (3.0 * r * r * s) + c * (3.0 * r * s * s) + d * (s * s * s);
            worst = worst.max(on.distance(curve.point(start + (end - start) * s)));
        }
    }
    worst
}
