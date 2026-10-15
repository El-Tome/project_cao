//! A torus about the axis it was turned about, read in `(θ, φ)`: the angle
//! about the axis as on a cylinder of the same axis, and the angle along the
//! meridian circle about its centre, nought at the crest furthest from the
//! axis (#576).
//!
//! The meridian circle is kept as its centre `(h_c, R)` in the half-plane
//! `(h, ρ)` and its radius, `R` signed as the profile reads it: a ring where
//! `R > r`, a horn where `R = r`, a spindle where `|R| < r` — the part of it
//! on the profile's own side of the axis, `ρ(φ) ≥ 0`, the mirror lobe never
//! being the surface. A centre within the turn's band of the axis is a
//! sphere, never a torus a hair off it.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};

use super::round::{ROUNDINGS, Round, polished};
use super::{canonical_direction, reference};
use crate::brep::curve::Circle;

/// How many halvings a root is closed in on by, at most.
const HALVINGS: usize = 100;

/// The points at `radius` from the circle about the axis at height
/// `center.x` along `axis` from `origin`, the point of the axis nearest the
/// world's origin, and at `center.y` from the axis; `u` and `v` are those
/// every cylinder of the axis has. `seam` is the angle along the meridian
/// the chart is centred on: the chart is cut at its antipode, and every face
/// the turn lays sits inside the window `(seam − π, seam + π]`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Torus {
    pub origin: DVec3,
    pub axis: DVec3,
    pub u: DVec3,
    pub v: DVec3,
    pub center: DVec2,
    pub radius: f64,
    pub seam: f64,
}

impl Torus {
    /// The torus an arc about `center`, in `(h, ρ)` from `point` along
    /// `axis`, turns into about the line through `point` along `axis`, the
    /// chart centred on `middle`, a point of the arc: the same bits whichever
    /// way the run is read and whichever way the axis points, and the
    /// `origin`, `u` and `v` a cylinder about the same line has.
    pub fn through(point: DVec3, axis: DVec3, center: DVec2, radius: f64, middle: DVec2) -> Torus {
        let (axis, turned) = canonical_direction(axis);
        let shift = axis.dot(point);
        let laid =
            |corner: DVec2| DVec2::new(if turned { -corner.x } else { corner.x } + shift, corner.y);
        let center = laid(center);
        let from = laid(middle) - center;
        let u = reference(axis);
        Torus {
            origin: point - axis * shift,
            axis,
            u,
            v: axis.cross(u),
            center,
            radius,
            seam: from.x.atan2(from.y),
        }
    }

    fn round(&self) -> Round {
        Round {
            origin: self.origin,
            axis: self.axis,
            u: self.u,
            v: self.v,
            center: self.center,
            radius: self.radius,
        }
    }

    /// The direction square to the axis at angle `theta`.
    pub fn radial(&self, theta: f64) -> DVec3 {
        self.round().radial(theta)
    }

    pub fn point(&self, at: DVec2) -> DVec3 {
        self.round().point(at)
    }

    /// `(θ, φ)`, with `θ` in `(-π, π]` and `φ` in `(seam − π, seam + π]`.
    pub fn parameters(&self, point: DVec3) -> DVec2 {
        let round = self.round();
        let (theta, _) = round.seen(point);
        DVec2::new(theta, self.windowed(round.phi(point)))
    }

    /// An angle along the meridian moved by whole turns into the window.
    pub fn windowed(&self, phi: f64) -> f64 {
        let phi = phi + TAU * ((self.seam - phi) / TAU).round();
        if phi <= self.seam - PI {
            phi + TAU
        } else {
            phi
        }
    }

    /// The `(h, ρ)` of the point of the meridian circle the chart is
    /// centred on: what the torus is built again through, moved or turned.
    pub fn middle(&self) -> DVec2 {
        self.round().meridian_point(self.seam)
    }

    /// The torus's own normal at `at`, of unit length, the poles included:
    /// `∂θ × ∂φ` points along it, away from the meridian circle's centre.
    pub fn normal(&self, at: DVec2) -> DVec3 {
        self.round().normal(at)
    }

    /// Where the meridian circle meets the axis, as the angle `φ_max` of the
    /// own lobe `|φ| ≤ φ_max`; none on a ring, which never reaches it.
    fn lobe(&self) -> Option<f64> {
        let (big, r) = (self.center.y, self.radius);
        (big.abs() <= r).then(|| (-big / r).acos())
    }

    /// How far `point` stands outside the torus, negative inside: across the
    /// meridian circle where its foot stands on the own lobe, and from the
    /// nearer pole where it stands on the mirror lobe of a spindle, so that
    /// a point of the mirror lobe never reads as on the surface.
    pub fn distance(&self, point: DVec3) -> f64 {
        let round = self.round();
        let level = round.level(point);
        let phi = round.phi(point);
        match self.lobe() {
            Some(reach) if phi.abs() > reach => {
                let (_, meridian) = round.seen(point);
                let nearest = [-reach, reach]
                    .map(|pole| meridian.distance(round.meridian_point(pole)))
                    .into_iter()
                    .fold(f64::INFINITY, f64::min);
                nearest.copysign(level)
            }
            _ => level,
        }
    }

    /// The point of the own lobe nearest `point` in its meridian half-plane.
    pub fn nearest(&self, point: DVec3) -> DVec3 {
        let round = self.round();
        let (theta, _) = round.seen(point);
        let phi = round.phi(point);
        let phi = match self.lobe() {
            Some(reach) => phi.clamp(-reach, reach),
            None => phi,
        };
        self.point(DVec2::new(theta, phi))
    }

    /// The angles along the meridian the poles stand at, in the window, the
    /// low end of the own lobe first and the high end second: none on a
    /// ring, two on a spindle, the one point of a horn twice.
    pub fn poles(&self) -> Vec<f64> {
        let Some(reach) = self.lobe() else {
            return Vec::new();
        };
        vec![self.windowed(-reach), self.windowed(reach)]
    }

    /// The circle about the axis at height `h` and of `radius`: the very
    /// circle a cylinder of the axis has there.
    pub fn circle(&self, h: f64, radius: f64) -> Circle {
        self.round().circle(h, radius)
    }

    /// Whether `circle` is a latitude circle of the torus within `eps`:
    /// about the axis, its rim within `eps` of the meridian circle.
    pub fn holds(&self, circle: &Circle, eps: f64) -> bool {
        self.round().holds(circle, eps)
    }

    /// Whether `circle` is a meridian circle of the torus within `eps`: in a
    /// plane holding the axis, about the meridian circle's centre, of its
    /// radius.
    pub fn meridian(&self, circle: &Circle, eps: f64) -> bool {
        self.round().meridian(circle, eps)
    }

    /// How far along the unit `direction` the torus reaches, from its lowest
    /// point to its highest.
    pub fn extent(&self, direction: DVec3) -> [f64; 2] {
        self.round().extent(direction)
    }

    /// Where the line through `origin` along `direction` crosses the own
    /// lobe, in order: the roots of the quartic `(q + (h − h_c)² + R² − r²)²
    /// = 4R²q`, closed in on between its critical points, which are closed
    /// in on between the roots of its second derivative, each kept where it
    /// stands on the own lobe to rounding and taken onto it by Newton's
    /// steps on the level `hypot(ρ − R, h − h_c) − r`; a line touching the
    /// torus is found at its touch, once.
    pub fn roots(&self, origin: DVec3, direction: DVec3) -> [Option<f64>; 4] {
        let round = self.round();
        let from = origin - self.origin;
        let (height, rise) = (from.dot(self.axis), direction.dot(self.axis));
        let (radial, speed) = (from - self.axis * height, direction - self.axis * rise);
        let (big, r) = (self.center.y, self.radius);
        let middle = self.axis * self.center.x - from;
        let pace = direction.length_squared();
        let nearest = middle.dot(direction) / pace;
        let hold = big.abs() + r;
        let off = (middle - direction * nearest).length();
        if off > hold {
            return [None; 4];
        }
        let half = ((hold - off) * (hold + off)).sqrt() / pace.sqrt();
        let room = 1e-6 * hold / pace.sqrt();
        let (low, high) = (nearest - half - room, nearest + half + room);

        let delta = height - self.center.x;
        let q = |t: f64| (radial + speed * t).length_squared();
        let a = |t: f64| {
            let h = delta + rise * t;
            q(t) + h * h + big * big - r * r
        };
        let p = |t: f64| {
            let a = a(t);
            a * a - 4.0 * big * big * q(t)
        };
        let dq = |t: f64| 2.0 * (radial + speed * t).dot(speed);
        let dp = |t: f64| {
            let da = dq(t) + 2.0 * (delta + rise * t) * rise;
            2.0 * a(t) * da - 4.0 * big * big * dq(t)
        };
        let (q1, q2) = (2.0 * radial.dot(speed), speed.length_squared());
        let a0 = radial.length_squared() + delta * delta + big * big - r * r;
        let a1 = q1 + 2.0 * delta * rise;
        let a2 = q2 + rise * rise;
        let bend = 2.0 * (a1 * a1 + 2.0 * a0 * a2 - 4.0 * big * big * q2);
        let inflections = quadratic(12.0 * a2 * a2, 12.0 * a1 * a2, bend);
        let critical = between(&dp, low, high, &inflections);

        let along = from.length() + (direction * high.abs().max(low.abs())).length();
        let rounding = ROUNDINGS * f64::EPSILON * along.max(1.0);
        let mut found = Vec::new();
        let mut touched = vec![false; critical.len() + 1];
        for (rank, t) in critical.iter().enumerate() {
            if round.level(origin + direction * t).abs() <= rounding {
                found.push(*t);
                touched[rank] = true;
                touched[rank + 1] = true;
            }
        }
        let mut cuts = vec![low];
        cuts.extend(
            critical
                .iter()
                .copied()
                .filter(|cut| *cut > low && *cut < high),
        );
        cuts.push(high);
        for (piece, pair) in cuts.windows(2).enumerate() {
            if touched[piece] {
                continue;
            }
            found.extend(halved(&p, pair[0], pair[1]));
        }
        let kept = kept(self, origin, direction, found, rounding);
        [0, 1, 2, 3].map(|rank| kept.get(rank).copied())
    }
}

/// The real roots of `a t² + b t + c`, ascending.
fn quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
    if a == 0.0 {
        return if b == 0.0 { Vec::new() } else { vec![-c / b] };
    }
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return Vec::new();
    }
    let far = -(b + discriminant.sqrt().copysign(b)) / 2.0;
    if far == 0.0 {
        return vec![0.0];
    }
    let mut roots = vec![far / a, c / far];
    roots.sort_by(f64::total_cmp);
    roots
}

/// Where `f`, monotone between consecutive `cuts` within `[low, high]`,
/// crosses nought on each stretch, ascending.
fn between(f: &dyn Fn(f64) -> f64, low: f64, high: f64, cuts: &[f64]) -> Vec<f64> {
    let mut ends = vec![low];
    ends.extend(cuts.iter().copied().filter(|cut| *cut > low && *cut < high));
    ends.push(high);
    ends.windows(2)
        .filter_map(|pair| halved(f, pair[0], pair[1]))
        .collect()
}

/// The polished roots, each taken onto the own lobe, that stand on it to
/// rounding, apart from each other, ascending.
fn kept(
    torus: &Torus,
    origin: DVec3,
    direction: DVec3,
    found: Vec<f64>,
    rounding: f64,
) -> Vec<f64> {
    let round = torus.round();
    let mut kept: Vec<f64> = Vec::new();
    for t in found {
        let (t, off) = polished(&round, origin, direction, t);
        let own = off.abs() <= rounding
            && torus.lobe().is_none_or(|reach| {
                round.phi(origin + direction * t).abs() <= reach + rounding / torus.radius
            });
        if own && kept.iter().all(|found| (found - t).abs() > rounding) {
            kept.push(t);
        }
    }
    kept.sort_by(f64::total_cmp);
    kept
}

/// The root of `f` between `low` and `high` where it changes sign, closed in
/// on by halving.
fn halved(f: &dyn Fn(f64) -> f64, mut low: f64, mut high: f64) -> Option<f64> {
    let (mut at_low, at_high) = (f(low), f(high));
    if at_low == 0.0 {
        return Some(low);
    }
    if at_high == 0.0 {
        return Some(high);
    }
    if (at_low > 0.0) == (at_high > 0.0) {
        return None;
    }
    for _ in 0..HALVINGS {
        let middle = (low + high) / 2.0;
        if middle <= low || middle >= high {
            break;
        }
        let at_middle = f(middle);
        if at_middle == 0.0 {
            return Some(middle);
        }
        if (at_middle > 0.0) == (at_low > 0.0) {
            (low, at_low) = (middle, at_middle);
        } else {
            high = middle;
        }
    }
    Some((low + high) / 2.0)
}

#[cfg(test)]
mod tests;
