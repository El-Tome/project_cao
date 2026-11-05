//! The curve where two walls meet: exact, as four arcs over the height
//! across both axes.
//!
//! In the frame whose third axis is the first wall's, whose first axis is
//! the second's and whose origin is on the first's, two cylinders are
//! `x² + y² = a²` and `(y − d)² + (z − e)² = b²`. The curve spans `y` over
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
//!
//! An elliptic wall (#571) cuts each chord at the height `y` about a middle
//! moving along the diameter conjugate to the chords: `x = κy ± α√(a² − y²)`,
//! `a` how far the wall reaches along the height (`pair.rs`). Everything
//! that hangs on the span — its ends, the gaps, the configurations, the
//! nodes, the contact, the parameter and the factored roots — is a circular
//! pair's with `a` and `b` the two half-widths; only the point and its
//! derivatives gain the shear `κ` and the scale `α`. A circular wall is read
//! by its own expressions, bit for bit.
//!
//! Walls at a skew angle (#577) share the height, along the line square to
//! both axes, and the second's section is read across its own axis: each
//! ruling of the first meets the second where that section does, so the
//! span and everything hanging on it are the square pair's, and the height
//! along the first's axis is `z = (ζ + x cos φ) / sin φ`, `ζ` the second's
//! coordinate across its chords (`pair.rs`). A pair square to within
//! rounding is read as square, bit for bit.

mod meeting;
mod pair;
mod ruled;
mod section;
mod seen;

use std::cell::RefCell;
use std::f64::consts::{PI, TAU};
use std::rc::Rc;

use glam::DVec3;

use super::curve::{Meet, Wall};
use super::scale::Scale;
pub use meeting::{Meeting, Node};
pub(super) use meeting::{goes_first, moved, moved_instead};
pub use pair::Configuration;
use pair::Pair;
use ruled::{Lap, Ruled};

/// Newton doubles the digits at each step: from the height's guess, a few
/// are enough, and the count is fixed so the same point gives the same bits.
const NEWTON_STEPS: usize = 8;

/// How many pairs of a cone and a wall the reading of their curve is kept
/// for: finding where `D` vanishes costs hundreds of evaluations, and each
/// point asked of the curve would pay it again.
const KEPT: usize = 16;

thread_local! {
    static READ: RefCell<Vec<(Wall, Wall, Rc<Ruled>)>> = const { RefCell::new(Vec::new()) };
}

/// The curve a cone meets another wall along, read once per pair.
fn ruled(cone: &Wall, other: &Wall) -> Option<Rc<Ruled>> {
    let Wall::Cone(own) = cone else {
        return None;
    };
    READ.with(|read| {
        let mut read = read.borrow_mut();
        if let Some((_, _, ruled)) = read
            .iter()
            .find(|(one, two, _)| one == cone && two == other)
        {
            return Some(ruled.clone());
        }
        let ruled = Rc::new(Ruled::of(own, other)?);
        if read.len() == KEPT {
            read.remove(0);
        }
        read.push((*cone, *other, ruled.clone()));
        Some(ruled)
    })
}

/// Whether a cone's rulings meet `other` in a true quadratic all round, or,
/// `other` a cone, the other's rulings meet the first so: the pairs
/// `ruled.rs` reads.
pub(super) fn steady(cone: &super::surface::Cone, other: &Wall) -> bool {
    ruled(&Wall::Cone(*cone), other).is_some()
        || matches!(other, Wall::Cone(_) if ruled(other, &Wall::Cone(*cone)).is_some())
}

/// Where the rulings of a cone come nearest `other` — at each angle where
/// `D` turns, the double root, kept on the cone's own nappe — read along the
/// cone's rulings or, `other` a cone, along its own: where two walls that
/// do not quite meet come closest, or barely cross.
pub(in crate::brep) fn nearest_approaches(cone: &super::surface::Cone, other: &Wall) -> Vec<DVec3> {
    let mut readings = vec![ruled(&Wall::Cone(*cone), other)];
    if let Wall::Cone(_) = other {
        readings.push(ruled(other, &Wall::Cone(*cone)));
    }
    readings
        .into_iter()
        .flatten()
        .flat_map(|read| read.nearest())
        .collect()
}

impl Meet {
    /// The curve read along a cone's rulings, and the lap this component
    /// runs; none for two cylinders or elliptic walls.
    fn ruled(&self) -> Option<(Rc<Ruled>, Option<Lap>)> {
        let ruled = ruled(&self.first, &self.second)?;
        let lap = ruled.laps.get(usize::from(self.component)).cloned();
        Some((ruled, lap))
    }

    /// Whether the component goes round the axis of `wall`, one of its
    /// two cones: on the cone giving the rulings, a lap over a whole turn of
    /// `θ`; on the other, a lap its angle winds round. None for a wall the
    /// curve is not read along a cone's rulings on.
    pub(in crate::brep) fn winds_round(&self, wall: &Wall) -> Option<bool> {
        let (_, lap) = self.ruled()?;
        let lap = lap?;
        if *wall == self.first {
            Some(lap.span.is_none())
        } else if *wall == self.second {
            let [first, last] = [lap.around[0][1], lap.around[lap.around.len() - 1][1]];
            Some(((last - first) / TAU).round() != 0.0)
        } else {
            None
        }
    }

    fn pair(&self) -> Pair {
        Pair::of(&self.first, &self.second)
    }

    fn local(&self, t: f64) -> (Pair, [DVec3; 3]) {
        let pair = self.pair();
        let local = pair.local(pair.signs(self.component), t);
        (pair, local)
    }

    pub fn point(&self, t: f64) -> DVec3 {
        if let Some((ruled, Some(lap))) = self.ruled() {
            return ruled.point(&lap, t);
        }
        let (pair, [point, ..]) = self.local(t);
        pair.world(point)
    }

    pub fn derivative(&self, t: f64) -> DVec3 {
        if let Some((ruled, Some(lap))) = self.ruled() {
            return ruled.derivative(&lap, t);
        }
        let (pair, [_, speed, _]) = self.local(t);
        pair.direction(speed)
    }

    /// Read off the height across both axes, which fixes the parameter up to
    /// the arc it lies on, then brought to the point by Newton from each arc,
    /// the nearest kept: near an end of the span the height says little, and
    /// near a node two branches pass the same height.
    pub fn parameter(&self, point: DVec3) -> f64 {
        if let Some((ruled, Some(lap))) = self.ruled() {
            return ruled.parameter(&lap, point);
        }
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
        if let Some((ruled, _)) = self.ruled() {
            return meeting::ruled_nodes(&ruled)
                .into_iter()
                .filter(|node| node.point.distance(point) <= eps)
                .flat_map(|node| node.on)
                .filter(|(component, _)| *component == self.component)
                .map(|(_, t)| t)
                .collect();
        }
        meeting::nodes(&self.pair())
            .into_iter()
            .filter(|node| node.point.distance(point) <= eps)
            .flat_map(|node| node.on)
            .filter(|(component, _)| *component == self.component)
            .map(|(_, t)| t)
            .collect()
    }

    /// Whether `wall` is one of the two the curve runs round, within `eps`:
    /// the first, or the second. None for any other, a wall parallel to one
    /// of them a hair off it among them.
    pub(in crate::brep) fn own(&self, wall: &Wall, eps: f64) -> Option<bool> {
        let alike = |own: &Wall| match (own, wall) {
            (Wall::Cylinder(own), Wall::Cylinder(cylinder)) => {
                let from = own.origin - cylinder.origin;
                own.axis.cross(cylinder.axis).length() <= Scale::RELATIVE
                    && (from - cylinder.axis * from.dot(cylinder.axis)).length() <= eps
                    && (own.radius - cylinder.radius).abs() <= eps
            }
            (Wall::Elliptic(own), Wall::Elliptic(oval)) => {
                own.axis.cross(oval.axis).length() <= Scale::RELATIVE && own.within(oval, eps)
            }
            (Wall::Cone(own), Wall::Cone(cone)) => {
                own.axis.cross(cone.axis).length() <= Scale::RELATIVE
                    && (own.origin - cone.origin).length() <= eps
                    && (own.foot - cone.foot).length() <= eps
                    && (own.ruling - cone.ruling).length() <= Scale::RELATIVE
            }
            (Wall::Cylinder(_), Wall::Elliptic(_) | Wall::Cone(_))
            | (Wall::Elliptic(_), Wall::Cylinder(_) | Wall::Cone(_))
            | (Wall::Cone(_), Wall::Cylinder(_) | Wall::Elliptic(_)) => false,
        };
        if alike(&self.first) {
            Some(true)
        } else if alike(&self.second) {
            Some(false)
        } else {
            None
        }
    }

    /// How far `t` runs round the component, or nothing when the pair has no
    /// such component.
    pub fn period(&self) -> Option<f64> {
        if let Some((_, lap)) = self.ruled() {
            return lap.map(|_| TAU);
        }
        let pair = self.pair();
        (self.component < pair.components()).then(|| pair.period())
    }

    /// A box holding the component from `from` to `to`: the height across
    /// both axes spans what `m − l cos t` covers there, and each root is at
    /// most what that height leaves it, `x² = a² − y²` and
    /// `(z − e)² = b² − (y − d)²`, whichever its sign — on an elliptic wall
    /// scaled by `α` about the middles `κy` of the chords over that height.
    pub(in crate::brep) fn bounds(&self, from: f64, to: f64) -> [DVec3; 2] {
        if let Some((ruled, Some(lap))) = self.ruled() {
            return ruled.bounds(&lap, from.min(to), from.max(to));
        }
        let pair = self.pair();
        let (from, to) = (from.min(to), from.max(to));
        let half = (pair.high - pair.low) / 2.0;
        let middle = (pair.high + pair.low) / 2.0;
        let [start, end] = [from, to].map(f64::cos);
        let most = if passes(from, to, 0.0) {
            1.0
        } else {
            start.max(end)
        };
        let least = if passes(from, to, PI) {
            -1.0
        } else {
            start.min(end)
        };
        let (low, high) = (middle - half * most, middle - half * least);
        let nearest = |centre: f64| (low - centre).max(centre - high).max(0.0);
        let room = |radius: f64, centre: f64| {
            (radius * radius - nearest(centre).powi(2)).max(0.0).sqrt() + pair.rounding
        };
        let (x, z) = (room(pair.a, 0.0), room(pair.b, pair.d));
        let [x, z] = [(x, 0, 0.0), (z, 1, pair.d)]
            .map(|(room, wall, from)| pair.reach_across(wall, room, [low - from, high - from]));
        let (low, high) = (low - pair.rounding, high + pair.rounding);
        let mut bounds = [DVec3::INFINITY, DVec3::NEG_INFINITY];
        for corner in 0..8 {
            let local = DVec3::new(
                if corner & 1 == 0 { x[0] } else { x[1] },
                if corner & 2 == 0 { low } else { high },
                pair.e + if corner & 4 == 0 { z[0] } else { z[1] },
            );
            let world = pair.world(local);
            bounds = [bounds[0].min(world), bounds[1].max(world)];
        }
        bounds
    }
}

/// Whether the stretch from `from` to `to` holds an angle a whole number of
/// turns from `angle`.
fn passes(from: f64, to: f64, angle: f64) -> bool {
    angle + TAU * ((from - angle) / TAU).ceil() <= to
}

#[cfg(test)]
mod tests;
