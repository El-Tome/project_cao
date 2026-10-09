//! Decision 10: two parallel planes, one of each operand, standing within
//! `Scale::HAIR` tolerances of each other, are one surface where a wall touches
//! one of them and so stands within the hair of the other. Two planes a
//! hair apart never cross, and decision 1 keeps them two; but a wall
//! touching one crosses the other at a grazing angle, along two rulings a
//! band apart, or passes it by a hair it was meant to touch: as ill
//! conditioned as two walls of one radius a hair apart (decision 8), and
//! bounded by the same hair.
//!
//! The second operand's plane is taken for the first's, and what the second
//! operand built on it goes along, as `carried.rs` moves it.

use glam::DVec3;

use super::carried::carried_along;
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Plane, Surface};
use crate::brep::topology::Body;

/// The second operand with each of its planes standing within `Scale::HAIR`
/// tolerances of one of the first's, a wall touching either, moved onto
/// it; none when nothing moves.
pub(in crate::brep) fn flush(first: &Body, second: &Body, scale: Scale) -> Option<Body> {
    carried_along(first, second, scale, |current, rank| {
        let plane = match current.surfaces[rank] {
            Surface::Plane(plane) => plane,
            Surface::Cylinder(_) | Surface::Cone(_) => return None,
        };
        offset(first, current, &plane, scale)
    })
}

/// The move along the normal that lays `plane` on the nearest of the first
/// operand's planes standing within `Scale::HAIR` tolerances of it, a wall of
/// either operand touching one of the two; none where decision 1 takes it
/// for one already.
fn offset(first: &Body, second: &Body, plane: &Plane, scale: Scale) -> Option<DVec3> {
    let eps = scale.eps();
    let mut nearest: Option<f64> = None;
    for known in &first.surfaces {
        if matches!(
            relation(known, &Surface::Plane(*plane), scale),
            Relation::Same { .. }
        ) {
            return None;
        }
        let known = match known {
            Surface::Plane(known) => known,
            Surface::Cylinder(_) | Surface::Cone(_) => continue,
        };
        if known.normal.cross(plane.normal).length() * 2.0 * scale.reach() > eps {
            continue;
        }
        let gap = known.offset() * known.normal.dot(plane.normal).signum() - plane.offset();
        if gap.abs() > Scale::HAIR * eps || nearest.is_some_and(|least| least.abs() <= gap.abs()) {
            continue;
        }
        let walled = first
            .surfaces
            .iter()
            .chain(&second.surfaces)
            .any(|wall| touches_one(wall, plane, gap, eps));
        if walled {
            nearest = Some(gap);
        }
    }
    nearest.map(|gap| plane.normal * gap)
}

/// Whether `wall` is a cylinder along `plane` touching it, or the plane
/// `gap` along its normal, within `eps`.
fn touches_one(wall: &Surface, plane: &Plane, gap: f64, eps: f64) -> bool {
    let cylinder = match wall {
        Surface::Cylinder(cylinder) => cylinder,
        Surface::Plane(_) | Surface::Cone(_) => return false,
    };
    if cylinder.axis.dot(plane.normal).abs() > eps {
        return false;
    }
    let away = plane.distance(cylinder.origin);
    [away, away - gap]
        .iter()
        .any(|away| (away.abs() - cylinder.radius).abs() <= eps)
}
