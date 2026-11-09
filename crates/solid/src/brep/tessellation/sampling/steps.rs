//! How many equal steps a whole turn of a circle is cut into.

use std::f64::consts::PI;

use crate::brep::topology::Body;

const LEAST: usize = 16;
const MOST: usize = 1024;

/// How many equal steps a whole turn of a circle of `radius`, on a part
/// spanning `span`, is cut into, so that no chord of one step stands further
/// than `tolerance` from its arc.
///
/// A multiple of four, so that the quarters from `u` are samples: where the
/// planes of the origin touch a cylinder standing square to one of them. At
/// least sixteen, so that a coarse tolerance still draws a round; at most
/// [`most`], so that a fine one does not draw a million triangles. Read off
/// the circle and the part alone, so that every face on one circle stands on
/// the same samples.
pub(in crate::brep) fn divisions_in(radius: f64, tolerance: f64, span: f64) -> usize {
    let most = most(radius, span);
    match needed(radius, tolerance) {
        Some(needed) => ((needed / 4.0).ceil() * 4.0).clamp(LEAST as f64, most as f64) as usize,
        None => most,
    }
}

/// The most steps a turn of `radius` is cut into on a part spanning `span`:
/// a thousand and twenty-four for a circle no wider than the part, and as
/// many more as it is wider, so that the shortest chord is bounded by the
/// part rather than by a whole turn, and an arc of a circle far wider than
/// the part is still drawn within the tolerance. A whole circle of the part
/// fits in its box, so it is never wider than the part.
pub(super) fn most(radius: f64, span: f64) -> usize {
    let wider = (MOST as f64 * radius / span / 4.0).floor() * 4.0;
    if wider.is_finite() && wider > MOST as f64 {
        wider as usize
    } else {
        MOST
    }
}

/// Half the diagonal of the box `body` fits in: the radius of the widest
/// circle it can hold whole, wherever it stands and however it leans.
pub(super) fn span(body: &Body) -> f64 {
    body.bounds()
        .map_or(0.0, |(low, high)| (high - low).length() / 2.0)
}

pub(super) fn needed(radius: f64, tolerance: f64) -> Option<f64> {
    let ratio = tolerance / radius;
    if ratio.is_nan() || ratio <= 0.0 {
        return None;
    }
    let half_step = (1.0 - ratio.min(2.0)).acos();
    Some((PI / half_step).ceil())
}
