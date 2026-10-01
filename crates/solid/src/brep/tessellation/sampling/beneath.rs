//! The samples of curves standing just inside a wall they do not lie on.
//!
//! A wall's chords sag towards its axis, by up to a chord's sag between two
//! steps of its grid. A curve passing inside the wall closer than that — the
//! rim of a bore's cap ending inside a lying post, a hair under its wall —
//! has its samples outside the wall's triangles, and the faces it bounds
//! poke through them. A vertex there gives the wall's circles a ray through
//! it; a sample of such a curve does too, where the wall holds a face.

use std::collections::BTreeMap;
use std::f64::consts::PI;

use glam::DVec3;

use super::super::contact::Wall;
use super::{Samples, divisions};
use crate::brep::curve::Curve;
use crate::brep::domain::Location;
use crate::brep::scale::Scale;
use crate::brep::topology::{Body, SurfaceId};

/// For each wall of the body, the samples of the curves not parallel to it
/// standing inside it closer than twice a chord's sag where it holds a face:
/// circles about another axis and the curves two other cylinders meet along.
pub(super) fn walls(
    body: &Body,
    walls: &[Wall],
    samples: &Samples,
    tolerance: f64,
) -> BTreeMap<SurfaceId, Vec<DVec3>> {
    let eps = body.scale().eps();
    let mut beneath: BTreeMap<SurfaceId, Vec<DVec3>> = BTreeMap::new();
    for (id, cylinder) in walls {
        if id.0 as usize >= body.surfaces.len() {
            continue;
        }
        let sag =
            cylinder.radius * (1.0 - (PI / divisions(cylinder.radius, tolerance) as f64).cos());
        let holds = |point: DVec3| {
            let at = cylinder.parameters(point);
            body.face_ids().any(|face| {
                body.face(face).surface == *id
                    && !matches!(body.locate(face, at, eps), Ok(Location::Outside))
            })
        };
        for edge in body.edge_ids() {
            let across = match body.curve(body.edge(edge).curve) {
                Curve::Circle(circle) => {
                    circle.axis.cross(cylinder.axis).length() > Scale::RELATIVE
                }
                Curve::Meet(meet) => meet.first != *cylinder && meet.second != *cylinder,
                Curve::Line(_) => false,
            };
            if !across {
                continue;
            }
            for sample in samples.edge(edge) {
                if samples.is_vertex(*sample) {
                    continue;
                }
                let point = samples.point(*sample);
                let inside = cylinder.distance(point);
                if (-2.0 * sag..-eps).contains(&inside) && holds(point) {
                    beneath.entry(*id).or_default().push(point);
                }
            }
        }
    }
    beneath
}
