//! Which side of a line or of an edge a point stands on, and which way a
//! loop turns, decided exactly (#585): Shewchuk's adaptive tests, the plain
//! `f64` answer where its error bound allows it and the exact sum of every
//! rounding otherwise. Every point stays in `f64`; what changes is that a
//! sign asked twice is answered the same twice, a hair from the line
//! included. `docs/exact-kernel.md`, "Which side a point stands on", says
//! which of the kernel's decisions are asked here and why the others are not.

mod expansion;

use std::cmp::Ordering;

use glam::{DVec2, DVec3};

use expansion::Expansion;

/// How the path from `a` through `b` turns to reach `c`: `Greater` when `c`
/// stands to the left of the line from `a` to `b`, `Equal` on it.
pub(crate) fn turn(a: DVec2, b: DVec2, c: DVec2) -> Ordering {
    let left = (a.x - c.x) * (b.y - c.y);
    let right = (a.y - c.y) * (b.x - c.x);
    let rough = left - right;
    let alike = (left > 0.0 && right > 0.0) || (left < 0.0 && right < 0.0);
    if !alike || rough.abs() > TURN * (left.abs() + right.abs()) {
        return sign(rough);
    }
    let [ax, ay, bx, by] = [
        Expansion::difference(a.x, c.x),
        Expansion::difference(a.y, c.y),
        Expansion::difference(b.x, c.x),
        Expansion::difference(b.y, c.y),
    ];
    ax.times(&by).minus(&ay.times(&bx)).sign()
}

/// Shewchuk's bound on the rounding of the product form above: past it, the
/// sign of the rough value is the sign of the exact one. Two products of
/// opposite signs, or one of them nought, need none: each difference keeps
/// its sign, and so does each product — three samples along a rim of a
/// chart are told on a line at once.
const TURN: f64 = 3.330_669_073_875_471_6e-16;

/// Which way a loop through `points`, closed from the last back to the
/// first, turns: `Greater` anticlockwise, `Equal` when it bounds no area —
/// the sign of the shoelace over its corners. The rough shoelace rounds
/// each product, each difference and each of the additions once, some
/// `n + 1` units of the last place of the sum over absolute values at most;
/// four times that is allowed.
pub(crate) fn rotation(points: &[DVec2]) -> Ordering {
    let pairs = || points.iter().zip(points.iter().cycle().skip(1));
    let (rough, size) = pairs().fold((0.0, 0.0), |(sum, size), (from, to)| {
        let (left, right) = (from.x * to.y, from.y * to.x);
        (sum + (left - right), size + left.abs() + right.abs())
    });
    if rough.abs() > (2 * points.len() + 2) as f64 * f64::EPSILON * size {
        return sign(rough);
    }
    let mut sum = Expansion::default();
    for (from, to) in pairs() {
        sum.add_product(from.x, to.y);
        sum.add_product(-from.y, to.x);
    }
    sum.sign()
}

/// Whether `c` and `d` stand on one side of the line through `a` and `b`:
/// `Greater` on the same side, `Less` on opposite sides, `Equal` where
/// either stands on the line — the sign of
/// `((b - a) × (c - a)) · ((b - a) × (d - a))`. For two points off one plane
/// with the line it is the sign of the cosine of the angle between the
/// half-planes holding them, so two a third of a turn apart read `Less`.
pub(crate) fn beside(a: DVec3, b: DVec3, c: DVec3, d: DVec3) -> Ordering {
    let [along, mine, theirs] = [b - a, c - a, d - a];
    let rough = along.cross(mine).dot(along.cross(theirs));
    let size = |one: DVec3, other: DVec3| {
        let (one, other) = (one.abs(), other.abs());
        DVec3::new(
            one.y * other.z + one.z * other.y,
            one.z * other.x + one.x * other.z,
            one.x * other.y + one.y * other.x,
        )
    };
    if rough.abs() > BESIDE * size(along, mine).dot(size(along, theirs)) {
        return sign(rough);
    }
    let parts = |point: DVec3| [0, 1, 2].map(|axis| Expansion::difference(point[axis], a[axis]));
    let [along, mine, theirs] = [b, c, d].map(parts);
    let cross = |one: &[Expansion; 3], other: &[Expansion; 3]| {
        [(1, 2), (2, 0), (0, 1)].map(|(first, second)| {
            one[first]
                .times(&other[second])
                .minus(&one[second].times(&other[first]))
        })
    };
    let (one, other) = (cross(&along, &mine), cross(&along, &theirs));
    (0..3)
        .fold(Expansion::default(), |sum, axis| {
            sum.plus(&one[axis].times(&other[axis]))
        })
        .sign()
}

/// A bound on the rounding of the rough form above, as a share of the same
/// form taken over absolute values: the differences, the two cross
/// products and their dot round it by some eleven units of the last place
/// at most, and this is thirty-two.
const BESIDE: f64 = 16.0 * f64::EPSILON;

/// The sign of a value, nought either way round: a product of nought by a
/// negative number is `-0.0`.
fn sign(value: f64) -> Ordering {
    value.partial_cmp(&0.0).unwrap_or(Ordering::Equal)
}

#[cfg(test)]
mod tests;
