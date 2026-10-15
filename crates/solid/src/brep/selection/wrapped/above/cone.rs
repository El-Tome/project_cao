//! Which side of a cone a plane stands on, and of the plane the cone: along
//! the lobe between two rulings the plane holds, along the ruling it
//! touches, or all round where the plane holds the axis.

use glam::DVec3;

use super::middle_of_the_lobe;
use crate::brep::curve::Line;
use crate::brep::surface::{Cone, Surface};

/// Whether the second of a cone and a plane through its apex at a slant
/// stands on the side of the first its own normal points to, all along the
/// lobe of the cone between the two rulings the plane holds that `place`
/// stands on: read along the ruling at the lobe's middle, where the two
/// stand furthest apart. The cone stands on the side of the plane that
/// ruling leaves the apex towards. The plane runs inside the cone along the
/// lobe whose middle ruling it holds the axis on the other side of — a
/// chord of each circle about the axis, short of the axis — and outside it
/// along the other, beyond the rulings: the side the cone's normal points
/// to where the cone opens along its axis.
pub(super) fn across_the_cone_s_lobe(
    first: &Surface,
    second: &Surface,
    lines: &[Line; 2],
    place: DVec3,
) -> Option<bool> {
    let (cone, plane) = match (first, second) {
        (Surface::Cone(cone), Surface::Plane(plane))
        | (Surface::Plane(plane), Surface::Cone(cone)) => (cone, plane),
        (Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_), _) => {
            return None;
        }
    };
    let rulings = lines.map(|line| ruling_angle(cone, &line));
    let middle = middle_of_the_lobe(cone.parameters(place).x, rulings);
    let leaving = cone.axis * cone.ruling.x + cone.radial(middle) * cone.ruling.y;
    let towards = plane.normal.dot(leaving);
    match first {
        Surface::Cone(_) => {
            let axis_side = plane.normal.dot(cone.axis * cone.ruling.x);
            let outside = towards * axis_side > 0.0;
            Some(outside == (cone.ruling.x > 0.0))
        }
        Surface::Plane(_) | Surface::Cylinder(_) | Surface::Elliptic(_) => Some(towards > 0.0),
    }
}

/// The angle about a cone's axis of a ruling given as a line through its
/// apex, read a reach off the apex along the line, on whichever side runs
/// on the cone's own nappe rather than the other.
fn ruling_angle(cone: &Cone, line: &Line) -> f64 {
    let apex = cone.apex();
    let length = 1.0 + apex.abs().max_element();
    let at = line.parameter(apex);
    let [ahead, behind] = [length, -length].map(|run| line.point(at + run));
    let own = if cone.distance(ahead).abs() <= cone.distance(behind).abs() {
        ahead
    } else {
        behind
    };
    cone.parameters(own).x
}

/// Whether the second of a cone and a plane touching it along a ruling
/// stands on the side of the first its own normal points to: the plane
/// stands outside the cone, where the cone's normal points when it opens
/// along its axis, and the cone on the side of the plane its axis runs
/// towards from the apex.
pub(super) fn touching_the_cone(first: &Surface, second: &Surface) -> Option<bool> {
    match (first, second) {
        (Surface::Cone(cone), Surface::Plane(_)) => Some(cone.ruling.x > 0.0),
        (Surface::Plane(plane), Surface::Cone(cone)) => {
            Some(plane.normal.dot(cone.axis) * cone.ruling.x > 0.0)
        }
        (Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_), _) => {
            None
        }
    }
}

/// Whether a plane holding a cone's axis stands on the side of the cone its
/// own normal points to: on the axis side all round, the same on either
/// lobe the two rulings part the cone into, where the cone's normal points
/// when it opens against its axis. A place of the plane stands at the very
/// angle of a ruling, on neither lobe, and the cone stands on both sides of
/// the plane there: none.
pub(super) fn across_the_rulings(first: &Surface, second: &Surface) -> Option<bool> {
    match (first, second) {
        (Surface::Cone(cone), Surface::Plane(_)) => Some(cone.ruling.x < 0.0),
        (Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_), _) => {
            None
        }
    }
}
