//! Every registered curve cut at the corners lying on it, as their supports
//! say, into arcs; an arc is kept when an operand's edge covers it, or when it
//! lies in a face of each operand on two of its surfaces. An arc lies on the
//! surfaces of its curve, and on those an edge covering it lies on over its
//! stretch alone.

use glam::DVec3;

use super::Arena;
use super::held::Held;
use super::identified::identified;
use super::operands::Operands;
use crate::brep::Declined;
use crate::brep::canonical::{Pool, Registered, Registry, lies_on};
use crate::brep::curve::Curve;
use crate::brep::topology::{Body, CurveId, Edge, SurfaceId, Vertex, VertexId};

pub(super) fn cut(
    operands: &Operands,
    registry: &Registry,
    pool: &Pool,
    held: &[Held],
) -> Result<Arena, Declined> {
    let eps = operands.eps();
    let corners = pool.supports(registry);
    let points: Vec<DVec3> = pool
        .corners
        .iter()
        .zip(&corners)
        .map(|(corner, support)| placed(corner.point, support, registry, eps))
        .collect();
    let mut edges = Vec::new();
    let mut lying = Vec::new();
    for (rank, registered) in registry.list.iter().enumerate() {
        let curve = &registered.curve;
        let mut on: Vec<(f64, VertexId)> = points
            .iter()
            .enumerate()
            .filter(|(vertex, point)| lies_on(**point, &corners[*vertex], rank, registry, eps))
            .flat_map(|(vertex, point)| {
                passes(curve, *point, eps)
                    .into_iter()
                    .map(move |at| (at, VertexId(vertex as u32)))
            })
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
                lying.push(lies(rank, registered, held, &edge));
                edges.push(edge);
            }
        }
    }
    let curves: Vec<Curve> = registry.list.iter().map(|known| known.curve).collect();
    let (edges, supports) = identified(edges, lying, &curves, &registry.apart, eps);
    let body = Body {
        surfaces: operands.surfaces.list.clone(),
        curves,
        vertices: points
            .iter()
            .zip(&corners)
            .map(|(point, support)| Vertex {
                point: *point,
                on: support.clone(),
            })
            .collect(),
        edges,
        faces: Vec::new(),
        scale: operands.scale,
        arrivals: operands.arrivals(),
    };
    Ok(Arena { body, supports })
}

/// Where a corner stands: where three planes it lies on meet, when their
/// normals span space and it was found off that place by more than
/// rounding — each plane taken for another within the tolerance, the place
/// they fix moved — and where it was found otherwise. A cylinder the corner
/// lies on too holds the planes' place when it was decided to touch one of
/// them: the line they touch along is fixed by the planes the corner's
/// line lies on, and corners found along it, some on the planes, some on
/// the touch, would lean an edge between them a hair across both.
fn placed(point: DVec3, support: &[SurfaceId], registry: &Registry, eps: f64) -> DVec3 {
    let touching = |cylinder: SurfaceId| {
        support
            .iter()
            .any(|&plane| registry.planes.is_plane(plane) && registry.apart.touch(cylinder, plane))
    };
    match registry.planes.place(support, touching) {
        Some(place) if place.distance(point) > eps * ROUNDING => place,
        _ => point,
    }
}

/// Under this share of the tolerance, a corner is where its planes meet.
const ROUNDING: f64 = 1e-3;

/// The surfaces an arc lies on: its curve's, and those an operand's edge
/// covering it lies on over its stretch alone.
fn lies(rank: usize, registered: &Registered, held: &[Held], edge: &Edge) -> Vec<SurfaceId> {
    let middle = (edge.from + edge.to) / 2.0;
    let period = registered.curve.period();
    let mut support = registered.support.clone();
    for held in held
        .iter()
        .filter(|held| held.curve == rank && held.covers(middle, period, 0.0))
    {
        support.extend(held.partly.iter().copied());
    }
    support.sort();
    support.dedup();
    support
}

/// Every parameter at which a curve passes through a corner lying on it:
/// once, or at a node of the curve two cylinders meet along, at each pass.
fn passes(curve: &Curve, point: DVec3, eps: f64) -> Vec<f64> {
    let at_node = match curve {
        Curve::Meet(meet) => meet.passes(point, eps),
        _ => Vec::new(),
    };
    if at_node.is_empty() {
        vec![curve.parameter(point)]
    } else {
        at_node
    }
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
