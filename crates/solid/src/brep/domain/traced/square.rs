//! A curve seen on a wall square to one of its own walls, an elliptic one
//! among them (#571): a rim taken for an arc of the curve the two walls meet
//! along (decision 6) is seen as that curve, as a circle square to a
//! cylinder's axis is; and the curve two walls meet along, taken for a rim
//! on its plane, is seen as the section that plane cuts the wall square to
//! it along.

use super::{met, parallel, square};
use crate::brep::conic::{Conic, Round};
use crate::brep::curve::{Circle, Curve, Meet, Wall};
use crate::brep::surface::{Cylinder, EllipticCylinder, Plane, Surface};
use crate::brep::trace::Trace;

/// A stretch of a section of an elliptic wall — a rim, its plane square to
/// the wall's axis — lying on a wall square to that axis, seen there as the
/// curve the two walls meet along. None for any other conic or surface.
pub(super) fn rim(
    curve: &Curve,
    conic: &Conic,
    surface: &Surface,
    stretch: [f64; 2],
    eps: f64,
) -> Option<Trace> {
    let Round::Elliptic(oval) = conic.round else {
        return None;
    };
    let other = Wall::of(surface)?;
    if !parallel(conic.plane.normal, oval.axis) || !square(other.axis(), oval.axis) {
        return None;
    }
    met(curve, Wall::Elliptic(oval), other, stretch, eps)
}

/// A stretch of a circle lying on an elliptic wall square to its own,
/// seen there as the curve its own wall meets the elliptic one along. None
/// where the two stand at any other angle.
pub(super) fn circle(
    curve: &Curve,
    circle: &Circle,
    oval: &EllipticCylinder,
    stretch: [f64; 2],
    eps: f64,
) -> Option<Trace> {
    if !square(circle.axis, oval.axis) {
        return None;
    }
    let own = Wall::Cylinder(Cylinder::about(circle.center, circle.axis, circle.radius));
    met(curve, own, Wall::Elliptic(*oval), stretch, eps)
}

/// The wall whose section a plane is read through when it carries a stretch
/// of the curve two walls meet along: the first wall, for two cylinders, as
/// it always was; with an elliptic wall among them, the one square to the
/// plane, whose section it cuts.
pub(super) fn round(meet: &Meet, plane: &Plane) -> Round {
    let elliptic =
        matches!(meet.first, Wall::Elliptic(_)) || matches!(meet.second, Wall::Elliptic(_));
    let wall = if elliptic && parallel(meet.second.axis(), plane.normal) {
        meet.second
    } else {
        meet.first
    };
    match wall {
        Wall::Cylinder(cylinder) => Round::Cylinder(cylinder),
        Wall::Elliptic(oval) => Round::Elliptic(oval),
    }
}
