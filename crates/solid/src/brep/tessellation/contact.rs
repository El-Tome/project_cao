//! Two cylinders closer than a chord sags — a small one inside a large one,
//! touching its wall or nearly, or two side by side touching — and how their
//! circles are sampled so that their chords keep their order and their
//! distance.
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
//!
//! Where two walls touch along a line, inside or side by side, they part only
//! as the square of the distance from it: a sample a hair from that line
//! stands closer to the other wall than the kernel tells places apart, and its
//! chords are as good as lying on the other wall's. Such a step of the grid is
//! withheld from both, so that the first chords from the line reach out to
//! where the walls stand apart.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::sampling::divisions;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::{Body, SurfaceId};

/// What the circles of a cylinder take besides their grid, and what of it
/// they leave out, for the cylinders it stands close to.
#[derive(Debug, Default)]
pub(super) struct Contact {
    /// Directions square to its axis, from its axis, its circles are also
    /// sampled along.
    pub(super) rays: Vec<DVec3>,
    /// Steps of its grid its circles are not sampled at.
    pub(super) withheld: Vec<usize>,
}

impl Contact {
    fn join(&mut self, other: Contact) {
        self.rays.extend(other.rays);
        self.withheld.extend(other.withheld);
    }
}

/// For each cylinder close to another, parallel to it, what its circles take
/// and leave out.
pub(super) fn contacts(body: &Body, tolerance: f64) -> BTreeMap<SurfaceId, Contact> {
    let cylinders: Vec<(SurfaceId, Cylinder)> = (0..body.surfaces.len() as u32)
        .map(SurfaceId)
        .filter_map(|id| match body.surface(id) {
            Surface::Cylinder(cylinder) => Some((id, *cylinder)),
            Surface::Plane(_) => None,
        })
        .collect();
    let eps = body.scale().eps();
    let mut contacts: BTreeMap<SurfaceId, Contact> = BTreeMap::new();
    for (at, one) in cylinders.iter().enumerate() {
        for other in &cylinders[at + 1..] {
            if one.1.axis.cross(other.1.axis).length() > Scale::RELATIVE {
                continue;
            }
            let (outer, inner) = if one.1.radius >= other.1.radius {
                (one, other)
            } else {
                (other, one)
            };
            if let Some([on_outer, on_inner]) = common(body, *outer, *inner, tolerance) {
                contacts.entry(outer.0).or_default().join(on_outer);
                contacts.entry(inner.0).or_default().join(on_inner);
                continue;
            }
            for (near, far) in [(one, other), (other, one)] {
                let withheld = touching(&near.1, &far.1, tolerance, eps);
                if !withheld.is_empty() {
                    contacts
                        .entry(near.0)
                        .or_default()
                        .withheld
                        .extend(withheld);
                }
            }
        }
    }
    contacts
}

/// The steps of a cylinder's grid standing within `eps` of another cylinder
/// parallel to it: none unless their circles come that close somewhere.
fn touching(cylinder: &Cylinder, other: &Cylinder, tolerance: f64, eps: f64) -> Vec<usize> {
    let between = cylinder.origin - other.origin;
    let apart = (between - other.axis * other.axis.dot(between)).length();
    let (low, high) = (
        (cylinder.radius - other.radius).abs(),
        cylinder.radius + other.radius,
    );
    if apart < low - eps || apart > high + eps {
        return Vec::new();
    }
    let steps = divisions(cylinder.radius, tolerance);
    (0..steps)
        .filter(|step| {
            let angle = TAU * *step as f64 / steps as f64;
            let point = cylinder.origin + cylinder.radial(angle) * cylinder.radius;
            other.distance(point).abs() < eps
        })
        .collect()
}

/// What the outer cylinder's circles and the inner one's take and leave out,
/// when the inner stands inside the outer within twice the sag of the outer's
/// chords from its wall: every ray the one takes the other takes too, and a
/// ray along which the two stand closer than `eps` neither takes.
fn common(
    body: &Body,
    (outer_id, outer): (SurfaceId, Cylinder),
    (inner_id, inner): (SurfaceId, Cylinder),
    tolerance: f64,
) -> Option<[Contact; 2]> {
    let eps = body.scale().eps();
    let axis = outer.axis;
    if inner.radius > outer.radius - eps {
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
    let mut through_outer: Vec<(DVec3, Option<usize>)> = (0..outer_steps)
        .map(|step| {
            let angle = TAU * step as f64 / outer_steps as f64;
            (outer.radial(angle) * outer.radius, Some(step))
        })
        .collect();
    let mut from_inner: Vec<(DVec3, Option<usize>)> = (0..inner_steps)
        .map(|step| {
            (
                inner.radial(TAU * step as f64 / inner_steps as f64),
                Some(step),
            )
        })
        .collect();
    for vertex in body.vertex_ids().map(|id| body.vertex(id)) {
        let (on_outer, on_inner) = (vertex.on.contains(&outer_id), vertex.on.contains(&inner_id));
        if on_outer && !on_inner {
            through_outer.push((flat(vertex.point - outer.origin), None));
        } else if on_inner && !on_outer {
            from_inner.push((flat(vertex.point - inner.origin).normalize(), None));
        }
    }

    let mut on_outer = Contact::default();
    let mut on_inner = Contact::default();
    for (point, step) in through_outer {
        let from_axis = point - offset;
        if from_axis.length() - inner.radius >= eps {
            on_inner.rays.push(from_axis.normalize());
        } else if let Some(step) = step {
            on_outer.withheld.push(step);
        }
    }
    let outside = offset.length_squared() - outer.radius * outer.radius;
    for (way, step) in from_inner {
        let along = offset.dot(way);
        let reach = -along + (along * along - outside).sqrt();
        if reach - inner.radius >= eps {
            on_outer.rays.push((offset + way * reach).normalize());
        } else if let Some(step) = step {
            on_inner.withheld.push(step);
        }
    }
    Some([on_outer, on_inner])
}
