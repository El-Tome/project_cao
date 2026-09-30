//! The curve where two perpendicular cylinders meet: exact, as four arcs over
//! the height across both axes.
//!
//! In the frame whose third axis is the first cylinder's, whose first axis is
//! the second's and whose origin is the first's, the first is `x² + y² = a²`
//! and the second `(y − d)² + (z − e)² = b²`. The curve spans `y` over
//! `[lo, hi] = [max(−a, d − b), min(a, d + b)]`, and its parameter `t` runs it
//! as `y = m − l cos t`, `m` the middle of the span and `l` its half-width.
//! With `c = cos(t/2)` and `s = sin(t/2)`, `hi − y = 2l c²` and
//! `y − lo = 2l s²`, so each root factors with no cancellation near an end:
//!
//! ```text
//! x² = (a − hi + 2l c²) (lo + a + 2l s²)
//! ```
//!
//! and likewise `(z − e)²`. Where a gap vanishes its factor is `√(2l) c` or
//! `√(2l) s` instead of a square root: the root changes sign through that end
//! rather than folding back, and the four arcs chain into curves analytic all
//! the way round. A component closes after `2π` when each root vanishes at
//! both ends of the span or at neither, after `4π` otherwise; a node, where
//! both roots vanish at once, is crossed along the branch going straight
//! through it.

mod meeting;
mod pair;
mod seen;

use std::f64::consts::TAU;

use glam::DVec3;

use super::curve::Meet;
pub(super) use meeting::goes_first;
pub use meeting::{Meeting, Node};
pub use pair::Configuration;
use pair::Pair;

/// Newton doubles the digits at each step: from the height's guess, a few
/// are enough, and the count is fixed so the same point gives the same bits.
const NEWTON_STEPS: usize = 8;

impl Meet {
    fn pair(&self) -> Pair {
        Pair::of(&self.first, &self.second)
    }

    fn local(&self, t: f64) -> (Pair, [DVec3; 3]) {
        let pair = self.pair();
        let local = pair.local(pair.signs(self.component), t);
        (pair, local)
    }

    pub fn point(&self, t: f64) -> DVec3 {
        let (pair, [point, ..]) = self.local(t);
        pair.world(point)
    }

    pub fn derivative(&self, t: f64) -> DVec3 {
        let (pair, [_, speed, _]) = self.local(t);
        pair.direction(speed)
    }

    /// Read off the height across both axes, which fixes the parameter up to
    /// the arc it lies on, then brought to the point by Newton from each arc,
    /// the nearest kept: near an end of the span the height says little, and
    /// near a node two branches pass the same height.
    pub fn parameter(&self, point: DVec3) -> f64 {
        let pair = self.pair();
        let signs = pair.signs(self.component);
        let period = pair.period();
        let local = pair.local_of(point);
        let half = (local.y - pair.low)
            .max(0.0)
            .sqrt()
            .atan2((pair.high - local.y).max(0.0).sqrt());
        let away = |t: f64| (pair.local(signs, t)[0] - local).length_squared();
        let refined = |mut t: f64| {
            for _ in 0..NEWTON_STEPS {
                let [at, speed, _] = pair.local(signs, t);
                let step = (at - local).dot(speed) / speed.length_squared();
                t -= step;
                if step.abs() <= f64::EPSILON * period {
                    break;
                }
            }
            t
        };
        let t = [
            2.0 * half,
            TAU - 2.0 * half,
            TAU + 2.0 * half,
            2.0 * TAU - 2.0 * half,
        ]
        .into_iter()
        .filter(|&t| t < period)
        .map(refined)
        .min_by(|left, right| away(*left).total_cmp(&away(*right)))
        .unwrap_or(0.0)
        .rem_euclid(period);
        if t < period { t } else { 0.0 }
    }

    /// Every parameter at which the component passes through a node of its
    /// curve standing within `eps` of `point`, in order: a vertex there cuts
    /// the curve at each. Nothing where no node stands.
    pub fn passes(&self, point: DVec3, eps: f64) -> Vec<f64> {
        meeting::nodes(&self.pair())
            .into_iter()
            .filter(|node| node.point.distance(point) <= eps)
            .flat_map(|node| node.on)
            .filter(|(component, _)| *component == self.component)
            .map(|(_, t)| t)
            .collect()
    }

    /// How far `t` runs round the component, or nothing when the pair has no
    /// such component.
    pub fn period(&self) -> Option<f64> {
        let pair = self.pair();
        (self.component < pair.components()).then(|| pair.period())
    }
}

#[cfg(test)]
mod tests;
