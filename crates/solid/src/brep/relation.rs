//! How two surfaces meet, decided once: decision 2 of `docs/exact-kernel.md`,
//! and the only place a tangency is decided. What is decided is built exactly
//! from a formula — a tangent line is never a double root.

mod conical;
mod crossing;
mod cylinders;
mod elliptic;
mod perpendicular;
mod planar;
mod revolved;
mod sliced;
mod walls;

use glam::DVec3;

use super::conic::Conic;
use super::curve::{Circle, Curve, Line};
use super::meet::Meeting;
use super::scale::Scale;
use super::surface::Surface;
pub use crossing::{
    Crossing, Crossings, Lying, Touches, crossings, crossings_given, crossings_lying,
};
pub(in crate::brep) use revolved::apart_by;
pub use walls::Ruled;
pub(in crate::brep) use walls::{abreast, closest, feet_of_touches};

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
    /// A plane along a cylinder's or an elliptic wall's axis cutting it, or
    /// two parallel walls crossing.
    Lines([Line; 2]),
    /// A plane touching a cylinder or an elliptic wall, or two parallel
    /// walls touching from outside or from inside.
    Tangent(Line),
    /// Two parallel walls, an elliptic one among them, meeting along more
    /// than `Lines` or `Tangent` hold — four crossings, two touches, a touch
    /// beside crossings: each ruling, and whether the walls only touch along
    /// it.
    Parallel(Vec<Ruled>),
    /// A plane square to a cylinder's axis, or a cone and a surface of its
    /// axis crossing off it.
    Circle(Circle),
    /// Two perpendicular walls, circular or elliptic.
    Meet(Meeting),
    /// A plane through a cone's apex steeper than its slope, holding its axis
    /// or at a slant to it: the two rulings it cuts, and the apex they cross
    /// at where it stands within the box.
    Rulings {
        lines: [Line; 2],
        apex: Option<DVec3>,
    },
    /// Two surfaces touching at a cone's apex alone.
    Apex(DVec3),
    /// A plane at a slant to a cylinder's axis, or to a cone's axis without
    /// passing through its apex within the box: the conic it cuts on the
    /// wall's own nappe, whole or as the arms it is split into. A plane square
    /// to an elliptic wall's axis, or at a slant to it, cuts its conic too.
    Conic(Vec<Conic>),
    /// A plane touching a cone along a ruling: that line, and the apex where
    /// it stands within the box.
    Ruling {
        line: Line,
        apex: Option<DVec3>,
    },
    /// Two cylinders at a skew angle, a cone and a cylinder or a cone not of
    /// its axis, a cone within the tolerance of a plane or a cylinder of its
    /// axis all across the box, an elliptic wall and a wall neither parallel
    /// nor square to it or a cone, two parallel walls within the tolerance of
    /// each other all round though not one.
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
        (Surface::Elliptic(_), Surface::Cylinder(_) | Surface::Elliptic(_))
        | (Surface::Cylinder(_), Surface::Elliptic(_))
            if let Some(met) = perpendicular::walls(one, other, scale) =>
        {
            met
        }
        (Surface::Elliptic(one), Surface::Elliptic(other)) => elliptic::walls(one, other, scale),
        (Surface::Plane(plane), Surface::Elliptic(wall))
        | (Surface::Elliptic(wall), Surface::Plane(plane)) => {
            elliptic::plane_and_elliptic(plane, wall, scale)
        }
        (Surface::Cylinder(round), Surface::Elliptic(wall))
        | (Surface::Elliptic(wall), Surface::Cylinder(round)) => {
            walls::round_and_elliptic(round, wall, scale)
        }
    }
}

/// Whether two surfaces are a plane at a slant to a cylinder's or an
/// elliptic wall's axis — neither square to it nor along it — or to a
/// cone's — neither square to it nor holding it: the pairs whose curve is a
/// conic, which only the conic's own decision builds.
pub(in crate::brep) fn at_a_slant(one: &Surface, other: &Surface, scale: Scale) -> bool {
    match (one, other) {
        (Surface::Plane(plane), Surface::Cylinder(cylinder))
        | (Surface::Cylinder(cylinder), Surface::Plane(plane)) => {
            !parallel(plane.normal, cylinder.axis, scale)
                && !square(plane.normal, cylinder.axis, scale)
        }
        (Surface::Plane(plane), Surface::Cone(cone))
        | (Surface::Cone(cone), Surface::Plane(plane)) => {
            !parallel(plane.normal, cone.axis, scale)
                && !(square(plane.normal, cone.axis, scale)
                    && plane.distance(cone.origin).abs() <= scale.eps())
        }
        (Surface::Plane(plane), Surface::Elliptic(wall))
        | (Surface::Elliptic(wall), Surface::Plane(plane)) => {
            !parallel(plane.normal, wall.axis, scale) && !square(plane.normal, wall.axis, scale)
        }
        (Surface::Plane(_), Surface::Plane(_))
        | (Surface::Cylinder(_) | Surface::Cone(_), Surface::Cylinder(_) | Surface::Cone(_))
        | (Surface::Elliptic(_), _)
        | (_, Surface::Elliptic(_)) => false,
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
            Relation::Conic(conics) => conics.iter().copied().map(Curve::Conic).collect(),
            Relation::Ruling { line, .. } => vec![Curve::Line(*line)],
            Relation::Parallel(rulings) => rulings
                .iter()
                .map(|ruled| Curve::Line(ruled.line))
                .collect(),
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
            Relation::Rulings { apex, .. } | Relation::Ruling { apex, .. } => {
                apex.iter().copied().collect()
            }
            Relation::Apex(apex) => vec![*apex],
            Relation::Apart
            | Relation::Same { .. }
            | Relation::Line(_)
            | Relation::Lines(_)
            | Relation::Tangent(_)
            | Relation::Parallel(_)
            | Relation::Circle(_)
            | Relation::Conic(_)
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
