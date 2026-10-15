//! The arcs lying on a surface, and the regions they part the surface into.

use std::collections::BTreeMap;

use glam::DVec2;

use crate::brep::Declined;
use crate::brep::combine::{Arena, Operands};
use crate::brep::domain::floor;
use crate::brep::overlay::{Arc, Overlay};
use crate::brep::surface::Surface;
use crate::brep::topology::{EdgeId, SurfaceId, VertexId};

/// Whether a face of either operand lying on `surface` holds its cone's apex
/// within it.
pub(super) fn holds_an_apex(operands: &Operands, surface: SurfaceId) -> bool {
    (0..2).any(|operand| {
        operands.lying[operand][surface.0 as usize]
            .iter()
            .any(|&face| operands.bodies[operand].face(face).apex.is_some())
    })
}

/// The arcs lying on a surface, by the edge each is, and the regions they
/// part the surface into.
///
/// On a cone, each arc's end at the apex is a vertex of its own, at the
/// angle the arc arrives at, since the apex's own angle is rounding; the
/// floor joins them ([`floor::arcs`]), and closes below the face that
/// `holds` the apex within it, so that the region at the tip is bounded.
/// No region keeps a stretch of the floor.
pub(super) fn parted(
    arena: &Arena,
    surface: SurfaceId,
    holds: bool,
) -> Result<(Vec<EdgeId>, Overlay), Declined> {
    let body = &arena.body;
    let geometry = body.surface(surface);
    let edges: Vec<EdgeId> = body
        .edge_ids()
        .filter(|edge| {
            arena.supports[edge.0 as usize]
                .binary_search(&surface)
                .is_ok()
        })
        .collect();
    let apex = match geometry {
        Surface::Cone(cone) => Some(cone.apex()),
        Surface::Plane(_)
        | Surface::Cylinder(_)
        | Surface::Elliptic(_)
        | Surface::Sphere(_)
        | Surface::Torus(_) => None,
    };
    let at_the_apex = |vertex: VertexId| {
        apex.is_some_and(|apex| body.vertex(vertex).point.distance(apex) <= body.scale().eps())
    };
    let mut local: BTreeMap<VertexId, usize> = BTreeMap::new();
    let mut tips: BTreeMap<u64, usize> = BTreeMap::new();
    let mut vertices = Vec::new();
    let mut arcs = Vec::with_capacity(edges.len());
    for &edge in &edges {
        let trace = body.trace(edge, surface)?;
        let ends = match body.edge(edge).ends {
            Some(ends) => {
                let mut ranks = [0; 2];
                for ((rank, vertex), at) in
                    ranks.iter_mut().zip(ends).zip([trace.start(), trace.end()])
                {
                    let (slot, at) = if at_the_apex(vertex) {
                        (tips.entry(at.x.to_bits()).or_insert(usize::MAX), at)
                    } else {
                        let at = geometry.parameters(body.vertex(vertex).point);
                        (local.entry(vertex).or_insert(usize::MAX), at)
                    };
                    if *slot == usize::MAX {
                        vertices.push(at);
                        *slot = vertices.len() - 1;
                    }
                    *rank = *slot;
                }
                Some(ranks)
            }
            None => None,
        };
        arcs.push(Arc { trace, ends });
    }
    if let Surface::Cone(cone) = geometry {
        let ends: Vec<(usize, DVec2)> = tips
            .into_values()
            .map(|rank| (rank, vertices[rank]))
            .collect();
        arcs.extend(floor::arcs(cone, &ends, holds));
    }
    let mut overlay = Overlay::of(&vertices, &arcs, geometry.period())?;
    for region in &mut overlay.regions {
        region.cycles.retain_mut(|cycle| {
            cycle.retain(|&(arc, _)| arc < edges.len());
            !cycle.is_empty()
        });
    }
    Ok((edges, overlay))
}
