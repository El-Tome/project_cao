//! Decision 2's move of a cylinder onto two touches at once. A cylinder the
//! second operand alone carries, a hair from touching two walls square to
//! it at the two ends of its reach along one way — a tunnel as wide as a
//! tube's wall, touching its outside and its bore — is no translation from
//! touching both: moved onto one, it stands twice as far from the other.
//! Moved midway between the two and given half their distance as its
//! radius, it touches both exactly, as a cylinder between two planes does.
//!
//! An elliptic wall is not moved so: the curve it meets a wall square to it
//! along is built on the two walls as they stand, not on a copy of one
//! moved onto the touch, and a tunnel of it a hair too wide for the tube's
//! wall is drawn as it is.

use glam::DVec3;

use crate::brep::curve::Wall;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};

/// A cylinder's move onto one touch with a wall square to it: the
/// translation, and the end of its reach the touch stands at, of unit
/// length.
pub(super) struct Onto {
    by: DVec3,
    end: DVec3,
}

/// How `wall`, a cylinder moved to `moved` onto its touch with `partner`, a
/// wall square to it, was moved: none where the move is no translation.
pub(super) fn onto(wall: &Surface, partner: &Surface, moved: &Surface) -> Option<Onto> {
    let (Surface::Cylinder(own), Surface::Cylinder(after)) = (wall, moved) else {
        return None;
    };
    let theirs = Wall::of(partner)?;
    if own.axis.dot(theirs.axis()).abs() > Scale::RELATIVE {
        return None;
    }
    let by = translation(own, after)?;
    let way = by.normalize();
    let center = own.origin + own.axis * own.axis.dot(theirs.origin() - own.origin);
    let off = |side: f64| partner.distance(center + way * (side * own.radius)).abs();
    let side = if off(1.0) <= off(-1.0) { 1.0 } else { -1.0 };
    Some(Onto {
        by,
        end: way * side,
    })
}

/// `wall` moved onto two touches at opposite ends of its reach along one
/// way, where they ask two different translations: its axis moved by the
/// mean of the two, its radius grown by the mean of what each moves its end
/// outwards. None where the two touches stand at one end, along two ways,
/// or ask one translation, or for any surface but a cylinder.
pub(super) fn across(wall: &Surface, one: &Onto, other: &Onto) -> Option<Surface> {
    let Surface::Cylinder(round) = wall else {
        return None;
    };
    if one.end.cross(other.end).length() > Scale::RELATIVE
        || one.end.dot(other.end) > 0.0
        || one.by == other.by
    {
        return None;
    }
    let shift = (one.by + other.by) / 2.0;
    let grown = (one.by.dot(one.end) + other.by.dot(other.end)) / 2.0;
    Some(Surface::Cylinder(Cylinder::about(
        round.origin + shift,
        round.axis,
        round.radius + grown,
    )))
}

/// The move that takes `before` to `after`, square to its axis, where it is
/// a translation: the radius kept, which the moves onto a touch pass on to
/// the bit, and the axis parallel, which rebuilding the cylinder may round
/// by a bit on a tilted one. None where the radius changed or nothing
/// moved.
fn translation(before: &Cylinder, after: &Cylinder) -> Option<DVec3> {
    if before.radius != after.radius || before.axis.cross(after.axis).length() > Scale::RELATIVE {
        return None;
    }
    let by = after.origin - before.origin;
    let by = by - before.axis * before.axis.dot(by);
    (by.length() > 0.0).then_some(by)
}
