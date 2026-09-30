//! Where each edge is sampled: once, whichever faces use it, so that every
//! face along an edge stands on the same points, bit for bit, and the
//! triangles close by construction.
//!
//! A line is sampled at its ends. A circle at the grid of the cylinder it lies
//! on — the angles `2πk/N` from the cylinder's `u`, which a circle of that
//! cylinder shares — so that on a cylinder every curve has a point at every
//! grid angle, and no triangle cut between two of them spans more than one
//! step. The ends of an edge are its vertices' own points. A circle of a
//! cylinder in contact with another is also sampled on the rays [`contact`]
//! gives it.

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::contact;
use crate::brep::curve::{Circle, Curve, Meet};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, Edge, EdgeId, SurfaceId};

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
        let rays = contact::rays(body, tolerance);
        for id in body.edge_ids() {
            let edge = body.edge(id);
            let between = match body.curve(edge.curve) {
                Curve::Line(_) => Vec::new(),
                Curve::Circle(circle) => {
                    let extra = (0..body.surfaces.len() as u32)
                        .map(SurfaceId)
                        .find(|surface| lies_on(body, circle, *surface))
                        .and_then(|surface| rays.get(&surface))
                        .map_or(&[][..], Vec::as_slice);
                    on_circle(circle, edge, tolerance, eps, extra)
                }
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

/// Whether a circle is one of the circles of a surface of the body: a
/// cylinder of its radius about its axis.
fn lies_on(body: &Body, circle: &Circle, surface: SurfaceId) -> bool {
    let Surface::Cylinder(cylinder) = body.surface(surface) else {
        return false;
    };
    let eps = body.scale().eps();
    let from = circle.center - cylinder.origin;
    cylinder.axis.cross(circle.axis).length() <= Scale::RELATIVE
        && (cylinder.radius - circle.radius).abs() <= eps
        && (from - cylinder.axis * cylinder.axis.dot(from)).length() <= eps
}

/// The points of a circle's edge between its ends, in the way the edge runs:
/// on the grid, and along the directions `extra` from its centre. A place
/// closer than `eps` to an end is that end, and to a grid angle that angle.
fn on_circle(
    circle: &Circle,
    edge: &Edge,
    tolerance: f64,
    eps: f64,
    extra: &[DVec3],
) -> Vec<DVec3> {
    let steps = divisions(circle.radius, tolerance);
    let step = TAU / steps as f64;
    let gap = eps / circle.radius;
    let whole = edge.ends.is_none();
    let (low, high) = if whole {
        (edge.from, edge.from + TAU)
    } else {
        (edge.from.min(edge.to) + gap, edge.from.max(edge.to) - gap)
    };
    let inside = |at: f64| {
        if whole {
            low <= at && at < high
        } else {
            low < at && at < high
        }
    };

    let mut places: Vec<(f64, bool, f64)> = ((low / step).floor() as i64
        ..=(high / step).ceil() as i64)
        .filter(|rank| inside(*rank as f64 * step))
        .map(|rank| {
            let angle = TAU * rank.rem_euclid(steps as i64) as f64 / steps as f64;
            (rank as f64 * step, true, angle)
        })
        .collect();
    for way in extra {
        let angle = way.dot(circle.v).atan2(way.dot(circle.u));
        let mut at = angle + TAU * ((low - angle) / TAU).ceil();
        while at < high {
            if inside(at) {
                places.push((at, false, angle));
            }
            at += TAU;
        }
    }
    places.sort_by(|one, other| one.0.total_cmp(&other.0).then(other.1.cmp(&one.1)));
    let mut kept: Vec<(f64, bool, f64)> = Vec::with_capacity(places.len());
    for place in places {
        match kept.last_mut() {
            Some(last) if place.0 - last.0 <= gap => {
                if place.1 && !last.1 {
                    *last = place;
                }
            }
            _ => kept.push(place),
        }
    }
    if whole && kept.len() > 1 && kept[0].0 + TAU - kept[kept.len() - 1].0 <= gap {
        kept.pop();
    }
    if edge.to < edge.from {
        kept.reverse();
    }
    kept.into_iter()
        .map(|(_, _, angle)| circle.point(angle))
        .collect()
}

/// The points of an edge along the curve two perpendicular cylinders meet
/// along, between its ends. Not sampled yet: `meet.rs` does not evaluate the
/// curve, so the edge keeps its two ends and nothing between, and a face
/// bounded by a whole loop of it is left open.
///
/// What it is to take, once the curve is evaluated: the parameters where its
/// angle on either cylinder, read through `Meet::seen_on`, crosses that
/// cylinder's grid, and those where it turns back in either angle, so that on
/// both cylinders it has a point at every grid angle as circles do.
fn on_meet(_meet: &Meet, _edge: &Edge) -> Vec<DVec3> {
    Vec::new()
}

#[cfg(test)]
mod tests;
