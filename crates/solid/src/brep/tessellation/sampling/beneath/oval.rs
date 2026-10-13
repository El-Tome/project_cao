//! The samples of curves standing just inside an elliptic wall they do not
//! lie on (#571), as a cylinder's are found: a circle on a wall square to
//! it — the rim of a bar's end ending inside an elliptic boss, a hair above
//! the boss's own rim — has its samples outside the wall's triangles where
//! the wall's rims chord across them, and the faces it bounds poke through
//! the faces those rims bound. The wall's rims take a sample at each of
//! their angles, through `Contact::oval`.

use std::collections::BTreeMap;
use std::f64::consts::PI;

use glam::DVec3;

use super::super::{Samples, divisions};
use crate::brep::conic::Round;
use crate::brep::curve::{Curve, Wall};
use crate::brep::domain::Location;
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, SurfaceId};

/// For each elliptic wall of the body, the samples of the curves about
/// another axis standing inside it closer than twice a chord's sag across a
/// step of its grid, where it holds a face: circles, the curves two walls
/// meet along that it is not one of, conics of another wall.
pub(in crate::brep::tessellation::sampling) fn walls(
    body: &Body,
    samples: &Samples,
    tolerance: f64,
) -> BTreeMap<SurfaceId, Vec<DVec3>> {
    let eps = body.scale().eps();
    let mut beneath: BTreeMap<SurfaceId, Vec<DVec3>> = BTreeMap::new();
    for (rank, surface) in body.surfaces.iter().enumerate() {
        let Surface::Elliptic(oval) = surface else {
            continue;
        };
        let id = SurfaceId(rank as u32);
        let sag = oval.major * (1.0 - (PI / divisions(oval.major, tolerance) as f64).cos());
        let holds = |point: DVec3| {
            let at = oval.parameters(point);
            body.face_ids().any(|face| {
                body.face(face).surface == id
                    && !matches!(body.locate(face, at, eps), Ok(Location::Outside))
            })
        };
        for edge in body.edge_ids() {
            let across = match body.curve(body.edge(edge).curve) {
                Curve::Circle(_) => true,
                Curve::Meet(meet) => {
                    let own = Wall::Elliptic(*oval);
                    meet.first != own && meet.second != own
                }
                Curve::Conic(conic) => conic.round != Round::Elliptic(*oval),
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
                if (-2.0 * sag..0.0).contains(&oval.distance(point)) && holds(point) {
                    beneath.entry(id).or_default().push(point);
                }
            }
        }
    }
    beneath
}
