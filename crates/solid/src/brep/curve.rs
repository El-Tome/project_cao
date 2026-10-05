//! The curves an edge runs along, each with its own parameter: a length along
//! a line, an angle round a circle, and for the curve two perpendicular
//! cylinders meet along, the parameter `meet.rs` gives it.

use glam::DVec3;
use serde::{Deserialize, Serialize};

use super::surface::{Cylinder, canonical_direction};

/// The line through `origin`, its point nearest the world's origin, along
/// `direction`, of unit length and with its sign fixed. Its parameter is the
/// signed length from `origin`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub origin: DVec3,
    pub direction: DVec3,
}

/// The circle about `center` in the plane square to `axis`, its angle read
/// from `u` towards `v` as on the cylinder it lies on.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Circle {
    pub center: DVec3,
    pub axis: DVec3,
    pub radius: f64,
    pub u: DVec3,
    pub v: DVec3,
}

/// One closed component of the curve where two perpendicular cylinders meet.
/// Its evaluation lives in `meet.rs`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Meet {
    pub first: Cylinder,
    pub second: Cylinder,
    pub component: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Curve {
    Line(Line),
    Circle(Circle),
    Meet(Meet),
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

impl Curve {
    pub fn point(&self, t: f64) -> DVec3 {
        match self {
            Curve::Line(line) => line.point(t),
            Curve::Circle(circle) => circle.point(t),
            Curve::Meet(meet) => meet.point(t),
        }
    }

    /// The derivative of the point with respect to the parameter: the way the
    /// curve runs as its parameter grows, and how fast.
    pub fn derivative(&self, t: f64) -> DVec3 {
        match self {
            Curve::Line(line) => line.direction,
            Curve::Circle(circle) => circle.derivative(t),
            Curve::Meet(meet) => meet.derivative(t),
        }
    }

    /// The parameter of a point lying on the curve. On a closed curve, the one
    /// in the curve's first period.
    pub fn parameter(&self, point: DVec3) -> f64 {
        match self {
            Curve::Line(line) => line.parameter(point),
            Curve::Circle(circle) => circle.parameter(point),
            Curve::Meet(meet) => meet.parameter(point),
        }
    }

    /// How far the parameter runs before a closed curve comes back on itself.
    pub fn period(&self) -> Option<f64> {
        match self {
            Curve::Line(_) => None,
            Curve::Circle(_) => Some(std::f64::consts::TAU),
            Curve::Meet(meet) => meet.period(),
        }
    }
}

#[cfg(test)]
mod tests;
