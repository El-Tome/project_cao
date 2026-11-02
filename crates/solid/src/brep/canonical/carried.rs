//! The second operand moved by a translation with what it built on a moving
//! surface, so that its own decisions hold exactly as it took them and
//! nothing of it is left a hair behind the surface it drew on: every
//! surface a corner or an edge of a moving surface lies on moves too — a
//! slot's sides with its cap, its other cap with its sides — but a surface
//! the move slides along itself, a cap square to the axis or a side along
//! the move, stays. The corners on the moving surfaces and the curves of
//! their edges move along. Decision 8 moves the second operand this way onto
//! a wall of its radius a hair off; decision 2 slides a wall its operand
//! drew corners on along a touch this way.
//!
//! A move that would carry a surface of the second operand further off one
//! of the first's it was one with, part it from one it touched, lay one of its
//! surfaces on another of its own it leaves behind, or move a curve the
//! kernel does not translate, is not made.

mod kept;

use std::collections::BTreeSet;

use glam::DVec3;

use crate::brep::conic::{Conic, Round};
use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder, Plane, Sphere, Surface, Torus};
use crate::brep::topology::{Body, EdgeId, SurfaceId};
use kept::kept;

/// Under this share of the tolerance, a surface moved along itself stays.
const ROUNDING: f64 = 1e-6;

/// The second operand with each surface `by` gives a move for translated by
/// it, with what it carries; none when nothing moves. `by` is asked of the
/// operand as moved so far, and a move whose surfaces another move carried
/// already is not made.
pub(in crate::brep) fn carried_along(
    first: &Body,
    second: &Body,
    scale: Scale,
    by: impl Fn(&Body, usize) -> Option<DVec3>,
) -> Option<Body> {
    let mut body: Option<Body> = None;
    let mut moved = BTreeSet::new();
    for rank in 0..second.surfaces.len() {
        let current = body.as_ref().unwrap_or(second);
        if moved.contains(&rank) {
            continue;
        }
        let Some(by) = by(current, rank) else {
            continue;
        };
        let carried = carried(current, rank, by, scale);
        if carried.iter().any(|surface| moved.contains(surface)) {
            continue;
        }
        let Some(shifted) = translated(current, &carried, by, scale) else {
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

/// The surfaces of the second operand that move with the surface of rank
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
/// cylinder's or an elliptic wall's axis; a cone, which a move along its
/// axis takes its apex with, a sphere or a torus, only under no move at
/// all.
fn slides(surface: &Surface, by: DVec3, scale: Scale) -> bool {
    let off = match surface {
        Surface::Plane(plane) => plane.normal.dot(by).abs(),
        Surface::Cylinder(cylinder) => (by - cylinder.axis * cylinder.axis.dot(by)).length(),
        Surface::Elliptic(wall) => (by - wall.axis * wall.axis.dot(by)).length(),
        Surface::Cone(_) | Surface::Sphere(_) | Surface::Torus(_) => by.length(),
    };
    off <= ROUNDING * scale.eps()
}

/// The body with the surfaces `carried`, the corners on them and the curves
/// of the edges beside them moved by `by`; none where a curve to move is one
/// two perpendicular cylinders meet along. A cone is built again through
/// its meridian line moved, canonical; a conic is cut again by its plane
/// moved on its wall moved, the arm on its side kept, so that it has the
/// bits the moved surfaces' own relation gives it.
fn translated(body: &Body, carried: &BTreeSet<usize>, by: DVec3, scale: Scale) -> Option<Body> {
    let mut moved = body.clone();
    for &rank in carried {
        moved.surfaces[rank] = shifted(&body.surfaces[rank], by);
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
            Curve::Conic(conic) => Curve::Conic(recut(&conic, by, scale)?),
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

/// A surface translated by `by`, canonical.
fn shifted(surface: &Surface, by: DVec3) -> Surface {
    match surface {
        Surface::Plane(plane) => Surface::Plane(shifted_plane(plane, by)),
        Surface::Cylinder(cylinder) => Surface::Cylinder(shifted_cylinder(cylinder, by)),
        Surface::Cone(cone) => Surface::Cone(shifted_cone(cone, by)),
        Surface::Elliptic(wall) => Surface::Elliptic(wall.moved(by)),
        Surface::Sphere(sphere) => Surface::Sphere(Sphere::through(
            sphere.origin + by,
            sphere.axis,
            sphere.center,
            sphere.radius,
        )),
        Surface::Torus(torus) => Surface::Torus(Torus::through(
            torus.origin + by,
            torus.axis,
            torus.center,
            torus.radius,
            torus.middle(),
        )),
    }
}

fn shifted_plane(plane: &Plane, by: DVec3) -> Plane {
    Plane::through(plane.origin + by, plane.normal).0
}

fn shifted_cylinder(cylinder: &Cylinder, by: DVec3) -> Cylinder {
    Cylinder::about(cylinder.origin + by, cylinder.axis, cylinder.radius)
}

fn shifted_cone(cone: &Cone, by: DVec3) -> Cone {
    Cone::through(
        cone.origin + by,
        cone.axis,
        [cone.foot, cone.foot + cone.ruling],
    )
}

/// The conic translated by `by`: cut again on its plane and its wall
/// translated, the component on the same side of the plane of symmetry;
/// none where the move parts the cut into arms or joins its arms.
fn recut(conic: &Conic, by: DVec3, scale: Scale) -> Option<Conic> {
    let round = match &conic.round {
        Round::Cylinder(cylinder) => Round::Cylinder(shifted_cylinder(cylinder, by)),
        Round::Cone(cone) => Round::Cone(shifted_cone(cone, by)),
        Round::Elliptic(wall) => Round::Elliptic(wall.moved(by)),
    };
    Conic::cut(&shifted_plane(&conic.plane, by), &round, scale)
        .into_iter()
        .find(|cut| cut.arm == conic.arm)
}
