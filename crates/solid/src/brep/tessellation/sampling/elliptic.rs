//! How an elliptic wall's rims are sampled besides on its grid (#571): the
//! angles of the vertices on it or just inside it and of the planes
//! reaching it, where a whole rim starts, and how near an end a sample
//! grazes.

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::abreast::Abreast;
use super::ends::{grazed_at, grazed_near};
use crate::brep::conic::{Conic, On, Round};
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::surface::{EllipticCylinder, Plane, Surface};
use crate::brep::tessellation::contact;
use crate::brep::topology::{Body, Edge, SurfaceId, VertexId};

/// The angles about an elliptic wall of every vertex lying on it, where a
/// ruling leaves that every rim of the wall takes a sample on, and of every
/// vertex standing inside it closer than twice the sag of a chord across a
/// step of its grid of `steps` steps a turn: a chord across that angle
/// would pass inside the vertex, as a circular wall's would
/// (`contact/own.rs`).
pub(super) fn on_the_wall(body: &Body, wall: &EllipticCylinder, steps: usize) -> Vec<f64> {
    let id = body
        .surfaces
        .iter()
        .position(|surface| *surface == Surface::Elliptic(*wall))
        .map(|rank| SurfaceId(rank as u32));
    let sag = wall.major * (1.0 - (PI / steps as f64).cos());
    body.vertices
        .iter()
        .filter(|vertex| {
            id.is_some_and(|id| vertex.on.contains(&id))
                || (-2.0 * sag..0.0).contains(&wall.distance(vertex.point))
        })
        .map(|vertex| wall.parameters(vertex.point).x)
        .collect()
}

/// The angles about an elliptic wall of its support along the normal of
/// each plane of the body running along its axis and reaching it: where
/// such a plane touches the wall, or where the strip it leaves between two
/// rulings bulges furthest past it. A strip thinner than a step of the grid
/// has no sample of its own otherwise, and its triangles lie flat on the
/// plane's.
pub(super) fn reached_by_planes(body: &Body, wall: &EllipticCylinder) -> Vec<f64> {
    let eps = body.scale().eps();
    body.surfaces
        .iter()
        .filter_map(|surface| match surface {
            Surface::Plane(plane) => Some(plane),
            Surface::Cylinder(_)
            | Surface::Cone(_)
            | Surface::Elliptic(_)
            | Surface::Sphere(_)
            | Surface::Torus(_) => None,
        })
        .filter(|plane| {
            plane.normal.dot(wall.axis).abs() <= Scale::RELATIVE
                && plane.distance(wall.origin).abs() - wall.support(plane.normal) <= eps
        })
        .map(|plane| {
            let toward = plane.normal * -plane.distance(wall.origin).signum();
            (wall.minor * toward.dot(wall.v)).atan2(wall.major * toward.dot(wall.u))
        })
        .collect()
}

/// A whole ring of an elliptic wall read from half way between two steps of
/// the wall's grid of `steps` steps a turn, as long as it ran; any other
/// stretch as it is. The ring starts at the end of its major axis, a step of
/// the grid, and that step's angle, read back off the ring's anchor, may
/// stand a rounding past both ends of the turn and be sampled at neither.
pub(super) fn between_steps(conic: &Conic, edge: &Edge, steps: usize) -> Edge {
    if edge.ends.is_some() {
        return edge.clone();
    }
    let step = TAU / steps as f64;
    let start = conic.seen_on(On::Wall, edge.from)[0].x;
    let middle = (start / step).floor() * step + step / 2.0;
    let shift = (middle - start) / conic.k.sqrt();
    Edge {
        from: edge.from + shift,
        to: edge.to + shift,
        ..edge.clone()
    }
}

/// Whether two places stand a step or less apart round an elliptic wall, on
/// its grid of `steps` steps a turn: the window in which a sample grazing a
/// surface an end lies on is dropped, as a circle's is a step or less from
/// the end in a straight line. A straight line across a flat ellipse finds
/// its far side within a step, half a turn away.
pub(super) fn a_step_apart(wall: EllipticCylinder, steps: usize) -> impl Fn(DVec3, DVec3) -> bool {
    let step = TAU / steps as f64;
    move |one: DVec3, other: DVec3| {
        let turned = wall.parameters(one).x - wall.parameters(other).x;
        ((turned + PI).rem_euclid(TAU) - PI).abs() <= step
    }
}

/// Whether a place of an edge along the conic `plane` cuts `wall` along is
/// kept: it does not stand within the room of a surface an end of the edge
/// lies on, a step or less from that end on the wall's grid of `steps`
/// steps a turn, the curve's own surfaces aside; nor, where a rim of an
/// elliptic wall sampled with `wall` from a hub ends at that end on the
/// same plane, does the place that wall takes on its ray, those walls aside
/// too. Ending at one vertex, the two rims of a lens a hair thick both read
/// their places on a ray there: one leaving out its own as a hair from a
/// plane their end lies on, and the other keeping its own, left the other's
/// chord across it (campaign seed 5585709823). Where no rim of the other
/// wall ends there, its place on the ray may be a vertex another curve ends
/// at, which no rim leaves out, and so must this one keep its own.
pub(super) fn kept<'a>(
    body: &'a Body,
    edge: &'a Edge,
    plane: Plane,
    wall: EllipticCylinder,
    steps: usize,
    id: Option<SurfaceId>,
    abreast: &'a Abreast,
) -> impl Fn(DVec3) -> bool + 'a {
    let room = body.scale().eps() * contact::APART;
    let own = [Surface::Plane(plane), Surface::Elliptic(wall)];
    let grazing = grazed_near(body, edge, &own, a_step_apart(wall, steps));
    let beside: Vec<SurfaceId> = id
        .into_iter()
        .flat_map(|id| abreast.ovals_beside(id))
        .collect();
    let mut walls = own.to_vec();
    walls.extend(beside.iter().map(|other| *body.surface(*other)));
    let ending = |end: VertexId, other: SurfaceId| {
        body.edge_ids().map(|rim| body.edge(rim)).any(|rim| {
            rim.ends.is_some_and(|ends| ends.contains(&end))
                && match body.curve(rim.curve) {
                    Curve::Conic(conic) => {
                        let Round::Elliptic(oval) = conic.round else {
                            return false;
                        };
                        conic.plane == plane && Surface::Elliptic(oval) == *body.surface(other)
                    }
                    Curve::Line(_) | Curve::Circle(_) | Curve::Meet(_) => false,
                }
        })
    };
    let grazed_beside: Vec<_> = beside
        .iter()
        .map(|&other| {
            let shared: Vec<VertexId> = edge
                .ends
                .into_iter()
                .flatten()
                .filter(|end| ending(*end, other))
                .collect();
            (
                other,
                grazed_at(body, &shared, &walls, a_step_apart(wall, steps)),
            )
        })
        .collect();
    move |point: DVec3| {
        !grazing(point, room)
            && id.is_none_or(|id| {
                abreast
                    .beside(body, id, point)
                    .into_iter()
                    .all(|(other, at)| {
                        grazed_beside
                            .iter()
                            .filter(|(wall, _)| *wall == other)
                            .all(|(_, grazed)| !grazed(at, room))
                    })
            })
    }
}
