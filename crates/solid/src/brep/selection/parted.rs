//! The arcs lying on a surface, and the regions they part the surface into.

mod glued;

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use crate::brep::Declined;
use crate::brep::combine::{Arena, Operands};
use crate::brep::curve::Curve;
use crate::brep::domain::floor;
use crate::brep::overlay::{Arc, Overlay};
use crate::brep::relation::{Relation, relation};
use crate::brep::surface::Surface;
use crate::brep::topology::{EdgeId, SurfaceId, VertexId};
use crate::brep::trace::Trace;
use glued::glued;

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
///
/// A ring torus has no pole, and its chart is a window one turn wide along
/// the meridian, cut at the antipode of its seam: the window's bottom and
/// top are its floors (#576). A whole meridian circle is read from the one
/// to the other, ending on each at a tip of its own, as an arc at a pole
/// does, and the floors join the tips and close the window, so that a whole
/// ring is one region and a ring turned part of the way the strip between
/// its two meridians, each a loop of its own once the floors are left out,
/// as any whole curve with no vertex is. A meridian arc running past the
/// cut is read as two arcs of one edge, the one ending at a tip on the floor
/// it leaves by, the other starting at a tip on the floor it comes back
/// through; the regions the cut parts are joined back across it and their
/// loops walked through it ([`glued`]). A rim along which another surface
/// only touches a whole ring — a slab resting on its crest — parts nothing:
/// a ring whose arcs are all such rims is read whole. That holds only where
/// the ring's own face is whole, `whole`; a ring whose face is bounded by
/// its own tangent rims — a meridian arc's two latitude circles — keeps
/// them.
pub(super) fn parted(
    arena: &Arena,
    surface: SurfaceId,
    held: &[DVec3],
    whole: bool,
) -> Result<(Vec<EdgeId>, Overlay), Declined> {
    let body = &arena.body;
    let geometry = body.surface(surface);
    let eps = body.scale().eps();
    let mut edges: Vec<EdgeId> = body
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
    let cut: Vec<f64> = match floor::seam(geometry) {
        Some(seam) if poles.is_empty() => vec![seam - PI, seam + PI],
        _ => Vec::new(),
    };
    if whole && !cut.is_empty() && edges.iter().all(|edge| touching(arena, surface, *edge)) {
        edges.clear();
    }
    let at_a_pole = |vertex: VertexId| {
        poles
            .iter()
            .position(|(point, _, _)| body.vertex(vertex).point.distance(*point) <= eps)
    };
    let mut local: BTreeMap<VertexId, usize> = BTreeMap::new();
    let mut tips: BTreeMap<(usize, u64), usize> = BTreeMap::new();
    let mut on_the_cut: BTreeMap<(usize, u64), usize> = BTreeMap::new();
    let mut vertices = Vec::new();
    let mut arcs = Vec::with_capacity(edges.len());
    let mut traced = Vec::with_capacity(edges.len());
    for &edge in &edges {
        let mut trace = body.trace(edge, surface)?;
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
            None => match (&cut[..], trace) {
                ([bottom, top], Trace::Segment { from, to })
                    if from.x == to.x && (to.y - from.y).abs() > PI =>
                {
                    let up = to.y > from.y;
                    let [low, high] = [*bottom, *top].map(|floor| DVec2::new(from.x, floor));
                    trace = if up {
                        Trace::Segment {
                            from: low,
                            to: high,
                        }
                    } else {
                        Trace::Segment {
                            from: high,
                            to: low,
                        }
                    };
                    let mut ranks = [0; 2];
                    for (rank, (line, at)) in ranks.iter_mut().zip([(0, low), (1, high)]) {
                        let slot = on_the_cut
                            .entry((line, at.x.to_bits()))
                            .or_insert(usize::MAX);
                        if *slot == usize::MAX {
                            vertices.push(at);
                            *slot = vertices.len() - 1;
                        }
                        *rank = *slot;
                    }
                    Some(if up { ranks } else { [ranks[1], ranks[0]] })
                }
                _ => None,
            },
        };
        let mut tip = |line: usize, at: DVec2| {
            let slot = on_the_cut
                .entry((line, at.x.to_bits()))
                .or_insert(usize::MAX);
            if *slot == usize::MAX {
                vertices.push(at);
                *slot = vertices.len() - 1;
            }
            *slot
        };
        match (&cut[..], trace, ends) {
            ([bottom, top], Trace::Segment { from, to }, Some([start, end]))
                if from.x == to.x && (to.y > *top || to.y < *bottom) =>
            {
                let (leaving, arriving, by) = if to.y > *top {
                    (*top, *bottom, -TAU)
                } else {
                    (*bottom, *top, TAU)
                };
                let (left, back) = (DVec2::new(from.x, leaving), DVec2::new(from.x, arriving));
                let (out, into) = (
                    tip(usize::from(to.y > *top), left),
                    tip(usize::from(to.y <= *bottom), back),
                );
                arcs.push(Arc {
                    trace: Trace::Segment { from, to: left },
                    ends: Some([start, out]),
                });
                traced.push(edge);
                arcs.push(Arc {
                    trace: Trace::Segment {
                        from: back,
                        to: DVec2::new(to.x, to.y + by),
                    },
                    ends: Some([into, end]),
                });
                traced.push(edge);
            }
            _ => {
                arcs.push(Arc { trace, ends });
                traced.push(edge);
            }
        }
    }
    let edges = traced;
    for (line, &floor) in cut.iter().enumerate() {
        let ends: Vec<(usize, DVec2)> = on_the_cut
            .iter()
            .filter(|((at, _), _)| *at == line)
            .map(|(_, &vertex)| (vertex, vertices[vertex]))
            .collect();
        arcs.extend(floor::arcs_at(floor, &ends, true));
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
    if !cut.is_empty() {
        let mut twins = BTreeMap::new();
        for (&(line, angle), &tip) in &on_the_cut {
            if let Some(&other) = on_the_cut.get(&(1 - line, angle)) {
                twins.insert(tip, other);
            }
        }
        let overlay = glued(overlay, &arcs, &edges, &twins)?;
        return Ok((edges, overlay));
    }
    for region in &mut overlay.regions {
        let mut cycles = Vec::new();
        for cycle in &region.cycles {
            let mut run = Vec::new();
            for &(arc, forward) in cycle.iter().filter(|(arc, _)| *arc < edges.len()) {
                if body.edge(edges[arc]).ends.is_none() {
                    cycles.push(vec![(arc, forward)]);
                } else {
                    run.push((arc, forward));
                }
            }
            if !run.is_empty() {
                cycles.push(run);
            }
        }
        region.cycles = cycles;
    }
    Ok((edges, overlay))
}

/// Whether an edge of a sphere or a torus is a rim along which a surface it
/// also lies on only touches the round (#576).
fn touching(arena: &Arena, surface: SurfaceId, edge: EdgeId) -> bool {
    let body = &arena.body;
    let Curve::Circle(circle) = body.curve(body.edge(edge).curve) else {
        return false;
    };
    let eps = body.scale().eps();
    arena.supports[edge.0 as usize]
        .iter()
        .filter(|on| **on != surface)
        .any(|on| {
            matches!(
                relation(body.surface(surface), body.surface(*on), body.scale()),
                Relation::Rims(rims) if rims.iter().any(|rim| {
                    rim.touching
                        && rim.circle.center.distance(circle.center) <= eps
                        && (rim.circle.radius - circle.radius).abs() <= eps
                })
            )
        })
}
