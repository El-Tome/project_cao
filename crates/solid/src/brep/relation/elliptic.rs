//! An elliptic wall against another surface (#571). Two are one surface
//! (decision 1) where their axes are parallel and within the tolerance of
//! each other across the box, their half-axes within it of each other, and
//! the turn between their major half-axes moves the ends of one by no more
//! than it. Any other pair is unsupported, which declines a boolean unless
//! the two faces stand clear of each other.

use super::{Relation, parallel};
use crate::brep::scale::Scale;
use crate::brep::surface::EllipticCylinder;

pub(super) fn walls(one: &EllipticCylinder, other: &EllipticCylinder, scale: Scale) -> Relation {
    let eps = scale.eps();
    let between = other.origin - one.origin;
    let across = (between - one.axis * one.axis.dot(between)).length();
    let alike = parallel(one.axis, other.axis, scale)
        && across <= eps
        && (one.major - other.major).abs() <= eps
        && (one.minor - other.minor).abs() <= eps
        && one.u.cross(other.u).length() * one.major <= eps;
    if alike {
        Relation::Same { agree: true }
    } else {
        Relation::Unsupported
    }
}
