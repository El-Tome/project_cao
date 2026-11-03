//! How an elliptic wall's rims are sampled besides on its grid (#571): the
//! angles of the vertices on it or just inside it and of the planes
//! reaching it, where a whole rim starts, and how near an end a sample
//! grazes.

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use crate::brep::conic::{Conic, On};
use crate::brep::curve::Line;
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{EllipticCylinder, Surface};
use crate::brep::topology::{Body, Edge, SurfaceId};

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

/// The sine of the widest angle two walls crossing along a ruling meet at
/// for the lens between them to be thinner than a chord's sag a step from
/// the ruling.
const GRAZING: f64 = 1e-2;

/// Whether a place of an edge on an elliptic wall stands near an end where
/// another wall crosses its own at a grazing angle, and close to that wall:
/// the two bound a lens thinner than a chord's sag there, and a sample one
/// of them takes alone leaves its chord across the other's. Near and close
/// are read the same from both walls — a step of the coarser grid from the
/// end, and twice the deeper sag — so that the two drop the same places;
/// the rays both take, shared from a point inside both (`abreast.rs`), are
/// kept. Where the two touch, they take the rays they share.
pub(super) fn in_a_lens<'a>(
    body: &'a Body,
    edge: &'a Edge,
    wall: EllipticCylinder,
    tolerance: f64,
) -> impl Fn(DVec3) -> bool + use<'a> {
    let own = Surface::Elliptic(wall);
    let reach = |surface: &Surface| match surface {
        Surface::Elliptic(oval) => oval.major,
        Surface::Cylinder(round) => round.radius,
        Surface::Plane(_) | Surface::Cone(_) | Surface::Sphere(_) | Surface::Torus(_) => 0.0,
    };
    let ends: Vec<(DVec3, Vec<Lens>)> = edge
        .ends
        .into_iter()
        .flatten()
        .map(|end| {
            let vertex = body.vertex(end);
            let crossing = vertex
                .on
                .iter()
                .map(|id| *body.surface(*id))
                .filter(|other| {
                    *other != own && crosses_at(&own, other, vertex.point, body.scale())
                })
                .map(|other| {
                    let widest = reach(&own).max(reach(&other));
                    let steps = super::divisions(widest, tolerance) as f64;
                    let step = TAU * widest / steps;
                    let sag = widest * (1.0 - (PI / steps).cos());
                    Lens {
                        wall: other,
                        step,
                        room: 2.0 * sag,
                    }
                })
                .collect();
            (vertex.point, crossing)
        })
        .collect();
    move |point: DVec3| {
        ends.iter().any(|(end, walls)| {
            walls.iter().any(|lens| {
                (point - *end).length() <= lens.step && lens.wall.distance(point).abs() <= lens.room
            })
        })
    }
}

/// A wall crossing an elliptic one at an end of its edge, how far from the
/// end the lens they bound is read, and how close to that wall.
struct Lens {
    wall: Surface,
    step: f64,
    room: f64,
}

/// Whether two walls about parallel axes cross, rather than touch, along a
/// ruling through `at`, at a grazing angle: a sine of a hundredth or less,
/// as decision 2 has two such walls graze.
fn crosses_at(own: &Surface, other: &Surface, at: DVec3, scale: Scale) -> bool {
    if !matches!(other, Surface::Cylinder(_) | Surface::Elliptic(_)) {
        return false;
    }
    let facing = |wall: &Surface| wall.normal(wall.parameters(at));
    if facing(own).cross(facing(other)).length() > GRAZING {
        return false;
    }
    let through = |line: &Line| {
        let off = at - line.origin;
        (off - line.direction * line.direction.dot(off)).length() <= scale.eps()
    };
    match relation(own, other, scale) {
        Relation::Lines(lines) => lines.iter().any(through),
        Relation::Parallel(rulings) => rulings
            .iter()
            .any(|ruled| !ruled.touching && through(&ruled.line)),
        _ => false,
    }
}
