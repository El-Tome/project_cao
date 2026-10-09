//! A plane at a slant to a wall (#552): neither square to a cylinder's axis
//! nor along it, or neither square to a cone's axis nor holding it — the
//! pairs `at_a_slant` picks out, which `planar.rs` and `conical.rs` hand
//! over once they have read every row before it, decided here and nowhere
//! else.
//!
//! A cylinder is cut along an ellipse, whole or in its two arms: never apart,
//! never touched, since a plane along a ruling is square to the axis.
//!
//! A cone is read in its own frame — `n` the plane's normal, `a` the axis,
//! `(w_h, w_ρ)` the ruling, `μ` the length of `n` across the axis and `φ`
//! its angle about it — where the ruling at the angle `φ + ψ` runs along the
//! plane as much as `D(ψ) = A + B cos ψ` says, `A = (n·a) w_h`, `B = w_ρ μ`.
//! Rows are read in order, space before angle, as a cylinder is decided by
//! its gap before anything else:
//!
//! - the apex in the box and within the tolerance of the plane: the plane
//!   holds the two rulings where `D` vanishes when it is steeper than the
//!   slope by more than the angle `parallel` reads, `|A| − B < −tol`; it
//!   touches the cone along the ruling where `|D|` is least when it is
//!   within that angle of the slope; it meets the cone at its apex alone
//!   when flatter;
//! - the cut on the cone's own nappe all within the tolerance of the axis, a
//!   ring a hair across: the point of the axis on the plane, as `revolved.rs`
//!   reads a circle within the tolerance of the axis;
//! - otherwise the conic on the own nappe, whole or in arms, and nothing
//!   where the plane meets only the other nappe.

use glam::DVec2;

use super::revolved::in_the_box;
use super::{Relation, sorted};
use crate::brep::conic::{Conic, Round};
use crate::brep::curve::Line;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder, Plane};

pub(super) fn plane_and_cylinder(plane: &Plane, cylinder: &Cylinder, scale: Scale) -> Relation {
    built(Conic::cut(plane, &Round::Cylinder(*cylinder), scale))
}

pub(super) fn plane_and_cone(cone: &Cone, plane: &Plane, scale: Scale) -> Relation {
    let eps = scale.eps();
    let rise = plane.normal.dot(cone.axis);
    let across = DVec2::new(plane.normal.dot(cone.u), plane.normal.dot(cone.v));
    let [w_h, w_rho] = cone.ruling.to_array();
    let (a, b) = (rise * w_h, w_rho * across.length());
    let phi = across.y.atan2(across.x);
    let apex = cone.apex();
    if in_the_box(apex, scale) && plane.distance(apex).abs() <= eps {
        let tol = eps / (2.0 * scale.reach());
        let gap = a.abs() - b;
        if gap > tol {
            return Relation::Apex(apex);
        }
        if gap >= -tol {
            let least = if a >= 0.0 {
                phi + std::f64::consts::PI
            } else {
                phi
            };
            return Relation::Ruling {
                line: ruling(cone, least),
                apex: Some(apex),
            };
        }
        let psi = (-a / b).acos();
        return Relation::Rulings {
            lines: sorted([ruling(cone, phi - psi), ruling(cone, phi + psi)]),
            apex: Some(apex),
        };
    }
    let [h, rho] = cone.foot.to_array();
    let below = plane.normal.dot(cone.origin) + rise * h - plane.offset();
    let c = rise * w_h * rho - w_rho * below;
    let ring = a.abs() > b && a * c > 0.0 && c.abs() <= eps * (a.abs() - b);
    if ring {
        let height = (plane.offset() - plane.normal.dot(cone.origin)) / rise;
        let on_the_axis = cone.origin + cone.axis * height;
        return if in_the_box(on_the_axis, scale) {
            Relation::Apex(on_the_axis)
        } else {
            Relation::Apart
        };
    }
    built(Conic::cut(plane, &Round::Cone(*cone), scale))
}

/// The ruling at the angle `theta`, through its foot rather than through the
/// apex, as `conical.rs` builds a ruling of a plane holding the axis.
fn ruling(cone: &Cone, theta: f64) -> Line {
    Line::through(
        cone.point(DVec2::new(theta, 0.0)),
        cone.axis * cone.ruling.x + cone.radial(theta) * cone.ruling.y,
    )
}

/// The conics cut, or nothing where none meets the box.
fn built(conics: Vec<Conic>) -> Relation {
    if conics.is_empty() {
        Relation::Apart
    } else {
        Relation::Conic(conics)
    }
}
