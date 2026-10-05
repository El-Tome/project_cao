//! Decision 2's slides, carried through the second operand. A wall the
//! second operand drew corners on, slid along a touch it holds exactly onto
//! another, would take the line of that touch along and leave the corners
//! drawn on it behind, a hair off the line; left where it is, it stays a
//! hair off the other touch. Each such slide is made with the operand: the
//! wall moves with its corners, its curves and the surfaces they stand on,
//! as decision 8 moves them, before the boolean reads it. Where that move
//! is not made, the wall is not slid.

use glam::DVec3;

use super::Operands;
use crate::brep::canonical::carried_along;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::Body;

/// The second operand with every wall decision 2 slides along an exact
/// touch, against its own corners, moved with what it built on it; none
/// when no such slide is made.
pub(super) fn slid(first: &Body, second: &Body, scale: Scale) -> Option<Body> {
    let sliding = Operands::sliding(first, second, scale);
    let pinned = Operands::of(first, second, scale);
    let moves: Vec<Option<DVec3>> = (0..second.surfaces.len())
        .map(|rank| {
            let (shared, _) = sliding.surfaces.mapped[1][rank];
            let [free, held] =
                [&sliding, &pinned].map(|laid| laid.surfaces.list[shared.0 as usize]);
            if free == held {
                return None;
            }
            translation(&second.surfaces[rank], &free)
        })
        .collect();
    carried_along(first, second, scale, |_, rank| moves[rank])
}

/// The move that takes `before` to `after` where it is a translation: a
/// cylinder moved square to its axis keeping its radius, a plane along its
/// normal. None where nothing moved, or a radius changed.
fn translation(before: &Surface, after: &Surface) -> Option<DVec3> {
    let by = match (before, after) {
        (Surface::Cylinder(one), Surface::Cylinder(other))
            if one.radius == other.radius && one.axis == other.axis =>
        {
            other.origin - one.origin
        }
        (Surface::Plane(one), Surface::Plane(other)) if one.normal == other.normal => {
            one.normal * (other.offset() - one.offset())
        }
        _ => return None,
    };
    (by != DVec3::ZERO).then_some(by)
}
