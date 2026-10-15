//! A sphere about a point of the axis it was turned about, read in `(θ, φ)`:
//! the angle about the axis as on a cylinder of the same axis, and the
//! latitude along the meridian, nought at the equator and a quarter turn at
//! either pole (#576).
//!
//! The sphere carries the turn's axis. It is round about every line through
//! its centre, and keeping the one it was turned about, with the `origin`,
//! `u` and `v` every cylinder of that axis has, makes every coaxial circle it
//! meets the very circle a cylinder or a cone of the axis has there. A
//! surface of another axis through its centre is exact mathematics, and
//! declined in this chart.

use std::f64::consts::FRAC_PI_2;

use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};

use super::round::{ROUNDINGS, Round, polished};
use super::{canonical_direction, reference};
use crate::brep::curve::Circle;

/// The points at `radius` from the point of the axis at height `center`
/// along `axis` from `origin`, the point of the axis nearest the world's
/// origin; `u` and `v` are those every cylinder of the axis has.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sphere {
    pub origin: DVec3,
    pub axis: DVec3,
    pub u: DVec3,
    pub v: DVec3,
    pub center: f64,
    pub radius: f64,
}

impl Sphere {
    /// The sphere an arc centred on the axis, at `h` along it from `point`,
    /// turns into about the line through `point` along `axis`: the same bits
    /// whichever way the axis points, and the `origin`, `u` and `v` a
    /// cylinder about the same line has.
    pub fn through(point: DVec3, axis: DVec3, h: f64, radius: f64) -> Sphere {
        let (axis, turned) = canonical_direction(axis);
        let shift = axis.dot(point);
        let u = reference(axis);
        Sphere {
            origin: point - axis * shift,
            axis,
            u,
            v: axis.cross(u),
            center: if turned { -h } else { h } + shift,
            radius,
        }
    }

    fn round(&self) -> Round {
        Round {
            origin: self.origin,
            axis: self.axis,
            u: self.u,
            v: self.v,
            center: DVec2::new(self.center, 0.0),
            radius: self.radius,
        }
    }

    /// The direction square to the axis at angle `theta`.
    pub fn radial(&self, theta: f64) -> DVec3 {
        self.round().radial(theta)
    }

    /// The centre.
    pub fn middle(&self) -> DVec3 {
        self.origin + self.axis * self.center
    }

    pub fn point(&self, at: DVec2) -> DVec3 {
        self.round().point(at)
    }

    /// `(θ, φ)`, with `θ` in `(-π, π]` and `φ` in `[-π/2, π/2]`.
    pub fn parameters(&self, point: DVec3) -> DVec2 {
        let (theta, meridian) = self.round().seen(point);
        DVec2::new(theta, (meridian.x - self.center).atan2(meridian.y))
    }

    /// The sphere's own normal at `at`, of unit length, the poles included:
    /// `∂θ × ∂φ` points along it, away from the centre.
    pub fn normal(&self, at: DVec2) -> DVec3 {
        self.round().normal(at)
    }

    /// How far `point` stands outside the sphere, negative inside.
    pub fn distance(&self, point: DVec3) -> f64 {
        point.distance(self.middle()) - self.radius
    }

    /// The point of the sphere nearest `point`.
    pub fn nearest(&self, point: DVec3) -> DVec3 {
        self.point(self.parameters(point))
    }

    /// The latitudes of the two poles, the chart's lines `φ = ∓π/2`.
    pub fn poles(&self) -> Vec<f64> {
        vec![-FRAC_PI_2, FRAC_PI_2]
    }

    /// The circle about the axis at height `h` and of `radius`: the very
    /// circle a cylinder of the axis has there.
    pub fn circle(&self, h: f64, radius: f64) -> Circle {
        self.round().circle(h, radius)
    }

    /// Whether `circle` is a latitude circle of the sphere within `eps`:
    /// about the axis, its rim within `eps` of the sphere.
    pub fn holds(&self, circle: &Circle, eps: f64) -> bool {
        self.round().holds(circle, eps)
    }

    /// Whether `circle` is a meridian of the sphere within `eps`: in a plane
    /// holding the axis, about the centre, of the sphere's radius.
    pub fn meridian(&self, circle: &Circle, eps: f64) -> bool {
        self.round().meridian(circle, eps)
    }

    /// How far along the unit `direction` the sphere reaches, from its lowest
    /// point to its highest.
    pub fn extent(&self, direction: DVec3) -> [f64; 2] {
        let at = self.middle().dot(direction);
        [at - self.radius, at + self.radius]
    }

    /// Where the line through `origin` along `direction` crosses the sphere,
    /// in order: the roots of its quadratic solved so that neither is found
    /// by taking two close numbers from each other. A line touching the
    /// sphere, whose discriminant rounding leaves either side of nought, is
    /// found at its touch, once, taken onto the sphere and kept only where it
    /// lands on it to rounding.
    pub fn roots(&self, origin: DVec3, direction: DVec3) -> [Option<f64>; 2] {
        let from = origin - self.middle();
        let speed = direction.length_squared();
        let radius = self.radius;
        let half = from.dot(direction);
        let rest = (from.length() - radius) * (from.length() + radius);
        let discriminant = half * half - speed * rest;
        let size = half * half + (speed * rest).abs();
        if discriminant.abs() <= ROUNDINGS * f64::EPSILON * size {
            let (root, off) = polished(&self.round(), origin, direction, -half / speed);
            let along = (origin - self.origin).length() + (direction * root).length();
            let on = off.abs() <= ROUNDINGS * f64::EPSILON * along.max(1.0);
            return [on.then_some(root), None];
        }
        if discriminant < 0.0 {
            return [None, None];
        }
        let far = -(half + discriminant.sqrt().copysign(half));
        let [one, other] = [far / speed, rest / far];
        [Some(one.min(other)), Some(one.max(other))]
    }
}

#[cfg(test)]
mod tests;
