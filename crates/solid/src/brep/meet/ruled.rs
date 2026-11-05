//! The curve a cone meets a cylinder, an elliptic wall or a cone of another
//! axis along, at any angle (#577), read along the cone's rulings: each
//! ruling, at the angle `θ` about the cone's axis, is a line `F(θ) + l·d(θ)`
//! meeting the other wall's quadric where `A l² + B l + C = 0`, and the
//! cone's nappe is the side of its apex `l` stands on. The curve is seen on
//! the cone as `(θ, l)`, the cone's own parameters.
//!
//! A cylinder or an elliptic wall only grows away from its axis, so `A ≥ 0`.
//! A cone's quadric does not: the pair is read only where `A` stays above
//! nought all round — of two cones, along the rulings of the one whose least
//! `A` is the larger — and a root counts on the other cone's own nappe only.
//!
//! `D = B² − 4AC` is a trigonometric polynomial of degree four in `θ`. Where
//! the apex stands inside the other wall, each ruling meets it once on the
//! nappe and the curve winds round the cone. Elsewhere each span of `θ`
//! where `D ≥ 0` is one loop, its two roots joining at the span's ends, run
//! as `θ = m − h cos t`; a double zero of `D` inside a span is a node the
//! loop passes twice, going straight through; one standing alone is a
//! contact.

mod configured;
mod local;
mod met;
mod quadric;
mod seen;

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::pair::Configuration;
use crate::brep::curve::Wall;
use crate::brep::surface::Cone;
pub(super) use met::{meeting, nodes as ruled_nodes, snapped};
pub(super) use quadric::Quadric;

/// Below this, relative to the size of the terms, `D` is rounding.
const ROUNDING: f64 = 1024.0 * f64::EPSILON;
/// How many angles `D` is sampled at round the cone to find its zeros.
const SAMPLES: usize = 720;
/// Enough halvings to bring a bracket down to the rounding of an angle.
const HALVINGS: usize = 80;
/// The least share of its greatest `A` may fall to round the cone for the
/// curve to be read along its rulings: a ruling within a thousandth of a
/// radian of the other wall's axis meets it too far off to bound.
const STEADY: f64 = 1e-6;
/// How far from a simple zero of `D`, in radians, its root is read off the
/// zero's first Taylor term rather than off `D`, whose cancellation leaves
/// the square root of the rounding there: where the two errors cross.
pub(super) const NEAR: f64 = 1e-8;
/// The same about a double zero, read off its second term; and how far
/// from one the speed is taken across rather than off `f_θ / √D`, both
/// nought there.
const NEAR_A_NODE: f64 = 1e-6;
pub(super) const ACROSS_A_NODE: f64 = 1e-4;

/// One closed component: a span of `θ` its two roots run over and join at
/// the ends of, or a whole turn round the cone on one root.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Lap {
    pub span: Option<[f64; 2]>,
    pub sign: f64,
    pub start: f64,
    /// The angles of the nodes the lap passes through.
    pub nodes: Vec<f64>,
    /// The other wall's angle along the lap, unwrapped, each with its
    /// parameter.
    pub around: Vec<[f64; 2]>,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Ruled {
    pub cone: Cone,
    pub other: Quadric,
    pub laps: Vec<Lap>,
    pub configuration: Configuration,
    /// The angle of a contact, where the walls touch at a point alone.
    pub contact: Option<f64>,
    /// The simple zeros of `D`, where two roots join, and the double ones.
    pub zeros: Vec<f64>,
    pub doubles: Vec<f64>,
    /// Every angle where `D` turns, a touch within reach or not.
    pub turning: Vec<f64>,
    pub rounding: f64,
}

/// `A`, `B`, `C` and their first derivatives in `θ`.
#[derive(Clone, Copy, Debug)]
pub(super) struct Terms {
    pub a: [f64; 2],
    pub b: [f64; 2],
    pub c: [f64; 2],
}

impl Terms {
    pub fn discriminant(&self) -> [f64; 2] {
        let [a, a1] = self.a;
        let [b, b1] = self.b;
        let [c, c1] = self.c;
        [b * b - 4.0 * a * c, 2.0 * b * b1 - 4.0 * (a1 * c + a * c1)]
    }

    fn size(&self) -> f64 {
        self.b[0] * self.b[0] + 4.0 * (self.a[0] * self.c[0]).abs()
    }
}

impl Ruled {
    /// The least `A` round the cone, its rulings read against `other`, as a
    /// share of the greatest: above `STEADY`, every ruling meets the other
    /// wall in a true quadratic, its far root no further than the walls'
    /// size over `STEADY`; and of two cones the one that gives it the
    /// rulings. Sampled, then the least brought down by thirds, since a
    /// ruling parallel to the other wall's axis is a double zero of `A`.
    pub fn steadiness(cone: &Cone, other: &Wall) -> f64 {
        let read = Ruled {
            cone: *cone,
            other: Quadric::of(other),
            laps: Vec::new(),
            configuration: Configuration::Apart,
            contact: None,
            zeros: Vec::new(),
            doubles: Vec::new(),
            turning: Vec::new(),
            rounding: 0.0,
        };
        let a = |theta: f64| read.terms(theta).a[0];
        let step = TAU / SAMPLES as f64;
        let at = |rank: usize| step * rank as f64;
        let values: Vec<f64> = (0..SAMPLES).map(|rank| a(at(rank))).collect();
        let greatest = values.iter().copied().fold(0.0, f64::max);
        let least = (0..SAMPLES)
            .min_by(|one, other| values[*one].total_cmp(&values[*other]))
            .map_or(0.0, at);
        let (mut from, mut to) = (least - step, least + step);
        for _ in 0..HALVINGS {
            let [one, other] = [(2.0 * from + to) / 3.0, (from + 2.0 * to) / 3.0];
            if a(one) < a(other) {
                to = other;
            } else {
                from = one;
            }
        }
        if greatest > 0.0 {
            a((from + to) / 2.0) / greatest
        } else {
            0.0
        }
    }

    pub fn of(cone: &Cone, other: &Wall) -> Option<Ruled> {
        if Ruled::steadiness(cone, other) <= STEADY {
            return None;
        }
        let other = Quadric::of(other);
        let mut ruled = Ruled {
            cone: *cone,
            other,
            laps: Vec::new(),
            configuration: Configuration::Apart,
            contact: None,
            zeros: Vec::new(),
            doubles: Vec::new(),
            turning: Vec::new(),
            rounding: 0.0,
        };
        ruled.laid();
        Some(ruled)
    }

    /// The direction of the ruling at `θ` and its derivative.
    pub(super) fn ruling(&self, theta: f64) -> [DVec3; 2] {
        let (sin, cos) = theta.sin_cos();
        let cone = &self.cone;
        let radial = cone.u * cos + cone.v * sin;
        let turned = cone.v * cos - cone.u * sin;
        [
            cone.axis * cone.ruling.x + radial * cone.ruling.y,
            turned * cone.ruling.y,
        ]
    }

    pub fn terms(&self, theta: f64) -> Terms {
        let (sin, cos) = theta.sin_cos();
        let cone = &self.cone;
        let radial = cone.u * cos + cone.v * sin;
        let turned = cone.v * cos - cone.u * sin;
        let foot = cone.origin + cone.axis * cone.foot.x + radial * cone.foot.y;
        let [d, d1] = self.ruling(theta);
        self.other.along([foot, turned * cone.foot.y], [d, d1])
    }

    pub(super) fn discriminant(&self, theta: f64) -> [f64; 2] {
        self.terms(theta).discriminant()
    }

    fn laid(&mut self) {
        let at = |step: usize| TAU * step as f64 / SAMPLES as f64;
        let samples: Vec<(f64, [f64; 2], f64)> = (0..SAMPLES)
            .map(|step| {
                let terms = self.terms(at(step));
                (at(step), terms.discriminant(), terms.size())
            })
            .collect();
        self.rounding = ROUNDING * samples.iter().map(|s| s.2).fold(0.0, f64::max);
        let rounding = self.rounding;
        let mut doubles = Vec::new();
        let mut zeros = Vec::new();
        let mut turning = Vec::new();
        for step in 0..SAMPLES {
            let (here, [d, slope], _) = samples[step];
            let (_, [next_d, next_slope], _) = samples[(step + 1) % SAMPLES];
            let there = here + TAU / SAMPLES as f64;
            if slope.signum() != next_slope.signum() {
                let turn = halved(|theta| self.discriminant(theta)[1], here, there);
                turning.push(turn.rem_euclid(TAU));
                if self.discriminant(turn)[0].abs() <= rounding {
                    doubles.push(turn.rem_euclid(TAU));
                }
            }
            if d.signum() != next_d.signum() {
                zeros
                    .push(halved(|theta| self.discriminant(theta)[0], here, there).rem_euclid(TAU));
            }
        }
        zeros.retain(|zero| {
            doubles
                .iter()
                .all(|double| angle_between(*zero, *double) > NEAR_A_NODE.sqrt())
        });
        zeros.sort_by(f64::total_cmp);
        doubles.sort_by(f64::total_cmp);
        doubles.dedup_by(|one, other| angle_between(*one, *other) <= NEAR_A_NODE);
        self.zeros = zeros;
        self.doubles = doubles;
        self.turning = turning;
        self.configured();
        let around: Vec<Vec<[f64; 2]>> = self.laps.iter().map(|lap| self.around(lap)).collect();
        for (lap, around) in self.laps.iter_mut().zip(around) {
            lap.around = around;
        }
    }

    /// An angle where `D` stands clear of its zeros, to read a root at.
    fn probe(&self) -> f64 {
        (0..8)
            .map(|step| PI / 4.0 * step as f64 + 0.1)
            .find(|theta| {
                self.doubles
                    .iter()
                    .all(|double| angle_between(*theta, *double) > 0.05)
            })
            .unwrap_or(0.1)
    }

    /// The root on the side `sign` of the ruling at `θ`, read off `D`.
    fn root(&self, theta: f64, sign: f64) -> f64 {
        let terms = self.terms(theta);
        let [discriminant, _] = terms.discriminant();
        solved(&terms, sign * discriminant.max(0.0).sqrt())
    }

    /// `√D` at `θ` signed `sign`, read off the Taylor term of a zero of `D`
    /// standing nearer than `NEAR`.
    pub fn signed_root(&self, theta: f64, sign: f64) -> f64 {
        for zero in &self.zeros {
            let off = theta - (zero + TAU * ((theta - zero) / TAU).round());
            if off.abs() < NEAR {
                let [_, slope] = self.discriminant(*zero);
                return sign * (slope * off).max(0.0).sqrt();
            }
        }
        for double in &self.doubles {
            let off = theta - (double + TAU * ((theta - double) / TAU).round());
            if off.abs() < NEAR_A_NODE {
                let curve = curvature(|theta| self.discriminant(theta)[1], *double);
                return sign * off.abs() * (curve / 2.0).max(0.0).sqrt();
            }
        }
        sign * self.discriminant(theta)[0].max(0.0).sqrt()
    }

    /// The sign of the root a lap runs on at `θ`, the half `positive` of
    /// its span: across each node the branch going straight through.
    pub fn sign(&self, lap: &Lap, theta: f64, positive: bool) -> f64 {
        let half = if positive { 1.0 } else { -1.0 };
        lap.nodes.iter().fold(
            lap.sign * half,
            |sign, node| if theta < *node { -sign } else { sign },
        )
    }

    /// The point where the ruling at `θ` touches the other wall: the double
    /// root.
    pub fn touching(&self, theta: f64) -> DVec3 {
        let terms = self.terms(theta);
        self.cone
            .point(DVec2::new(theta, -terms.b[0] / (2.0 * terms.a[0])))
    }

    /// The double roots at the angles where `D` turns, those on the nappe.
    pub fn nearest(&self) -> Vec<DVec3> {
        self.turning
            .iter()
            .filter_map(|&theta| {
                let terms = self.terms(theta);
                let l = -terms.b[0] / (2.0 * terms.a[0]);
                (l >= self.cone.apex_at()).then(|| self.cone.point(DVec2::new(theta, l)))
            })
            .collect()
    }

    /// The parameters at which lap `lap` passes through the node at `θ`.
    pub fn through(&self, lap: &Lap, node: f64) -> Vec<f64> {
        match lap.span {
            Some([from, to]) => {
                let cos = (((from + to) / 2.0 - node) / ((to - from) / 2.0)).clamp(-1.0, 1.0);
                let t = cos.acos();
                vec![t, TAU - t]
            }
            None => vec![(node - lap.start).rem_euclid(TAU)],
        }
    }
}

/// The root `l` of `A l² + B l + C` given `√D` signed, the form free of
/// cancellation.
pub(super) fn solved(terms: &Terms, root: f64) -> f64 {
    let (a, b, c) = (terms.a[0], terms.b[0], terms.c[0]);
    if (-b) * root >= 0.0 && a != 0.0 {
        (-b + root) / (2.0 * a)
    } else if -b - root != 0.0 {
        2.0 * c / (-b - root)
    } else {
        -b / (2.0 * a)
    }
}

/// Where `f` changes sign between `from` and `to`, by halving.
pub(super) fn halved(f: impl Fn(f64) -> f64, mut from: f64, mut to: f64) -> f64 {
    let start = f(from).signum();
    for _ in 0..HALVINGS {
        let middle = (from + to) / 2.0;
        if middle <= from || middle >= to {
            break;
        }
        if f(middle).signum() == start {
            from = middle;
        } else {
            to = middle;
        }
    }
    (from + to) / 2.0
}

/// The derivative of `f` at `x`, centred.
pub(super) fn curvature(f: impl Fn(f64) -> f64, x: f64) -> f64 {
    let step = 1e-4;
    (f(x + step) - f(x - step)) / (2.0 * step)
}

/// How far apart two angles stand round the turn.
pub(super) fn angle_between(one: f64, other: f64) -> f64 {
    let gap = (one - other).rem_euclid(TAU);
    gap.min(TAU - gap)
}
