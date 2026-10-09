//! The curve a plane cuts a cylinder or a cone along at a slant: an ellipse, a
//! parabola or a branch of a hyperbola, held by one formula anchored at a
//! point of it,
//!
//! ```text
//! p(τ) = A + T·τ·s(kτ²) + N·τ²·c(kτ²)
//! ```
//!
//! `s` and `c` the Stumpff functions, `T` the unit tangent at `A`, `N` the
//! second derivative there. The curve is the solution of `p''' = −k·p'`, so
//! the sign of `k` alone tells the ellipse from the hyperbola, and the
//! formula runs through the parabola with no case of its own. It is built
//! from differences that vanish with `τ` — `2 sin²(√z/2)` rather than
//! `1 − cos √z` — so a point is held to rounding times its distance from the
//! anchor along the curve, wherever the centre stands: a plane a hair off
//! parallel to a bore's axis cuts an ellipse whose centre and far vertex stand
//! a billion reaches away. How it is built is `built.rs`; how it is seen on
//! its plane and on its wall, `seen.rs`.

mod built;
mod seen;

use std::f64::consts::TAU;

use glam::DVec3;
use serde::{Deserialize, Serialize};

use super::surface::{Cone, Cylinder, Plane};

/// The wall a plane cuts a conic on.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Round {
    Cylinder(Cylinder),
    Cone(Cone),
}

/// Which stretch of the cut the curve is: all of it, or the arm on one side
/// of the plane of symmetry, between two vertices standing beyond
/// [`LONG`] reaches, on the side where the wall's angle less the angle of the
/// plane's normal has the sign `side`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arm {
    Whole,
    Split { side: i8 },
}

/// Which surface of its own a conic is seen on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum On {
    Plane,
    Wall,
}

/// `p(τ) = anchor + tangent·τ·s(kτ²) + bend·τ²·c(kτ²)`, cut by `plane` on
/// `round`, `τ` growing with the round's angle.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Conic {
    pub plane: Plane,
    pub round: Round,
    /// A point of the cut, within [`LONG`] reaches of the world's origin.
    pub anchor: DVec3,
    /// Of unit length.
    pub tangent: DVec3,
    /// The second derivative at the anchor.
    pub bend: DVec3,
    /// Above nought an ellipse, nought a parabola, below a hyperbola.
    pub k: f64,
    pub arm: Arm,
}

/// How far, in reaches, the anchor of a conic may stand from the world's
/// origin: one parameter holds two places a hundred thousand reaches apart
/// along a curve to a tenth of the tolerance, and a vertex further than this
/// splits the curve into arms anchored in the box.
pub const LONG: f64 = 1e4;

/// Below this `|z|`, the Stumpff functions are summed from their series.
const SERIES: f64 = 0.1;

/// Terms of the series: the twelfth is below rounding at `|z| = 0.1`.
const TERMS: u32 = 12;

/// Newton's steps polishing a parameter read off a point: fixed, so that the
/// same point gives the same bits.
const NEWTON_STEPS: usize = 4;

/// `s(z) = sin √z / √z`, `c(z) = 2 sin²(√z/2) / z` and `C(z) = cos √z`, and
/// their continuations through `sinh` and `cosh` below nought.
fn stumpff(z: f64) -> [f64; 3] {
    if z.abs() < SERIES {
        return [1, 2, 0].map(|first| series(z, first));
    }
    if z > 0.0 {
        let root = z.sqrt();
        [
            root.sin() / root,
            2.0 * (root / 2.0).sin().powi(2) / z,
            root.cos(),
        ]
    } else {
        let root = (-z).sqrt();
        [
            root.sinh() / root,
            -2.0 * (root / 2.0).sinh().powi(2) / z,
            root.cosh(),
        ]
    }
}

/// `Σ (−z)ʲ / (2j + first)!`, nested from its last term.
fn series(z: f64, first: u32) -> f64 {
    let mut sum = 1.0;
    for term in (1..TERMS).rev() {
        let [low, high] = [2 * term + first - 1, 2 * term + first].map(f64::from);
        sum = 1.0 - z / (low * high) * sum;
    }
    if first == 2 { sum / 2.0 } else { sum }
}

impl Conic {
    pub fn point(&self, t: f64) -> DVec3 {
        let [s, c, _] = stumpff(self.k * t * t);
        self.anchor + self.tangent * (t * s) + self.bend * (t * t * c)
    }

    pub fn derivative(&self, t: f64) -> DVec3 {
        let [s, _, cosine] = stumpff(self.k * t * t);
        self.tangent * cosine + self.bend * (t * s)
    }

    /// The second derivative.
    pub fn bending(&self, t: f64) -> DVec3 {
        let [s, _, cosine] = stumpff(self.k * t * t);
        self.bend * cosine - self.tangent * (self.k * t * s)
    }

    /// A whole ellipse closes; an arm, a parabola or a hyperbola never does.
    pub fn period(&self) -> Option<f64> {
        (self.arm == Arm::Whole && self.k > 0.0).then(|| TAU / self.k.sqrt())
    }

    /// The parameter of the point of the curve nearest `point`: read off the
    /// wall's angle on a cylinder, where the eccentric angle is that angle,
    /// or off the point's place along the tangent and the bend on a cone,
    /// then polished by Newton's steps; in the first period on a whole
    /// ellipse.
    pub fn parameter(&self, point: DVec3) -> f64 {
        let mut t = self.seed(point);
        for _ in 0..NEWTON_STEPS {
            let speed = self.derivative(t);
            let step = (self.point(t) - point).dot(speed) / speed.length_squared();
            if !step.is_finite() {
                break;
            }
            t -= step;
            if step.abs() <= f64::EPSILON * t.abs() {
                break;
            }
        }
        match self.period() {
            Some(period) => {
                let wrapped = t - period * (t / period).round();
                if wrapped <= -period / 2.0 {
                    wrapped + period
                } else {
                    wrapped
                }
            }
            None => t,
        }
    }

    /// Where Newton starts: on a cylinder, the eccentric angle; on a cone, the
    /// nearer of the parameter the point's place along the tangent and the
    /// bend gives, and of its place along the tangent alone, which is all
    /// that holds on an arm nearly straight across the box, its bend all but
    /// along its tangent.
    fn seed(&self, point: DVec3) -> f64 {
        if let Round::Cylinder(cylinder) = self.round {
            let turned = cylinder.parameters(point).x - cylinder.parameters(self.anchor).x;
            let principal = turned - TAU * (turned / TAU).round();
            return principal / self.k.sqrt();
        }
        let away = |t: &f64| {
            let off = self.point(*t).distance(point);
            if off.is_finite() { off } else { f64::INFINITY }
        };
        [
            self.read_across(point),
            (point - self.anchor).dot(self.tangent),
        ]
        .into_iter()
        .min_by(|one, other| away(one).total_cmp(&away(other)))
        .unwrap_or(0.0)
    }

    /// The parameter read off where `point` stands along the tangent and the
    /// bend from the anchor, `σ` and `γ`: `(sin, 1 − cos)` of the eccentric
    /// angle on an ellipse, its hyperbolic sine on a hyperbola.
    fn read_across(&self, point: DVec3) -> f64 {
        let from = point - self.anchor;
        let along = self.bend.dot(self.tangent);
        let across = self.bend - self.tangent * along;
        let gamma = if across.length_squared() > 0.0 {
            from.dot(across) / across.length_squared()
        } else {
            0.0
        };
        let sigma = from.dot(self.tangent) - gamma * along;
        if self.k > 0.0 {
            let root = self.k.sqrt();
            (root * sigma).atan2(1.0 - self.k * gamma) / root
        } else if self.k < 0.0 {
            let root = (-self.k).sqrt();
            (root * sigma).asinh() / root
        } else {
            sigma
        }
    }

    /// The parameters strictly between `from` and `to`, in order, where the
    /// curve runs square to `direction`: `(w·T) C(kτ²) + (w·N) τ s(kτ²) = 0`,
    /// a tangent on an ellipse, a hyperbolic tangent on a hyperbola, linear on
    /// a parabola.
    pub fn turning(&self, direction: DVec3, from: f64, to: f64) -> Vec<f64> {
        let (low, high) = (from.min(to), from.max(to));
        let (along, bent) = (direction.dot(self.tangent), direction.dot(self.bend));
        let found: Vec<f64> = if self.k > 0.0 {
            let root = self.k.sqrt();
            let first = (-along).atan2(bent / root);
            let count = |at: f64| ((root * at - first) / std::f64::consts::PI).floor();
            let (start, end) = (count(low), count(high));
            if !(end - start).is_finite() || end - start > 64.0 {
                return Vec::new();
            }
            (start as i64..=end as i64)
                .map(|turn| (first + std::f64::consts::PI * turn as f64) / root)
                .collect()
        } else if self.k < 0.0 {
            let root = (-self.k).sqrt();
            let ratio = -along * root / bent;
            if ratio.abs() < 1.0 {
                vec![ratio.atanh() / root]
            } else {
                Vec::new()
            }
        } else if bent != 0.0 {
            vec![-along / bent]
        } else {
            Vec::new()
        };
        found
            .into_iter()
            .filter(|t| *t > low && *t < high)
            .collect()
    }

    /// A box holding the curve from `from` to `to`: its ends and where each
    /// coordinate turns back, grown by rounding.
    pub fn bounds(&self, from: f64, to: f64) -> [DVec3; 2] {
        let mut points = vec![self.point(from), self.point(to)];
        for axis in DVec3::AXES {
            points.extend(
                self.turning(axis, from, to)
                    .into_iter()
                    .map(|t| self.point(t)),
            );
        }
        let far = points
            .iter()
            .map(|point| point.distance(self.anchor))
            .fold(0.0, f64::max);
        let room = 16.0 * f64::EPSILON * (self.anchor.abs().max_element() + far);
        let [low, high] = points.iter().fold(
            [DVec3::INFINITY, DVec3::NEG_INFINITY],
            |[low, high], point| [low.min(*point), high.max(*point)],
        );
        [low - room, high + room]
    }
}

#[cfg(test)]
mod tests;
