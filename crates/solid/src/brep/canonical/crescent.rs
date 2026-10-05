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

use glam::DVec3;

use super::carried::carried_along;
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::Body;

/// The second operand with each of its cylinders standing within `Scale::HAIR`
/// tolerances of one of the first's, of one radius, moved onto it; none
/// when nothing moves.
pub(in crate::brep) fn closed(first: &Body, second: &Body, scale: Scale) -> Option<Body> {
    carried_along(first, second, scale, |current, rank| {
        let Surface::Cylinder(cylinder) = current.surfaces[rank] else {
            return None;
        };
        offset(first, &cylinder, scale)
    })
}

/// The move square to the axes that lays `cylinder` on the nearest of the
/// first operand's cylinders of its radius whose axis stands within `Scale::HAIR`
/// tolerances of its own; none where decision 1 takes it for one already.
fn offset(first: &Body, cylinder: &Cylinder, scale: Scale) -> Option<DVec3> {
    let eps = scale.eps();
    let mut nearest: Option<(f64, DVec3)> = None;
    for known in &first.surfaces {
        if matches!(
            relation(known, &Surface::Cylinder(*cylinder), scale),
            Relation::Same { .. }
        ) {
            return None;
        }
        let Surface::Cylinder(known) = known else {
            continue;
        };
        if known.axis.cross(cylinder.axis).length() * 2.0 * scale.reach() > eps
            || (known.radius - cylinder.radius).abs() > eps
        {
            continue;
        }
        let between = known.origin - cylinder.origin;
        let across = between - known.axis * known.axis.dot(between);
        let distance = across.length();
        if distance <= Scale::HAIR * eps && nearest.is_none_or(|(least, _)| distance < least) {
            nearest = Some((distance, across));
        }
    }
    nearest.map(|(_, across)| across)
}

#[cfg(test)]
mod tests;
