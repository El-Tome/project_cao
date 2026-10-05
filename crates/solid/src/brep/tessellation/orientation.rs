//! Which side of a line a point stands on, decided exactly: the one question
//! the cutting of a face into triangles asks, and the one it cannot afford to
//! answer two ways.

use std::cmp::Ordering;

use glam::DVec2;

/// How the path from `a` through `b` turns to reach `c`: `Greater` when `c`
/// stands to the left of the line from `a` to `b`, `Equal` on it.
pub(super) fn turn(a: DVec2, b: DVec2, c: DVec2) -> Ordering {
    let left = (a.x - c.x) * (b.y - c.y);
    let right = (a.y - c.y) * (b.x - c.x);
    let rough = left - right;
    if rough.abs() > BOUND * (left.abs() + right.abs()) {
        return rough.total_cmp(&0.0);
    }
    exact(a, b, c)
}

/// Shewchuk's bound on the rounding of the product form above: past it, the
/// sign of the rough value is the sign of the exact one.
const BOUND: f64 = 3.330_669_073_875_471_6e-16;

/// The same determinant carried out with every rounding kept, as a sum of
/// parts none of which overlaps the next: its sign is the sign of its largest.
fn exact(a: DVec2, b: DVec2, c: DVec2) -> Ordering {
    let [ax, ay, bx, by] = [
        difference(a.x, c.x),
        difference(a.y, c.y),
        difference(b.x, c.x),
        difference(b.y, c.y),
    ];
    let mut sum = Vec::with_capacity(16);
    for (one, other, sign) in [(ax, by, 1.0), (ay, bx, -1.0)] {
        for first in one {
            for second in other {
                let (product, error) = product(first, second);
                grow(&mut sum, sign * product);
                grow(&mut sum, sign * error);
            }
        }
    }
    sum.last().copied().unwrap_or(0.0).total_cmp(&0.0)
}

fn difference(a: f64, b: f64) -> [f64; 2] {
    let rounded = a - b;
    let virtual_a = rounded + b;
    let virtual_b = virtual_a - rounded;
    [rounded, (a - virtual_a) + (virtual_b - b)]
}

fn product(a: f64, b: f64) -> (f64, f64) {
    let rounded = a * b;
    (rounded, a.mul_add(b, -rounded))
}

fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let rounded = a + b;
    let virtual_b = rounded - a;
    let virtual_a = rounded - virtual_b;
    (rounded, (a - virtual_a) + (b - virtual_b))
}

/// Adds `value` to an expansion kept from smallest part to largest, dropping
/// the parts that come out nought.
fn grow(parts: &mut Vec<f64>, value: f64) {
    let mut carried = value;
    let mut kept = 0;
    for index in 0..parts.len() {
        let (sum, error) = two_sum(carried, parts[index]);
        carried = sum;
        if error != 0.0 {
            parts[kept] = error;
            kept += 1;
        }
    }
    parts.truncate(kept);
    if carried != 0.0 {
        parts.push(carried);
    }
}

#[cfg(test)]
mod tests;
