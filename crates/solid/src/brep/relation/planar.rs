//! A plane against a plane, and a plane against a cylinder.

use super::{Relation, parallel, sorted, square};
use crate::brep::curve::{Circle, Line};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane};

/// Parallel, one surface when their offsets are within the tolerance. Across
/// each other, the line through the point of both nearest the origin, from a
/// formula symmetric in the two.
pub(super) fn planes(one: &Plane, other: &Plane, scale: Scale) -> Relation {
    if parallel(one.normal, other.normal, scale) {
        let agree = one.normal.dot(other.normal) > 0.0;
        let facing = if agree { 1.0 } else { -1.0 };
        return if (one.offset() - facing * other.offset()).abs() <= scale.eps() {
            Relation::Same { agree }
        } else {
            Relation::Apart
        };
    }
    let direction = one.normal.cross(other.normal);
    let point = (other.normal.cross(direction) * one.offset()
        + direction.cross(one.normal) * other.offset())
        / direction.length_squared();
    Relation::Line(Line::through(point, direction))
}

/// Square to the axis, the circle at the plane's height. Along the axis, the
/// lines the plane cuts, or the one it touches: through the foot of the axis
/// on the plane and along the axis, whenever the axis stands within the
/// tolerance of a radius from the plane.
pub(super) fn plane_and_cylinder(plane: &Plane, cylinder: &Cylinder, scale: Scale) -> Relation {
    if parallel(plane.normal, cylinder.axis, scale) {
        let height =
            (plane.offset() - plane.normal.dot(cylinder.origin)) / plane.normal.dot(cylinder.axis);
        return Relation::Circle(Circle::on(cylinder, height));
    }
    if !square(plane.normal, cylinder.axis, scale) {
        return Relation::Unsupported;
    }
    let away = plane.distance(cylinder.origin);
    let foot = cylinder.origin - plane.normal * away;
    let gap = away.abs() - cylinder.radius;
    if gap > scale.eps() {
        return Relation::Apart;
    }
    if gap >= -scale.eps() {
        return Relation::Tangent(Line::through(foot, cylinder.axis));
    }
    let across = plane.normal.cross(cylinder.axis).normalize()
        * ((cylinder.radius - away.abs()) * (cylinder.radius + away.abs())).sqrt();
    Relation::Lines(sorted([
        Line::through(foot - across, cylinder.axis),
        Line::through(foot + across, cylinder.axis),
    ]))
}
