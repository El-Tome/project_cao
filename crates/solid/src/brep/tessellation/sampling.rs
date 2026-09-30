//! Where each edge is sampled: once, whichever faces use it, so that every
//! face along an edge stands on the same points, bit for bit, and the
//! triangles close by construction.
//!
//! A line is sampled at its ends. A circle at the grid of the cylinder it lies
//! on — the angles `2πk/N` from the cylinder's `u`, which a circle of that
//! cylinder shares — so that on a cylinder every curve has a point at every
//! grid angle, and no triangle cut between two of them spans more than one
//! step. The ends of an edge are its vertices' own points.

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use crate::brep::curve::{Circle, Curve, Meet};
use crate::brep::topology::{Body, Edge, EdgeId};

const LEAST: usize = 16;
const MOST: usize = 1024;

/// How many equal steps a whole turn of a circle of `radius` is cut into, so
/// that no chord of one step stands further than `tolerance` from its arc.
///
/// A multiple of four, so that the quarters from `u` are samples: where the
/// planes of the origin touch a cylinder standing square to one of them. At
/// least sixteen, so that a coarse tolerance still draws a round; at most a
/// thousand and twenty-four, so that a fine one does not draw a million
/// triangles.
pub(super) fn divisions(radius: f64, tolerance: f64) -> usize {
    let ratio = tolerance / radius;
    if ratio.is_nan() || ratio <= 0.0 {
        return MOST;
    }
    let half_step = (1.0 - ratio.min(2.0)).acos();
    let needed = (PI / half_step).ceil();
    ((needed / 4.0).ceil() * 4.0).clamp(LEAST as f64, MOST as f64) as usize
}

/// Every edge's points, in the way the edge runs: its first vertex, the points
/// between, its last vertex. A ring's points go round from its start and do
/// not come back to it.
///
/// Points are numbered: the vertices first, by rank, then the rest.
pub(super) struct Samples {
    points: Vec<DVec3>,
    edges: Vec<Vec<usize>>,
    vertices: usize,
}

impl Samples {
    pub(super) fn of(body: &Body, tolerance: f64) -> Samples {
        let mut samples = Samples {
            points: body.vertex_ids().map(|id| body.vertex(id).point).collect(),
            edges: Vec::new(),
            vertices: body.vertex_ids().count(),
        };
        let eps = body.scale().eps();
        for id in body.edge_ids() {
            let edge = body.edge(id);
            let between = match body.curve(edge.curve) {
                Curve::Line(_) => Vec::new(),
                Curve::Circle(circle) => on_circle(circle, edge, tolerance, eps),
                Curve::Meet(meet) => on_meet(meet, edge),
            };
            let first = samples.points.len();
            samples.points.extend(between);
            let inner = first..samples.points.len();
            samples.edges.push(match edge.ends {
                Some([start, end]) => std::iter::once(start.0 as usize)
                    .chain(inner)
                    .chain(std::iter::once(end.0 as usize))
                    .collect(),
                None => inner.collect(),
            });
        }
        samples
    }

    pub(super) fn point(&self, id: usize) -> DVec3 {
        self.points[id]
    }

    pub(super) fn edge(&self, edge: EdgeId) -> &[usize] {
        &self.edges[edge.0 as usize]
    }

    /// Whether a sample is a vertex's own point.
    pub(super) fn is_vertex(&self, id: usize) -> bool {
        id < self.vertices
    }
}

/// The points of a circle's edge between its ends, on the grid, in the way
/// the edge runs; a grid angle closer to an end than `eps` is that end.
fn on_circle(circle: &Circle, edge: &Edge, tolerance: f64, eps: f64) -> Vec<DVec3> {
    let steps = divisions(circle.radius, tolerance);
    let step = TAU / steps as f64;
    let gap = eps / circle.radius;
    let ranks: Vec<i64> = if edge.ends.is_none() {
        let first = (edge.from / step).ceil() as i64;
        (first..first + steps as i64).collect()
    } else {
        let (low, high) = (edge.from.min(edge.to) + gap, edge.from.max(edge.to) - gap);
        let mut ranks: Vec<i64> = ((low / step).floor() as i64..=(high / step).ceil() as i64)
            .filter(|rank| {
                let angle = *rank as f64 * step;
                low < angle && angle < high
            })
            .collect();
        if edge.to < edge.from {
            ranks.reverse();
        }
        ranks
    };
    ranks
        .into_iter()
        .map(|rank| circle.point(TAU * rank.rem_euclid(steps as i64) as f64 / steps as f64))
        .collect()
}

/// The curve two perpendicular cylinders meet along is to be sampled at the
/// grid angles of both, seen on either. `meet.rs` does not evaluate it yet, so
/// an edge along it keeps its two ends and nothing between.
fn on_meet(_meet: &Meet, _edge: &Edge) -> Vec<DVec3> {
    Vec::new()
}

#[cfg(test)]
mod tests;
