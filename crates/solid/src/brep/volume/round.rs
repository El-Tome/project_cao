//! The flux of the position through a face of a sphere or a torus (#576).
//!
//! On the surface the position's component along the normal is `h sin φ +
//! ρ cos φ + o(θ) cos φ`, with `o(θ)` the origin's component along the
//! radial direction, and the area element `r ρ(φ) dθ dφ`: the 1-form whose
//! derivative that is, `−[P(φ) + o(θ)·Q(φ)] dθ`, is defined all the way
//! round, so a band with no seam needs none. A meridian adds nothing; a
//! latitude circle has a closed form; anything else is left to the
//! quadrature.
//!
//! `P` and `Q` vanish at the angle `anchor`: a pole the face reaches or
//! holds, so that the stretch of that pole no edge runs along adds nothing,
//! as it must. A face reaching the other pole too is closed along it by the
//! floor the domain lays, which is read as a latitude circle.

use glam::{DVec2, DVec3};

use crate::brep::surface::{Sphere, Surface, Torus};

/// The meridian circle a face's flux is read about, and the frame it turns
/// in.
pub(super) struct Meridian {
    origin: DVec3,
    axis: DVec3,
    u: DVec3,
    v: DVec3,
    center: DVec2,
    radius: f64,
}

impl Meridian {
    pub(super) fn of(surface: &Surface) -> Option<Meridian> {
        match surface {
            Surface::Sphere(sphere) => Some(Meridian::of_sphere(sphere)),
            Surface::Torus(torus) => Some(Meridian::of_torus(torus)),
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
                None
            }
        }
    }

    fn of_sphere(sphere: &Sphere) -> Meridian {
        Meridian {
            origin: sphere.origin,
            axis: sphere.axis,
            u: sphere.u,
            v: sphere.v,
            center: DVec2::new(sphere.center, 0.0),
            radius: sphere.radius,
        }
    }

    fn of_torus(torus: &Torus) -> Meridian {
        Meridian {
            origin: torus.origin,
            axis: torus.axis,
            u: torus.u,
            v: torus.v,
            center: torus.center,
            radius: torus.radius,
        }
    }

    /// `∫ r ρ(φ) (h_c sin φ + r + R cos φ) dφ` from nought.
    fn p(&self, phi: f64) -> f64 {
        let (h_c, big, r) = (self.center.x, self.center.y, self.radius);
        let (sin, cos) = phi.sin_cos();
        r * (1.5 * big * r * phi - big * h_c * cos
            + (big * big + r * r) * sin
            + r * h_c / 2.0 * sin * sin
            + big * r / 4.0 * (2.0 * phi).sin())
    }

    /// `∫ r ρ(φ) cos φ dφ` from nought.
    fn q(&self, phi: f64) -> f64 {
        let (big, r) = (self.center.y, self.radius);
        r * (big * phi.sin() + r * phi / 2.0 + r / 4.0 * (2.0 * phi).sin())
    }

    /// What a straight trace in `(θ, φ)` adds: nothing along a meridian,
    /// the closed form along a latitude circle, and none for anything else.
    pub(super) fn straight(&self, anchor: f64, from: DVec2, to: DVec2) -> Option<f64> {
        if from.x == to.x {
            return Some(0.0);
        }
        if from.y != to.y {
            return None;
        }
        let (along_u, along_v) = (self.origin.dot(self.u), self.origin.dot(self.v));
        let (to_sin, to_cos) = to.x.sin_cos();
        let (from_sin, from_cos) = from.x.sin_cos();
        let moment = along_u * (to_sin - from_sin) - along_v * (to_cos - from_cos);
        let (p, q) = (
            self.p(from.y) - self.p(anchor),
            self.q(from.y) - self.q(anchor),
        );
        Some(-(p * (to.x - from.x) + q * moment))
    }

    /// The flux a curve passing through `point` at `speed` adds per unit of
    /// its parameter, from the same 1-form: nought on the axis, where the
    /// angle about it turns no area.
    pub(super) fn density(&self, anchor: f64, point: DVec3, speed: DVec3) -> f64 {
        let from = point - self.origin;
        let height = from.dot(self.axis);
        let radial = from - self.axis * height;
        let rho = radial.length();
        if rho == 0.0 {
            return 0.0;
        }
        let outward = radial / rho;
        let turning = speed.dot(self.axis.cross(outward)) / rho;
        let phi = (height - self.center.x).atan2(rho - self.center.y);
        let along = self.origin.dot(outward);
        let (p, q) = (self.p(phi) - self.p(anchor), self.q(phi) - self.q(anchor));
        -(p + along * q) * turning
    }
}
