//! Where each edge is sampled: once, whichever faces use it, so that every
//! face along an edge stands on the same points, bit for bit, and the
//! triangles close by construction.
//!
//! A line is sampled at its ends. A circle at the grid of the cylinder it lies
//! on — the angles `2πk/N` from the cylinder's `u`, which a circle of that
//! cylinder shares — so that on a cylinder every curve has a point at every
//! grid angle, and no triangle cut between two of them spans more than one
//! step. The ends of an edge are its vertices' own points. A circle is also
//! sampled at the angle of every vertex on its cylinder, so that a ruling
//! from a vertex meets a sample on every rim of its wall, and that sample
//! stands square to the axis from the vertex nearest it, where the kernel
//! put the vertex — within its tolerance of the circle, and on the other
//! surface the kernel decided the wall touches there. A circle of a
//! cylinder in contact with another is also sampled on the rays [`contact`]
//! gives it, and not at the steps it withholds: there the triangle next to
//! the line two walls touch along spans more than one step, by the stretch
//! withheld. A circle whose wall is gone — a cylinder swallowed by one it
//! touches leaves its circles on the caps — is sampled as the circles of a
//! cylinder of its own. The curve two perpendicular cylinders meet along is
//! sampled on the grids of both and on the rays their circles take, by
//! [`meet`]: the rays come of the curves' own samples, so the curves are
//! sampled twice, the second time on the rays the first gave. Nowhere is it
//! sampled where either cylinder all but lies on a wall facing it, as a
//! circle is not at the steps it withholds, nor on a plane touching either
//! off the line they touch along. Two edges touching between their vertices
//! share the place they touch at ([`touches`]).

mod beneath;
mod circle;
mod ends;
mod meet;
mod planes;
mod touches;

use std::collections::BTreeMap;
use std::f64::consts::PI;

use glam::DVec3;

use super::contact::{self, Contact, Zones};
use crate::brep::curve::Curve;
use crate::brep::surface::Cylinder;
use crate::brep::topology::{Body, EdgeId, SurfaceId};
use circle::on_circle;
use ends::Ends;
pub(super) use planes::{on_a_plane, touching_planes};
use touches::{Places, on_the_line};

/// How far from another surface, as a share of the kernel's tolerance, a
/// place of the grid a step or less from an end lying on that surface must
/// stand to be kept: far over the rounding of a point, a ten-millionth of
/// the tolerance, so that exact signs tell it from the other surface, and
/// far under what the arc covers in a step, so that dropping it lengthens
/// no chord by much. Two circles of one radius crossing a hair apart stand
/// within the rounding of each other a long way round from the vertex they
/// cross at, and a place there on each, not on a ray the two share, has the
/// arcs cross back and forth between them.
const TOLD: f64 = 1e-3;

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
/// not come back to it. No point between stands within the kernel's
/// tolerance of an end, which it would be twice. A place within rounding of
/// another vertex — a corner the kernel left on the curve, not at its end —
/// is that vertex's own point: with none, the edge's chord would pass a hair
/// beside the corner, on the wrong side of the curve ending there; a face
/// beside the edge pinches there, or runs out to it and back as a hair. A
/// place a hair from a vertex off the edge stays: on the ray through the end
/// of a curve beside it, it keeps the two in order.
///
/// Points are numbered: the vertices first, by rank, then the rest.
pub(super) struct Samples {
    points: Vec<DVec3>,
    edges: Vec<Vec<usize>>,
    vertices: usize,
}

impl Samples {
    /// Sampled twice where a curve's samples stand just inside a wall they
    /// do not lie on, the second time on the rays through them.
    pub(super) fn of(body: &Body, tolerance: f64) -> Samples {
        let walls = contact::walls(body);
        let zones = Zones::of(body, &walls);
        let first = Samples::through(body, &walls, &zones, &BTreeMap::new(), tolerance);
        let beneath = beneath::walls(body, &walls, &first, tolerance);
        if beneath.is_empty() {
            return first;
        }
        Samples::through(body, &walls, &zones, &beneath, tolerance)
    }

    /// Every edge sampled, each wall's circles also on the rays through the
    /// points `beneath` gives it.
    fn through(
        body: &Body,
        walls: &[contact::Wall],
        zones: &Zones,
        beneath: &BTreeMap<SurfaceId, Vec<DVec3>>,
        tolerance: f64,
    ) -> Samples {
        let mut samples = Samples {
            points: body.vertex_ids().map(|id| body.vertex(id).point).collect(),
            edges: Vec::new(),
            vertices: body.vertex_ids().count(),
        };
        let eps = body.scale().eps();
        let alone = BTreeMap::new();
        let mut meets = on_meets(body, walls, zones, &alone, tolerance);
        let mut contacts = contact::contacts(body, walls, zones, &meets, beneath, tolerance);
        let meeting = body
            .edge_ids()
            .any(|id| matches!(body.curve(body.edge(id).curve), Curve::Meet(_)));
        if meeting {
            meets = on_meets(body, walls, zones, &contacts, tolerance);
            contacts = contact::contacts(body, walls, zones, &meets, beneath, tolerance);
        }
        let vertices = samples.points.clone();
        let alone = Contact::default();
        let mut places = Places::new(eps * TOLD);
        for id in body.edge_ids() {
            let edge = body.edge(id);
            let between = match body.curve(edge.curve) {
                Curve::Line(_) => Vec::new(),
                Curve::Circle(circle) => {
                    let contact = contact::wall_of(circle, walls, eps)
                        .and_then(|(surface, _)| contacts.get(surface))
                        .unwrap_or(&alone);
                    let ends = Ends::of(body, circle, edge);
                    let planes = touching_planes(body, circle.center, circle.axis, circle.radius);
                    on_circle(circle, edge, tolerance, eps, contact, &ends, &planes)
                }
                Curve::Meet(_) => meets[id.0 as usize].clone(),
            };
            let ends: Vec<DVec3> = edge
                .ends
                .into_iter()
                .flatten()
                .map(|end| body.vertex(end).point)
                .collect();
            let mut inner = Vec::new();
            for point in between {
                if ends.iter().any(|end| (point - *end).length() <= eps) {
                    continue;
                }
                match vertices
                    .iter()
                    .position(|vertex| (point - *vertex).length() <= eps * TOLD)
                {
                    Some(vertex) => {
                        if inner.last() != Some(&vertex) {
                            inner.push(vertex);
                        }
                    }
                    None => match places.found(&samples.points, point, id.0 as usize) {
                        Some(shared) => {
                            if inner.last() != Some(&shared) {
                                inner.push(shared);
                            }
                        }
                        None => {
                            places.place(point, samples.points.len(), id.0 as usize);
                            inner.push(samples.points.len());
                            samples.points.push(point);
                        }
                    },
                }
            }
            samples.edges.push(match edge.ends {
                Some([start, end]) => std::iter::once(start.0 as usize)
                    .chain(inner)
                    .chain(std::iter::once(end.0 as usize))
                    .collect(),
                None => {
                    if inner.len() > 1 && inner.first() == inner.last() {
                        inner.pop();
                    }
                    inner
                }
            });
        }
        for id in body.edge_ids() {
            let edge = body.edge(id);
            if let (Curve::Line(_), Some(_)) = (body.curve(edge.curve), edge.ends) {
                let run = &samples.edges[id.0 as usize];
                let on = on_the_line(&samples.points, [run[0], run[1]], eps * TOLD, eps);
                samples.edges[id.0 as usize].splice(1..1, on);
            }
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

/// The points of every edge along a meet between its ends, each on the rays
/// its two cylinders take in `contacts`, and none where either cylinder all
/// but lies on a wall facing it, nor on a plane touching it but off the line
/// they touch along, as a circle takes no ray there; none for any other
/// edge.
fn on_meets(
    body: &Body,
    walls: &[contact::Wall],
    zones: &Zones,
    contacts: &BTreeMap<SurfaceId, Contact>,
    tolerance: f64,
) -> Vec<Vec<DVec3>> {
    let eps = body.scale().eps();
    let angles = |cylinder: &Cylinder| -> Vec<f64> {
        walls
            .iter()
            .find(|(_, wall)| wall == cylinder)
            .and_then(|(id, _)| contacts.get(id))
            .map_or_else(Vec::new, |contact| {
                contact
                    .rays
                    .iter()
                    .map(|way| way.dot(cylinder.v).atan2(way.dot(cylinder.u)))
                    .collect()
            })
    };
    body.edge_ids()
        .map(|id| {
            let edge = body.edge(id);
            match body.curve(edge.curve) {
                Curve::Meet(meet) => {
                    let [first, second] = [angles(&meet.first), angles(&meet.second)];
                    let walls = [meet.first, meet.second];
                    let planes = walls
                        .map(|wall| touching_planes(body, wall.origin, wall.axis, wall.radius));
                    let grazed = meet::grazing_ends(body, edge, &walls, tolerance);
                    let clear = |point: DVec3| {
                        !grazed(point, eps * contact::APART)
                            && walls.iter().zip(&planes).all(|(wall, planes)| {
                                !zones.crowded(wall, point)
                                    && !planes.iter().any(|plane| {
                                        on_a_plane(plane, wall.origin, wall.axis, point, eps)
                                    })
                            })
                    };
                    meet::on_meet(meet, edge, tolerance, eps, [&first, &second], &clear)
                }
                Curve::Line(_) | Curve::Circle(_) => Vec::new(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests;
