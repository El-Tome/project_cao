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
//! sampled on the grids of both, by [`meet`].

mod meet;

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::contact::{self, Contact};
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::topology::{Body, Edge, EdgeId, VertexId};

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
        let meets: Vec<Vec<DVec3>> = body
            .edge_ids()
            .map(|id| {
                let edge = body.edge(id);
                match body.curve(edge.curve) {
                    Curve::Meet(meet) => meet::on_meet(meet, edge, tolerance, eps),
                    Curve::Line(_) | Curve::Circle(_) => Vec::new(),
                }
            })
            .collect();
        let walls = contact::walls(body);
        let contacts = contact::contacts(body, &walls, &meets, tolerance);
        let alone = Contact::default();
        for id in body.edge_ids() {
            let edge = body.edge(id);
            let between = match body.curve(edge.curve) {
                Curve::Line(_) => Vec::new(),
                Curve::Circle(circle) => {
                    let contact = walls
                        .iter()
                        .find(|(_, wall)| contact::lies_on(circle, wall, eps))
                        .and_then(|(surface, _)| contacts.get(surface))
                        .unwrap_or(&alone);
                    let touched = touched_ends(body, circle, edge);
                    let planes = touching_planes(body, circle);
                    on_circle(circle, edge, tolerance, eps, contact, touched, &planes)
                }
                Curve::Meet(_) => meets[id.0 as usize].clone(),
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

/// The points of a circle's edge between its ends, in the way the edge runs:
/// on the grid but for the steps its contact withholds, and along the rays it
/// adds. A place closer than `eps` to a grid angle is that angle, and to an
/// end that end — or, at an end a plane touches the wall at, a place so near
/// it that the circle there stands within a fifth of `eps` of the plane: the
/// sample would lie on the plane's edge. Nor is a ray taken, on any circle of
/// the wall, where the circle stands so near such a plane but not on the
/// line it touches along: a ray a partner passed on, through its own grid,
/// would lay a strip of the wall on the plane's face. So too beside a line
/// the wall touches or crosses another along: a ray is not taken where the
/// circle stands within a fifth of `eps` of the other wall a step or less
/// from an end standing so too. The vertex is the line's sample there, and a
/// second a hair round from it — through the other end of a line the kernel
/// laid leaning a hair — would stand as good as on the other wall's circle,
/// which ends at the same vertex.
fn on_circle(
    circle: &Circle,
    edge: &Edge,
    tolerance: f64,
    eps: f64,
    contact: &Contact,
    touched: [bool; 2],
    planes: &[Plane],
) -> Vec<DVec3> {
    let steps = divisions(circle.radius, tolerance);
    let step = TAU / steps as f64;
    let gap = eps / circle.radius;
    let beside_end = gap.max((2.0 * eps * contact::APART / circle.radius).sqrt());
    let [at_from, at_to] = touched.map(|touched| if touched { beside_end } else { gap });
    let whole = edge.ends.is_none();
    let (low, high) = if whole {
        (edge.from, edge.from + TAU)
    } else if edge.from <= edge.to {
        (edge.from + at_from, edge.to - at_to)
    } else {
        (edge.to + at_to, edge.from - at_from)
    };
    let inside = |at: f64| {
        if whole {
            low <= at && at < high
        } else {
            low < at && at < high
        }
    };
    let ends = if whole {
        Vec::new()
    } else {
        vec![edge.from, edge.to]
    };
    let beside =
        |at: f64, other: &Cylinder| other.distance(circle.point(at)).abs() < eps * contact::APART;
    let beside_a_plane = |at: f64| {
        let point = circle.point(at);
        planes.iter().any(|plane| {
            let touching = circle.center - plane.normal * plane.distance(circle.center);
            plane.distance(point).abs() < eps * contact::APART && (point - touching).length() > eps
        })
    };
    let by_an_end = |at: f64| {
        contact.beside.iter().any(|other| {
            beside(at, other)
                && ends
                    .iter()
                    .any(|end| (end - at).abs() <= step && beside(*end, other))
        })
    };

    let mut places: Vec<(f64, bool, f64)> = ((low / step).floor() as i64
        ..=(high / step).ceil() as i64)
        .filter(|rank| inside(*rank as f64 * step))
        .filter(|rank| {
            !contact
                .withheld
                .contains(&(rank.rem_euclid(steps as i64) as usize))
        })
        .map(|rank| {
            let angle = TAU * rank.rem_euclid(steps as i64) as f64 / steps as f64;
            (rank as f64 * step, true, angle)
        })
        .collect();
    for way in &contact.rays {
        let angle = way.dot(circle.v).atan2(way.dot(circle.u));
        let mut at = angle + TAU * ((low - angle) / TAU).ceil();
        if whole && at >= high {
            at = low;
        }
        while at < high {
            if inside(at) && !by_an_end(at) && !beside_a_plane(at) {
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
    let apart = |one: f64, other: f64| ((one - other + PI).rem_euclid(TAU) - PI).abs();
    let anchored = |angle: f64| {
        contact
            .anchors
            .iter()
            .map(|point| {
                let from = *point - circle.center;
                let along = circle.axis.dot(from);
                (from - circle.axis * along, along.abs())
            })
            .filter(|(flat, _)| apart(flat.dot(circle.v).atan2(flat.dot(circle.u)), angle) <= gap)
            .min_by(|one, other| one.1.total_cmp(&other.1))
    };
    let mut points: Vec<DVec3> = kept
        .into_iter()
        .map(|(_, _, angle)| match anchored(angle) {
            Some((flat, _)) => circle.center + flat,
            None => circle.point(angle),
        })
        .collect();
    points.dedup();
    if whole && points.len() > 1 && points[0] == points[points.len() - 1] {
        points.pop();
    }
    points
}

/// The planes of the body touching a circle's wall along a line.
fn touching_planes(body: &Body, circle: &Circle) -> Vec<Plane> {
    let eps = body.scale().eps();
    body.surfaces
        .iter()
        .filter_map(|surface| match surface {
            Surface::Plane(plane) => Some(*plane),
            Surface::Cylinder(_) => None,
        })
        .filter(|plane| {
            plane.normal.dot(circle.axis).abs() <= Scale::RELATIVE
                && (plane.distance(circle.center).abs() - circle.radius).abs() <= eps
        })
        .collect()
}

/// Whether each end of a circle's edge, its start then its end, is a vertex
/// lying on a plane that touches the circle's wall there.
fn touched_ends(body: &Body, circle: &Circle, edge: &Edge) -> [bool; 2] {
    let eps = body.scale().eps();
    let touches = |vertex: VertexId| {
        body.vertex(vertex)
            .on
            .iter()
            .any(|surface| match body.surface(*surface) {
                Surface::Plane(plane) => {
                    plane.normal.dot(circle.axis).abs() <= Scale::RELATIVE
                        && (plane.distance(circle.center).abs() - circle.radius).abs() <= eps
                }
                Surface::Cylinder(_) => false,
            })
    };
    edge.ends.map_or([false; 2], |ends| ends.map(touches))
}

#[cfg(test)]
mod tests;
