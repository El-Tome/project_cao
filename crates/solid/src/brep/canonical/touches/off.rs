//! How far a touch decision 2 reads stands from exact: two surfaces decided
//! to touch, or a wall decided to stand about a cone's axis.

use glam::DVec3;

use super::walls::off_the_touches;
use crate::brep::curve::Circle;
use crate::brep::curve::Wall;
use crate::brep::meet::moved;
use crate::brep::relation::{Relation, abreast, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder, Surface};

/// How far two surfaces decided to touch stand from touching exactly, or a
/// wall decided to stand about a cone's axis from standing on it; none where
/// their pair is neither. A cone touches nothing along a line, and holds a
/// wall about its axis only where faces of the two `meet` within the box
/// round the circle they cross along: a section's pointed cone, crossing
/// its widest step below the section, holds nothing (5365100952).
pub(super) fn off(
    one: &Surface,
    other: &Surface,
    scale: Scale,
    meet: impl Fn([DVec3; 2]) -> bool,
) -> Option<f64> {
    if let (
        Surface::Cone(cone),
        Surface::Cylinder(Cylinder { origin, .. }) | Surface::Cone(Cone { origin, .. }),
    )
    | (Surface::Cylinder(Cylinder { origin, .. }), Surface::Cone(cone)) = (one, other)
    {
        let about = match relation(one, other, scale) {
            Relation::Circle(circle) => meet(boxed(&circle)),
            Relation::Conic(_) | Relation::Ruling { .. } | Relation::Unsupported => false,
            Relation::Apart
            | Relation::Same { .. }
            | Relation::Line(_)
            | Relation::Lines(_)
            | Relation::Tangent(_)
            | Relation::Parallel(_)
            | Relation::Meet(_)
            | Relation::Rulings { .. }
            | Relation::Apex(_) => true,
        };
        return about.then(|| cone.off_axis(*origin));
    }
    match relation(one, other, scale) {
        Relation::Tangent(_) | Relation::Parallel(_) if abreast(one, other) => {
            off_the_touches(one, other, scale)
        }
        Relation::Tangent(_) => match (one, other) {
            (Surface::Plane(plane), Surface::Cylinder(cylinder))
            | (Surface::Cylinder(cylinder), Surface::Plane(plane)) => {
                Some((plane.distance(cylinder.origin).abs() - cylinder.radius).abs())
            }
            (Surface::Cylinder(one), Surface::Cylinder(other)) => {
                let between = other.origin - one.origin;
                let across = (between - one.axis * one.axis.dot(between)).length();
                let outside = across - (one.radius + other.radius);
                let inside = (one.radius - other.radius).abs() - across;
                Some(outside.abs().min(inside.abs()))
            }
            (Surface::Plane(plane), Surface::Elliptic(wall))
            | (Surface::Elliptic(wall), Surface::Plane(plane)) => {
                Some((plane.distance(wall.origin).abs() - wall.support(plane.normal)).abs())
            }
            (Surface::Plane(_), Surface::Plane(_)) => Some(0.0),
            (Surface::Cone(_) | Surface::Elliptic(_), _)
            | (_, Surface::Cone(_) | Surface::Elliptic(_)) => None,
        },
        Relation::Meet(meeting) if !meeting.nodes.is_empty() || meeting.contact.is_some() => {
            let (one, other) = (Wall::of(one)?, Wall::of(other)?);
            Some(moved(&one, &other, scale).map_or(0.0, |(rank, to)| {
                match ([one, other][rank], to) {
                    (Wall::Cylinder(from), Wall::Cylinder(to)) => {
                        from.origin.distance(to.origin) + (from.radius - to.radius).abs()
                    }
                    (Wall::Elliptic(from), Wall::Elliptic(to)) => from.gap(&to),
                    (Wall::Cylinder(_), Wall::Elliptic(_))
                    | (Wall::Elliptic(_), Wall::Cylinder(_)) => f64::INFINITY,
                }
            }))
        }
        Relation::Apart
        | Relation::Same { .. }
        | Relation::Line(_)
        | Relation::Lines(_)
        | Relation::Parallel(_)
        | Relation::Circle(_)
        | Relation::Meet(_)
        | Relation::Rulings { .. }
        | Relation::Apex(_)
        | Relation::Conic(_)
        | Relation::Ruling { .. }
        | Relation::Unsupported => None,
    }
}

/// The box round a circle.
fn boxed(circle: &Circle) -> [DVec3; 2] {
    let square = DVec3::ONE - circle.axis * circle.axis;
    let reach = square.max(DVec3::ZERO).map(f64::sqrt) * circle.radius;
    [circle.center - reach, circle.center + reach]
}
