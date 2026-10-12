//! The normals an ellipse sends through a point of its plane: where the
//! distance from the point to the ellipse turns, so that along each stretch
//! between two of them the distance only grows or only falls.
//!
//! In the ellipse's own frame, half-axes `a ≥ b`, the foot of a normal
//! through `q` is `(a² q₁/(a² + t), b² q₂/(b² + t))` for each root `t` of
//! `(a q₁/(a² + t))² + (b q₂/(b² + t))² = 1` (Eberly, "Distance from a point
//! to an ellipse", 2013). With `m = (b² + t)/b²`, `A = a q₁/b²`, `B = q₂/b`
//! and `E = (a² − b²)/b²`, the foot's eccentric angle has
//!
//! ```text
//! cos θ = A/(E + m),   sin θ = B/m,   Φ(m) = cos² + sin² − 1 = 0.
//! ```
//!
//! Φ is convex wherever it is defined, so it has one root past the pole at
//! nought, where it falls — the nearest foot — one before the pole at `−E`,
//! where it rises — the farthest — and none, one or two between them, where
//! its least value is `(|A|^⅔ + |B|^⅔)³/E² − 1`, in closed form: below
//! nought where `q` stands inside the ellipse's evolute. Each root is found
//! by halving on the stretch it is the one root of, never by sampling, and
//! read off the distance to the nearer pole, which keeps its digits there. A
//! point on an axis loses a pole: the vertices on that axis are feet, and so
//! are the two places square to it the other pole's root gives, where they
//! stand on the ellipse.

use std::f64::consts::{FRAC_PI_2, PI};

use glam::DVec2;

/// Halvings of a bracket: past the fifty-three bits of a double the midpoint
/// stands on an end, and the search stops there.
const HALVINGS: usize = 1100;

/// The eccentric angles, in `(−π, π]` and in order, of the feet of the
/// normals through `q` of the ellipse of half-axes `major ≥ minor > 0` about
/// the origin, its major one along the first axis: two or four.
pub(in crate::brep) fn feet(major: f64, minor: f64, q: DVec2) -> Vec<f64> {
    let big = major * q.x / (minor * minor);
    let small = q.y / minor;
    let spread = (major - minor) * (major + minor) / (minor * minor);
    let mut angles = if spread <= 0.0 {
        if q == DVec2::ZERO {
            vec![0.0, FRAC_PI_2, PI, -FRAC_PI_2]
        } else {
            let toward = q.y.atan2(q.x);
            vec![toward, toward + PI]
        }
    } else if big == 0.0 && small == 0.0 {
        vec![0.0, FRAC_PI_2, PI, -FRAC_PI_2]
    } else if big == 0.0 {
        let mut found = vec![FRAC_PI_2, -FRAC_PI_2];
        if small.abs() < spread {
            let sin = -small / spread;
            let cos = ((1.0 - sin) * (1.0 + sin)).sqrt();
            found.extend([sin.atan2(cos), sin.atan2(-cos)]);
        }
        found
    } else if small == 0.0 {
        let mut found = vec![0.0, PI];
        if big.abs() < spread {
            let cos = big / spread;
            let sin = ((1.0 - cos) * (1.0 + cos)).sqrt();
            found.extend([sin.atan2(cos), (-sin).atan2(cos)]);
        }
        found
    } else {
        general(big, small, spread)
    };
    for angle in &mut angles {
        if *angle <= -PI {
            *angle += 2.0 * PI;
        } else if *angle > PI {
            *angle -= 2.0 * PI;
        }
    }
    angles.sort_by(f64::total_cmp);
    angles
}

/// Off both axes: the nearest foot, the farthest, and the two the stretch
/// between the poles holds where `q` stands inside the evolute.
fn general(big: f64, small: f64, spread: f64) -> Vec<f64> {
    let reach = big.hypot(small);
    let near = halved([small.abs(), reach], |m| {
        (big / (spread + m)).powi(2) + (small / m).powi(2) - 1.0
    });
    let far = halved([big.abs(), reach], |w| {
        (big / w).powi(2) + (small / (spread + w)).powi(2) - 1.0
    });
    let mut found = vec![
        (small / near).atan2(big / (spread + near)),
        (-small / (spread + far)).atan2(-big / far),
    ];
    let ratio = (small / big).powi(2).cbrt();
    let [left, right] = [spread / (1.0 + ratio), spread * ratio / (1.0 + ratio)];
    let least = (big / left).powi(2) + (small / right).powi(2) - 1.0;
    if least < 0.0 {
        let from_left = halved([big.abs(), left], |p| {
            (big / p).powi(2) + (small / (spread - p)).powi(2) - 1.0
        });
        let from_right = halved([small.abs(), right], |n| {
            (big / (spread - n)).powi(2) + (small / n).powi(2) - 1.0
        });
        found.push((-small / (spread - from_left)).atan2(big / from_left));
        found.push((-small / from_right).atan2(big / (spread - from_right)));
    }
    found
}

/// The root of `falling`, which is no less than nought at the low end of
/// `bracket` and no more at the high end, by halving.
fn halved([mut low, mut high]: [f64; 2], falling: impl Fn(f64) -> f64) -> f64 {
    let mut middle = (low + high) / 2.0;
    for _ in 0..HALVINGS {
        middle = (low + high) / 2.0;
        if middle <= low || middle >= high {
            break;
        }
        let value = falling(middle);
        if value > 0.0 {
            low = middle;
        } else if value < 0.0 {
            high = middle;
        } else {
            break;
        }
    }
    middle
}

#[cfg(test)]
mod tests;
