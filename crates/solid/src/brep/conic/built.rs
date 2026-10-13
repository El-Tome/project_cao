//! A conic built from the wall's own ruled form, `P(θ) = F(θ) + l(θ)·d(θ)`:
//! `F` the foot of the ruling at the wall's angle `θ`, `d` its direction, `l`
//! how far along it the plane stands. A cylinder is the cone whose rulings
//! run along its axis from its foot circle.
//!
//! With `n` the plane's normal, `a` the axis, `k_n = n·a`, `μ` the length of
//! `n` across the axis and `ψ = θ − φ` the angle from it, the ruling at `ψ`
//! meets the plane where `l = −s_F(ψ)/D(ψ)`, with `D(ψ) = A + B cos ψ`, and
//! stands `ρ = C/D(ψ)` from the axis: `C`, which is `w_ρ` times how far the
//! apex stands below the plane, holds all along. The cut is on the wall's own
//! nappe where `D` has the sign of `C`; its kind is the sign of
//! `Δ = (A − B)(A + B)`, which is whether `D` changes sign. Its vertices are
//! at `ψ = 0` and `ψ = π`, on the plane of symmetry. Nothing here forms the
//! apex, which may stand a hundred million reaches away.
//!
//! At an anchor at `ψ`, with `S² = D² + μ² sin²ψ`, the universal form's
//! tangent, bend and `k` are closed forms read off the section's quadratic
//! part and the wall's normal there, never off its constant term, which
//! squares the reach near an apex:
//!
//! ```text
//! T = (D·R'(θ) + μ sin ψ·d(θ)) sign(D) / S
//! k = Δ D² / (C² S²)
//! N = (w_h (A cos ψ + B)·e₁ − Δ sin ψ·e₂) D / (C S²)
//! ```
//!
//! `e₁` the plane's direction along the axis, `e₂` across it.

use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use super::elliptic::Slice;
use super::{Arm, Conic, LONG, Round};
use crate::brep::scale::Scale;
use crate::brep::surface::Plane;

/// The plane and the wall read in the wall's frame: the wall's foot and
/// ruling in `(h, ρ)`, a cylinder's `(0, r)` and `(1, 0)`; `φ` the angle of
/// the plane's normal about the axis, `μ` its length across the axis and
/// `rise` along it; `A`, `B`, `C` and `Δ` as above, and `s = s_F − μ f_ρ cos ψ`.
pub(super) struct Section {
    plane: Plane,
    round: Round,
    origin: DVec3,
    axis: DVec3,
    u: DVec3,
    v: DVec3,
    foot: DVec2,
    ruling: DVec2,
    pub phi: f64,
    mu: f64,
    rise: f64,
    a: f64,
    b: f64,
    pub c: f64,
    s: f64,
    delta: f64,
}

impl Section {
    /// The plane and a circular wall read in the wall's frame; none for an
    /// elliptic wall, whose foot is no circle (`elliptic.rs`).
    pub fn of(plane: &Plane, round: &Round) -> Option<Section> {
        let (origin, axis, u, v, foot, ruling) = match *round {
            Round::Cylinder(cylinder) => (
                cylinder.origin,
                cylinder.axis,
                cylinder.u,
                cylinder.v,
                DVec2::new(0.0, cylinder.radius),
                DVec2::X,
            ),
            Round::Cone(cone) => (
                cone.origin,
                cone.axis,
                cone.u,
                cone.v,
                cone.foot,
                cone.ruling,
            ),
            Round::Elliptic(_) => return None,
        };
        let normal = plane.normal;
        let rise = normal.dot(axis);
        let (across_u, across_v) = (normal.dot(u), normal.dot(v));
        let mu = across_u.hypot(across_v);
        let [w_h, w_rho] = ruling.to_array();
        let s = normal.dot(origin) + rise * foot.x - plane.offset();
        let (a, b) = (rise * w_h, w_rho * mu);
        Some(Section {
            plane: *plane,
            round: *round,
            origin,
            axis,
            u,
            v,
            foot,
            ruling,
            phi: across_v.atan2(across_u),
            mu,
            rise,
            a,
            b,
            c: rise * w_h * foot.y - w_rho * s,
            s,
            delta: (a - b) * (a + b),
        })
    }

    fn radial(&self, theta: f64) -> DVec3 {
        let (sin, cos) = theta.sin_cos();
        self.u * cos + self.v * sin
    }

    fn turned(&self, theta: f64) -> DVec3 {
        let (sin, cos) = theta.sin_cos();
        self.v * cos - self.u * sin
    }

    fn direction(&self, theta: f64) -> DVec3 {
        self.axis * self.ruling.x + self.radial(theta) * self.ruling.y
    }

    /// `D(ψ)`, read from the vertex nearer `ψ` so that it keeps its digits
    /// where it vanishes close to one.
    pub fn level(&self, psi: f64) -> f64 {
        let (sin, cos) = (psi / 2.0).sin_cos();
        if psi.cos() >= 0.0 {
            (self.a + self.b) - 2.0 * self.b * sin * sin
        } else {
            (self.a - self.b) + 2.0 * self.b * cos * cos
        }
    }

    /// Whether the cut at `ψ` stands on the wall's own nappe.
    pub fn own(&self, psi: f64) -> bool {
        self.level(psi) * self.c > 0.0
    }

    /// The point of the cut on the ruling at the wall's angle `theta`, whose
    /// part across the axis is that angle's radial direction times a length.
    pub fn point(&self, theta: f64) -> DVec3 {
        let psi = theta - self.phi;
        let across = self.s + self.mu * self.foot.y * psi.cos();
        let length = -across / self.level(psi);
        self.origin
            + self.axis * self.foot.x
            + self.radial(theta) * self.foot.y
            + self.direction(theta) * length
    }

    /// The angle `ψ ≥ 0` at which the cut crosses the circle of the wall at
    /// length `length` along its rulings — `s_F + l·D = 0` — mirrored by
    /// `−ψ`; none where it crosses it nowhere.
    pub fn crossing(&self, length: f64) -> Option<f64> {
        let across = self.mu * self.foot.y + length * self.b;
        let cosine = -(self.s + length * self.a) / across;
        (across != 0.0 && cosine.abs() <= 1.0).then(|| cosine.acos())
    }

    /// The conic anchored at the cut's point at `ψ`.
    fn anchored(&self, psi: f64, arm: Arm) -> Conic {
        let theta = self.phi + psi;
        let level = self.level(psi);
        let (sin, cos) = psi.sin_cos();
        let lean = self.mu * sin;
        let spread = level * level + lean * lean;
        let tangent = (self.turned(theta) * level + self.direction(theta) * lean).normalize()
            * level.signum();
        let along = self.axis * self.mu - self.radial(self.phi) * self.rise;
        let across = self.turned(self.phi);
        let bend = (along * (self.ruling.x * (self.a * cos + self.b))
            - across * (self.delta * sin))
            * (level / (self.c * spread));
        Conic {
            plane: self.plane,
            round: self.round,
            anchor: self.point(theta),
            tangent,
            bend,
            k: self.delta / self.c * (level / self.c) * (level / spread),
            arm,
        }
    }

    /// The two rulings a plane through the apex holds, where `D` vanishes,
    /// each from its foot along its direction: a cut through an apex outside
    /// the box, which bends nowhere.
    fn rulings(&self) -> Vec<Conic> {
        if self.b == 0.0 || (self.a / self.b).abs() > 1.0 {
            return Vec::new();
        }
        let psi = (-self.a / self.b).acos();
        sides(psi)
            .iter()
            .map(|side| {
                let theta = self.phi + f64::from(*side) * psi;
                Conic {
                    plane: self.plane,
                    round: self.round,
                    anchor: self.origin
                        + self.axis * self.foot.x
                        + self.radial(theta) * self.foot.y,
                    tangent: self.direction(theta),
                    bend: DVec3::ZERO,
                    k: 0.0,
                    arm: Arm::Split { side: *side },
                }
            })
            .collect()
    }
}

impl Conic {
    /// The components of the cut `plane` makes on `round`'s own nappe, built
    /// for a plane neither square to the axis nor along it: one curve
    /// anchored at its vertex nearest the world's origin where that stands
    /// within [`LONG`] reaches, or else its arms, each anchored where it
    /// crosses the wall's foot circle — an arm crossing it nowhere never
    /// meets the box, and is not built. Nothing where the plane meets only
    /// the other nappe, or runs along the axis of a cylinder. An elliptic
    /// wall is cut by `elliptic.rs`, square to its axis as well.
    pub fn cut(plane: &Plane, round: &Round, scale: Scale) -> Vec<Conic> {
        let section = match round {
            Round::Elliptic(wall) => return Slice::of(plane, wall).cut(scale),
            Round::Cylinder(_) | Round::Cone(_) => Section::of(plane, round),
        };
        let Some(section) = section else {
            return Vec::new();
        };
        if let Round::Cylinder(_) = round
            && section.rise == 0.0
        {
            return Vec::new();
        }
        if section.c == 0.0 {
            return section.rulings();
        }
        let nearest = [0.0, PI]
            .into_iter()
            .filter(|psi| section.own(*psi))
            .map(|psi| (psi, section.point(section.phi + psi).length()))
            .min_by(|one, other| one.1.total_cmp(&other.1));
        let Some((vertex, away)) = nearest else {
            return Vec::new();
        };
        if away <= LONG * scale.reach() {
            return vec![section.anchored(vertex, Arm::Whole)];
        }
        let Some(psi) = section.crossing(0.0) else {
            return Vec::new();
        };
        sides(psi)
            .iter()
            .filter(|side| section.own(f64::from(**side) * psi))
            .map(|side| section.anchored(f64::from(*side) * psi, Arm::Split { side: *side }))
            .collect()
    }
}

/// The sides of the plane of symmetry an angle `ψ` and its mirror `−ψ` stand
/// on: one alone where the two are one, on the plane itself.
pub(super) fn sides(psi: f64) -> &'static [i8] {
    if psi == 0.0 || psi == PI {
        &[1]
    } else {
        &[1, -1]
    }
}
