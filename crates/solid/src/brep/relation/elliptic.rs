//! An elliptic wall against another surface (#571).
//!
//! A plane along its axis is read in the wall's section, where its line
//! stands `c` from the axis along its normal `n` and the wall reaches
//! `R = √((A n·u)² + (B n·v)²)` along it, at the angle `φ` of `(A n·u,
//! B n·v)`: apart where `|c| − R` exceeds the tolerance, touching along the
//! ruling at `φ` or `φ + π` within it — decided once, on that gap — and
//! otherwise cutting along the two rulings at `φ ± acos(c/R)`, where
//! `R cos(θ − φ) = c`. Each line runs along the axis through its point of
//! the plane, as a circular wall's do. Any other plane, square to the axis
//! or at a slant, cuts the conic `conic/elliptic.rs` builds.
//!
//! Two elliptic walls are one surface (decision 1) where their axes are
//! parallel and within the tolerance of each other across the box, their
//! half-axes within it of each other, and the turn between their major
//! half-axes moves the ends of one by no more than it. Any other pair is
//! unsupported, which declines a boolean unless the two faces stand clear of
//! each other.

use super::{Relation, parallel, sorted, square};
use crate::brep::conic::{Conic, Round};
use crate::brep::curve::Line;
use crate::brep::scale::Scale;
use crate::brep::surface::{EllipticCylinder, Plane};

pub(super) fn plane_and_elliptic(plane: &Plane, wall: &EllipticCylinder, scale: Scale) -> Relation {
    if !square(plane.normal, wall.axis, scale) {
        let conics = Conic::cut(plane, &Round::Elliptic(*wall), scale);
        return if conics.is_empty() {
            Relation::Apart
        } else {
            Relation::Conic(conics)
        };
    }
    let eps = scale.eps();
    let normal = plane.normal;
    let (along_u, along_v) = (
        wall.major * normal.dot(wall.u),
        wall.minor * normal.dot(wall.v),
    );
    let reach = along_u.hypot(along_v);
    let offset = -plane.distance(wall.origin);
    let gap = offset.abs() - reach;
    if gap > eps {
        return Relation::Apart;
    }
    let phi = along_v.atan2(along_u);
    let ruling = |theta: f64| {
        let point = wall.point(glam::DVec2::new(theta, 0.0));
        Line::through(point - normal * plane.distance(point), wall.axis)
    };
    if gap >= -eps {
        let toward = if offset >= 0.0 {
            0.0
        } else {
            std::f64::consts::PI
        };
        return Relation::Tangent(ruling(phi + toward));
    }
    let spread = (offset / reach).acos();
    Relation::Lines(sorted([ruling(phi - spread), ruling(phi + spread)]))
}

pub(super) fn walls(one: &EllipticCylinder, other: &EllipticCylinder, scale: Scale) -> Relation {
    if parallel(one.axis, other.axis, scale) && one.within(other, scale.eps()) {
        Relation::Same { agree: true }
    } else {
        Relation::Unsupported
    }
}
