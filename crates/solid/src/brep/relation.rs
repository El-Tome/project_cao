//! How two surfaces meet, decided once: decision 2 of `docs/exact-kernel.md`,
//! and the only place a tangency is decided. What is decided is built exactly
//! from a formula — a tangent line is never a double root.

mod conical;
mod crossing;
mod cylinders;
mod planar;
mod revolved;

use glam::DVec3;

use super::curve::{Circle, Curve, Line};
use super::meet::Meeting;
use super::scale::Scale;
use super::surface::Surface;
pub use crossing::{Crossing, Crossings, Touches, crossings, crossings_given};
pub(in crate::brep) use revolved::apart_by;

#[derive(Clone, Debug, PartialEq)]
pub enum Relation {
    Apart,
    /// One surface within the tolerance, and whether their own normals point
    /// the same way.
    Same {
        agree: bool,
    },
    /// Two planes across each other.
    Line(Line),
    /// A plane along a cylinder's axis cutting it, or two parallel cylinders
    /// crossing.
    Lines([Line; 2]),
    /// A plane touching a cylinder, or two parallel cylinders touching from
    /// outside or from inside.
    Tangent(Line),
    /// A plane square to a cylinder's axis, or a cone and a surface of its
    /// axis crossing off it.
    Circle(Circle),
    /// Two perpendicular cylinders.
    Meet(Meeting),
    /// A plane holding a cone's axis: the two rulings it cuts, and the apex
    /// they cross at where it stands within the box.
    Rulings {
        lines: [Line; 2],
        apex: Option<DVec3>,
    },
    /// Two surfaces touching at a cone's apex alone.
    Apex(DVec3),
    /// A plane oblique to a cylinder's axis, two cylinders at a skew angle,
    /// a cone and a plane neither square to its axis nor holding it, a cone
    /// and a cylinder or a cone not of its axis.
    Unsupported,
}

pub fn relation(one: &Surface, other: &Surface, scale: Scale) -> Relation {
    match (one, other) {
        (Surface::Plane(one), Surface::Plane(other)) => planar::planes(one, other, scale),
        (Surface::Plane(plane), Surface::Cylinder(cylinder))
        | (Surface::Cylinder(cylinder), Surface::Plane(plane)) => {
            planar::plane_and_cylinder(plane, cylinder, scale)
        }
        (Surface::Cylinder(one), Surface::Cylinder(other)) => {
            cylinders::cylinders(one, other, scale)
        }
        (Surface::Cone(cone), other) | (other, Surface::Cone(cone)) => {
            conical::relation(cone, other, scale)
        }
    }
}

/// Two lines in an order read off the lines alone, whichever surface came
/// first.
fn sorted(lines: [Line; 2]) -> [Line; 2] {
    let [one, other] = lines;
    if other.origin.to_array() < one.origin.to_array() {
        [other, one]
    } else {
        [one, other]
    }
}

impl Relation {
    /// The curves the two surfaces share.
    pub fn curves(&self) -> Vec<Curve> {
        match self {
            Relation::Line(line) | Relation::Tangent(line) => vec![Curve::Line(*line)],
            Relation::Lines(lines) | Relation::Rulings { lines, .. } => {
                lines.map(Curve::Line).to_vec()
            }
            Relation::Circle(circle) => vec![Curve::Circle(*circle)],
            Relation::Meet(meeting) => meeting
                .components
                .iter()
                .copied()
                .map(Curve::Meet)
                .collect(),
            Relation::Apart | Relation::Same { .. } | Relation::Apex(_) | Relation::Unsupported => {
                Vec::new()
            }
        }
    }

    /// The points that must be vertices: where a curve crosses itself, where
    /// two cylinders touch at a point, a cone's apex where the pair meets
    /// there.
    pub fn points(&self) -> Vec<DVec3> {
        match self {
            Relation::Meet(meeting) => meeting
                .nodes
                .iter()
                .map(|node| node.point)
                .chain(meeting.contact)
                .collect(),
            Relation::Rulings { apex, .. } => apex.iter().copied().collect(),
            Relation::Apex(apex) => vec![*apex],
            Relation::Apart
            | Relation::Same { .. }
            | Relation::Line(_)
            | Relation::Lines(_)
            | Relation::Tangent(_)
            | Relation::Circle(_)
            | Relation::Unsupported => Vec::new(),
        }
    }
}

/// Two directions are parallel when they part by less than the tolerance
/// across the whole box.
fn parallel(one: DVec3, other: DVec3, scale: Scale) -> bool {
    one.cross(other).length() * 2.0 * scale.reach() <= scale.eps()
}

/// Two directions are square when either leaves the other's normal plane by
/// less than the tolerance across the whole box.
fn square(one: DVec3, other: DVec3, scale: Scale) -> bool {
    one.dot(other).abs() * 2.0 * scale.reach() <= scale.eps()
}

#[cfg(test)]
mod tests;
