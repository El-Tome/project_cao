//! The chart a sphere and a torus share: `(θ, φ)`, the angle about the axis
//! and the angle along the meridian circle, read about the circle's centre
//! `(h_c, R)` in the meridian half-plane `(h, ρ)`. A sphere is the chart at
//! `R = 0`; nothing here reads the seam a torus cuts its chart at.

use glam::{DVec2, DVec3};

use crate::brep::curve::Circle;
use crate::brep::scale::Scale;

/// How many roundings of the reach a root may stand off the surface and
/// still be read as on it.
pub(super) const ROUNDINGS: f64 = 16.0;

/// A surface of revolution whose meridian is the circle about `center`, in
/// `(h, ρ)`, of `radius`; `origin`, `axis`, `u` and `v` as every cylinder of
/// the axis has them.
#[derive(Clone, Copy, Debug)]
pub(super) struct Round {
    pub origin: DVec3,
    pub axis: DVec3,
    pub u: DVec3,
    pub v: DVec3,
    pub center: DVec2,
    pub radius: f64,
}

impl Round {
    pub fn radial(&self, theta: f64) -> DVec3 {
        let (sin, cos) = theta.sin_cos();
        self.u * cos + self.v * sin
    }

    /// The `(h, ρ)` of the meridian circle's point at `phi`. Past the
    /// quarter turn `ρ` is read as `(R − r) + 2r cos²(φ/2)`, which takes no
    /// two close numbers from each other where a horn's point stands.
    pub fn meridian_point(&self, phi: f64) -> DVec2 {
        let (sin, cos) = phi.sin_cos();
        let (r, big) = (self.radius, self.center.y);
        let rho = if cos < 0.0 {
            let half = (phi / 2.0).cos();
            (big - r) + 2.0 * r * half * half
        } else {
            big + r * cos
        };
        DVec2::new(self.center.x + r * sin, rho)
    }

    pub fn point(&self, at: DVec2) -> DVec3 {
        let meridian = self.meridian_point(at.y);
        self.origin + self.axis * meridian.x + self.radial(at.x) * meridian.y
    }

    /// The angle of `point` about the axis, as a cylinder of the axis reads
    /// it, and its `(h, ρ)`.
    pub fn seen(&self, point: DVec3) -> (f64, DVec2) {
        let from = point - self.origin;
        let h = from.dot(self.axis);
        let rho = (from - self.axis * h).length();
        (from.dot(self.v).atan2(from.dot(self.u)), DVec2::new(h, rho))
    }

    /// The angle along the meridian circle `point` stands at, in `(-π, π]`.
    pub fn phi(&self, point: DVec3) -> f64 {
        let (_, meridian) = self.seen(point);
        let from = meridian - self.center;
        from.x.atan2(from.y)
    }

    /// `axis·sin φ + radial(θ)·cos φ`: of unit length everywhere, pointing
    /// away from the meridian circle's centre.
    pub fn normal(&self, at: DVec2) -> DVec3 {
        let (sin, cos) = at.y.sin_cos();
        self.axis * sin + self.radial(at.x) * cos
    }

    /// How far `point` stands from the meridian circle, in its half-plane,
    /// positive outside it.
    pub fn level(&self, point: DVec3) -> f64 {
        let (_, meridian) = self.seen(point);
        (meridian - self.center).length() - self.radius
    }

    pub fn circle(&self, h: f64, radius: f64) -> Circle {
        Circle {
            center: self.origin + self.axis * h,
            axis: self.axis,
            radius,
            u: self.u,
            v: self.v,
        }
    }

    /// Whether `circle` is a latitude circle within `eps`: about the axis,
    /// its rim within `eps` of the meridian circle.
    pub fn holds(&self, circle: &Circle, eps: f64) -> bool {
        let from = circle.center - self.origin;
        let h = from.dot(self.axis);
        circle.axis.cross(self.axis).length() <= Scale::RELATIVE
            && (from - self.axis * h).length() <= eps
            && ((DVec2::new(h, circle.radius) - self.center).length() - self.radius).abs() <= eps
    }

    /// Whether `circle` is a meridian circle within `eps`: in a plane holding
    /// the axis, about the meridian circle's centre, of its radius.
    pub fn meridian(&self, circle: &Circle, eps: f64) -> bool {
        let from = circle.center - self.origin;
        let h = from.dot(self.axis);
        let rho = (from - self.axis * h).length();
        circle.axis.dot(self.axis).abs() <= Scale::RELATIVE
            && from.dot(circle.axis).abs() <= eps
            && (h - self.center.x).abs() <= eps
            && (rho - self.center.y.abs()).abs() <= eps
            && (circle.radius - self.radius).abs() <= eps
    }

    /// How far along the unit `direction` the whole surface reaches, from
    /// its lowest point to its highest.
    pub fn extent(&self, direction: DVec3) -> [f64; 2] {
        let along = direction.dot(self.axis);
        let across = (direction - self.axis * along).length();
        let at = direction.dot(self.origin + self.axis * self.center.x);
        let spread = across * self.center.y.abs() + self.radius;
        [at - spread, at + spread]
    }
}

/// A root of the line through `origin` along `direction` taken onto the
/// surface by Newton's steps on its level, and how far off it is left.
pub(super) fn polished(round: &Round, origin: DVec3, direction: DVec3, root: f64) -> (f64, f64) {
    let rise = direction.dot(round.axis);
    let speed = direction - round.axis * rise;
    let off = |t: f64| {
        let point = origin + direction * t;
        let (_, meridian) = round.seen(point);
        let from = meridian - round.center;
        let distance = from.length();
        let level = distance - round.radius;
        let radial = point - round.origin - round.axis * meridian.x;
        let spread = if meridian.y == 0.0 {
            0.0
        } else {
            radial.dot(speed) / meridian.y
        };
        let slope = if distance == 0.0 {
            0.0
        } else {
            (from.x * rise + from.y * spread) / distance
        };
        (level, slope)
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
