//! The rays the circles of a wall take on their own: through its vertices,
//! and through the curves it meets a perpendicular wall along.

use std::collections::BTreeMap;
use std::f64::consts::PI;

use glam::DVec3;

use super::super::sampling::divisions;
use super::{Wall, bears};
use crate::brep::curve::Curve;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::{Body, SurfaceId};

/// For each wall, the vertices lying on it; the rays through those and
/// through the vertices standing just inside it, which it keeps whatever
/// the room of a wall close to it; and the rays through the second alone.
#[derive(Default)]
pub(super) struct Through {
    pub(super) anchors: BTreeMap<SurfaceId, Vec<DVec3>>,
    pub(super) pinned: BTreeMap<SurfaceId, Vec<DVec3>>,
    pub(super) beneath: BTreeMap<SurfaceId, Vec<DVec3>>,
}

/// What the circles of two perpendicular cylinders take for the curve they
/// meet along: the rays through every sample of it and through its ends.
///
/// Where one wall is cut from the other, each lies over the other along the
/// curve, and they part slowly: as the square of the distance from a node
/// where they touch, or from a waist where they all but touch, and along
/// the curve wherever it runs down a wall's ruling, at the tip of a window.
/// A triangle of the large wall fanned from a sample of its circle a grid
/// step away sags there below the small wall, and at a fine tolerance the
/// curve's own samples are much closer than a step of the grid. Sampled along
/// these rays, each wall is cut into strips between two samples of the curve,
/// the large one's square to them and the small one's, between its two mirror
/// lobes, across them: within a strip each sags as the square of its width,
/// and the two stay ordered.
pub(super) fn along_meets(
    body: &Body,
    meets: &[Vec<DVec3>],
    own: &mut BTreeMap<SurfaceId, Vec<DVec3>>,
) {
    for id in body.edge_ids() {
        let edge = body.edge(id);
        let Curve::Meet(meet) = body.curve(edge.curve) else {
            continue;
        };
        let ends = edge
            .ends
            .into_iter()
            .flatten()
            .map(|end| body.vertex(end).point);
        let through: Vec<DVec3> = meets[id.0 as usize].iter().copied().chain(ends).collect();
        for cylinder in [meet.first, meet.second] {
            let flat = |point: DVec3| {
                let from = point - cylinder.origin;
                from - cylinder.axis * cylinder.axis.dot(from)
            };
            let rays: Vec<DVec3> = through
                .iter()
                .map(|point| flat(*point).normalize())
                .collect();
            for surface in (0..body.surfaces.len() as u32).map(SurfaceId) {
                if *body.surface(surface) == Surface::Cylinder(cylinder) {
                    own.entry(surface).or_default().extend(rays.iter().copied());
                }
            }
        }
    }
}

/// What the circles of every cylinder take for the vertices lying on it: the
/// rays through them. A ruling or a seam leaves its wall at a vertex off the
/// grid, and a strip of the wall beside it must meet a sample on every rim at
/// that angle, or its triangle reaches from the vertex to the other rim's next
/// step — lying flat where the wall is a hair high, over the face beside it.
///
/// So too for a vertex standing inside the wall closer than twice a chord's
/// sag — the corner of a pocket a hair inside it: the wall's chords sag
/// towards the axis, and a chord across that angle would pass inside the
/// corner. Sampled there, the wall stands on its own surface at the corner.
pub(super) fn through_vertices(
    body: &Body,
    walls: &[Wall],
    tolerance: f64,
    own: &mut BTreeMap<SurfaceId, Vec<DVec3>>,
) -> Through {
    let mut through = Through::default();
    for wall in walls {
        let (id, cylinder) = wall;
        let sag =
            cylinder.radius * (1.0 - (PI / divisions(cylinder.radius, tolerance) as f64).cos());
        for vertex in body.vertex_ids().map(|id| body.vertex(id)) {
            let on = bears(body, vertex, wall);
            let inside = cylinder.distance(vertex.point);
            if !on && !(-2.0 * sag..0.0).contains(&inside) {
                continue;
            }
            let flat = flat_from(cylinder, vertex.point);
            if flat.length() > 0.0 {
                own.entry(*id).or_default().push(flat.normalize());
                through
                    .pinned
                    .entry(*id)
                    .or_default()
                    .push(flat.normalize());
                if on {
                    through.anchors.entry(*id).or_default().push(vertex.point);
                } else {
                    through
                        .beneath
                        .entry(*id)
                        .or_default()
                        .push(flat.normalize());
                }
            }
        }
    }
    through
}

/// The way from a cylinder's axis to `point`, square to the axis.
fn flat_from(cylinder: &Cylinder, point: DVec3) -> DVec3 {
    let from = point - cylinder.origin;
    from - cylinder.axis * cylinder.axis.dot(from)
}
