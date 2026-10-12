//! Which side of a surface another stands on, read off the pair as it was
//! decided: along the line they touch along, along the stretch between two
//! lines they cross along, about a cone; and measured where they stand apart.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::ROUNDING;
use crate::brep::curve::Line;
use crate::brep::relation::{Relation, at_a_slant};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder, Surface};

/// Whether the second of two surfaces `related` so at `scale` stands on the
/// side of the first its own normal points to: along the line they touch
/// along, all along the stretch between the two lines they cross along that
/// `place` stands on, all round a cone a plane holding its axis cuts, on the
/// lobe of a cone between the two rulings a plane through its apex at a
/// slant holds that `place` stands on, along the ruling a plane touches a
/// cone along; or at `place` where two neither touching nor crossing along
/// two lines — a plane and a wall, two parallel walls, two planes, a cone
/// and anything, a plane and a wall crossing along a conic — stand further
/// apart than `rounding`; none otherwise.
pub(in crate::brep::selection) fn stands_above(
    first: &Surface,
    second: &Surface,
    related: &Relation,
    place: DVec3,
    scale: Scale,
    rounding: f64,
) -> Option<bool> {
    match related {
        Relation::Lines(lines) => across_the_lobe(first, second, lines, place),
        Relation::Tangent(_) => touching_above(first, second),
        Relation::Rulings { lines, .. } if at_a_slant(first, second, scale) => {
            across_the_cone_s_lobe(first, second, lines, place)
        }
        Relation::Rulings { .. } => across_the_rulings(first, second),
        Relation::Ruling { .. } => touching_the_cone(first, second),
        Relation::Conic(_) => measured(first, second, place, rounding),
        Relation::Apart
        | Relation::Same { .. }
        | Relation::Line(_)
        | Relation::Circle(_)
        | Relation::Meet(_)
        | Relation::Apex(_)
        | Relation::Unsupported => match (first, second) {
            (Surface::Cylinder(first), Surface::Cylinder(second)) => {
                above_at(first, second, place, rounding)
            }
            (Surface::Plane(plane), Surface::Cylinder(cylinder)) => {
                let gap = plane.distance(beside(cylinder, place));
                (gap.abs() > rounding).then_some(gap > 0.0)
            }
            (Surface::Plane(first), Surface::Plane(second)) => {
                let facing = first.normal.dot(second.normal);
                let gap = -second.distance(place) / facing;
                (gap.abs() > rounding).then_some(gap > 0.0)
            }
            (Surface::Cylinder(_), Surface::Plane(_)) => None,
            (Surface::Cone(_), _) | (_, Surface::Cone(_)) => {
                measured(first, second, place, rounding)
            }
        },
    }
}

/// Whether the second of two surfaces stands on the side of the first its
/// own normal points to at `place`, measured from the first to the point of
/// the second nearest it, where that stands further than `rounding` off.
fn measured(first: &Surface, second: &Surface, place: DVec3, rounding: f64) -> Option<bool> {
    let gap = first.distance(nearest(second, place));
    (gap.abs() > rounding).then_some(gap > 0.0)
}

/// Whether the second of two surfaces decided to touch along a line stands
/// on the side of the first its own normal points to.
fn touching_above(first: &Surface, second: &Surface) -> Option<bool> {
    match (first, second) {
        (Surface::Plane(plane), Surface::Cylinder(cylinder)) => {
            Some(plane.distance(cylinder.origin) > 0.0)
        }
        (Surface::Cylinder(_), Surface::Plane(_)) => Some(true),
        (Surface::Cylinder(first), Surface::Cylinder(second)) => {
            let between = second.origin - first.origin;
            let across = (between - first.axis * first.axis.dot(between)).length();
            let outside = (across - (first.radius + second.radius)).abs();
            let inside = (across - (first.radius - second.radius).abs()).abs();
            Some(outside <= inside || second.radius > first.radius)
        }
        (Surface::Plane(_), Surface::Plane(_)) | (Surface::Cone(_), _) | (_, Surface::Cone(_)) => {
            None
        }
    }
}

/// Whether the second of two surfaces crossing along two lines parallel to a
/// cylinder's axis stands above the first all along the stretch of that
/// cylinder between the two lines that `place` stands on: read at the
/// stretch's middle, where the two stand furthest apart. Within the band
/// about either line they stand within rounding of each other, and the side
/// changes only across a line. Of two walls, a point of the first inside
/// the second has the second's wall beyond it where the two face one way at
/// `place`, a bore crossing the wall of a bore it all but touches inside,
/// and behind it where they face each other, a pin dipping into a stock.
fn across_the_lobe(
    first: &Surface,
    second: &Surface,
    lines: &[Line; 2],
    place: DVec3,
) -> Option<bool> {
    let wall = match (first, second) {
        (Surface::Cylinder(wall), _) | (_, Surface::Cylinder(wall)) => wall,
        (Surface::Plane(_) | Surface::Cone(_), Surface::Plane(_) | Surface::Cone(_)) => {
            return None;
        }
    };
    let at = wall.parameters(place);
    let [one, other] = lines.map(|line| wall.parameters(line.origin).x);
    let middle = wall.point(DVec2::new(middle_of_the_lobe(at.x, [one, other]), at.y));
    match (first, second) {
        (Surface::Cylinder(first), Surface::Cylinder(second)) => {
            let facing = |cylinder: &Cylinder| cylinder.radial(cylinder.parameters(place).x);
            let agree = facing(first).dot(facing(second)) > 0.0;
            Some((second.distance(middle) < 0.0) == agree)
        }
        (Surface::Plane(plane), Surface::Cylinder(_)) => Some(plane.distance(middle) > 0.0),
        (Surface::Cylinder(wall), Surface::Plane(plane)) => {
            Some(plane.distance(middle) * plane.distance(wall.origin) > 0.0)
        }
        (Surface::Plane(_), Surface::Plane(_)) | (Surface::Cone(_), _) | (_, Surface::Cone(_)) => {
            None
        }
    }
}

/// The angle half way round the lobe between the angles `one` and `other`
/// of two lines that the angle `at` stands on.
fn middle_of_the_lobe(at: f64, [one, other]: [f64; 2]) -> f64 {
    let span = (other - one).rem_euclid(TAU);
    if (at - one).rem_euclid(TAU) < span {
        one + span / 2.0
    } else {
        other + (TAU - span) / 2.0
    }
}

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
fn across_the_cone_s_lobe(
    first: &Surface,
    second: &Surface,
    lines: &[Line; 2],
    place: DVec3,
) -> Option<bool> {
    let (cone, plane) = match (first, second) {
        (Surface::Cone(cone), Surface::Plane(plane))
        | (Surface::Plane(plane), Surface::Cone(cone)) => (cone, plane),
        (Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_), _) => return None,
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
        Surface::Plane(_) | Surface::Cylinder(_) => Some(towards > 0.0),
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
fn touching_the_cone(first: &Surface, second: &Surface) -> Option<bool> {
    match (first, second) {
        (Surface::Cone(cone), Surface::Plane(_)) => Some(cone.ruling.x > 0.0),
        (Surface::Plane(plane), Surface::Cone(cone)) => {
            Some(plane.normal.dot(cone.axis) * cone.ruling.x > 0.0)
        }
        (Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_), _) => None,
    }
}

/// Whether a plane holding a cone's axis stands on the side of the cone its
/// own normal points to: on the axis side all round, the same on either
/// lobe the two rulings part the cone into, where the cone's normal points
/// when it opens against its axis. A place of the plane stands at the very
/// angle of a ruling, on neither lobe, and the cone stands on both sides of
/// the plane there: none.
fn across_the_rulings(first: &Surface, second: &Surface) -> Option<bool> {
    match (first, second) {
        (Surface::Cone(cone), Surface::Plane(_)) => Some(cone.ruling.x < 0.0),
        (Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_), _) => None,
    }
}

/// The point of a surface nearest `place`.
fn nearest(surface: &Surface, place: DVec3) -> DVec3 {
    match surface {
        Surface::Plane(plane) => place - plane.normal * plane.distance(place),
        Surface::Cylinder(cylinder) => beside(cylinder, place),
        Surface::Cone(cone) => cone.nearest(place),
    }
}

/// The point of a wall nearest `place`, off its axis.
fn beside(cylinder: &Cylinder, place: DVec3) -> DVec3 {
    let from = place - cylinder.origin;
    let foot = cylinder.origin + cylinder.axis * cylinder.axis.dot(from);
    foot + (place - foot).normalize_or_zero() * cylinder.radius
}

/// Whether `second`, parallel to `first`, stands outside it at the angle of
/// `place`, where the two stand further than `rounding` apart there.
fn above_at(first: &Cylinder, second: &Cylinder, place: DVec3, rounding: f64) -> Option<bool> {
    let square = |v: DVec3| v - first.axis * first.axis.dot(v);
    if first.axis.cross(second.axis).length() > ROUNDING {
        return None;
    }
    let outward = square(place - first.origin).normalize_or_zero();
    let gap = square(second.origin - first.origin).dot(outward) + second.radius - first.radius;
    (gap.abs() > rounding).then_some(gap > 0.0)
}
