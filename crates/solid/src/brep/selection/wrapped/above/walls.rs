//! Which side of a wall a parallel one stands on, an elliptic one among the
//! two (#571), read off the rulings their pair was decided to meet along.
//!
//! Between two consecutive rulings round the first wall the two stand one
//! above the other all along, whether those rulings cross or touch: a
//! crossing changes the side, and a touch stands for a crossing within its
//! band or for none. So the stretch of the first wall `place` stands on is
//! read where it is widest apart from its ends — at its middle or a quarter
//! of the way from either end, whichever of the three stands furthest from
//! the second wall. Of two walls, a point of the first inside the second
//! has the second's wall beyond it where the two face one way at `place`,
//! and behind it where they face each other, as for two circular walls.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use crate::brep::curve::Line;
use crate::brep::surface::Surface;

/// Whether `second` stands on the side of `first` its own normal points to,
/// all along the stretch of `first` between two of `rulings` that `place`
/// stands on; none where the two stand within `rounding` of each other
/// wherever the stretch is read.
pub(super) fn standing_above(
    first: &Surface,
    second: &Surface,
    rulings: &[Line],
    place: DVec3,
    rounding: f64,
) -> Option<bool> {
    let at = first.parameters(place);
    let mut angles: Vec<f64> = rulings
        .iter()
        .map(|line| first.parameters(line.origin).x)
        .collect();
    angles.sort_by(f64::total_cmp);
    let [from, to] = stretch(&angles, at.x)?;
    let gap = [0.25, 0.5, 0.75]
        .map(|share| {
            let theta = from + (to - from) * share;
            second.distance(first.point(DVec2::new(theta, at.y)))
        })
        .into_iter()
        .max_by(|one, other| one.abs().total_cmp(&other.abs()))?;
    if gap.abs() <= rounding {
        return None;
    }
    let facing = |surface: &Surface| surface.normal(surface.parameters(place));
    let agree = facing(first).dot(facing(second)) > 0.0;
    Some((gap < 0.0) == agree)
}

/// The stretch between two consecutive angles of `angles`, in order, that
/// `at` stands on, its end past its start: the whole turn from the one angle
/// there is.
fn stretch(angles: &[f64], at: f64) -> Option<[f64; 2]> {
    let first = *angles.first()?;
    let turned = |angle: f64| (angle - first).rem_euclid(TAU);
    let here = turned(at);
    let index = angles
        .iter()
        .rposition(|&angle| turned(angle) <= here)
        .unwrap_or(0);
    let from = first + turned(angles[index]);
    let to = angles
        .get(index + 1)
        .map_or(first + TAU, |&next| first + turned(next));
    Some([from, to])
}

#[cfg(test)]
mod tests;
