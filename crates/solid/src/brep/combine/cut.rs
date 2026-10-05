//! Every registered curve cut at the corners lying on it, as their supports
//! say, into arcs; an arc is kept when an operand's edge covers it, or when it
//! lies in a face of each operand on two of its surfaces — or in faces on two
//! surfaces decided to touch, of either operand. An arc lies on the surfaces
//! of its curve, on those an edge covering it lies on over its stretch alone,
//! and on those its band lays it on (decision 9).

use glam::DVec3;

use super::Arena;
use super::band::{self, Band};
use super::held::Held;
use super::identified::identified;
use super::operands::Operands;
use crate::brep::Declined;
use crate::brep::canonical::{Pool, Registered, Registry, lies_on, standing};
use crate::brep::curve::Curve;
use crate::brep::domain::traced;
use crate::brep::topology::{Body, CurveId, Edge, SurfaceId, Vertex, VertexId};

pub(super) fn cut(
    operands: &Operands,
    registry: &Registry,
    pool: &Pool,
    held: &[Held],
    bands: &[Band],
) -> Result<Arena, Declined> {
    let eps = operands.eps();
    let corners = pool.supports(registry);
    let points: Vec<DVec3> = pool
        .corners
        .iter()
        .zip(&corners)
        .map(|(corner, support)| standing(corner.point, support, registry, eps))
        .collect();
    let corners = corners
        .iter()
        .zip(&points)
        .map(|(support, point)| banded(operands, registry, bands, support, *point))
        .collect::<Result<Vec<_>, _>>()?;
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
        for edge in pieces.into_iter().filter(|edge| edge.from < edge.to) {
            let mut support = lies(rank, registered, held, &edge);
            support.extend(beside(
                operands, registry, bands, registered, &edge, &corners, &support,
            )?);
            support.sort();
            if kept(operands, registry, rank, registered, &support, held, &edge)? {
                lying.push(support);
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
/// the other on another — or of faces of either on two surfaces decided to
/// touch, or two walls to cross at a grazing angle, which bound the strips
/// of their band: where one operand alone carries both, it holds a skin or
/// a crack there — the crescent between two walls of one radius a hair
/// apart — that this operation's band parts.
///
/// An arc is read at its middle. A closed curve no corner cuts crosses no
/// boundary of a face, so it lies in the faces all round or nowhere but
/// where it grazes one's boundary, which its middle may be: the curve two
/// walls meet along, turning at the very place a plane touching the one
/// wall bounds a face of the other (8554524). It is read at three places a
/// third of a turn apart, and kept where all three lie in the faces.
fn kept(
    operands: &Operands,
    registry: &Registry,
    rank: usize,
    registered: &Registered,
    support: &[SurfaceId],
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
    let shares: &[f64] = if edge.ends.is_none() {
        &[0.5, 0.5 + 1.0 / 3.0, 0.5 - 1.0 / 3.0]
    } else {
        &[0.5]
    };
    for share in shares {
        let point = registered
            .curve
            .point(edge.from + (edge.to - edge.from) * share);
        if !in_faces(operands, registry, support, point)? {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Whether a place lies inside or on the boundary of a face of one operand
/// on one of `support`'s surfaces and of a face of the other on another, or
/// of faces of either on two surfaces decided to touch or to graze.
fn in_faces(
    operands: &Operands,
    registry: &Registry,
    support: &[SurfaceId],
    point: DVec3,
) -> Result<bool, Declined> {
    for (index, &one) in support.iter().enumerate() {
        for &other in &support[index + 1..] {
            let pairs: &[(usize, usize)] =
                if registry.apart.touch(one, other) || registry.apart.graze(one, other) {
                    &[(0, 1), (1, 0), (0, 0), (1, 1)]
                } else {
                    &[(0, 1), (1, 0)]
                };
            for &(first, second) in pairs {
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

/// A corner's support grown by the surfaces its band lays it on: each a
/// surface of its own touches, within the tolerance of the corner.
fn banded(
    operands: &Operands,
    registry: &Registry,
    bands: &[Band],
    support: &[SurfaceId],
    point: DVec3,
) -> Result<Vec<SurfaceId>, Declined> {
    let mut grown = support.to_vec();
    for rank in 0..operands.surfaces.list.len() {
        let other = SurfaceId(rank as u32);
        if grown.contains(&other) || registry.apart.across(support, &[other]) {
            continue;
        }
        for &own in support {
            if band::beside(operands, registry, bands, [own, other], point)? {
                grown.push(other);
                break;
            }
        }
    }
    grown.sort();
    Ok(grown)
}

/// The surfaces an arc's band lays it on besides its own: each one of its
/// own touches, both its corners lie on, standing within the tolerance of
/// it all along and seeing it as a curve it can carry.
fn beside(
    operands: &Operands,
    registry: &Registry,
    bands: &[Band],
    registered: &Registered,
    edge: &Edge,
    corners: &[Vec<SurfaceId>],
    support: &[SurfaceId],
) -> Result<Vec<SurfaceId>, Declined> {
    let Some([from, to]) = edge.ends else {
        return Ok(Vec::new());
    };
    let eps = operands.eps();
    let curve = &registered.curve;
    let middle = curve.point((edge.from + edge.to) / 2.0);
    let mut found = Vec::new();
    for &other in &corners[from.0 as usize] {
        if support.contains(&other)
            || !corners[to.0 as usize].contains(&other)
            || registry.apart.across(support, &[other])
        {
            continue;
        }
        let surface = &operands.surfaces.list[other.0 as usize];
        let along = (0..=STRETCHES).all(|step| {
            let at = edge.from + (edge.to - edge.from) * step as f64 / STRETCHES as f64;
            surface.distance(curve.point(at)).abs() <= eps
        });
        if !along || traced(curve, surface, edge.from, edge.to, eps).is_err() {
            continue;
        }
        for &own in support {
            if band::beside(operands, registry, bands, [own, other], middle)? {
                found.push(other);
                break;
            }
        }
    }
    Ok(found)
}

/// How many stretches an arc is measured over against a surface of its band.
const STRETCHES: usize = 16;
