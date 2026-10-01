//! Decision 8: two parallel cylinders of one radius, one of each operand,
//! whose axes stand within `HAIR` tolerances of each other are one surface.
//! Two such walls cross along two rulings at an angle the offset over the
//! radius, a crossing so ill conditioned that a band a thousand tolerances
//! wide surrounds it; the crescent between them is at most `HAIR`
//! tolerances thick, a tenth of what a line of measure resolves.
//!
//! The second operand's wall is taken for the first's as decision 1 takes
//! one within the tolerance, and what the second operand built on it goes
//! along: the second operand is moved square to the axis, by the offset,
//! over every surface, curve and corner that would otherwise be left a
//! hair off it — a slot's sides touching the cap moved, its other cap
//! after them — so that its own decisions hold exactly as it took them. A
//! surface the offset slides along itself, a cap square to the axis, stays.
//! A move that would part a surface of the second operand from one of the
//! first's it was one with or touched, or move a curve the kernel does not
//! translate, is not made, and the two walls are left two.

use std::collections::BTreeSet;

use glam::DVec3;

use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::topology::{Body, EdgeId, SurfaceId};

/// How far apart, in tolerances, the axes of two cylinders of one radius
/// may stand and still be one surface.
pub(in crate::brep) const HAIR: f64 = 100.0;

/// Under this share of the tolerance, a surface moved along itself stays.
const ROUNDING: f64 = 1e-6;

/// The second operand with each of its cylinders standing within `HAIR`
/// tolerances of one of the first's, of one radius, moved onto it; none
/// when nothing moves.
pub(in crate::brep) fn closed(first: &Body, second: &Body, scale: Scale) -> Option<Body> {
    let mut body: Option<Body> = None;
    let mut moved = BTreeSet::new();
    for rank in 0..second.surfaces.len() {
        let current = body.as_ref().unwrap_or(second);
        let Surface::Cylinder(cylinder) = current.surfaces[rank] else {
            continue;
        };
        if moved.contains(&rank) {
            continue;
        }
        let Some(by) = offset(first, &cylinder, scale) else {
            continue;
        };
        let carried = carried(current, rank, by, scale);
        if carried.iter().any(|surface| moved.contains(surface)) {
            continue;
        }
        let Some(shifted) = translated(current, &carried, by) else {
            continue;
        };
        if !kept(first, current, &shifted, &carried, rank, scale) {
            continue;
        }
        moved.extend(carried);
        body = Some(shifted);
    }
    body
}

/// The move square to the axes that lays `cylinder` on the nearest of the
/// first operand's cylinders of its radius whose axis stands within `HAIR`
/// tolerances of its own; none where decision 1 takes it for one already.
fn offset(first: &Body, cylinder: &Cylinder, scale: Scale) -> Option<DVec3> {
    let eps = scale.eps();
    let mut nearest: Option<(f64, DVec3)> = None;
    for known in &first.surfaces {
        if matches!(
            relation(known, &Surface::Cylinder(*cylinder), scale),
            Relation::Same { .. }
        ) {
            return None;
        }
        let Surface::Cylinder(known) = known else {
            continue;
        };
        if known.axis.cross(cylinder.axis).length() * 2.0 * scale.reach() > eps
            || (known.radius - cylinder.radius).abs() > eps
        {
            continue;
        }
        let between = known.origin - cylinder.origin;
        let across = between - known.axis * known.axis.dot(between);
        let distance = across.length();
        if distance <= HAIR * eps && nearest.is_none_or(|(least, _)| distance < least) {
            nearest = Some((distance, across));
        }
    }
    nearest.map(|(_, across)| across)
}

/// The surfaces of the second operand that move with the cylinder of rank
/// `start`: every surface a corner or a curve of a moving one lies on, but
/// those the move slides along themselves.
fn carried(body: &Body, start: usize, by: DVec3, scale: Scale) -> BTreeSet<usize> {
    let mut carried = BTreeSet::from([start]);
    let mut waiting = vec![start];
    while let Some(surface) = waiting.pop() {
        let mut touching = BTreeSet::new();
        for vertex in &body.vertices {
            if vertex.on.contains(&SurfaceId(surface as u32)) {
                touching.extend(vertex.on.iter().map(|on| on.0 as usize));
            }
        }
        for face in body.face_ids() {
            if body.face(face).surface.0 as usize != surface {
                continue;
            }
            for coedge in body.face(face).loops.iter().flatten() {
                touching.extend(beside(body, coedge.edge));
            }
        }
        for other in touching {
            if !carried.contains(&other) && !slides(&body.surfaces[other], by, scale) {
                carried.insert(other);
                waiting.push(other);
            }
        }
    }
    carried
}

/// The surfaces of the faces along every edge on the curve of `edge`.
fn beside(body: &Body, edge: EdgeId) -> Vec<usize> {
    let curve = body.edge(edge).curve;
    body.edge_ids()
        .filter(|other| body.edge(*other).curve == curve)
        .flat_map(|other| body.uses(other))
        .map(|(face, _)| body.face(face).surface.0 as usize)
        .collect()
}

/// Whether a move leaves a surface where it is: along a plane, or along a
/// cylinder's axis.
fn slides(surface: &Surface, by: DVec3, scale: Scale) -> bool {
    let off = match surface {
        Surface::Plane(plane) => plane.normal.dot(by).abs(),
        Surface::Cylinder(cylinder) => (by - cylinder.axis * cylinder.axis.dot(by)).length(),
    };
    off <= ROUNDING * scale.eps()
}

/// The body with the surfaces `carried`, the corners on them and the curves
/// of the edges beside them moved by `by`; none where a curve to move is one
/// two perpendicular cylinders meet along.
fn translated(body: &Body, carried: &BTreeSet<usize>, by: DVec3) -> Option<Body> {
    let mut moved = body.clone();
    for &rank in carried {
        moved.surfaces[rank] = match body.surfaces[rank] {
            Surface::Plane(plane) => {
                Surface::Plane(Plane::through(plane.origin + by, plane.normal).0)
            }
            Surface::Cylinder(cylinder) => Surface::Cylinder(Cylinder::about(
                cylinder.origin + by,
                cylinder.axis,
                cylinder.radius,
            )),
        };
    }
    let mut corners = BTreeSet::new();
    for (rank, vertex) in moved.vertices.iter_mut().enumerate() {
        if vertex
            .on
            .iter()
            .any(|on| carried.contains(&(on.0 as usize)))
        {
            vertex.point += by;
            corners.insert(rank);
        }
    }
    let mut curves = BTreeSet::new();
    for edge in body.edge_ids() {
        if body
            .uses(edge)
            .iter()
            .any(|(face, _)| carried.contains(&(body.face(*face).surface.0 as usize)))
        {
            curves.insert(body.edge(edge).curve.0 as usize);
        }
    }
    for &rank in &curves {
        moved.curves[rank] = match body.curves[rank] {
            Curve::Line(line) => Curve::Line(Line::through(line.origin + by, line.direction)),
            Curve::Circle(circle) => Curve::Circle(Circle {
                center: circle.center + by,
                ..circle
            }),
            Curve::Meet(_) => return None,
        };
    }
    for edge in &mut moved.edges {
        let Some([from, to]) = edge.ends else {
            continue;
        };
        if !curves.contains(&(edge.curve.0 as usize))
            && !corners.contains(&(from.0 as usize))
            && !corners.contains(&(to.0 as usize))
        {
            continue;
        }
        let curve = &moved.curves[edge.curve.0 as usize];
        let [start, end] = [from, to].map(|vertex| moved.vertices[vertex.0 as usize].point);
        let near = |at: f64, point: DVec3| match curve.period() {
            Some(period) => {
                let turned = curve.parameter(point);
                turned + period * ((at - turned) / period).round()
            }
            None => curve.parameter(point),
        };
        let (start, end) = (near(edge.from, start), near(edge.to, end));
        edge.from = start;
        edge.to = end;
    }
    Some(moved)
}

/// Whether every surface the move carries along with the cylinder of rank
/// `taken` stands with each of the first operand's as it stood before: one
/// with it, or touching it, still. The cylinder itself becomes the first's,
/// and stands with every other as the first operand decided.
fn kept(
    first: &Body,
    before: &Body,
    after: &Body,
    carried: &BTreeSet<usize>,
    taken: usize,
    scale: Scale,
) -> bool {
    carried.iter().filter(|&&rank| rank != taken).all(|&rank| {
        first.surfaces.iter().all(|known| {
            let held = |surface: &Surface| {
                matches!(
                    relation(known, surface, scale),
                    Relation::Same { .. } | Relation::Tangent(_)
                )
            };
            !held(&before.surfaces[rank]) || held(&after.surfaces[rank])
        })
    })
}

#[cfg(test)]
mod tests;
