//! How far a touch decision 2 reads stands from exact: two surfaces decided
//! to touch, or a wall decided to stand about a cone's axis.

use crate::brep::meet::moved;
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder, Surface};

/// How far two surfaces decided to touch stand from touching exactly, or a
/// wall decided to stand about a cone's axis from standing on it; none where
/// their pair is neither. A cone touches nothing along a line.
pub(super) fn off(one: &Surface, other: &Surface, scale: Scale) -> Option<f64> {
    if let (
        Surface::Cone(cone),
        Surface::Cylinder(Cylinder { origin, .. }) | Surface::Cone(Cone { origin, .. }),
    )
    | (Surface::Cylinder(Cylinder { origin, .. }), Surface::Cone(cone)) = (one, other)
    {
        let about = relation(one, other, scale) != Relation::Unsupported;
        return about.then(|| cone.off_axis(*origin));
    }
    match relation(one, other, scale) {
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
            (Surface::Plane(_), Surface::Plane(_)) => Some(0.0),
            (Surface::Cone(_), _) | (_, Surface::Cone(_)) => None,
        },
        Relation::Meet(meeting) if !meeting.nodes.is_empty() || meeting.contact.is_some() => {
            let (one, other) = match (one, other) {
                (Surface::Cylinder(one), Surface::Cylinder(other)) => (one, other),
                (Surface::Plane(_) | Surface::Cone(_), _)
                | (_, Surface::Plane(_) | Surface::Cone(_)) => return None,
            };
            Some(moved(one, other, scale).map_or(0.0, |(rank, to)| {
                let from = [one, other][rank];
                from.origin.distance(to.origin) + (from.radius - to.radius).abs()
            }))
        }
        Relation::Apart
        | Relation::Same { .. }
        | Relation::Line(_)
        | Relation::Lines(_)
        | Relation::Circle(_)
        | Relation::Meet(_)
        | Relation::Rulings { .. }
        | Relation::Apex(_)
        | Relation::Unsupported => None,
    }
}
