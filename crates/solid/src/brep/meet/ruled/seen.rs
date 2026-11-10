//! The curve a cone meets another wall along, seen in the parameters of
//! either: on the cone its own `(θ, l)`; on the other wall its angle —
//! the eccentric angle on an elliptic one — and the height along its axis,
//! the angle unwrapped along the component, winding round the wall or
//! facing one way.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::{Lap, Ruled, halved};

/// How many steps round a lap the other wall's angle is unwrapped over, at
/// the least — the table where it reaches a value or turns back is found
/// from; how far it may move over one; how many times a step is halved at
/// most.
const AROUND: usize = 256;
const LEAP: f64 = PI / 8.0;
const DEEPEST: u32 = 40;
const MOST: usize = 16 * AROUND;
/// The step central differences are taken over, in the parameter.
const STEP: f64 = 1e-5;
/// How near its seam a root found along a lap is the one the seam holds: the
/// two ends of a winding lap's table disagree by a few ulps of its angle,
/// carried into the parameter by its rate; two real roots that close to the
/// seam are a tangency there.
const SEAM: f64 = 1024.0 * TAU * f64::EPSILON;

impl Ruled {
    pub fn point(&self, lap: &Lap, t: f64) -> DVec3 {
        let [at, _] = self.local(lap, t);
        self.point_of(at)
    }

    pub fn derivative(&self, lap: &Lap, t: f64) -> DVec3 {
        self.speed_of(self.local(lap, t))
    }

    /// The point, its derivative and its second derivative in `t`.
    pub fn world(&self, lap: &Lap, t: f64) -> [DVec3; 3] {
        let [ahead, behind] = [t + STEP, t - STEP].map(|t| self.derivative(lap, t));
        [
            self.point(lap, t),
            self.derivative(lap, t),
            (ahead - behind) / (2.0 * STEP),
        ]
    }

    /// The parameter of `point` on lap `lap`: its angle about the cone's
    /// axis fixes it up to the half of the span it lies on, the nearer of
    /// the two kept, brought to the point by Newton.
    pub fn parameter(&self, lap: &Lap, point: DVec3) -> f64 {
        let theta = self.cone.parameters(point).x;
        let candidates: Vec<f64> = match lap.span {
            Some([from, to]) => {
                let (middle, half) = ((from + to) / 2.0, (to - from) / 2.0);
                let near = theta + TAU * ((middle - theta) / TAU).round();
                let t = ((middle - near) / half).clamp(-1.0, 1.0).acos();
                vec![t, TAU - t]
            }
            None => vec![(theta - lap.start).rem_euclid(TAU)],
        };
        let away = |t: f64| self.point(lap, t).distance_squared(point);
        let refined = |mut t: f64| {
            for _ in 0..8 {
                let at = self.point(lap, t);
                let speed = self.derivative(lap, t);
                let squared = speed.length_squared();
                if squared == 0.0 {
                    break;
                }
                let step = (at - point).dot(speed) / squared;
                t -= step;
                if step.abs() <= f64::EPSILON * TAU {
                    break;
                }
            }
            t
        };
        let t = candidates
            .into_iter()
            .map(refined)
            .min_by(|one, other| away(*one).total_cmp(&away(*other)))
            .unwrap_or(0.0)
            .rem_euclid(TAU);
        if t < TAU { t } else { 0.0 }
    }

    /// The point at `t` in the parameters of the cone, or of the other wall,
    /// with its first and second derivatives.
    pub fn seen_on(&self, lap: &Lap, on_cone: bool, t: f64) -> [DVec2; 3] {
        if on_cone {
            let [at, rate] = self.local(lap, t);
            let [_, ahead] = self.local(lap, t + STEP);
            let [_, behind] = self.local(lap, t - STEP);
            return [at, rate, (ahead - behind) / (2.0 * STEP)];
        }
        let [point, speed, bend] = self.world(lap, t);
        let flat = |v: DVec3| self.flat(v);
        let from = point - self.other.origin;
        let [p, p1, p2] = [flat(from), flat(speed), flat(bend)];
        let square = p.length_squared();
        let turn = p.perp_dot(p1) / square;
        let sway = p.perp_dot(p2) / square - 2.0 * turn * p.dot(p1) / square;
        let raw = p.y.atan2(p.x);
        let reference = self.reference(lap, t);
        let angle = raw + TAU * ((reference - raw) / TAU).round();
        let [rise, rate] = self.other.rise(point, speed);
        let [ahead, behind] = [t + STEP, t - STEP]
            .map(|t| self.other.rise(self.point(lap, t), self.derivative(lap, t))[1]);
        [
            DVec2::new(angle, rise),
            DVec2::new(turn, rate),
            DVec2::new(sway, (ahead - behind) / (2.0 * STEP)),
        ]
    }

    fn flat(&self, v: DVec3) -> DVec2 {
        self.other.flat(v)
    }

    /// The other wall's angle at `t`, unwrapped as `seen_on` reads it,
    /// from the point alone.
    fn facing(&self, lap: &Lap, t: f64) -> f64 {
        let p = self.flat(self.point(lap, t) - self.other.origin);
        let raw = p.y.atan2(p.x);
        raw + TAU * ((self.reference(lap, t) - raw) / TAU).round()
    }

    /// The angle the other wall's own is unwrapped near at `t`: read off
    /// the angles sampled along the lap, unwrapped from one to the next,
    /// running on by the turns the lap winds round the wall past a turn of
    /// `t`. A lap facing the wall may sweep more than half a turn of it.
    fn reference(&self, lap: &Lap, t: f64) -> f64 {
        let around = &lap.around;
        let [first, last] = [around[0][1], around[around.len() - 1][1]];
        let turns = TAU * ((last - first) / TAU).round();
        let laps = (t / TAU).floor();
        let local = t - laps * TAU;
        let after = around
            .partition_point(|[at, _]| *at <= local)
            .clamp(1, around.len() - 1);
        let ([from, one], [to, other]) = (around[after - 1], around[after]);
        let share = if to > from {
            (local - from) / (to - from)
        } else {
            0.0
        };
        one + (other - one) * share + laps * turns
    }

    /// The other wall's angle along lap `lap`, unwrapped, at parameters
    /// close enough that it moves by less than `LEAP` from one to the next:
    /// near the other wall's axis a small step along the lap sweeps a wide
    /// angle of it.
    pub(super) fn around(&self, lap: &Lap) -> Vec<[f64; 2]> {
        let angle = |t: f64| {
            let p = self.flat(self.point(lap, t) - self.other.origin);
            p.y.atan2(p.x)
        };
        let mut unwrapped = vec![[0.0, angle(0.0)]];
        for step in 0..AROUND {
            let [from, to] = [step, step + 1].map(|step| TAU * step as f64 / AROUND as f64);
            unwrap(&angle, from, to, &mut unwrapped, DEEPEST);
        }
        unwrapped
    }

    /// Every parameter in the first turn where lap `lap` stands at the angle
    /// `theta` of the cone, or of the other wall, sorted.
    pub fn at_angle(&self, lap: &Lap, on_cone: bool, theta: f64) -> Vec<f64> {
        let mut found: Vec<f64> = if on_cone {
            match lap.span {
                Some([from, to]) => {
                    let (middle, half) = ((from + to) / 2.0, (to - from) / 2.0);
                    let near = theta + TAU * ((middle - theta) / TAU).round();
                    if (near - middle).abs() > half {
                        Vec::new()
                    } else {
                        let t = ((middle - near) / half).clamp(-1.0, 1.0).acos();
                        vec![t, TAU - t]
                    }
                }
                None => vec![(theta - lap.start).rem_euclid(TAU)],
            }
        } else {
            let angle = |t: f64| self.facing(lap, t);
            let mut found = Vec::new();
            for pair in lap.around.windows(2) {
                let ([from, one], [to, other]) = (pair[0], pair[1]);
                let (low, high) = (one.min(other), one.max(other));
                let target = theta + TAU * ((low - theta) / TAU).ceil();
                if target <= high {
                    let off = |t: f64| angle(t) - target;
                    let (start, end) = (off(from), off(to));
                    if start == 0.0 {
                        found.push(from);
                    } else if end == 0.0 {
                        found.push(to.rem_euclid(TAU));
                    } else if (start < 0.0) != (end < 0.0) {
                        found.push(halved(off, from, to).rem_euclid(TAU));
                    }
                }
            }
            let [first, last] = [0.0, TAU].map(angle);
            let back = last - TAU * ((last - first) / TAU).round();
            let target = theta + TAU * ((first.min(back) - theta) / TAU).ceil();
            if target <= first.max(back) {
                found.push(0.0);
            }
            found
                .into_iter()
                .map(|t| if t.min(TAU - t) <= SEAM { 0.0 } else { t })
                .collect()
        };
        found.retain(|t| (0.0..TAU).contains(t));
        found.sort_by(f64::total_cmp);
        found.dedup();
        found
    }

    /// Every parameter in the first turn where lap `lap` turns back in the
    /// angle of the cone, or of the other wall, sorted: on the other wall,
    /// across the seam too, the table's last step before it read a turn
    /// back, shifted by the turns the lap winds.
    pub fn turns(&self, lap: &Lap, on_cone: bool) -> Vec<f64> {
        if on_cone {
            return match lap.span {
                Some(_) => vec![0.0, TAU / 2.0],
                None => Vec::new(),
            };
        }
        let turn = |t: f64| {
            let p = self.flat(self.point(lap, t) - self.other.origin);
            p.perp_dot(self.flat(self.derivative(lap, t)))
        };
        let around = &lap.around;
        let count = around.len();
        let winding = around[count - 1][1] - around[0][1];
        let at = |rank: usize| around[rank];
        let mut found = Vec::new();
        for rank in 0..count.saturating_sub(1) {
            let [before_t, before] = if rank == 0 {
                let [t, angle] = at(count - 2);
                [t - TAU, angle - winding]
            } else {
                at(rank - 1)
            };
            let ([_, here], [after_t, after]) = (at(rank), at(rank + 1));
            if (here - before).signum() != (after - here).signum()
                && turn(before_t).signum() != turn(after_t).signum()
            {
                found.push(halved(turn, before_t, after_t).rem_euclid(TAU));
            }
        }
        found.sort_by(f64::total_cmp);
        found.dedup();
        found
    }

    /// A box holding lap `lap` from `from` to `to`: its points sampled, the
    /// box grown by the longest step between two, which no stretch between
    /// them strays further than.
    pub fn bounds(&self, lap: &Lap, from: f64, to: f64) -> [DVec3; 2] {
        let steps = 64;
        let points: Vec<DVec3> = (0..=steps)
            .map(|step| self.point(lap, from + (to - from) * step as f64 / steps as f64))
            .collect();
        let grown = points
            .windows(2)
            .map(|pair| pair[0].distance(pair[1]))
            .fold(0.0, f64::max);
        let (low, high) = points.iter().fold(
            (DVec3::INFINITY, DVec3::NEG_INFINITY),
            |(low, high), point| (low.min(*point), high.max(*point)),
        );
        [low - DVec3::splat(grown), high + DVec3::splat(grown)]
    }
}

/// `angle` from `from` to `to` pushed onto `unwrapped`, each value the
/// nearest to the one before, the stretch halved where it leaps — up to
/// `MOST` values in all: a lap all but a point leaps everywhere.
fn unwrap(
    angle: &impl Fn(f64) -> f64,
    from: f64,
    to: f64,
    unwrapped: &mut Vec<[f64; 2]>,
    depth: u32,
) {
    let before = unwrapped[unwrapped.len() - 1][1];
    let near = |t: f64| {
        let value = angle(t);
        value + TAU * ((before - value) / TAU).round()
    };
    let middle = (from + to) / 2.0;
    let [end, half] = [near(to), near(middle)];
    let steady = (end - before).abs() < LEAP && (half - (before + end) / 2.0).abs() < LEAP;
    if steady || depth == 0 || unwrapped.len() >= MOST {
        unwrapped.push([to, end]);
        return;
    }
    unwrap(angle, from, middle, unwrapped, depth - 1);
    unwrap(angle, middle, to, unwrapped, depth - 1);
}
