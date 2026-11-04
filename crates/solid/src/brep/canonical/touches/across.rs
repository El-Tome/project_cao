//! Decision 2's move of a wall onto two touches at once. A wall the second
//! operand alone carries, a hair from touching two walls square to it at
//! the two ends of its reach along one way — a tunnel as wide as a tube's
//! wall, touching its outside and its bore — is no translation from
//! touching both: moved onto one, it stands twice as far from the other.
//! Moved midway between the two and given half their distance as its
//! half-width along that way, it touches both exactly, as a cylinder
//! between two planes does.

use glam::DVec3;

use crate::brep::curve::Wall;
use crate::brep::meet::widened;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};

/// A wall's move onto one touch with a wall square to it: the translation,
/// and the end of its reach the touch stands at, of unit length.
pub(super) struct Onto {
    by: DVec3,
    end: DVec3,
}

/// How `wall`, moved to `moved` onto its touch with `partner`, a wall
/// square to it, was moved: none where the move is no translation.
pub(super) fn onto(wall: &Surface, partner: &Surface, moved: &Surface) -> Option<Onto> {
    let (own, theirs) = (Wall::of(wall)?, Wall::of(partner)?);
    if own.axis().dot(theirs.axis()).abs() > Scale::RELATIVE {
        return None;
    }
    let by = translation(wall, moved)?;
    let way = by.normalize();
    let center = own.origin() + own.axis() * own.axis().dot(theirs.origin() - own.origin());
    let reach = support(&own, way);
    let off = |side: f64| partner.distance(center + way * (side * reach)).abs();
    let side = if off(1.0) <= off(-1.0) { 1.0 } else { -1.0 };
    Some(Onto {
        by,
        end: way * side,
    })
}

/// `wall` moved onto two touches at opposite ends of its reach along one
/// way, where they ask two different translations: its middle moved by the
/// mean of the two, its half-width along that way grown by the mean of what
/// each moves its end outwards. None where the two touches stand at one end,
/// along two ways, or ask one translation.
pub(super) fn across(wall: &Surface, one: &Onto, other: &Onto) -> Option<Surface> {
    let way = one.end;
    if way.cross(other.end).length() > Scale::RELATIVE
        || way.dot(other.end) > 0.0
        || one.by == other.by
    {
        return None;
    }
    let shift = (one.by + other.by) / 2.0;
    let grown = (one.by.dot(one.end) + other.by.dot(other.end)) / 2.0;
    match wall {
        Surface::Cylinder(round) => Some(Surface::Cylinder(Cylinder::about(
            round.origin + shift,
            round.axis,
            round.radius + grown,
        ))),
        Surface::Elliptic(oval) => Some(Surface::Elliptic(widened(
            &oval.moved(shift),
            way,
            oval.support(way) + grown,
        ))),
        Surface::Plane(_) | Surface::Cone(_) | Surface::Sphere(_) | Surface::Torus(_) => None,
    }
}

/// How far a wall reaches from its axis along `way`, square to it.
fn support(wall: &Wall, way: DVec3) -> f64 {
    match wall {
        Wall::Cylinder(round) => round.radius,
        Wall::Elliptic(oval) => oval.support(way),
    }
}

/// The move that takes `before` to `after` where it is a translation, its
/// shape kept to the bit; none where nothing moved or the shape changed.
fn translation(before: &Surface, after: &Surface) -> Option<DVec3> {
    let by = match (before, after) {
        (Surface::Cylinder(one), Surface::Cylinder(other))
            if one.radius == other.radius && one.axis == other.axis =>
        {
            other.origin - one.origin
        }
        (Surface::Elliptic(one), Surface::Elliptic(other))
            if *other == one.moved(other.origin - one.origin) =>
        {
            other.origin - one.origin
        }
        _ => return None,
    };
    (by != DVec3::ZERO).then_some(by)
}
