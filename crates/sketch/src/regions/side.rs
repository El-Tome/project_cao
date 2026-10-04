//! Which side of a line a corner lies on, read exactly.
//!
//! The rounded cross product gets the sign wrong when the corner is within a
//! few ulps of the line, and ear clipping trusts that sign twice per ear: once
//! to call a corner convex, once to call another corner outside the ear. One
//! wrong answer cuts an ear across the outline (#504).
//!
//! The product is first taken rounded, with the bound on its error; only when
//! the bound does not settle the sign is it taken again exactly, each of its
//! six products split into a value and its rounding error and the twelve
//! summed without loss (Shewchuk, *Adaptive precision floating-point
//! arithmetic and fast robust geometric predicates*, 1997).

use glam::DVec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Side {
    Left,
    On,
    Right,
}

/// The side of the line running `from` → `to` the point lies on, left being
/// counter-clockwise.
pub(super) fn side(point: DVec2, from: DVec2, to: DVec2) -> Side {
    let along = (to.x - from.x) * (point.y - from.y);
    let across = (to.y - from.y) * (point.x - from.x);
    let rounded = along - across;
    // Two products of opposite signs, or one of them zero, cannot be cancelled
    // into the wrong sign: only two of the same sign need the bound.
    if along * across <= 0.0 || rounded.abs() > ROUNDING_BOUND * (along.abs() + across.abs()) {
        return sign(rounded);
    }
    sign(exact(point, from, to))
}

/// Shewchuk's `ccwerrboundA`: past it, the rounded product has its sign right.
const ROUNDING_BOUND: f64 = (3.0 + 16.0 * f64::EPSILON / 2.0) * f64::EPSILON / 2.0;

fn sign(value: f64) -> Side {
    if value > 0.0 {
        Side::Left
    } else if value < 0.0 {
        Side::Right
    } else {
        Side::On
    }
}

/// The cross product expanded into six products of coordinates, summed with
/// no rounding: what comes back is the most significant part of the exact sum,
/// whose sign is the sum's.
fn exact(point: DVec2, from: DVec2, to: DVec2) -> f64 {
    let products = [
        (to.x, point.y),
        (-to.x, from.y),
        (-from.x, point.y),
        (-to.y, point.x),
        (to.y, from.x),
        (from.y, point.x),
    ];
    let mut sum: Vec<f64> = Vec::with_capacity(12);
    for (a, b) in products {
        let product = a * b;
        grow(&mut sum, product);
        grow(&mut sum, a.mul_add(b, -product));
    }
    sum.iter()
        .rev()
        .copied()
        .find(|part| *part != 0.0)
        .unwrap_or(0.0)
}

/// Adds `value` to a sum held as parts that do not overlap, smallest first,
/// without losing a bit.
fn grow(sum: &mut Vec<f64>, value: f64) {
    let mut carried = value;
    for part in sum.iter_mut() {
        let (total, lost) = two_sum(carried, *part);
        *part = lost;
        carried = total;
    }
    sum.push(carried);
}

fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let total = a + b;
    let b_kept = total - a;
    let a_kept = total - b_kept;
    (total, (a - a_kept) + (b - b_kept))
}

#[cfg(test)]
mod tests;
