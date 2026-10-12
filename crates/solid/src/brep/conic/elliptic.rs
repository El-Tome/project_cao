//! The conic a plane cuts an elliptic wall along (#571), built from the
//! wall's ruled form as `built.rs` builds it on a circular one: the ruling
//! at the wall's angle `θ` runs along the axis `a` from the section's point
//! `F(θ) = O + u·A cos θ + v·B sin θ`, and meets the plane `n·x = c` at the
//! height `h(θ) = (c − n·O − R cos(θ − φ))/(n·a)`, `R` and `φ` the length
//! and the angle of `(A n·u, B n·v)`: `R` is how far the wall reaches along
//! the plane's normal across the axis. The cut is
//!
//! ```text
//! P(θ) = C₀ + U cos θ + V sin θ,  U = A (u − a (n·u)/(n·a)),  V = B (v − a (n·v)/(n·a))
//! ```
//!
//! the image of a circle, along which the wall's angle runs as `θ_A + √k·τ`
//! from the anchor's: the law of a circular wall's cut. Neither `U`, `V`
//! nor `C₀` is formed, which a plane a hair off along the axis sends a
//! billion reaches away: the anchor, the tangent and the bend are read off
//! `F`, `h` and their derivatives, which keep their digits.
//!
//! The vertices stand where `tan 2θ = 2 U·V / (|U|² − |V|²)`, both sides
//! times `(n·a)²` so that nothing is divided. The cut is anchored at the end
//! of its major axis nearest the world's origin, where `k` is one over its
//! minor half-axis squared, whatever way the wall stands; where that end
//! stands beyond [`LONG`] reaches, it is split into two arms where `h` turns,
//! at `θ − φ` nought and π, each anchored where it crosses the height of the
//! wall's origin.

use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use super::built::sides;
use super::{Arm, Conic, LONG, Round};
use crate::brep::scale::Scale;
use crate::brep::surface::{EllipticCylinder, Plane};

/// A plane and an elliptic wall read in the wall's frame: `rise` the
/// plane's normal along the axis, `level` the plane's offset less the
/// origin's, `reach` and `phi` the length and the angle of `(A n·u, B n·v)`.
pub(super) struct Slice {
    plane: Plane,
    wall: EllipticCylinder,
    rise: f64,
    level: f64,
    reach: f64,
    pub phi: f64,
}

impl Slice {
    pub fn of(plane: &Plane, wall: &EllipticCylinder) -> Slice {
        let normal = plane.normal;
        let (along_u, along_v) = (
            wall.major * normal.dot(wall.u),
            wall.minor * normal.dot(wall.v),
        );
        Slice {
            plane: *plane,
            wall: *wall,
            rise: normal.dot(wall.axis),
            level: plane.offset() - normal.dot(wall.origin),
            reach: along_u.hypot(along_v),
            phi: along_v.atan2(along_u),
        }
    }

    /// The height along the axis at which the ruling at the wall's angle
    /// `theta` meets the plane.
    fn height(&self, theta: f64) -> f64 {
        (self.level - self.reach * (theta - self.phi).cos()) / self.rise
    }

    /// The point of the cut on the ruling at the wall's angle `theta`.
    pub fn point(&self, theta: f64) -> DVec3 {
        self.wall.point(DVec2::new(theta, self.height(theta)))
    }

    /// The wall's angle at an end of the cut's major axis.
    fn vertex(&self) -> f64 {
        let normal = self.plane.normal;
        let (major, minor) = (self.wall.major, self.wall.minor);
        let (along_u, along_v) = (
            major * normal.dot(self.wall.u),
            minor * normal.dot(self.wall.v),
        );
        let across = 2.0 * along_u * along_v;
        let apart = self.rise * self.rise * (major - minor) * (major + minor)
            + (along_u - along_v) * (along_u + along_v);
        across.atan2(apart) / 2.0
    }

    /// The conic anchored at the cut's point at the wall's angle `theta`.
    fn anchored(&self, theta: f64, arm: Arm) -> Conic {
        let wall = &self.wall;
        let (sin, cos) = theta.sin_cos();
        let (psi_sin, psi_cos) = (theta - self.phi).sin_cos();
        let running = wall.u * (-wall.major * sin)
            + wall.v * (wall.minor * cos)
            + wall.axis * (self.reach * psi_sin / self.rise);
        let out = wall.u * (wall.major * cos) + wall.v * (wall.minor * sin)
            - wall.axis * (self.reach * psi_cos / self.rise);
        let k = 1.0 / running.length_squared();
        Conic {
            plane: self.plane,
            round: Round::Elliptic(*wall),
            anchor: self.point(theta),
            tangent: running.normalize(),
            bend: -out * k,
            k,
            arm,
        }
    }

    /// The components of the cut: one curve anchored at the end of its
    /// major axis nearest the world's origin where that stands within
    /// [`LONG`] reaches, or else its two arms, each anchored where it
    /// crosses the height of the wall's origin; none where the plane runs
    /// along the axis, or where the arms cross that height nowhere and never
    /// meet the box.
    pub fn cut(&self, scale: Scale) -> Vec<Conic> {
        if self.rise == 0.0 {
            return Vec::new();
        }
        let vertex = self.vertex();
        let nearest = [vertex, vertex + PI]
            .into_iter()
            .map(|theta| (theta, self.point(theta).length()))
            .min_by(|one, other| one.1.total_cmp(&other.1));
        let Some((nearest, away)) = nearest else {
            return Vec::new();
        };
        if away <= LONG * scale.reach() {
            return vec![self.anchored(nearest, Arm::Whole)];
        }
        let cosine = self.level / self.reach;
        if cosine.is_nan() || cosine.abs() > 1.0 {
            return Vec::new();
        }
        let psi = cosine.acos();
        sides(psi)
            .iter()
            .map(|side| {
                let theta = self.phi + f64::from(*side) * psi;
                self.anchored(theta, Arm::Split { side: *side })
            })
            .collect()
    }
}

/// How far round the wall `point` stands from the angle the plane's normal
/// reaches furthest across the axis at: the side of an arm's split it is on.
pub(super) fn turned(plane: &Plane, wall: &EllipticCylinder, point: DVec3) -> f64 {
    wall.parameters(point).x - Slice::of(plane, wall).phi
}
