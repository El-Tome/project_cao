//! How far a point of a surface stands from a trace, measured on the surface:
//! straight in a plane's parameters, which are lengths; on a cylinder with its
//! angle unrolled at the radius and taken the nearest way round, which is a
//! length on the surface too since a cylinder unrolls flat; on a cone with its
//! angle unrolled at the point's own distance from the axis, nought at the
//! apex and never less, which is a length across a ruling to first order and
//! exact along one; on an elliptic wall with its angle unrolled at the speed
//! the wall's section runs at the point's angle, a length across a ruling to
//! first order.

use std::f64::consts::TAU;

use glam::DVec2;

use crate::brep::surface::{Cone, EllipticCylinder, Sphere, Torus};
use crate::brep::trace::Trace;

/// How a surface's parameters unroll into lengths: on a sphere or a torus
/// the angle about the axis at the point's own distance from it, and the
/// angle along the meridian at the tube's radius (#576).
#[derive(Clone, Copy, Debug)]
pub(super) enum Unrolling {
    /// A plane's, which are lengths already.
    Flat,
    /// A cylinder's of this radius.
    Round(f64),
    Cone(Cone),
    Elliptic(EllipticCylinder),
    Sphere(Sphere),
    Torus(Torus),
}

impl Unrolling {
    /// Whether the first parameter is an angle, read modulo a turn.
    pub(super) fn periodic(self) -> bool {
        !matches!(self, Unrolling::Flat)
    }

    /// The radius the angle unrolls at, at `at`; none on a plane.
    fn radius_at(self, at: DVec2) -> Option<f64> {
        match self {
            Unrolling::Flat => None,
            Unrolling::Round(radius) => Some(radius),
            Unrolling::Cone(cone) => Some(cone.radius_at(at.y).max(0.0)),
            Unrolling::Elliptic(wall) => Some(wall.speed(at.x)),
            Unrolling::Sphere(sphere) => Some((sphere.radius * at.y.cos()).max(0.0)),
            Unrolling::Torus(torus) => Some((torus.center.y + torus.radius * at.y.cos()).max(0.0)),
        }
    }

    /// What the second parameter unrolls at: one where it is a length
    /// already, the tube's radius where it is the angle along the meridian.
    fn along(self) -> f64 {
        match self {
            Unrolling::Flat | Unrolling::Round(_) | Unrolling::Cone(_) | Unrolling::Elliptic(_) => {
                1.0
            }
            Unrolling::Sphere(sphere) => sphere.radius,
            Unrolling::Torus(torus) => torus.radius,
        }
    }
}

pub(super) fn distance(trace: &Trace, at: DVec2, unrolling: Unrolling) -> f64 {
    let radius = unrolling.radius_at(at);
    let along = unrolling.along();
    match (*trace, radius) {
        (Trace::Segment { from, to }, None) => to_segment(at, from, to),
        (Trace::Segment { from, to }, Some(radius)) => {
            let unrolled = |point: DVec2| DVec2::new(point.x * radius, point.y * along);
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
        _ => sampled(trace, at, radius.map(|radius| (radius, along))),
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
/// between that sample's neighbours. A whole rim of an elliptic wall ends
/// where it starts, and a place beside that point stands as near the
/// sample at either end: closed in on beside the first alone, the place
/// past the start the rim reaches only at its end was read a ten-thousandth
/// of a turn off it (campaign seed 5571458568). Beside an end, both ends
/// are closed in on. `scales` are what the angle and the second parameter
/// unroll at, where the first is an angle.
pub(super) fn sampled(trace: &Trace, at: DVec2, scales: Option<(f64, f64)>) -> f64 {
    const SAMPLES: usize = 64;
    let gap = |u: f64| {
        let mut gap = trace.at(u)[0] - at;
        if let Some((radius, along)) = scales {
            gap.x = (gap.x - TAU * (gap.x / TAU).round()) * radius;
            gap.y *= along;
        }
        gap.length()
    };
    let nearest = (0..=SAMPLES)
        .min_by(|one, other| {
            gap(*one as f64 / SAMPLES as f64).total_cmp(&gap(*other as f64 / SAMPLES as f64))
        })
        .unwrap_or(0);
    let closed_in = |sample: usize| {
        let mut low = (sample.max(1) - 1) as f64 / SAMPLES as f64;
        let mut high = ((sample + 1).min(SAMPLES)) as f64 / SAMPLES as f64;
        for _ in 0..200 {
            let (left, right) = (low + (high - low) / 3.0, high - (high - low) / 3.0);
            if gap(left) < gap(right) {
                high = right;
            } else {
                low = left;
            }
        }
        gap((low + high) / 2.0)
    };
    let other_end = match nearest {
        0 => Some(SAMPLES),
        SAMPLES => Some(0),
        _ => None,
    };
    other_end
        .map_or(f64::INFINITY, closed_in)
        .min(closed_in(nearest))
        .min(gap(0.0))
        .min(gap(1.0))
}
