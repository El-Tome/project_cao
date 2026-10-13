//! The curves an edge runs along, each with its own parameter: a length along
//! a line, an angle round a circle, for the curve two perpendicular walls
//! meet along the parameter `meet.rs` gives it, and for the conic a plane cuts
//! a wall along at a slant the universal parameter of `conic.rs`.

use glam::DVec3;
use serde::{Deserialize, Serialize};

use super::conic::Conic;
use super::surface::{Cylinder, EllipticCylinder, Surface, canonical_direction};

/// The line through `origin`, its point nearest the world's origin, along
/// `direction`, of unit length and with its sign fixed. Its parameter is the
/// signed length from `origin`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub origin: DVec3,
    pub direction: DVec3,
}

/// The circle about `center` in the plane square to `axis`, its angle read
/// from `u` towards `v` as on the cylinder or the cone it lies on.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Circle {
    pub center: DVec3,
    pub axis: DVec3,
    pub radius: f64,
    pub u: DVec3,
    pub v: DVec3,
}

/// One closed component of the curve where two perpendicular walls meet,
/// circular or elliptic (#571). Its evaluation lives in `meet.rs`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Meet {
    pub first: Wall,
    pub second: Wall,
    pub component: u8,
}

/// A wall the curve two perpendicular walls meet along runs round.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Wall {
    Cylinder(Cylinder),
    Elliptic(EllipticCylinder),
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Curve {
    Line(Line),
    Circle(Circle),
    Meet(Meet),
    Conic(Conic),
}

impl Line {
    pub fn through(point: DVec3, direction: DVec3) -> Line {
        let (direction, _) = canonical_direction(direction);
        Line {
            origin: point - direction * direction.dot(point),
            direction,
        }
    }

    pub fn point(&self, t: f64) -> DVec3 {
        self.origin + self.direction * t
    }

    pub fn parameter(&self, point: DVec3) -> f64 {
        (point - self.origin).dot(self.direction)
    }
}

impl Circle {
    /// The circle a cylinder has at height `h`.
    pub fn on(cylinder: &Cylinder, h: f64) -> Circle {
        Circle {
            center: cylinder.origin + cylinder.axis * h,
            axis: cylinder.axis,
            radius: cylinder.radius,
            u: cylinder.u,
            v: cylinder.v,
        }
    }

    pub fn point(&self, theta: f64) -> DVec3 {
        let (sin, cos) = theta.sin_cos();
        self.center + (self.u * cos + self.v * sin) * self.radius
    }

    pub fn derivative(&self, theta: f64) -> DVec3 {
        let (sin, cos) = theta.sin_cos();
        (self.v * cos - self.u * sin) * self.radius
    }

    /// The angle of `point`, in `(-π, π]`.
    pub fn parameter(&self, point: DVec3) -> f64 {
        let from = point - self.center;
        from.dot(self.v).atan2(from.dot(self.u))
    }
}

impl Wall {
    /// The wall a surface is, if it is one.
    pub fn of(surface: &Surface) -> Option<Wall> {
        match surface {
            Surface::Cylinder(cylinder) => Some(Wall::Cylinder(*cylinder)),
            Surface::Elliptic(wall) => Some(Wall::Elliptic(*wall)),
            Surface::Plane(_) | Surface::Cone(_) => None,
        }
    }

    pub fn surface(&self) -> Surface {
        match self {
            Wall::Cylinder(cylinder) => Surface::Cylinder(*cylinder),
            Wall::Elliptic(wall) => Surface::Elliptic(*wall),
        }
    }

    pub fn origin(&self) -> DVec3 {
        match self {
            Wall::Cylinder(cylinder) => cylinder.origin,
            Wall::Elliptic(wall) => wall.origin,
        }
    }

    pub fn axis(&self) -> DVec3 {
        match self {
            Wall::Cylinder(cylinder) => cylinder.axis,
            Wall::Elliptic(wall) => wall.axis,
        }
    }
}

impl From<Cylinder> for Wall {
    fn from(cylinder: Cylinder) -> Wall {
        Wall::Cylinder(cylinder)
    }
}

impl From<EllipticCylinder> for Wall {
    fn from(wall: EllipticCylinder) -> Wall {
        Wall::Elliptic(wall)
    }
}

impl Curve {
    pub fn point(&self, t: f64) -> DVec3 {
        match self {
            Curve::Line(line) => line.point(t),
            Curve::Circle(circle) => circle.point(t),
            Curve::Meet(meet) => meet.point(t),
            Curve::Conic(conic) => conic.point(t),
        }
    }

    /// The derivative of the point with respect to the parameter: the way the
    /// curve runs as its parameter grows, and how fast.
    pub fn derivative(&self, t: f64) -> DVec3 {
        match self {
            Curve::Line(line) => line.direction,
            Curve::Circle(circle) => circle.derivative(t),
            Curve::Meet(meet) => meet.derivative(t),
            Curve::Conic(conic) => conic.derivative(t),
        }
    }

    /// The parameter of a point lying on the curve. On a closed curve, the one
    /// in the curve's first period.
    pub fn parameter(&self, point: DVec3) -> f64 {
        match self {
            Curve::Line(line) => line.parameter(point),
            Curve::Circle(circle) => circle.parameter(point),
            Curve::Meet(meet) => meet.parameter(point),
            Curve::Conic(conic) => conic.parameter(point),
        }
    }

    /// How far the parameter runs before a closed curve comes back on itself.
    pub fn period(&self) -> Option<f64> {
        match self {
            Curve::Line(_) => None,
            Curve::Circle(_) => Some(std::f64::consts::TAU),
            Curve::Meet(meet) => meet.period(),
            Curve::Conic(conic) => conic.period(),
        }
    }
}

#[cfg(test)]
mod tests;
