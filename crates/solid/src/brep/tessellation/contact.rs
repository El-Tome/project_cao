//! Two cylinders facing each other on their hollow side closer than a chord
//! sags — a small one inside a large one, touching its wall or nearly — and
//! the rays both are sampled along so that their chords keep their order.
//!
//! A chord sags towards its own axis, so against a plane, or a cylinder beside
//! it, it retreats. Inside a large cylinder, within a chord's sag of its wall,
//! the large one's chords sag onto the small one's, and each sampled on its own
//! grid the two can cross. Sampled instead on common rays from the small one's
//! axis — its own grid's, and those through the large one's grid and through
//! the vertices on either — both are graphs of the angle round that axis, the
//! large one's point further out on every ray, and chords between the same two
//! rays cannot cross. Every circle of either cylinder takes the same rays, so
//! their walls stay ordered from one end to the other.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::sampling::divisions;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::{Body, SurfaceId};

/// For each cylinder in contact with another, the directions square to its
/// axis, from its axis, its circles are sampled at besides its grid.
pub(super) fn rays(body: &Body, tolerance: f64) -> BTreeMap<SurfaceId, Vec<DVec3>> {
    let cylinders: Vec<(SurfaceId, Cylinder)> = (0..body.surfaces.len() as u32)
        .map(SurfaceId)
        .filter_map(|id| match body.surface(id) {
            Surface::Cylinder(cylinder) => Some((id, *cylinder)),
            Surface::Plane(_) => None,
        })
        .collect();
    let mut rays: BTreeMap<SurfaceId, Vec<DVec3>> = BTreeMap::new();
    for (at, one) in cylinders.iter().enumerate() {
        for other in &cylinders[at + 1..] {
            let (outer, inner) = if one.1.radius >= other.1.radius {
                (one, other)
            } else {
                (other, one)
            };
            if let Some((on_outer, on_inner)) = common(body, *outer, *inner, tolerance) {
                rays.entry(outer.0).or_default().extend(on_outer);
                rays.entry(inner.0).or_default().extend(on_inner);
            }
        }
    }
    rays
}

/// The directions the outer cylinder's circles and the inner one's take
/// besides their grids, when the inner stands inside the outer within twice
/// the sag of the outer's chords from its wall.
fn common(
    body: &Body,
    (outer_id, outer): (SurfaceId, Cylinder),
    (inner_id, inner): (SurfaceId, Cylinder),
    tolerance: f64,
) -> Option<(Vec<DVec3>, Vec<DVec3>)> {
    let eps = body.scale().eps();
    let axis = outer.axis;
    if axis.cross(inner.axis).length() > Scale::RELATIVE || inner.radius > outer.radius - eps {
        return None;
    }
    let flat = |vector: DVec3| vector - axis * axis.dot(vector);
    let offset = flat(inner.origin - outer.origin);
    let outer_steps = divisions(outer.radius, tolerance);
    let sag = outer.radius * (1.0 - (PI / outer_steps as f64).cos());
    let gap = outer.radius - inner.radius - offset.length();
    if gap > 2.0 * sag || gap < -eps {
        return None;
    }

    let inner_steps = divisions(inner.radius, tolerance);
    let mut through_outer: Vec<DVec3> = (0..outer_steps)
        .map(|step| outer.radial(TAU * step as f64 / outer_steps as f64) * outer.radius)
        .collect();
    let mut from_inner: Vec<DVec3> = (0..inner_steps)
        .map(|step| inner.radial(TAU * step as f64 / inner_steps as f64))
        .collect();
    for vertex in body.vertex_ids().map(|id| body.vertex(id)) {
        let (on_outer, on_inner) = (vertex.on.contains(&outer_id), vertex.on.contains(&inner_id));
        if on_outer && !on_inner {
            through_outer.push(flat(vertex.point - outer.origin));
        } else if on_inner && !on_outer {
            from_inner.push(flat(vertex.point - inner.origin).normalize());
        }
    }

    let on_inner = through_outer
        .iter()
        .map(|point| (*point - offset).normalize())
        .collect();
    let outside = offset.length_squared() - outer.radius * outer.radius;
    let on_outer = from_inner
        .iter()
        .map(|way| {
            let along = offset.dot(*way);
            let reach = -along + (along * along - outside).sqrt();
            (offset + *way * reach).normalize()
        })
        .collect();
    Some((on_outer, on_inner))
}
