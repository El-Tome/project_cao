//! Whether a surface carries a curve all along it, within the tolerance.

use glam::DVec3;

use crate::brep::conic::Round;
use crate::brep::curve::{Curve, Wall};
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;

/// Whether a surface carries a curve all along it, within the tolerance: a
/// line on a plane stands square to its normal and within the tolerance of
/// it, a line on a cylinder runs along its axis, a circle on a plane or a
/// cylinder stands square to it and about its axis, the curve two walls
/// meet along runs round one of its own two — not round a wall of one
/// radius a hair beside one of them, which carries it only over the stretch
/// an arc of it was taken for (decision 6). Nor does a plane a line crosses
/// at a grazing angle, within the tolerance of it over the stretch an edge
/// beside it covers: a block's side leaning 1e-3° off a cut's face carries
/// the cut's corner line over the 1.5e-4 between their crossing and the
/// block's top, and stands twenty tolerances off it at the cut's floor
/// (5541239472). A cone carries its rulings and its circles about its axis,
/// an elliptic wall its rulings and the curves it meets a wall square to it
/// along. A conic is carried by a surface
/// decided one with its plane or with its wall, and by nothing else.
pub(in crate::brep::combine) fn carries(surface: &Surface, curve: &Curve, scale: Scale) -> bool {
    let eps = scale.eps();
    let parallel =
        |one: DVec3, other: DVec3| one.cross(other).length() * 2.0 * scale.reach() <= eps;
    let square = |one: DVec3, other: DVec3| one.dot(other).abs() * 2.0 * scale.reach() <= eps;
    match (surface, curve) {
        (Surface::Plane(plane), Curve::Line(line)) => {
            square(line.direction, plane.normal) && plane.distance(line.origin).abs() <= eps
        }
        (Surface::Plane(plane), Curve::Circle(circle)) => {
            parallel(circle.axis, plane.normal) && plane.distance(circle.center).abs() <= eps
        }
        (Surface::Cylinder(cylinder), Curve::Line(line)) => parallel(line.direction, cylinder.axis),
        (Surface::Cylinder(cylinder), Curve::Circle(circle)) => {
            let from = circle.center - cylinder.origin;
            parallel(circle.axis, cylinder.axis)
                && (from - cylinder.axis * from.dot(cylinder.axis)).length() <= eps
                && (circle.radius - cylinder.radius).abs() <= eps
        }
        (Surface::Cylinder(cylinder), Curve::Meet(meet)) => {
            meet.own(&Wall::Cylinder(*cylinder), eps).is_some()
        }
        (Surface::Plane(_), Curve::Meet(_)) => false,
        (Surface::Cone(cone), Curve::Line(line)) => cone.rules(line, scale),
        (Surface::Cone(cone), Curve::Circle(circle)) => cone.holds(circle, eps),
        (Surface::Cone(_), Curve::Meet(_)) => false,
        (Surface::Elliptic(wall), Curve::Line(line)) => parallel(line.direction, wall.axis),
        (Surface::Elliptic(wall), Curve::Meet(meet)) => {
            meet.own(&Wall::Elliptic(*wall), eps).is_some()
        }
        (Surface::Elliptic(_), Curve::Circle(_)) => false,
        (_, Curve::Conic(conic)) => {
            let own = match (surface, conic.round) {
                (Surface::Plane(_), _) => Surface::Plane(conic.plane),
                (Surface::Cylinder(_), Round::Cylinder(cylinder)) => Surface::Cylinder(cylinder),
                (Surface::Cone(_), Round::Cone(cone)) => Surface::Cone(cone),
                (Surface::Elliptic(_), Round::Elliptic(wall)) => Surface::Elliptic(wall),
                (Surface::Cylinder(_), Round::Cone(_) | Round::Elliptic(_))
                | (Surface::Cone(_), Round::Cylinder(_) | Round::Elliptic(_))
                | (Surface::Elliptic(_), Round::Cylinder(_) | Round::Cone(_)) => {
                    return false;
                }
            };
            matches!(relation(surface, &own, scale), Relation::Same { .. })
        }
    }
}
