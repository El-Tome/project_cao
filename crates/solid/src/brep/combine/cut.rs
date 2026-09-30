//! Every registered curve cut at the corners lying on it, as their supports
//! say, into arcs; an arc is kept when an operand's edge covers it, or when it
//! lies in a face of each operand on two of its surfaces.

use super::Arena;
use super::held::Held;
use super::operands::Operands;
use crate::brep::Declined;
use crate::brep::canonical::{Pool, Registered, Registry, lies_on};
use crate::brep::topology::{Body, CurveId, Edge, SurfaceId, Vertex, VertexId};

pub(super) fn cut(
    operands: &Operands,
    registry: &Registry,
    pool: &Pool,
    held: &[Held],
) -> Result<Arena, Declined> {
    let eps = operands.eps();
    let supports = pool.supports(registry);
    let mut edges = Vec::new();
    for (rank, registered) in registry.list.iter().enumerate() {
        let curve = &registered.curve;
        let mut on: Vec<(f64, VertexId)> = pool
            .corners
            .iter()
            .enumerate()
            .filter(|(vertex, corner)| {
                lies_on(corner.point, &supports[*vertex], rank, registry, eps)
            })
            .map(|(vertex, corner)| (curve.parameter(corner.point), VertexId(vertex as u32)))
            .collect();
        on.sort_by(|one, other| one.0.total_cmp(&other.0).then(one.1.cmp(&other.1)));
        let id = CurveId(rank as u32);
        let pieces: Vec<Edge> = match (curve.period(), on.len()) {
            (Some(period), 0) => vec![Edge {
                curve: id,
                ends: None,
                from: 0.0,
                to: period,
            }],
            (Some(period), count) => (0..count)
                .map(|index| {
                    let (from, start) = on[index];
                    let (to, end) = match on.get(index + 1) {
                        Some(next) => *next,
                        None => (on[0].0 + period, on[0].1),
                    };
                    Edge {
                        curve: id,
                        ends: Some([start, end]),
                        from,
                        to,
                    }
                })
                .collect(),
            (None, _) => on
                .windows(2)
                .map(|pair| Edge {
                    curve: id,
                    ends: Some([pair[0].1, pair[1].1]),
                    from: pair[0].0,
                    to: pair[1].0,
                })
                .collect(),
        };
        for edge in pieces {
            if edge.from < edge.to && kept(operands, rank, registered, held, &edge)? {
                edges.push(edge);
            }
        }
    }
    let body = Body {
        surfaces: operands.surfaces.list.clone(),
        curves: registry.list.iter().map(|known| known.curve).collect(),
        vertices: pool
            .corners
            .iter()
            .zip(&supports)
            .map(|(corner, support)| Vertex {
                point: corner.point,
                on: support.clone(),
            })
            .collect(),
        edges,
        faces: Vec::new(),
        scale: operands.scale,
    };
    Ok(Arena {
        body,
        supports: registry
            .list
            .iter()
            .map(|known| known.support.clone())
            .collect(),
    })
}

/// Whether an arc is part of an operand's edge, or lies inside or on the
/// boundary of a face of one operand on one of its surfaces and of a face of
/// the other on another.
fn kept(
    operands: &Operands,
    rank: usize,
    registered: &Registered,
    held: &[Held],
    edge: &Edge,
) -> Result<bool, Declined> {
    let middle = (edge.from + edge.to) / 2.0;
    let period = registered.curve.period();
    if held
        .iter()
        .any(|held| held.curve == rank && held.covers(middle, period, 0.0))
    {
        return Ok(true);
    }
    let point = registered.curve.point(middle);
    let support: &[SurfaceId] = &registered.support;
    for (index, &one) in support.iter().enumerate() {
        for &other in &support[index + 1..] {
            for (first, second) in [(0, 1), (1, 0)] {
                if operands.carries(first, one)
                    && operands.carries(second, other)
                    && operands.touched(first, one, point)?
                    && operands.touched(second, other, point)?
                {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}
