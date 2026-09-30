//! Two cylinders closer than a chord sags — a small one inside a large one,
//! touching its wall or nearly, or two side by side touching — and how their
//! circles are sampled so that their chords keep their order and their
//! distance.
//!
//! A chord sags towards its own axis, so against a plane, or a cylinder beside
//! it, it retreats. Inside a large cylinder, within a chord's sag of its wall,
//! the large one's chords sag onto the small one's, and each sampled on its own
//! grid the two can cross. Sampled instead on common rays from the small one's
//! axis — its own grid's, and those through the large one's grid and through
//! the vertices on either — both are graphs of the angle round that axis, the
//! large one's point further out on every ray, and chords between the same two
//! rays cannot cross. Every circle of either cylinder takes the same rays, so
//! their walls stay ordered from one end to the other.
//!
//! Where two walls touch along a line, inside or side by side, they part only
//! as the square of the distance from it: a sample a hair from that line
//! stands closer to the other wall than the kernel tells places apart, and its
//! chords are as good as lying on the other wall's. Such a step of the grid is
//! withheld from both, and both take the line itself, so that the first chords
//! from the line reach out to where the walls stand apart.
//!
//! Two perpendicular cylinders meet along a curve, and touch, if they do, at
//! a node of it: the circles of both take the rays through that curve's
//! samples, so that both walls are cut in strips between them.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::sampling::divisions;
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::{Body, SurfaceId, Vertex};

/// What the circles of a cylinder take besides their grid, and what of it
/// they leave out, for the cylinders it stands close to.
#[derive(Debug, Default)]
pub(super) struct Contact {
    /// Directions square to its axis, from its axis, its circles are also
    /// sampled along.
    pub(super) rays: Vec<DVec3>,
    /// Steps of its grid its circles are not sampled at.
    pub(super) withheld: Vec<usize>,
}

impl Contact {
    fn join(&mut self, other: Contact) {
        self.rays.extend(other.rays);
        self.withheld.extend(other.withheld);
    }
}

/// A cylinder circles of the body lie on: a surface of the body, or one whose
/// wall is gone and whose circles are left on a cap — a cylinder swallowed by
/// a larger one it touches, say — numbered on past the body's surfaces.
pub(super) type Wall = (SurfaceId, Cylinder);

/// The cylinders of the body's surfaces, then those of its circles lying on
/// none of them.
pub(super) fn walls(body: &Body) -> Vec<Wall> {
    let eps = body.scale().eps();
    let mut walls: Vec<Wall> = (0..body.surfaces.len() as u32)
        .map(SurfaceId)
        .filter_map(|id| match body.surface(id) {
            Surface::Cylinder(cylinder) => Some((id, *cylinder)),
            Surface::Plane(_) => None,
        })
        .collect();
    let mut next = body.surfaces.len() as u32;
    for id in body.edge_ids() {
        let Curve::Circle(circle) = body.curve(body.edge(id).curve) else {
            continue;
        };
        if walls.iter().any(|(_, wall)| lies_on(circle, wall, eps)) {
            continue;
        }
        let wall = Cylinder {
            origin: circle.center - circle.axis * circle.axis.dot(circle.center),
            axis: circle.axis,
            radius: circle.radius,
            u: circle.u,
            v: circle.v,
        };
        walls.push((SurfaceId(next), wall));
        next += 1;
    }
    walls
}

/// Whether a circle is one of the circles of a cylinder: of its radius about
/// its axis.
pub(super) fn lies_on(circle: &Circle, cylinder: &Cylinder, eps: f64) -> bool {
    let from = circle.center - cylinder.origin;
    cylinder.axis.cross(circle.axis).length() <= Scale::RELATIVE
        && (cylinder.radius - circle.radius).abs() <= eps
        && (from - cylinder.axis * cylinder.axis.dot(from)).length() <= eps
}

/// Whether a vertex lies on a wall: as decided, on a surface of the body; by
/// its distance, on a wall that is none.
fn bears(body: &Body, vertex: &Vertex, (id, cylinder): &Wall) -> bool {
    if (id.0 as usize) < body.surfaces.len() {
        vertex.on.contains(id)
    } else {
        cylinder.distance(vertex.point).abs() <= body.scale().eps()
    }
}

/// For each wall close to another, parallel to it, what its circles take and
/// leave out.
pub(super) fn contacts(
    body: &Body,
    walls: &[Wall],
    tolerance: f64,
) -> BTreeMap<SurfaceId, Contact> {
    let eps = body.scale().eps();
    let mut contacts: BTreeMap<SurfaceId, Contact> = BTreeMap::new();
    for (at, one) in walls.iter().enumerate() {
        for other in &walls[at + 1..] {
            if one.1.axis.cross(other.1.axis).length() > Scale::RELATIVE {
                continue;
            }
            if let Some([to_one, to_other]) = touching_line(&one.1, &other.1, eps) {
                contacts.entry(one.0).or_default().rays.push(to_one);
                contacts.entry(other.0).or_default().rays.push(to_other);
            }
            let (outer, inner) = if one.1.radius >= other.1.radius {
                (one, other)
            } else {
                (other, one)
            };
            if let Some([on_outer, on_inner]) = common(body, outer, inner, tolerance) {
                contacts.entry(outer.0).or_default().join(on_outer);
                contacts.entry(inner.0).or_default().join(on_inner);
                continue;
            }
            for (near, far) in [(one, other), (other, one)] {
                let withheld = touching(&near.1, &far.1, tolerance, eps);
                if !withheld.is_empty() {
                    contacts
                        .entry(near.0)
                        .or_default()
                        .withheld
                        .extend(withheld);
                }
            }
        }
    }
    contacts
}

/// What the circles of two perpendicular cylinders take for the curve they
/// meet along: the rays through every sample of it and through its ends.
///
/// Where one wall is cut from the other, each lies over the other along the
/// curve, and they part slowly: as the square of the distance from a node
/// where they touch, or from a waist where they all but touch, and along
/// the curve wherever it runs down a wall's ruling, at the tip of a window.
/// A triangle of the large wall fanned from a sample of its circle a grid
/// step away sags there below the small wall, and at a fine tolerance the
/// curve's own samples are much closer than a step of the grid. Sampled along
/// these rays, each wall is cut into strips between two samples of the curve,
/// the large one's square to them and the small one's, between its two mirror
/// lobes, across them: within a strip each sags as the square of its width,
/// and the two stay ordered.
///
/// `meets` holds every edge's samples between its ends, empty but for the
/// curves two cylinders meet along.
pub(super) fn along_meets(
    body: &Body,
    meets: &[Vec<DVec3>],
    contacts: &mut BTreeMap<SurfaceId, Contact>,
) {
    for id in body.edge_ids() {
        let edge = body.edge(id);
        let Curve::Meet(meet) = body.curve(edge.curve) else {
            continue;
        };
        let ends = edge
            .ends
            .into_iter()
            .flatten()
            .map(|end| body.vertex(end).point);
        let through: Vec<DVec3> = meets[id.0 as usize].iter().copied().chain(ends).collect();
        for cylinder in [meet.first, meet.second] {
            let flat = |point: DVec3| {
                let from = point - cylinder.origin;
                from - cylinder.axis * cylinder.axis.dot(from)
            };
            let rays: Vec<DVec3> = through
                .iter()
                .map(|point| flat(*point).normalize())
                .collect();
            for surface in (0..body.surfaces.len() as u32).map(SurfaceId) {
                if *body.surface(surface) == Surface::Cylinder(cylinder) {
                    contacts
                        .entry(surface)
                        .or_default()
                        .rays
                        .extend(rays.iter().copied());
                }
            }
        }
    }
}

/// What the circles of every cylinder take for the vertices lying on it: the
/// rays through them. A ruling or a seam leaves its wall at a vertex off the
/// grid, and a strip of the wall beside it must meet a sample on every rim at
/// that angle, or its triangle reaches from the vertex to the other rim's next
/// step — lying flat where the wall is a hair high, over the face beside it.
pub(super) fn through_vertices(
    body: &Body,
    walls: &[Wall],
    contacts: &mut BTreeMap<SurfaceId, Contact>,
) {
    for wall in walls {
        let (id, cylinder) = wall;
        for vertex in body.vertex_ids().map(|id| body.vertex(id)) {
            if !bears(body, vertex, wall) {
                continue;
            }
            let from = vertex.point - cylinder.origin;
            let flat = from - cylinder.axis * cylinder.axis.dot(from);
            if flat.length() > 0.0 {
                contacts.entry(*id).or_default().rays.push(flat.normalize());
            }
        }
    }
}

/// The directions from the axes of two parallel cylinders to the line their
/// walls touch along, when they touch, side by side or one inside the other.
///
/// Every circle of either is sampled on that line, vertex or not: the steps
/// of the grid beside it are withheld, and a ring passing it with no vertex
/// there — its wall touching the other only further along, or not at all
/// where the other's wall is gone — would otherwise cut across it by a chord
/// several steps long.
fn touching_line(one: &Cylinder, other: &Cylinder, eps: f64) -> Option<[DVec3; 2]> {
    let between = other.origin - one.origin;
    let flat = between - one.axis * one.axis.dot(between);
    let apart = flat.length();
    if apart == 0.0 {
        return None;
    }
    let way = flat / apart;
    if (apart - (one.radius + other.radius)).abs() <= eps {
        Some([way, -way])
    } else if (apart - (one.radius - other.radius).abs()).abs() <= eps {
        let out = if one.radius >= other.radius {
            way
        } else {
            -way
        };
        Some([out, out])
    } else {
        None
    }
}

/// The steps of a cylinder's grid standing within `eps` of another cylinder
/// parallel to it: none unless their circles come that close somewhere.
fn touching(cylinder: &Cylinder, other: &Cylinder, tolerance: f64, eps: f64) -> Vec<usize> {
    let between = cylinder.origin - other.origin;
    let apart = (between - other.axis * other.axis.dot(between)).length();
    let (low, high) = (
        (cylinder.radius - other.radius).abs(),
        cylinder.radius + other.radius,
    );
    if apart < low - eps || apart > high + eps {
        return Vec::new();
    }
    let steps = divisions(cylinder.radius, tolerance);
    (0..steps)
        .filter(|step| {
            let angle = TAU * *step as f64 / steps as f64;
            let point = cylinder.origin + cylinder.radial(angle) * cylinder.radius;
            other.distance(point).abs() < eps
        })
        .collect()
}

/// What the outer cylinder's circles and the inner one's take and leave out,
/// when the inner stands inside the outer within twice the sag of the outer's
/// chords from its wall: every ray the one takes the other takes too, and a
/// ray along which the two stand closer than `eps` neither takes.
fn common(
    body: &Body,
    outer_wall: &Wall,
    inner_wall: &Wall,
    tolerance: f64,
) -> Option<[Contact; 2]> {
    let (outer, inner) = (outer_wall.1, inner_wall.1);
    let eps = body.scale().eps();
    let axis = outer.axis;
    if inner.radius > outer.radius - eps {
        return None;
    }
    let flat = |vector: DVec3| vector - axis * axis.dot(vector);
    let offset = flat(inner.origin - outer.origin);
    let outer_steps = divisions(outer.radius, tolerance);
    let sag = outer.radius * (1.0 - (PI / outer_steps as f64).cos());
    let gap = outer.radius - inner.radius - offset.length();
    if gap > 2.0 * sag || gap < -eps {
        return None;
    }

    let inner_steps = divisions(inner.radius, tolerance);
    let mut through_outer: Vec<(DVec3, Option<usize>)> = (0..outer_steps)
        .map(|step| {
            let angle = TAU * step as f64 / outer_steps as f64;
            (outer.radial(angle) * outer.radius, Some(step))
        })
        .collect();
    let mut from_inner: Vec<(DVec3, Option<usize>)> = (0..inner_steps)
        .map(|step| {
            (
                inner.radial(TAU * step as f64 / inner_steps as f64),
                Some(step),
            )
        })
        .collect();
    for vertex in body.vertex_ids().map(|id| body.vertex(id)) {
        let (on_outer, on_inner) = (
            bears(body, vertex, outer_wall),
            bears(body, vertex, inner_wall),
        );
        if on_outer && !on_inner {
            through_outer.push((flat(vertex.point - outer.origin), None));
        } else if on_inner && !on_outer {
            from_inner.push((flat(vertex.point - inner.origin).normalize(), None));
        }
    }

    let mut on_outer = Contact::default();
    let mut on_inner = Contact::default();
    for (point, step) in through_outer {
        let from_axis = point - offset;
        if from_axis.length() - inner.radius >= eps {
            on_inner.rays.push(from_axis.normalize());
        } else if let Some(step) = step {
            on_outer.withheld.push(step);
        }
    }
    let outside = offset.length_squared() - outer.radius * outer.radius;
    for (way, step) in from_inner {
        let along = offset.dot(way);
        let reach = -along + (along * along - outside).sqrt();
        if reach - inner.radius >= eps {
            on_outer.rays.push((offset + way * reach).normalize());
        } else if let Some(step) = step {
            on_inner.withheld.push(step);
        }
    }
    Some([on_outer, on_inner])
}
