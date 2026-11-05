//! The wall a cone's rulings are read against, as a quadric: a cylinder or
//! an elliptic wall `Σ ((x − origin)·e / s)² = 1`, or a cone of another axis
//! `w_h² |x⊥|² − (k + w_ρ x·a)² = 0` read off its meridian line without its
//! apex, which may stand far away, `k = w_h ρ₀ − w_ρ h₀` at its foot.

use glam::{DVec2, DVec3};

use super::Terms;
use crate::brep::curve::Wall;
use crate::brep::surface::Cone;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::brep::meet) struct Quadric {
    pub origin: DVec3,
    pub axis: DVec3,
    /// The directions square to the axis its angle is read in, each with
    /// what a unit of the angle's circle is worth along it.
    pub halves: [(DVec3, f64); 2],
    pub cone: Option<Cone>,
}

impl Quadric {
    pub fn of(wall: &Wall) -> Quadric {
        match wall {
            Wall::Cylinder(cylinder) => Quadric {
                origin: cylinder.origin,
                axis: cylinder.axis,
                halves: [(cylinder.u, cylinder.radius), (cylinder.v, cylinder.radius)],
                cone: None,
            },
            Wall::Elliptic(oval) => Quadric {
                origin: oval.origin,
                axis: oval.axis,
                halves: [(oval.u, oval.major), (oval.v, oval.minor)],
                cone: None,
            },
            Wall::Cone(cone) => Quadric {
                origin: cone.origin,
                axis: cone.axis,
                halves: [(cone.u, 1.0), (cone.v, 1.0)],
                cone: Some(*cone),
            },
        }
    }

    fn form(&self, x: DVec3, y: DVec3) -> f64 {
        self.halves
            .iter()
            .map(|(e, s)| x.dot(*e) * y.dot(*e) / (s * s))
            .sum()
    }

    /// `A`, `B`, `C` of the line `F + l·d` against the quadric, with their
    /// derivatives, from `F`, `d` and their derivatives.
    pub fn along(&self, [foot, foot1]: [DVec3; 2], [d, d1]: [DVec3; 2]) -> Terms {
        let g = foot - self.origin;
        match self.cone {
            None => {
                let q = |x: DVec3, y: DVec3| self.form(x, y);
                Terms {
                    a: [q(d, d), 2.0 * q(d1, d)],
                    b: [2.0 * q(g, d), 2.0 * (q(foot1, d) + q(g, d1))],
                    c: [q(g, g) - 1.0, 2.0 * q(foot1, g)],
                }
            }
            Some(cone) => {
                let a = self.axis;
                let (wh, wr) = (cone.ruling.x * cone.ruling.x, cone.ruling.y);
                let k = cone.ruling.x * cone.foot.y - wr * cone.foot.x;
                let flat = |x: DVec3| x - a * x.dot(a);
                let (gf, g1f, df, d1f) = (flat(g), flat(foot1), flat(d), flat(d1));
                let lift = k + wr * g.dot(a);
                Terms {
                    a: [
                        wh * df.dot(df) - wr * wr * d.dot(a).powi(2),
                        2.0 * (wh * df.dot(d1f) - wr * wr * d.dot(a) * d1.dot(a)),
                    ],
                    b: [
                        2.0 * (wh * gf.dot(df) - wr * lift * d.dot(a)),
                        2.0 * (wh * (g1f.dot(df) + gf.dot(d1f))
                            - wr * (wr * foot1.dot(a) * d.dot(a) + lift * d1.dot(a))),
                    ],
                    c: [
                        wh * gf.dot(gf) - lift * lift,
                        2.0 * (wh * gf.dot(g1f) - lift * wr * foot1.dot(a)),
                    ],
                }
            }
        }
    }

    /// Whether `point` stands inside the solid the quadric bounds.
    pub fn inside(&self, point: DVec3) -> bool {
        let x = point - self.origin;
        match self.cone {
            None => self.form(x, x) < 1.0,
            Some(cone) => {
                let (h, rho) = (
                    x.dot(self.axis),
                    (x - self.axis * x.dot(self.axis)).length(),
                );
                let line =
                    cone.ruling.x * cone.foot.y - cone.ruling.y * cone.foot.x + cone.ruling.y * h;
                (cone.ruling.x * rho).abs() < line.abs() && line * cone.ruling.x > 0.0
            }
        }
    }

    /// Whether `point`, on the quadric, stands on the wall itself: on a cone
    /// its own nappe rather than the other.
    pub fn holds(&self, point: DVec3) -> bool {
        match self.cone {
            None => true,
            Some(cone) => cone.radius_at(cone.parameters(point).y) >= 0.0,
        }
    }

    /// A direction square to the axis read in the coordinates the wall's
    /// own angle is measured in.
    pub fn flat(&self, v: DVec3) -> DVec2 {
        let [(u, a), (w, b)] = self.halves;
        DVec2::new(v.dot(u) / a, v.dot(w) / b)
    }
}

impl Quadric {
    /// The wall's own second parameter at `point` and its rate along
    /// `speed`: the height along the axis, or on a cone the length along
    /// its ruling.
    pub fn rise(&self, point: DVec3, speed: DVec3) -> [f64; 2] {
        let x = point - self.origin;
        let (h, h1) = (x.dot(self.axis), speed.dot(self.axis));
        let Some(cone) = self.cone else {
            return [h, h1];
        };
        let flat = x - self.axis * h;
        let rho = flat.length();
        let rho1 = if rho > 0.0 {
            flat.dot(speed) / rho
        } else {
            0.0
        };
        let at = DVec2::new(h, rho) - cone.foot;
        [at.dot(cone.ruling), cone.ruling.dot(DVec2::new(h1, rho1))]
    }
}
