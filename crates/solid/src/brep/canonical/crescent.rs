//! Decision 8: two parallel cylinders of one radius, one of each operand,
//! whose axes stand within `Scale::HAIR` tolerances of each other are one surface.
//! Two such walls cross along two rulings at an angle the offset over the
//! radius, a crossing so ill conditioned that a band a thousand tolerances
//! wide surrounds it; the crescent between them is at most `Scale::HAIR`
//! tolerances thick.
//!
//! Twenty and not more: a line crossing a wall at a slant sees the wall
//! moved by the move over the cosine, at each of its two crossings, and
//! lines are held down to a cosine of a twentieth. Twenty tolerances are
//! seen as eight hundred at worst, under the thousand a line resolves; at
//! fifty, a block bored twice was seen bored as the first bore alone.
//!
//! The second operand's wall is taken for the first's as decision 1 takes
//! one within the tolerance, and what the second operand built on it goes
//! along: the second operand is moved square to the axis, by the offset,
//! as `carried.rs` moves it. Where that move is not made, the two walls are
//! left two.
//!
//! So is a wall decision 1 takes for the first's a hair off, within the
//! tolerance: left where it is, what its operand built on it stays a hair
//! behind, and is measured again against the first's wall. A slot's cap
//! taken for a disc's wall at just under the tolerance left the slot's side,
//! which its operand made touch the cap, just over it from the disc, and the
//! side crossed the wall it should have touched (533613392).
//!
//! A cone is decided against a surface of revolution about one axis only
//! (#536): a cone and a cylinder or another cone whose axes stand a hair
//! apart meet along a curve the kernel does not build, and two circles of
//! theirs, read about each axis, stand a hair apart where they should be
//! one. So a cone and any wall or cone, one of each operand, whose axes
//! stand within `Scale::HAIR` tolerances are taken about one axis, whatever
//! their radii: the second operand is moved onto the first's axis the same
//! way (5361108529, 5361110937).

use glam::DVec3;

use super::carried::carried_along;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::Body;

/// The second operand with each of its cylinders standing within `Scale::HAIR`
/// tolerances of one of the first's, of one radius, moved onto it; each of
/// its cones whose axis stands as near a cylinder's or a cone's of the
/// first, and each of its cylinders whose axis stands as near a cone's of
/// the first, moved about that axis; none when nothing moves.
pub(in crate::brep) fn closed(first: &Body, second: &Body, scale: Scale) -> Option<Body> {
    carried_along(first, second, scale, |current, rank| {
        offset(first, &current.surfaces[rank], scale)
    })
}

/// The move square to the axes that lays `surface` about the axis of the
/// nearest of the first operand's surfaces of revolution decided about one
/// axis with it whose axis stands within `Scale::HAIR` tolerances of its
/// own; none where it stands about one already, but for rounding.
fn offset(first: &Body, surface: &Surface, scale: Scale) -> Option<DVec3> {
    let eps = scale.eps();
    let (origin, axis) = axis_of(surface)?;
    let mut nearest: Option<(f64, DVec3)> = None;
    for known in &first.surfaces {
        let Some((known_origin, known_axis)) = axis_of(known) else {
            continue;
        };
        let one = match (known, surface) {
            (Surface::Cylinder(known), Surface::Cylinder(other)) => {
                (known.radius - other.radius).abs() <= eps
            }
            _ => true,
        };
        if known_axis.cross(axis).length() * 2.0 * scale.reach() > eps || !one {
            continue;
        }
        let between = known_origin - origin;
        let across = between - known_axis * known_axis.dot(between);
        let distance = across.length();
        if distance <= Scale::HAIR * eps && nearest.is_none_or(|(least, _)| distance < least) {
            nearest = Some((distance, across));
        }
    }
    nearest
        .filter(|&(distance, _)| distance > ROUNDING * eps)
        .map(|(_, across)| across)
}

/// A point of the axis of a cylinder or a cone, and its direction; none for
/// a plane.
fn axis_of(surface: &Surface) -> Option<(DVec3, DVec3)> {
    match surface {
        Surface::Cylinder(cylinder) => Some((cylinder.origin, cylinder.axis)),
        Surface::Cone(cone) => Some((cone.origin, cone.axis)),
        Surface::Plane(_) => None,
    }
}

/// Under this share of the tolerance, a wall already stands on the first's.
const ROUNDING: f64 = 1e-6;

#[cfg(test)]
mod tests;
