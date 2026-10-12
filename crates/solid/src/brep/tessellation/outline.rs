//! A face's loops laid out in its surface's parameters, the region on the
//! left of every segment: what the sweep cuts.

mod cone;
mod cylinder;

use std::collections::BTreeMap;

use glam::DVec2;

use super::sampling::{Samples, divisions};
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, FaceId};

/// Points of the parameters, each standing for a sample, and the segments
/// between them.
pub(super) struct Outline {
    pub(super) points: Vec<DVec2>,
    pub(super) samples: Vec<usize>,
    pub(super) segments: Vec<[usize; 2]>,
    at: BTreeMap<[u64; 2], usize>,
}

impl Outline {
    /// The outline of a face, turned when the face is flipped so that its
    /// region is on the left in the surface's own parameters. None when the
    /// face cannot be laid out.
    pub(super) fn of(
        body: &Body,
        samples: &Samples,
        face: FaceId,
        tolerance: f64,
    ) -> Option<Outline> {
        let face = body.face(face);
        let laps: Vec<Vec<usize>> = face
            .loops
            .iter()
            .map(|lap| {
                let mut ids = Vec::new();
                for coedge in lap {
                    let along = samples.edge(coedge.edge);
                    let closed = body.edge(coedge.edge).ends.is_none();
                    let run: Vec<usize> = if coedge.forward {
                        along.to_vec()
                    } else {
                        along.iter().rev().copied().collect()
                    };
                    let kept = if closed { run.len() } else { run.len() - 1 };
                    ids.extend(&run[..kept]);
                }
                if face.flipped {
                    ids.reverse();
                }
                ids
            })
            .collect();
        if laps.iter().any(|lap| lap.len() < 2) {
            return None;
        }
        let mut outline = Outline {
            points: Vec::new(),
            samples: Vec::new(),
            segments: Vec::new(),
            at: BTreeMap::new(),
        };
        match body.surface(face.surface) {
            Surface::Plane(plane) => {
                for lap in laps {
                    let placed: Vec<(DVec2, usize)> = lap
                        .into_iter()
                        .map(|id| (plane.parameters(samples.point(id)), id))
                        .collect();
                    outline.close(&placed);
                }
            }
            Surface::Cylinder(cylinder) => {
                let steps = divisions(cylinder.radius, tolerance);
                outline.round(cylinder, samples, &laps, steps)?;
            }
            Surface::Cone(cone) => {
                let steps = samples.steps(face.surface)?;
                let apex = face.apex.map(|vertex| vertex.0 as usize);
                let eps = body.scale().eps();
                outline.conical(cone, samples, &laps, steps, apex, eps)?;
            }
        }
        Some(outline)
    }

    /// The point at `at` standing for sample `id`, made once.
    fn point(&mut self, at: DVec2, id: usize) -> usize {
        *self
            .at
            .entry([at.x.to_bits(), at.y.to_bits()])
            .or_insert_with(|| {
                self.points.push(at);
                self.samples.push(id);
                self.points.len() - 1
            })
    }

    /// A chain of points, each joined to the next.
    fn chain(&mut self, placed: &[(DVec2, usize)]) {
        let ids: Vec<usize> = placed.iter().map(|(at, id)| self.point(*at, *id)).collect();
        for pair in ids.windows(2) {
            self.segments.push([pair[0], pair[1]]);
        }
    }

    /// A loop of points, the last joined back to the first.
    fn close(&mut self, placed: &[(DVec2, usize)]) {
        if let Some(first) = placed.first() {
            let mut round = placed.to_vec();
            round.push(*first);
            self.chain(&round);
        }
    }
}
