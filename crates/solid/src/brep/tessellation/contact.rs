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
//! stands closer to the other wall than the rules tell places apart, and its
//! chords are as good as lying on the other wall's. Such a step of the grid is
//! withheld from both, and both take the line itself, so that the first chords
//! from the line reach out to where the walls stand apart. So too beside the
//! lines two walls cross along at a slant a hair from nought.
//!
//! Two perpendicular cylinders meet along a curve, and touch, if they do, at
//! a node of it: the circles of both take the rays through that curve's
//! samples, so that both walls are cut in strips between them.

mod together;

use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::DVec3;

use super::sampling::divisions;
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::{Body, SurfaceId, Vertex};

/// How close to another wall, as a share of the kernel's tolerance, no sample
/// of a wall stands but on the lines they meet along: twice what the rules
/// tell apart, a tenth of it. Not the whole of it: two walls barely more than
/// the kernel's tolerance apart — cylinders of one radius a hair off one axis
/// — stand closer than that round most of the turn, and withheld there, their
/// chords would cut across whole quarters.
pub(super) const APART: f64 = 0.2;

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

/// For each wall, what its circles take besides their grid and what of it
/// they leave out: the rays through its vertices and through the curves it
/// meets a perpendicular wall along, and for each wall parallel to it and
/// close, the rays through the lines they meet along and those the two share.
///
/// `meets` holds every edge's samples between its ends, empty but for the
/// curves two cylinders meet along.
pub(super) fn contacts(
    body: &Body,
    walls: &[Wall],
    meets: &[Vec<DVec3>],
    tolerance: f64,
) -> BTreeMap<SurfaceId, Contact> {
    let eps = body.scale().eps();
    let mut own: BTreeMap<SurfaceId, Vec<DVec3>> = BTreeMap::new();
    along_meets(body, meets, &mut own);
    through_vertices(body, walls, &mut own);
    let mut dropped: BTreeMap<SurfaceId, Vec<usize>> = BTreeMap::new();
    let mut contacts: BTreeMap<SurfaceId, Contact> = BTreeMap::new();
    for (at, one) in walls.iter().enumerate() {
        for other in &walls[at + 1..] {
            if one.1.axis.cross(other.1.axis).length() > Scale::RELATIVE {
                continue;
            }
            for [to_one, to_other] in meeting_lines(&one.1, &other.1, eps) {
                contacts.entry(one.0).or_default().rays.push(to_one);
                contacts.entry(other.0).or_default().rays.push(to_other);
            }
            let (outer, inner) = if one.1.radius >= other.1.radius {
                (one, other)
            } else {
                (other, one)
            };
            let taken = |wall: &Wall| own.get(&wall.0).map_or(&[][..], Vec::as_slice);
            let shared =
                together::sampled((outer, taken(outer)), (inner, taken(inner)), tolerance, eps);
            if let Some(shared) = shared {
                let [on_outer, on_inner] = shared.contacts;
                let [off_outer, off_inner] = shared.dropped;
                contacts.entry(outer.0).or_default().join(on_outer);
                contacts.entry(inner.0).or_default().join(on_inner);
                dropped.entry(outer.0).or_default().extend(off_outer);
                dropped.entry(inner.0).or_default().extend(off_inner);
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
    for (wall, rays) in own {
        let off = dropped.get(&wall).map_or(&[][..], Vec::as_slice);
        let kept = rays
            .into_iter()
            .enumerate()
            .filter(|(rank, _)| !off.contains(rank))
            .map(|(_, way)| way);
        contacts.entry(wall).or_default().rays.extend(kept);
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
fn along_meets(body: &Body, meets: &[Vec<DVec3>], own: &mut BTreeMap<SurfaceId, Vec<DVec3>>) {
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
                    own.entry(surface).or_default().extend(rays.iter().copied());
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
fn through_vertices(body: &Body, walls: &[Wall], own: &mut BTreeMap<SurfaceId, Vec<DVec3>>) {
    for wall in walls {
        let (id, cylinder) = wall;
        for vertex in body.vertex_ids().map(|id| body.vertex(id)) {
            if !bears(body, vertex, wall) {
                continue;
            }
            let from = vertex.point - cylinder.origin;
            let flat = from - cylinder.axis * cylinder.axis.dot(from);
            if flat.length() > 0.0 {
                own.entry(*id).or_default().push(flat.normalize());
            }
        }
    }
}

/// The directions from the axes of two parallel cylinders to each line their
/// walls meet along: the one they touch along, side by side or one inside
/// the other, or the two they cross along.
///
/// Every circle of either is sampled on those lines, vertex or not: the steps
/// of the grid beside them are withheld, and a ring passing one with no
/// vertex there — its wall meeting the other only further along, or not at
/// all where the other's wall is gone — would otherwise cut across it by a
/// chord several steps long.
fn meeting_lines(one: &Cylinder, other: &Cylinder, eps: f64) -> Vec<[DVec3; 2]> {
    let between = other.origin - one.origin;
    let flat = between - one.axis * one.axis.dot(between);
    let apart = flat.length();
    if apart == 0.0 {
        return Vec::new();
    }
    let way = flat / apart;
    let (near, far) = ((one.radius - other.radius).abs(), one.radius + other.radius);
    if (apart - far).abs() <= eps {
        return vec![[way, -way]];
    }
    if (apart - near).abs() <= eps {
        let out = if one.radius >= other.radius {
            way
        } else {
            -way
        };
        return vec![[out, out]];
    }
    if apart < near || apart > far {
        return Vec::new();
    }
    let along =
        apart / 2.0 + (one.radius - other.radius) * (one.radius + other.radius) / (2.0 * apart);
    let aside = (one.radius * one.radius - along * along).max(0.0).sqrt();
    let square = one.axis.cross(way);
    [-1.0, 1.0]
        .into_iter()
        .map(|side| {
            let line = way * along + square * side * aside;
            [line.normalize(), (line - flat).normalize()]
        })
        .collect()
}

/// The steps of a cylinder's grid standing within a fifth of `eps` of another
/// cylinder parallel to it: none unless their circles come that close
/// somewhere.
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
            other.distance(point).abs() < eps * APART
        })
        .collect()
}
