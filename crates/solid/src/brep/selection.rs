//! Steps 5 to 7 of `docs/exact-kernel.md`: each surface an operand has a face
//! on is parted by the arcs lying on it into regions, and a region is kept
//! where the operation says something different just above it and just below
//! it — the side its surface's own normal points to, and the other.
//!
//! On each side of a region, each operand wraps it some number of times: a
//! face of the operand covering the region says one on its matter's side and
//! nought on the other; no face covering it says the same on both sides, as
//! a ray cast through the operand counts. One rule gives coincident faces
//! once, drops a shared wall, cuts a flush hole, and leaves a non-manifold
//! edge its four uses.

mod wrapped;

use std::collections::BTreeMap;

use super::Declined;
use super::combine::{Arena, Operands, Operation};
use super::overlay::{Arc, Overlay, Region};
use super::topology::{Coedge, EdgeId, Face, SurfaceId, VertexId};
use wrapped::{covering, wound};

pub(super) fn selected(
    operands: &Operands,
    arena: &Arena,
    operation: Operation,
) -> Result<Vec<Face>, Declined> {
    let mut faces = Vec::new();
    for rank in 0..operands.surfaces.list.len() {
        let surface = SurfaceId(rank as u32);
        if operands.carries(0, surface) || operands.carries(1, surface) {
            faces.extend(on(operands, arena, surface, operation)?);
        }
    }
    Ok(faces)
}

/// The faces kept on one surface.
fn on(
    operands: &Operands,
    arena: &Arena,
    surface: SurfaceId,
    operation: Operation,
) -> Result<Vec<Face>, Declined> {
    let (edges, overlay) = parted(arena, surface)?;
    let geometry = arena.body.surface(surface);
    let mut faces = Vec::new();
    for region in &overlay.regions {
        if region.unbounded {
            continue;
        }
        let point = geometry.point(region.inside);
        let covered = [0, 1].map(|operand| covering(operands, operand, surface, point));
        let covered = [covered[0]?, covered[1]?];
        if covered.iter().all(Option::is_none) {
            continue;
        }
        let [first, second] = [0, 1].map(|operand| match covered[operand] {
            Some(wrapped) => Ok(wrapped),
            None => wound(operands, operand, point),
        });
        let (first, second) = (first?, second?);
        let above = operation.holds(first.above >= 1, second.above >= 1);
        let below = operation.holds(first.below >= 1, second.below >= 1);
        if above != below {
            faces.push(face(surface, above, region, &edges));
        }
    }
    Ok(faces)
}

/// The arcs lying on a surface, by the edge each is, and the regions they
/// part the surface into.
pub(super) fn parted(
    arena: &Arena,
    surface: SurfaceId,
) -> Result<(Vec<EdgeId>, Overlay), Declined> {
    let body = &arena.body;
    let geometry = body.surface(surface);
    let edges: Vec<EdgeId> = body
        .edge_ids()
        .filter(|edge| {
            arena.supports[body.edge(*edge).curve.0 as usize]
                .binary_search(&surface)
                .is_ok()
        })
        .collect();
    let mut local: BTreeMap<VertexId, usize> = BTreeMap::new();
    let mut vertices = Vec::new();
    let mut arcs = Vec::with_capacity(edges.len());
    for &edge in &edges {
        let trace = body.trace(edge, surface)?;
        let ends = match body.edge(edge).ends {
            Some(ends) => {
                let mut ranks = [0; 2];
                for (rank, vertex) in ranks.iter_mut().zip(ends) {
                    *rank = *local.entry(vertex).or_insert_with(|| {
                        vertices.push(geometry.parameters(body.vertex(vertex).point));
                        vertices.len() - 1
                    });
                }
                Some(ranks)
            }
            None => None,
        };
        arcs.push(Arc { trace, ends });
    }
    let overlay = Overlay::of(&vertices, &arcs, geometry.period())?;
    Ok((edges, overlay))
}

/// A region as a face, its matter on the side its surface's own normal
/// points to when `flipped`: its cycles keep it on their left in the
/// surface's parameters, which is seen from outside the matter only when it
/// is not flipped.
fn face(surface: SurfaceId, flipped: bool, region: &Region, edges: &[EdgeId]) -> Face {
    let loops = region
        .cycles
        .iter()
        .map(|cycle| {
            let coedges = cycle.iter().map(|&(arc, forward)| Coedge {
                edge: edges[arc],
                forward,
            });
            if flipped {
                coedges
                    .rev()
                    .map(|coedge| Coedge {
                        forward: !coedge.forward,
                        ..coedge
                    })
                    .collect()
            } else {
                coedges.collect()
            }
        })
        .collect();
    Face {
        surface,
        flipped,
        loops,
    }
}

#[cfg(test)]
mod tests;
