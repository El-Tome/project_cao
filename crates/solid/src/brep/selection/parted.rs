//! The arcs lying on a surface, and the regions they part the surface into.

use std::collections::BTreeMap;

use glam::{DVec2, DVec3};

use crate::brep::Declined;
use crate::brep::combine::{Arena, Operands};
use crate::brep::domain::floor;
use crate::brep::overlay::{Arc, Overlay};
use crate::brep::surface::Surface;
use crate::brep::topology::{EdgeId, SurfaceId, VertexId};

/// Where the poles stand that a face of either operand lying on `surface`
/// holds within it.
pub(super) fn held_poles(operands: &Operands, surface: SurfaceId) -> Vec<DVec3> {
    (0..2)
        .flat_map(|operand| {
            let body = operands.bodies[operand];
            operands.lying[operand][surface.0 as usize]
                .iter()
                .flat_map(move |&face| body.poles_held(face))
        })
        .collect()
}

/// The arcs lying on a surface, by the edge each is, and the regions they
/// part the surface into.
///
/// At a pole — a cone's apex, a sphere's or a spindle torus's ends — each
/// arc's end there is a vertex of its own, at the angle the arc arrives at,
/// since the pole's own angle is rounding; the floor joins them
/// ([`floor::arcs_at`]), and closes the face that holds the pole within it,
/// one of `held`, so that the region at the tip is bounded. No region keeps
/// a stretch of a floor.
pub(super) fn parted(
    arena: &Arena,
    surface: SurfaceId,
    held: &[DVec3],
) -> Result<(Vec<EdgeId>, Overlay), Declined> {
    let body = &arena.body;
    let geometry = body.surface(surface);
    let eps = body.scale().eps();
    let edges: Vec<EdgeId> = body
        .edge_ids()
        .filter(|edge| {
            arena.supports[edge.0 as usize]
                .binary_search(&surface)
                .is_ok()
        })
        .collect();
    let poles: Vec<(DVec3, f64, bool)> = match geometry {
        Surface::Cone(cone) => vec![(cone.apex(), cone.apex_at(), !held.is_empty())],
        Surface::Sphere(_) | Surface::Torus(_) => floor::poles_of(geometry, eps)
            .into_iter()
            .map(|pole| {
                let holds = held.iter().any(|point| point.distance(pole.point) <= eps);
                (pole.point, pole.at, holds)
            })
            .collect(),
        Surface::Plane(_) | Surface::Cylinder(_) | Surface::Elliptic(_) => Vec::new(),
    };
    let at_a_pole = |vertex: VertexId| {
        poles
            .iter()
            .position(|(point, _, _)| body.vertex(vertex).point.distance(*point) <= eps)
    };
    let mut local: BTreeMap<VertexId, usize> = BTreeMap::new();
    let mut tips: BTreeMap<(usize, u64), usize> = BTreeMap::new();
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
                    let (slot, at) = match at_a_pole(vertex) {
                        Some(pole) => {
                            (tips.entry((pole, at.x.to_bits())).or_insert(usize::MAX), at)
                        }
                        None => {
                            let at = geometry.parameters(body.vertex(vertex).point);
                            (local.entry(vertex).or_insert(usize::MAX), at)
                        }
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
    for (rank, &(_, line, holds)) in poles.iter().enumerate() {
        let ends: Vec<(usize, DVec2)> = tips
            .iter()
            .filter(|((pole, _), _)| *pole == rank)
            .map(|(_, &vertex)| (vertex, vertices[vertex]))
            .collect();
        arcs.extend(floor::arcs_at(line, &ends, holds));
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
