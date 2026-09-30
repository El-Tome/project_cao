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
//! off the line they touch along.

mod ends;
mod meet;

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::contact::{self, Contact, Zones};
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::topology::{Body, Edge, EdgeId, SurfaceId};
use ends::Ends;

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
/// tolerance of either vertex: a place of the grid a rounding past the end,
/// moved to where a vertex at its angle stands, would be that end twice.
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
        let walls = contact::walls(body);
        let zones = Zones::of(body, &walls);
        let alone = BTreeMap::new();
        let mut meets = on_meets(body, &walls, &zones, &alone, tolerance);
        let mut contacts = contact::contacts(body, &walls, &zones, &meets, tolerance);
        let meeting = body
            .edge_ids()
            .any(|id| matches!(body.curve(body.edge(id).curve), Curve::Meet(_)));
        if meeting {
            meets = on_meets(body, &walls, &zones, &contacts, tolerance);
            contacts = contact::contacts(body, &walls, &zones, &meets, tolerance);
        }
        for id in body.edge_ids() {
            let edge = body.edge(id);
            let between = match body.curve(edge.curve) {
                Curve::Line(_) => Vec::new(),
                Curve::Circle(circle) => {
                    let wall = contact::wall_of(circle, &walls, eps);
                    let mut contact = wall
                        .and_then(|(surface, _)| contacts.get(surface))
                        .map_or_else(Contact::default, Contact::clone);
                    if let Some((_, wall)) = wall {
                        let steps = divisions(wall.radius, tolerance);
                        let level = circle.center.dot(wall.axis);
                        contact.withheld = zones.withheld_at(wall, &contact.withheld, steps, level);
                    }
                    let ends = Ends::of(body, circle, edge);
                    let planes = touching_planes(body, circle.center, circle.axis, circle.radius);
                    on_circle(circle, edge, tolerance, eps, &contact, &ends, &planes)
                }
                Curve::Meet(_) => meets[id.0 as usize].clone(),
            };
            let ends: Vec<DVec3> = edge
                .ends
                .into_iter()
                .flatten()
                .map(|end| body.vertex(end).point)
                .collect();
            let between = between
                .into_iter()
                .filter(|point| ends.iter().all(|end| (*point - *end).length() > eps));
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
                    let clear = |point: DVec3| {
                        walls.iter().zip(&planes).all(|(wall, planes)| {
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
/// circle stands within the pair's room of the other wall a step or less
/// from an end standing so too. The vertex is the line's sample there, and a
/// second a hair round from it — through the other end of a line the kernel
/// laid leaning a hair — would stand as good as on the other wall's circle,
/// which ends at the same vertex. So too beside any surface an end lies on
/// and the circle does not, where the circle all but lies on it a step or
/// less from that end: a wall grazing the plane of the circle there.
fn on_circle(
    circle: &Circle,
    edge: &Edge,
    tolerance: f64,
    eps: f64,
    contact: &Contact,
    ends_of: &Ends,
    planes: &[Plane],
) -> Vec<DVec3> {
    let steps = divisions(circle.radius, tolerance);
    let step = TAU / steps as f64;
    let gap = eps / circle.radius;
    let beside_end = gap.max((2.0 * eps * contact::APART / circle.radius).sqrt());
    let [at_from, at_to] = ends_of
        .touched
        .map(|touched| if touched { beside_end } else { gap });
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
        |at: f64, (other, room): &(Cylinder, f64)| other.distance(circle.point(at)).abs() < *room;
    let beside_a_plane = |at: f64| {
        let point = circle.point(at);
        planes
            .iter()
            .any(|plane| on_a_plane(plane, circle.center, circle.axis, point, eps))
    };
    let grazed = |at: f64, room: f64| {
        ends.iter().zip(&ends_of.through).any(|(end, surfaces)| {
            (end - at).abs() <= step
                && surfaces
                    .iter()
                    .any(|surface| surface.distance(circle.point(at)).abs() < room)
        })
    };
    let by_an_end = |at: f64| {
        grazed(at, eps * contact::APART)
            || contact.beside.iter().any(|other| {
                beside(at, other)
                    && ends
                        .iter()
                        .any(|end| (end - at).abs() <= step && beside(*end, other))
            })
    };

    let mut places: Vec<(f64, bool, f64)> = ((low / step).floor() as i64
        ..=(high / step).ceil() as i64)
        .filter(|rank| inside(*rank as f64 * step) && !grazed(*rank as f64 * step, eps * TOLD))
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

/// Whether `point`, on a wall about `center` and `axis` that `plane` touches
/// along a line, stands within a fifth of `eps` of the plane but not on
/// that line: a sample there would lay a strip of the wall on the plane.
fn on_a_plane(plane: &Plane, center: DVec3, axis: DVec3, point: DVec3, eps: f64) -> bool {
    let foot = center - plane.normal * plane.distance(center);
    let touching = foot + axis * axis.dot(point - foot);
    plane.distance(point).abs() < eps * contact::APART && (point - touching).length() > eps
}

/// The planes of the body touching a wall of `radius` about `center` and
/// `axis` along a line.
fn touching_planes(body: &Body, center: DVec3, axis: DVec3, radius: f64) -> Vec<Plane> {
    let eps = body.scale().eps();
    body.surfaces
        .iter()
        .filter_map(|surface| match surface {
            Surface::Plane(plane) => Some(*plane),
            Surface::Cylinder(_) => None,
        })
        .filter(|plane| {
            plane.normal.dot(axis).abs() <= Scale::RELATIVE
                && (plane.distance(center).abs() - radius).abs() <= eps
        })
        .collect()
}

#[cfg(test)]
mod tests;
