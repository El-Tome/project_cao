//! The surfaces the kernel knows, in a canonical form: the same surface
//! reached by two roads comes out with the same numbers, and parallel
//! cylinders and cones start their angles from the same direction.

mod cone;

use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};

pub use cone::Cone;

/// The plane of points `x` with `normal · x = offset`, read in `(s, t)` along
/// `u` and `v` from `origin`, its point nearest the world's origin.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plane {
    pub origin: DVec3,
    pub normal: DVec3,
    pub u: DVec3,
    pub v: DVec3,
}

/// The points at `radius` from the line through `origin` along `axis`, read in
/// `(θ, h)`: the angle from `u` towards `v`, and the height along the axis.
/// `origin` is the point of the axis nearest the world's origin.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cylinder {
    pub origin: DVec3,
    pub axis: DVec3,
    pub radius: f64,
    pub u: DVec3,
    pub v: DVec3,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Surface {
    Plane(Plane),
    Cylinder(Cylinder),
    Cone(Cone),
}

/// A direction of unit length with its sign fixed: its largest component, the
/// first of them on a tie, is positive. Largest rather than first, so that a
/// plane tilted by a hair off a plane of the origin keeps the same sign.
///
/// Returns the direction, and whether it had to be turned round.
pub fn canonical_direction(direction: DVec3) -> (DVec3, bool) {
    let unit = direction.normalize();
    let largest = (0..3)
        .max_by(|&left, &right| {
            unit[left]
                .abs()
                .total_cmp(&unit[right].abs())
                .then(right.cmp(&left))
        })
        .unwrap_or(0);
    if unit[largest] < 0.0 {
        (-unit, true)
    } else {
        (unit, false)
    }
}

/// The world axis least along `direction`, made square to it: a reference read
/// off the direction alone, so that two parallel surfaces share it.
pub fn reference(direction: DVec3) -> DVec3 {
    let least = (0..3)
        .min_by(|&left, &right| {
            direction[left]
                .abs()
                .total_cmp(&direction[right].abs())
                .then(left.cmp(&right))
        })
        .unwrap_or(0);
    let axis = DVec3::AXES[least];
    (axis - direction * direction.dot(axis)).normalize()
}

impl Plane {
    /// The plane through `point` square to `normal`, canonical, and whether
    /// `normal` points against the canonical normal.
    pub fn through(point: DVec3, normal: DVec3) -> (Plane, bool) {
        let (normal, turned) = canonical_direction(normal);
        let u = reference(normal);
        let plane = Plane {
            origin: normal * normal.dot(point),
            normal,
            u,
            v: normal.cross(u),
        };
        (plane, turned)
    }

    pub fn offset(&self) -> f64 {
        self.normal.dot(self.origin)
    }

    /// How far `point` stands on the side the normal points to.
    pub fn distance(&self, point: DVec3) -> f64 {
        self.normal.dot(point) - self.offset()
    }

    pub fn point(&self, at: DVec2) -> DVec3 {
        self.origin + self.u * at.x + self.v * at.y
    }

    pub fn parameters(&self, point: DVec3) -> DVec2 {
        let from = point - self.origin;
        DVec2::new(from.dot(self.u), from.dot(self.v))
    }
}

impl Cylinder {
    /// The cylinder about the line through `point` along `axis`, canonical.
    pub fn about(point: DVec3, axis: DVec3, radius: f64) -> Cylinder {
        let (axis, _) = canonical_direction(axis);
        let u = reference(axis);
        Cylinder {
            origin: point - axis * axis.dot(point),
            axis,
            radius,
            u,
            v: axis.cross(u),
        }
    }

    /// The direction square to the axis at angle `theta`: the surface's
    /// normal there, pointing away from the axis.
    pub fn radial(&self, theta: f64) -> DVec3 {
        let (sin, cos) = theta.sin_cos();
        self.u * cos + self.v * sin
    }

    pub fn point(&self, at: DVec2) -> DVec3 {
        self.origin + self.radial(at.x) * self.radius + self.axis * at.y
    }

    /// `(θ, h)`, with `θ` in `(-π, π]`.
    pub fn parameters(&self, point: DVec3) -> DVec2 {
        let from = point - self.origin;
        DVec2::new(
            from.dot(self.v).atan2(from.dot(self.u)),
            from.dot(self.axis),
        )
    }

    /// How far `point` stands outside the cylinder, negative inside.
    pub fn distance(&self, point: DVec3) -> f64 {
        let from = point - self.origin;
        (from - self.axis * from.dot(self.axis)).length() - self.radius
    }
}

impl Surface {
    pub fn point(&self, at: DVec2) -> DVec3 {
        match self {
            Surface::Plane(plane) => plane.point(at),
            Surface::Cylinder(cylinder) => cylinder.point(at),
            Surface::Cone(cone) => cone.point(at),
        }
    }

    pub fn parameters(&self, point: DVec3) -> DVec2 {
        match self {
            Surface::Plane(plane) => plane.parameters(point),
            Surface::Cylinder(cylinder) => cylinder.parameters(point),
            Surface::Cone(cone) => cone.parameters(point),
        }
    }

    /// The surface's own normal at `at`: a plane's normal, a cylinder's radial
    /// direction, a cone's normal along its ruling. A face turns it round
    /// when its matter lies the other way.
    pub fn normal(&self, at: DVec2) -> DVec3 {
        match self {
            Surface::Plane(plane) => plane.normal,
            Surface::Cylinder(cylinder) => cylinder.radial(at.x),
            Surface::Cone(cone) => cone.normal(at.x),
        }
    }

    /// Signed: positive on the side the normal points to.
    pub fn distance(&self, point: DVec3) -> f64 {
        match self {
            Surface::Plane(plane) => plane.distance(point),
            Surface::Cylinder(cylinder) => cylinder.distance(point),
            Surface::Cone(cone) => cone.distance(point),
        }
    }

    /// The period of the first parameter, for a surface that closes on itself.
    pub fn period(&self) -> Option<f64> {
        match self {
            Surface::Plane(_) => None,
            Surface::Cylinder(_) | Surface::Cone(_) => Some(std::f64::consts::TAU),
        }
    }
}

#[cfg(test)]
mod tests;
