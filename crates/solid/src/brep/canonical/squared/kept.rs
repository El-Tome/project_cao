//! What was exact between the two operands before the turn, and must be so
//! after it.

use super::{ROUNDING, facing};
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::Body;

/// Whether every surface of the second operand stands with each of the
/// first's whose faces it meets as it stood before the turn, where it stood
/// exactly: one with it, touching it, along a ruling or at an apex of it, or
/// holding its cone's axis. A tool face lying on the body's, turned a hair
/// off it, or a plane holding a cone's axis turned into a hyperbola a hair
/// from the apex, would be what the turn is made against. Nor is a plane
/// laid a hair beside a cone's apex, but only exactly through it: the plane
/// through a point's apex a hair off its axis, turned about the middle of
/// its box along the axis, stood a hair beside the apex and was cut along
/// two rulings it does not hold (5543232861). A face laid on the body's, a
/// wall laid touching, parallel or square to another: these are what the
/// turn is for, and refused, they left a tool at its lean beside another
/// drawn at that lean and turned the step before (5543215175).
pub(in crate::brep::canonical) fn kept(
    first: &Body,
    before: &Body,
    after: &Body,
    scale: Scale,
) -> bool {
    let apex = |relation: &Relation| {
        matches!(
            relation,
            Relation::Ruling { .. } | Relation::Apex(_) | Relation::Rulings { .. }
        )
    };
    facing(first, before, scale.eps())
        .into_iter()
        .all(|(one, other)| {
            let (known, tool) = (first.surface(one), after.surface(other));
            let was = relation(known, before.surface(other), scale);
            let is = relation(known, tool, scale);
            let exact = apex(&was)
                || matches!(was, Relation::Same { .. } | Relation::Tangent(_))
                || touches(&was) > 0;
            if exact {
                std::mem::discriminant(&was) == std::mem::discriminant(&is)
                    && touches(&was) == touches(&is)
            } else {
                !apex(&is) || through_the_apex(known, tool, scale)
            }
        })
}

/// How many rulings two parallel walls, an elliptic one among them, touch
/// along beside crossings or with another touch (#571): a touch as exact as
/// a tangent's, which a turn must keep.
fn touches(relation: &Relation) -> usize {
    match relation {
        Relation::Parallel(rulings) => rulings.iter().filter(|ruled| ruled.touching).count(),
        _ => 0,
    }
}

/// Whether a plane passes through a cone's apex but for rounding.
fn through_the_apex(one: &Surface, other: &Surface, scale: Scale) -> bool {
    match (one, other) {
        (Surface::Cone(cone), Surface::Plane(plane))
        | (Surface::Plane(plane), Surface::Cone(cone)) => {
            plane.distance(cone.apex()).abs() <= 10.0 * ROUNDING * scale.eps()
        }
        _ => false,
    }
}
