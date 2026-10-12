//! The elliptic cylinder (#571): the wall an ellipse of a profile is raised
//! into, read in `(θ, h)` — `θ` the eccentric angle from its major half-axis
//! `u` towards `v`, `h` the height along its axis. It stands beside the
//! circular cylinder rather than generalising it, so that every circular
//! path keeps its bits.

use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};

use super::canonical_direction;

/// The points `origin + u·major·cos θ + v·minor·sin θ + axis·h`: `origin`
/// the point of the axis nearest the world's origin, `axis` with its sign
/// fixed, `u` the major half-axis's direction with its sign fixed, `v`
/// `axis × u`, and `major ≥ minor > 0`. Parallel elliptic walls do not share
/// where their angles start, as circular ones do: each starts from its own
/// major half-axis.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EllipticCylinder {
    pub origin: DVec3,
    pub axis: DVec3,
    pub u: DVec3,
    pub v: DVec3,
    pub major: f64,
    pub minor: f64,
}

/// Halvings of the foot point's bracket: past the fifty-three bits of a
/// double the midpoint stands on an end, and the search stops there.
const HALVINGS: usize = 1100;

impl EllipticCylinder {
    /// The elliptic cylinder about the line through `center` along `axis`,
    /// one half-axis `first`, its direction and its length, and the other,
    /// square to it, `second` long: canonical, the same bits whichever
    /// half-axis comes first and whichever way the axis and the half-axes
    /// point.
    pub fn about(center: DVec3, axis: DVec3, first: DVec3, second: f64) -> EllipticCylinder {
        let (axis, _) = canonical_direction(axis);
        let across = first - axis * axis.dot(first);
        let length = across.length();
        let (major, minor, way) = if length >= second {
            (length, second, across)
        } else {
            (second, length, axis.cross(across))
        };
        let (u, _) = canonical_direction(way);
        EllipticCylinder {
            origin: center - axis * axis.dot(center),
            axis,
            u,
            v: axis.cross(u),
            major,
            minor,
        }
    }

    pub fn point(&self, at: DVec2) -> DVec3 {
        let (sin, cos) = at.x.sin_cos();
        self.origin + self.u * (self.major * cos) + self.v * (self.minor * sin) + self.axis * at.y
    }

    /// `(θ, h)`, with `θ` in `(-π, π]`.
    pub fn parameters(&self, point: DVec3) -> DVec2 {
        let from = point - self.origin;
        DVec2::new(
            (from.dot(self.v) / self.minor).atan2(from.dot(self.u) / self.major),
            from.dot(self.axis),
        )
    }

    /// The wall's normal at angle `theta`, of unit length, pointing away
    /// from the axis.
    pub fn normal(&self, theta: f64) -> DVec3 {
        let (sin, cos) = theta.sin_cos();
        (self.u * (self.minor * cos) + self.v * (self.major * sin)).normalize()
    }

    /// How far the wall reaches from its axis along `way`, square to the
    /// axis and of unit length: a plane along the axis at `c` from it
    /// stands `|c|` less this from the wall.
    pub fn support(&self, way: DVec3) -> f64 {
        (self.major * way.dot(self.u)).hypot(self.minor * way.dot(self.v))
    }

    /// How fast a point of the section runs as its angle grows: the length of
    /// the section's tangent at `theta`, the minor half-axis at either end of
    /// the major one and the major at either end of the minor.
    pub fn speed(&self, theta: f64) -> f64 {
        let (sin, cos) = theta.sin_cos();
        (self.major * sin).hypot(self.minor * cos)
    }

    /// The wall moved by `by`, canonical: its axis, its half-axes and their
    /// directions kept to the bit.
    pub fn moved(&self, by: DVec3) -> EllipticCylinder {
        let center = self.origin + by;
        EllipticCylinder {
            origin: center - self.axis * self.axis.dot(center),
            ..*self
        }
    }

    /// How far another elliptic wall about a parallel axis stands from this
    /// one: the offset of its axis, the differences of its half-axes, and how
    /// far the turn of its major half-axis moves the ends of this one's.
    pub fn gap(&self, other: &EllipticCylinder) -> f64 {
        let between = other.origin - self.origin;
        let across = between - self.axis * self.axis.dot(between);
        across.length()
            + (self.major - other.major).abs()
            + (self.minor - other.minor).abs()
            + self.u.cross(other.u).length() * self.major
    }

    /// The point of the section's ellipse nearest `at`, read in `(u, v)` from
    /// the axis, and how far it stands: by Eberly's robust method ("Distance
    /// from a point to an ellipse", 2013) — the place folded into the first
    /// quadrant, the foot's multiplier found by halving on the stretch where
    /// it is the one root, a place on an axis of the section read apart.
    pub fn foot(&self, at: DVec2) -> (DVec2, f64) {
        let (a, b) = (self.major, self.minor);
        let (y0, y1) = (at.x.abs(), at.y.abs());
        let (x0, x1) = if y1 > 0.0 {
            if y0 > 0.0 {
                let (z0, z1) = (y0 / a, y1 / b);
                let g = z0 * z0 + z1 * z1 - 1.0;
                if g == 0.0 {
                    (y0, y1)
                } else {
                    let ratio = (a / b) * (a / b);
                    let s = root(ratio, z0, z1, g);
                    (ratio * y0 / (s + ratio), y1 / (s + 1.0))
                }
            } else {
                (0.0, b)
            }
        } else {
            let (numerator, denominator) = (a * y0, (a - b) * (a + b));
            if numerator < denominator {
                let share = numerator / denominator;
                (a * share, b * ((1.0 - share) * (1.0 + share)).sqrt())
            } else {
                (a, 0.0)
            }
        };
        let foot = DVec2::new(x0.copysign(at.x), x1.copysign(at.y));
        (foot, (foot - at).length())
    }

    /// The point of the wall nearest `point`: its foot on the section's
    /// ellipse at its own height.
    pub fn nearest(&self, point: DVec3) -> DVec3 {
        let from = point - self.origin;
        let (foot, _) = self.foot(DVec2::new(from.dot(self.u), from.dot(self.v)));
        self.origin + self.u * foot.x + self.v * foot.y + self.axis * from.dot(self.axis)
    }

    /// How far `point` stands outside the wall, negative inside: from its
    /// foot on the section's ellipse.
    pub fn distance(&self, point: DVec3) -> f64 {
        let from = point - self.origin;
        let at = DVec2::new(from.dot(self.u), from.dot(self.v));
        let (_, length) = self.foot(at);
        let inside = (at.x / self.major).powi(2) + (at.y / self.minor).powi(2) < 1.0;
        if inside { -length } else { length }
    }
}

/// The root `s` of `(r z₀/(s + r))² + (z₁/(s + 1))² = 1` above `−1`, where
/// the left side falls, by halving: `g` its value at nought, which says
/// which side of nought the root stands.
fn root(ratio: f64, z0: f64, z1: f64, g: f64) -> f64 {
    let n0 = ratio * z0;
    let mut low = z1 - 1.0;
    let mut high = if g < 0.0 { 0.0 } else { n0.hypot(z1) - 1.0 };
    let mut s = 0.0;
    for _ in 0..HALVINGS {
        s = (low + high) / 2.0;
        if s == low || s == high {
            break;
        }
        let (one, other) = (n0 / (s + ratio), z1 / (s + 1.0));
        let value = one * one + other * other - 1.0;
        if value > 0.0 {
            low = s;
        } else if value < 0.0 {
            high = s;
        } else {
            break;
        }
    }
    s
}

#[cfg(test)]
mod tests;
