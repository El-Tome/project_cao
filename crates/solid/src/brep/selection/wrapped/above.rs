//! Which side of a surface another stands on, read off the pair as it was
//! decided: along the line they touch along, along the stretch between two
//! lines they cross along, about a cone; and measured where they stand apart.

mod cone;
mod walls;

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::ROUNDING;
use crate::brep::curve::Line;
use crate::brep::relation::{Relation, abreast, at_a_slant};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
use cone::{across_the_cone_s_lobe, across_the_rulings, touching_the_cone};
use walls::standing_above;

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
        Relation::Lines(lines) if abreast(first, second) => {
            standing_above(first, second, lines, place, rounding)
        }
        Relation::Tangent(line) if abreast(first, second) => {
            standing_above(first, second, &[*line], place, rounding)
        }
        Relation::Parallel(rulings) => {
            let lines: Vec<Line> = rulings.iter().map(|ruled| ruled.line).collect();
            standing_above(first, second, &lines, place, rounding)
        }
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
            (Surface::Plane(plane), Surface::Elliptic(wall)) => {
                let gap = plane.distance(wall.nearest(place));
                (gap.abs() > rounding).then_some(gap > 0.0)
            }
            (Surface::Plane(first), Surface::Plane(second)) => {
                let facing = first.normal.dot(second.normal);
                let gap = -second.distance(place) / facing;
                (gap.abs() > rounding).then_some(gap > 0.0)
            }
            (Surface::Cylinder(_), Surface::Plane(_))
            | (Surface::Elliptic(_), _)
            | (_, Surface::Elliptic(_)) => None,
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
        (Surface::Cylinder(_) | Surface::Elliptic(_), Surface::Plane(_)) => Some(true),
        (Surface::Plane(plane), Surface::Elliptic(wall)) => Some(plane.distance(wall.origin) > 0.0),
        (Surface::Cylinder(first), Surface::Cylinder(second)) => {
            let between = second.origin - first.origin;
            let across = (between - first.axis * first.axis.dot(between)).length();
            let outside = (across - (first.radius + second.radius)).abs();
            let inside = (across - (first.radius - second.radius).abs()).abs();
            Some(outside <= inside || second.radius > first.radius)
        }
        (Surface::Plane(_), Surface::Plane(_))
        | (Surface::Cone(_) | Surface::Elliptic(_), _)
        | (_, Surface::Cone(_) | Surface::Elliptic(_)) => None,
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
    let middle = match (first, second) {
        (Surface::Cylinder(wall), _) | (_, Surface::Cylinder(wall)) => {
            let at = wall.parameters(place);
            let [one, other] = lines.map(|line| wall.parameters(line.origin).x);
            wall.point(DVec2::new(middle_of_the_lobe(at.x, [one, other]), at.y))
        }
        (Surface::Elliptic(wall), _) | (_, Surface::Elliptic(wall)) => {
            let at = wall.parameters(place);
            let [one, other] = lines.map(|line| wall.parameters(line.origin).x);
            wall.point(DVec2::new(middle_of_the_lobe(at.x, [one, other]), at.y))
        }
        (Surface::Plane(_) | Surface::Cone(_), Surface::Plane(_) | Surface::Cone(_)) => {
            return None;
        }
    };
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
        (Surface::Plane(plane), Surface::Elliptic(_)) => Some(plane.distance(middle) > 0.0),
        (Surface::Elliptic(wall), Surface::Plane(plane)) => {
            Some(plane.distance(middle) * plane.distance(wall.origin) > 0.0)
        }
        (Surface::Plane(_), Surface::Plane(_))
        | (Surface::Cone(_) | Surface::Elliptic(_), _)
        | (_, Surface::Cone(_) | Surface::Elliptic(_)) => None,
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

/// The point of a surface nearest `place`.
fn nearest(surface: &Surface, place: DVec3) -> DVec3 {
    match surface {
        Surface::Plane(plane) => place - plane.normal * plane.distance(place),
        Surface::Cylinder(cylinder) => beside(cylinder, place),
        Surface::Cone(cone) => cone.nearest(place),
        Surface::Elliptic(wall) => wall.nearest(place),
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
