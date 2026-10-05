//! The curve two perpendicular cylinders meet along, against a surface:
//! numeric, where lines and circles are closed forms.
//!
//! The distance to the surface is sampled round the component. Its extrema
//! are where its slope changes sign, found by halving; one within the
//! tolerance of the surface is a touch. Between two extrema the distance is
//! monotone, so a change of sign there is one crossing, found by halving. The
//! sampling is fixed, so the same curve gives the same bits; what it can miss
//! is two extrema closer than one step.

use std::f64::consts::TAU;

use glam::DVec3;

use super::Solved;
use crate::brep::curve::Meet;
use crate::brep::relation::Relation;
use crate::brep::relation::cylinders::cylinders;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;

/// Samples per turn of the parameter.
const SAMPLES: usize = 512;
/// Enough halvings to bring any step down to the rounding of the parameter.
const HALVINGS: usize = 64;

pub(super) fn meet_and_surface(meet: &Meet, surface: &Surface, scale: Scale) -> Solved {
    let eps = scale.eps();
    if let Surface::Cylinder(cylinder) = surface
        && [meet.first, meet.second]
            .iter()
            .any(|own| matches!(cylinders(own, cylinder, scale), Relation::Same { .. }))
    {
        return Solved::Along;
    }
    let Some(period) = meet.period() else {
        return Solved::At(Vec::new());
    };
    let count = SAMPLES * (period / TAU).round() as usize;
    let at = |step: usize| period * step as f64 / count as f64;
    let distance = |t: f64| surface.distance(meet.point(t));
    let slope = |t: f64| gradient(surface, meet.point(t)).dot(meet.derivative(t));

    if (0..count).all(|step| distance(at(step)).abs() <= eps) {
        return Solved::Along;
    }
    let slopes: Vec<f64> = (0..count).map(|step| slope(at(step))).collect();
    let mut extrema = Vec::new();
    for step in 0..count {
        let (here, next) = (slopes[step], slopes[(step + 1) % count]);
        if here == 0.0 {
            extrema.push(at(step));
        } else if here * next < 0.0 {
            extrema.push(halved(&slope, at(step), at(step + 1)));
        }
    }
    if extrema.is_empty() {
        return Solved::At(Vec::new());
    }

    let within = |t: f64| {
        let t = t.rem_euclid(period);
        if t < period { t } else { 0.0 }
    };
    let touching = |t: f64| distance(t).abs() <= eps;
    let mut found: Vec<(f64, bool)> = extrema
        .iter()
        .filter(|&&t| touching(t))
        .map(|&t| (within(t), true))
        .collect();
    for (index, &from) in extrema.iter().enumerate() {
        let to = match extrema.get(index + 1) {
            Some(&next) => next,
            None => extrema[0] + period,
        };
        if touching(from) || touching(to) || distance(from) * distance(to) > 0.0 {
            continue;
        }
        found.push((within(halved(&distance, from, to)), false));
    }
    Solved::At(found)
}

/// Which way the distance to the surface grows.
fn gradient(surface: &Surface, point: DVec3) -> DVec3 {
    match surface {
        Surface::Plane(plane) => plane.normal,
        Surface::Cylinder(cylinder) => {
            let from = point - cylinder.origin;
            (from - cylinder.axis * cylinder.axis.dot(from)).normalize_or_zero()
        }
    }
}

/// Where a function changing sign between `from` and `to` vanishes.
fn halved(function: &impl Fn(f64) -> f64, mut from: f64, mut to: f64) -> f64 {
    let rising = function(from) < 0.0;
    for _ in 0..HALVINGS {
        let middle = (from + to) / 2.0;
        if (function(middle) < 0.0) == rising {
            from = middle;
        } else {
            to = middle;
        }
    }
    (from + to) / 2.0
}
