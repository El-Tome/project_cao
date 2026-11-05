//! A curve no STEP entity holds within the tolerance — the meet of two
//! walls, a conic whose centre stands too far for doubles — as a cubic
//! B-spline of Hermite pieces: the kernel's points and derivatives at the
//! knots, the knots the kernel's own parameter, each piece halved on its own
//! until it stands within the tolerance of the closed form.

use glam::DVec3;

use super::super::Unwritten;
use super::super::instances::{Instances, flag, real};
use super::reach;
use crate::brep::{Curve, Scale};

const SAMPLES: usize = 32;
const FIRST_PIECES: usize = 4;
const MOST_PIECES: usize = 1 << 14;

pub fn written(
    instances: &mut Instances,
    curve: &Curve,
    from: f64,
    to: f64,
) -> Result<usize, Unwritten> {
    let (low, high) = (from.min(to), from.max(to));
    let closed = curve
        .period()
        .is_some_and(|period| (high - low - period).abs() <= period * Scale::RELATIVE);
    let (_, tolerance) = reach(curve, low, high);
    let knots = knots(curve, low, high, tolerance).ok_or(Unwritten::Unfitted)?;
    let mut poles = vec![curve.point(low)];
    for pair in knots.windows(2) {
        let [_, leaving, arriving, end] = piece(curve, pair[0], pair[1]);
        poles.extend([leaving, arriving, end]);
    }
    if closed {
        let last = poles.len() - 1;
        poles[last] = poles[0];
    }
    let poles: Vec<String> = poles
        .into_iter()
        .map(|pole| format!("#{}", instances.point(pole)))
        .collect();
    let last = knots.len() - 1;
    let multiplicities: Vec<&str> = (0..=last)
        .map(|index| {
            if index == 0 || index == last {
                "4"
            } else {
                "3"
            }
        })
        .collect();
    let knots: Vec<String> = knots.into_iter().map(real).collect();
    Ok(instances.add(format!(
        "B_SPLINE_CURVE_WITH_KNOTS('',3,({}),.UNSPECIFIED.,{},.F.,({}),({}),.UNSPECIFIED.)",
        poles.join(","),
        flag(closed),
        multiplicities.join(","),
        knots.join(",")
    )))
}

/// The knots, every piece between two of them halved while it strays, or
/// nothing once the pieces would pass the cap.
fn knots(curve: &Curve, low: f64, high: f64, tolerance: f64) -> Option<Vec<f64>> {
    let mut knots: Vec<f64> = (0..FIRST_PIECES)
        .map(|index| low + (high - low) * index as f64 / FIRST_PIECES as f64)
        .chain([high])
        .collect();
    loop {
        let mut halved = vec![low];
        for pair in knots.windows(2) {
            if strays(curve, pair[0], pair[1]) > tolerance {
                halved.push((pair[0] + pair[1]) / 2.0);
            }
            halved.push(pair[1]);
        }
        if halved.len() == knots.len() {
            return Some(knots);
        }
        if halved.len() > MOST_PIECES + 1 {
            return None;
        }
        knots = halved;
    }
}

fn piece(curve: &Curve, start: f64, end: f64) -> [DVec3; 4] {
    let third = (end - start) / 3.0;
    let (first, last) = (curve.point(start), curve.point(end));
    [
        first,
        first + curve.derivative(start) * third,
        last - curve.derivative(end) * third,
        last,
    ]
}

fn strays(curve: &Curve, start: f64, end: f64) -> f64 {
    let [a, b, c, d] = piece(curve, start, end);
    (1..SAMPLES)
        .map(|sample| {
            let s = sample as f64 / SAMPLES as f64;
            let r = 1.0 - s;
            let on =
                a * (r * r * r) + b * (3.0 * r * r * s) + c * (3.0 * r * s * s) + d * (s * s * s);
            on.distance(curve.point(start + (end - start) * s))
        })
        .fold(0.0, f64::max)
}
