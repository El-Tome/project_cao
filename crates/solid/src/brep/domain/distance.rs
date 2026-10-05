//! How far a point of a surface stands from a trace, measured on the surface:
//! straight in a plane's parameters, which are lengths; on a cylinder with its
//! angle unrolled at the radius and taken the nearest way round, which is a
//! length on the surface too since a cylinder unrolls flat.

use std::f64::consts::TAU;

use glam::DVec2;

use crate::brep::trace::Trace;

/// `radius` is the cylinder's the parameters are read on, none on a plane.
pub(super) fn distance(trace: &Trace, at: DVec2, radius: Option<f64>) -> f64 {
    match (*trace, radius) {
        (Trace::Segment { from, to }, None) => to_segment(at, from, to),
        (Trace::Segment { from, to }, Some(radius)) => {
            let unrolled = |point: DVec2| DVec2::new(point.x * radius, point.y);
            let turns = (((from.x + to.x) / 2.0 - at.x) / TAU).round();
            [-1.0, 0.0, 1.0]
                .map(|more| DVec2::new(at.x + (turns + more) * TAU, at.y))
                .map(|shifted| to_segment(unrolled(shifted), unrolled(from), unrolled(to)))
                .into_iter()
                .fold(f64::INFINITY, f64::min)
        }
        (
            Trace::Round {
                center,
                radius,
                start,
                sweep,
            },
            None,
        ) => {
            let away = at - center;
            let angle = away.y.atan2(away.x);
            let along = if sweep >= 0.0 {
                (angle - start).rem_euclid(TAU)
            } else {
                (start - angle).rem_euclid(TAU)
            };
            if along <= sweep.abs() {
                (away.length() - radius).abs()
            } else {
                at.distance(trace.start()).min(at.distance(trace.end()))
            }
        }
        _ => sampled(trace, at, radius),
    }
}

fn to_segment(at: DVec2, from: DVec2, to: DVec2) -> f64 {
    let along = to - from;
    let length = along.length_squared();
    let share = if length == 0.0 {
        0.0
    } else {
        ((at - from).dot(along) / length).clamp(0.0, 1.0)
    };
    at.distance(from + along * share)
}

/// The distance to any trace: the nearest of its samples, then closed in on
/// between that sample's neighbours.
pub(super) fn sampled(trace: &Trace, at: DVec2, radius: Option<f64>) -> f64 {
    const SAMPLES: usize = 64;
    let gap = |u: f64| {
        let mut gap = trace.at(u)[0] - at;
        if let Some(radius) = radius {
            gap.x = (gap.x - TAU * (gap.x / TAU).round()) * radius;
        }
        gap.length()
    };
    let nearest = (0..=SAMPLES)
        .min_by(|one, other| {
            gap(*one as f64 / SAMPLES as f64).total_cmp(&gap(*other as f64 / SAMPLES as f64))
        })
        .unwrap_or(0);
    let mut low = (nearest.max(1) - 1) as f64 / SAMPLES as f64;
    let mut high = ((nearest + 1).min(SAMPLES)) as f64 / SAMPLES as f64;
    for _ in 0..200 {
        let (left, right) = (low + (high - low) / 3.0, high - (high - low) / 3.0);
        if gap(left) < gap(right) {
            high = right;
        } else {
            low = left;
        }
    }
    gap((low + high) / 2.0).min(gap(0.0)).min(gap(1.0))
}
