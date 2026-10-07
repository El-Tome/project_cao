//! A cone about an axis, one nappe of it, read in `(θ, l)`: the angle about
//! the axis as on a cylinder of the same axis, and the length along the
//! ruling from the foot of its meridian line, growing away from the apex.
//!
//! The meridian line is kept as its foot and its direction rather than as an
//! apex and a slope: the apex of a cone a hair from a cylinder stands a
//! hundred million reaches away, and of a cone a hair from a disc the slope
//! is as large; the foot and the direction stay within the box and of unit
//! length at both limits, and no reading divides by either.

use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};

use super::{canonical_direction, reference};
use crate::brep::curve::{Circle, Line};
use crate::brep::scale::Scale;

/// The points whose meridian — `h` along `axis` from `origin`, `ρ ≥ 0` away
/// from it — lies on the half-line from the apex along `ruling`.
///
/// `origin` is the point of the axis nearest the world's origin, `u` and `v`
/// the same as every cylinder of that axis has; `foot` is the `(h, ρ)` of the
/// meridian line's point nearest `(0, 0)`, its `ρ` negative where the apex
/// stands beyond it, and `ruling` the line's direction, of unit length, away
/// from the apex: `w_ρ > 0`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cone {
    pub origin: DVec3,
    pub axis: DVec3,
    pub u: DVec3,
    pub v: DVec3,
    pub foot: DVec2,
    pub ruling: DVec2,
}

impl Cone {
    /// The cone a run between two corners `(h, ρ)` turns into about the line
    /// through `point` along `axis`, `h` read along `axis` from `point`: the
    /// same bits whichever way the run is read and whichever way the axis
    /// points, and the `origin`, `u` and `v` a cylinder about the same line
    /// has. The two corners stand at two distances from the axis.
    pub fn through(point: DVec3, axis: DVec3, corners: [DVec2; 2]) -> Cone {
        let (axis, turned) = canonical_direction(axis);
        let shift = axis.dot(point);
        let [one, other] = corners
            .map(|corner| DVec2::new(if turned { -corner.x } else { corner.x } + shift, corner.y));
        let (low, high) = if other.y < one.y {
            (other, one)
        } else {
            (one, other)
        };
        let ruling = (high - low).normalize();
        let u = reference(axis);
        Cone {
            origin: point - axis * shift,
            axis,
            u,
            v: axis.cross(u),
            foot: low - ruling * low.dot(ruling),
            ruling,
        }
    }

    /// The direction square to the axis at angle `theta`.
    pub fn radial(&self, theta: f64) -> DVec3 {
        let (sin, cos) = theta.sin_cos();
        self.u * cos + self.v * sin
    }

    /// The meridian line's normal in `(h, ρ)`, the ruling turned a quarter
    /// anticlockwise.
    fn across(&self) -> DVec2 {
        self.ruling.perp()
    }

    /// The angle of `point` about the axis, as a cylinder of the axis reads
    /// it, and its `(h, ρ)`.
    fn meridian(&self, point: DVec3) -> (f64, DVec2) {
        let from = point - self.origin;
        let h = from.dot(self.axis);
        let rho = (from - self.axis * h).length();
        (from.dot(self.v).atan2(from.dot(self.u)), DVec2::new(h, rho))
    }

    /// The height along the axis at length `l` along the ruling.
    pub fn height_at(&self, l: f64) -> f64 {
        self.foot.x + l * self.ruling.x
    }

    /// The distance from the axis at length `l` along the ruling, negative
    /// beyond the apex.
    pub fn radius_at(&self, l: f64) -> f64 {
        self.foot.y + l * self.ruling.y
    }

    pub fn point(&self, at: DVec2) -> DVec3 {
        self.origin + self.axis * self.height_at(at.y) + self.radial(at.x) * self.radius_at(at.y)
    }

    /// `(θ, l)`, with `θ` in `(-π, π]`.
    pub fn parameters(&self, point: DVec3) -> DVec2 {
        let (theta, meridian) = self.meridian(point);
        DVec2::new(theta, (meridian - self.foot).dot(self.ruling))
    }

    /// The cone's own normal at angle `theta`, of unit length whatever the
    /// length along the ruling, the apex included: `∂θ × ∂l` points along it.
    /// It points away from the axis only where the cone opens along it.
    pub fn normal(&self, theta: f64) -> DVec3 {
        self.radial(theta) * self.ruling.x - self.axis * self.ruling.y
    }

    /// The length along the ruling the apex stands at.
    pub fn apex_at(&self) -> f64 {
        -self.foot.y / self.ruling.y
    }

    /// The apex, which may stand far outside the box: read only where it
    /// stands within it.
    pub fn apex(&self) -> DVec3 {
        self.origin + self.axis * self.height_at(self.apex_at())
    }

    /// How far `point` stands from the nappe, positive on the side the
    /// normal points to: across the meridian line in front of the apex, and
    /// from the apex itself behind it, so that a point of the other nappe
    /// never reads as on this one.
    pub fn distance(&self, point: DVec3) -> f64 {
        let (_, meridian) = self.meridian(point);
        let from = meridian - self.foot;
        let across = self.across().dot(from);
        let behind = from.dot(self.ruling) - self.apex_at();
        if behind >= 0.0 {
            across
        } else {
            across.signum() * behind.hypot(across)
        }
    }

    /// The point of the nappe nearest `point` in its meridian half-plane.
    pub fn nearest(&self, point: DVec3) -> DVec3 {
        let at = self.parameters(point);
        self.point(DVec2::new(at.x, at.y.max(self.apex_at())))
    }

    /// The radius of the nappe at height `h`, negative beyond the apex.
    /// Asked only of a cone that is no disc: `w_h ≠ 0`.
    pub fn section(&self, h: f64) -> f64 {
        (self.across().dot(self.foot) + self.ruling.y * h) / self.ruling.x
    }

    /// The circle about the axis at height `h` and of `radius`: the very
    /// circle a cylinder of the axis has there.
    pub fn circle(&self, h: f64, radius: f64) -> Circle {
        Circle {
            center: self.origin + self.axis * h,
            axis: self.axis,
            radius,
            u: self.u,
            v: self.v,
        }
    }

    /// Whether `circle` lies on the nappe within `eps`: about the axis, its
    /// rim within `eps` of the meridian line.
    pub fn holds(&self, circle: &Circle, eps: f64) -> bool {
        let from = circle.center - self.origin;
        let h = from.dot(self.axis);
        circle.axis.cross(self.axis).length() <= Scale::RELATIVE
            && (from - self.axis * h).length() <= eps
            && self
                .across()
                .dot(DVec2::new(h, circle.radius) - self.foot)
                .abs()
                <= eps
    }

    /// Whether `line` runs along a ruling of the nappe within the tolerance
    /// across the box: read at either end of its stretch across the box, in
    /// the meridian half-plane that end stands in, never at the apex.
    pub fn rules(&self, line: &Line, scale: Scale) -> bool {
        let (eps, span) = (scale.eps(), 2.0 * scale.reach());
        let middle = line.parameter(self.origin);
        [middle - scale.reach(), middle + scale.reach()]
            .into_iter()
            .any(|t| {
                let from = line.point(t) - self.origin;
                let h = from.dot(self.axis);
                let radial = from - self.axis * h;
                let rho = radial.length();
                if rho <= eps {
                    return false;
                }
                let outward = radial / rho;
                let along = DVec2::new(line.direction.dot(self.axis), line.direction.dot(outward));
                self.axis.cross(outward).dot(line.direction).abs() * span <= eps
                    && self.across().dot(along).abs() * span <= eps
                    && self.across().dot(DVec2::new(h, rho) - self.foot).abs() <= eps
            })
    }

    /// The angle about the axis of the ruling a stretch of a line from `from`
    /// to `to` runs along: read at whichever end stands further from the
    /// axis, never at the apex, where the angle is rounding.
    pub fn ruling_angle(&self, line: &Line, from: f64, to: f64) -> f64 {
        let [one, other] = [from, to].map(|t| self.meridian(line.point(t)));
        if other.1.y > one.1.y { other.0 } else { one.0 }
    }

    /// Where the line through `origin` along `direction` crosses the nappe,
    /// in order: the roots of `w_h²|q|² = (c + w_ρ h)²` solved so that
    /// neither is found by taking two close numbers from each other, kept
    /// where they stand on this nappe rather than the other.
    pub fn roots(&self, origin: DVec3, direction: DVec3) -> [Option<f64>; 2] {
        let from = origin - self.origin;
        let (h, rise) = (from.dot(self.axis), direction.dot(self.axis));
        let (radial, speed) = (from - self.axis * h, direction - self.axis * rise);
        let [w_h, w_rho] = [self.ruling.x.abs(), self.ruling.y];
        let level = self.across().dot(self.foot) * self.ruling.x.signum();
        let reach = |h: f64| level + w_rho * self.ruling.x.signum() * h;
        let k = reach(h);
        let slope = w_rho * self.ruling.x.signum() * rise;
        let square = (w_h * speed.length() - slope.abs()) * (w_h * speed.length() + slope.abs());
        let half = w_h * w_h * radial.dot(speed) - k * slope;
        let rest = (w_h * radial.length() - k) * (w_h * radial.length() + k);
        let roots = if square == 0.0 {
            if half == 0.0 {
                [None, None]
            } else {
                [Some(-rest / (2.0 * half)), None]
            }
        } else {
            let discriminant = half * half - square * rest;
            let size = half * half + (square * rest).abs();
            if discriminant.abs() <= 16.0 * f64::EPSILON * size {
                return self.hair_apart(origin, direction, -half / square);
            }
            if discriminant < 0.0 {
                return [None, None];
            }
            let far = -(half + discriminant.sqrt().copysign(half));
            if far == 0.0 {
                [Some(0.0), None]
            } else {
                let [one, other] = [far / square, rest / far];
                [Some(one.min(other)), Some(one.max(other))]
            }
        };
        let kept: Vec<f64> = roots
            .into_iter()
            .flatten()
            .filter(|t| reach(h + rise * t) >= 0.0)
            .map(|t| self.polished(origin, direction, t).0)
            .collect();
        [kept.first().copied(), kept.get(1).copied()]
    }

    /// The root on this nappe of a line crossing a cone a hair from a disc,
    /// where the two roots, one on each nappe, stand so close that rounding
    /// takes the discriminant for nought, or below: the double root `middle`,
    /// which the nappe test then reads on either nappe or neither, taken onto
    /// the nappe and kept only where it lands on it to rounding. A line
    /// touching a cone is found at its touch, once.
    fn hair_apart(&self, origin: DVec3, direction: DVec3, middle: f64) -> [Option<f64>; 2] {
        let (root, off) = self.polished(origin, direction, middle);
        let along = (origin - self.origin).length() + (direction * root).length();
        let on = off.abs() <= 16.0 * f64::EPSILON * along.max(1.0);
        [on.then_some(root), None]
    }

    /// A root of the line through `origin` along `direction` taken onto the
    /// nappe by Newton's steps across its meridian line, and how far across
    /// that line it is left: the quadratic's two roots stand a hair apart on
    /// a cone a hair from a disc, one on each nappe, and its discriminant
    /// leaves them both that far off, where the distance across the meridian
    /// line is all but straight along the line.
    fn polished(&self, origin: DVec3, direction: DVec3, root: f64) -> (f64, f64) {
        let across = self.across();
        let rise = direction.dot(self.axis);
        let speed = direction - self.axis * rise;
        let off = |t: f64| {
            let from = origin + direction * t - self.origin;
            let h = from.dot(self.axis);
            let radial = from - self.axis * h;
            let rho = radial.length();
            let off = across.dot(DVec2::new(h, rho) - self.foot);
            let slope = across.x * rise + across.y * radial.dot(speed) / rho;
            (off, slope)
        };
        let mut best = root;
        let (mut least, mut slope) = off(root);
        for _ in 0..3 {
            if !slope.is_finite() || slope == 0.0 {
                break;
            }
            let next = best - least / slope;
            let (reached, onward) = off(next);
            if reached.abs() >= least.abs() {
                break;
            }
            (best, least, slope) = (next, reached, onward);
        }
        (best, least)
    }
}
