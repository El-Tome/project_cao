//! Whether two faces whose surfaces stand at a slant — two cylinders at a
//! skew angle, a cone and a wall not of its axis, a plane neither along nor
//! square to a wall's axis — stand clear of each other, decided on the
//! faces themselves rather than on the boxes round them. The kernel builds
//! no curve the first two share, so such a pair declines the operation
//! unless no place lies on both faces; the conic a plane cuts a wall along
//! is built only where their faces do not stand clear.
//!
//! A face reaches along any direction exactly as far as its boundary does: a
//! plane face is flat, and a point inside a wall lies on a ruling whose two
//! ends are on the boundary, between them along any direction. So the reach
//! of a face is read off its edges, exactly for lines, circles and conics —
//! a conic's turning points are closed forms; on a cone whose face holds its
//! apex within it, as a whole point's does, a ruling ends there, and the
//! apex is read too. How far a conic stands from an axis is read more
//! loosely: out to the farthest corner of its box, or on its own cone out
//! to its widest, and in to the axis itself, so that a face it bounds is
//! told clear less often than it might be, never wrongly (#552). A face
//! bounded by the curve two cylinders meet along is never decided clear.

mod hollow;
mod walls;

use std::f64::consts::PI;

use glam::DVec3;

use super::operands::Operands;
use crate::brep::conic::Conic;
use crate::brep::curve::{Circle, Curve};
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, FaceId, SurfaceId};
use hollow::{beyond, in_the_hollow};
use walls::{Wall, radii};

/// How many tolerances two faces must stand apart to be clear: each surface
/// may have been taken for one a tolerance away, and an edge stands within
/// the tolerance of the surfaces it lies on.
const CLEAR: f64 = 4.0;

/// Whether every face of one operand on one surface of the pair and every
/// face of the other on the other surface, their boxes meeting, stand clear.
pub(super) fn clear(operands: &Operands, [one, other]: [SurfaceId; 2]) -> bool {
    let room = CLEAR * operands.eps();
    [(one, other), (other, one)]
        .into_iter()
        .all(|(first, second)| {
            operands.lying[0][first.0 as usize].iter().all(|&face| {
                operands.lying[1][second.0 as usize].iter().all(|&against| {
                    !operands.near(face, against)
                        || apart(
                            (operands.bodies[0], face),
                            (operands.bodies[1], against),
                            room,
                        )
                })
            })
        })
}

/// Whether two faces stand more than `room` apart along a direction their
/// surfaces offer — a plane's normal, a cylinder's axis, the direction square
/// to both, the directions square to a wall's axis and to a straight edge of
/// the other face — or, for two walls, whether the stretches of their axes
/// the walls span stand further apart than the walls stand from them; and,
/// a cone among the two, whether the other face stands inside the hollow
/// the cone's face leaves about its axis, or beyond a plane touching the cone
/// along a ruling.
///
/// A bore drilled through a hexagonal bar, past its far side, is told apart
/// from that side's slanted neighbours only across its own axis, in the plane
/// of the hexagon: square to the bore and to the neighbours' long edges.
fn apart(one: (&Body, FaceId), other: (&Body, FaceId), room: f64) -> bool {
    let [first, second] = [one, other].map(|(body, face)| *body.surface(body.face(face).surface));
    let mut directions = vec![offered(&first), offered(&second)];
    directions.push(directions[0].cross(directions[1]));
    for (wall, edged) in [(&first, other), (&second, one)] {
        let axis = match wall {
            Surface::Cylinder(cylinder) => cylinder.axis,
            Surface::Cone(cone) => cone.axis,
            Surface::Elliptic(wall) => wall.axis,
            Surface::Sphere(sphere) => sphere.axis,
            Surface::Torus(torus) => torus.axis,
            Surface::Plane(_) => continue,
        };
        directions.extend(straight_edges(edged).map(|along| axis.cross(along)));
    }
    let separated =
        directions
            .into_iter()
            .filter_map(DVec3::try_normalize)
            .any(
                |direction| match (reach(one, direction), reach(other, direction)) {
                    (Some([low, high]), Some([from, to])) => from - high > room || low - to > room,
                    _ => false,
                },
            );
    let conical = [first, second]
        .iter()
        .any(|surface| matches!(surface, Surface::Cone(_)));
    separated
        || match (Wall::of(&first, one), Wall::of(&second, other)) {
            (Some(first), Some(second)) => {
                first.beside(&second, room)
                    || conical && (first.within(&second, room) || second.within(&first, room))
            }
            _ => false,
        }
        || [(&first, one, other), (&second, other, one)]
            .into_iter()
            .any(|(surface, nappe, face)| match surface {
                Surface::Cone(cone) => {
                    in_the_hollow(cone, nappe, face, room) || beyond(cone, face, room)
                }
                Surface::Plane(_)
                | Surface::Cylinder(_)
                | Surface::Elliptic(_)
                | Surface::Sphere(_)
                | Surface::Torus(_) => false,
            })
}

/// How far from the line through `origin` along `axis` a face stands at
/// most, or more: read off its boundary, since the distance from a line
/// grows along any straight line, a circle taken as far as its centre
/// stands plus its radius, a conic as its box's farthest corner; none where
/// an edge runs along the curve two cylinders meet along.
fn farthest((body, face): (&Body, FaceId), origin: DVec3, axis: DVec3) -> Option<f64> {
    let off = |point: DVec3| from_the_axis(point, origin, axis);
    let mut farthest = body
        .poles_held(face)
        .into_iter()
        .map(off)
        .fold(0.0, f64::max);
    for coedge in body.face(face).loops.iter().flatten() {
        let edge = body.edge(coedge.edge);
        let reached = match body.curve(edge.curve) {
            Curve::Line(line) => off(line.point(edge.from)).max(off(line.point(edge.to))),
            Curve::Circle(circle) => off(circle.center) + circle.radius,
            Curve::Conic(conic) => boxed(conic, edge.from, edge.to, off),
            Curve::Meet(_) => return None,
        };
        farthest = farthest.max(reached);
    }
    Some(farthest)
}

/// The direction a surface stands square to: a plane's normal, a wall's
/// axis.
fn offered(surface: &Surface) -> DVec3 {
    match surface {
        Surface::Plane(plane) => plane.normal,
        Surface::Cylinder(cylinder) => cylinder.axis,
        Surface::Cone(cone) => cone.axis,
        Surface::Elliptic(wall) => wall.axis,
        Surface::Sphere(sphere) => sphere.axis,
        Surface::Torus(torus) => torus.axis,
    }
}

/// The largest `off` reads at a corner of the box round a stretch of a
/// conic: no less than anywhere along the stretch, `off` a distance from a
/// line, which grows along any straight line.
fn boxed(conic: &Conic, from: f64, to: f64, off: impl Fn(DVec3) -> f64) -> f64 {
    let [low, high] = conic.bounds(from, to);
    (0..8)
        .map(|corner| {
            off(DVec3::select(
                glam::BVec3::new(corner & 1 != 0, corner & 2 != 0, corner & 4 != 0),
                high,
                low,
            ))
        })
        .fold(0.0, f64::max)
}

/// How far a point stands from the line through `origin` along `axis`.
fn from_the_axis(point: DVec3, origin: DVec3, axis: DVec3) -> f64 {
    let from = point - origin;
    (from - axis * axis.dot(from)).length()
}

/// Which way each straight edge of a face runs.
fn straight_edges((body, face): (&Body, FaceId)) -> impl Iterator<Item = DVec3> + '_ {
    body.face(face).loops.iter().flatten().filter_map(|coedge| {
        let edge = body.edge(coedge.edge);
        match body.curve(edge.curve) {
            Curve::Line(line) => Some(line.direction),
            _ => None,
        }
    })
}

/// The least and the largest of `direction · p` over a face, read off its
/// edges, where a circle or a conic turns back along `direction`, and the
/// apex it holds within it; none where an edge runs along the curve two
/// cylinders meet along.
fn reach((body, face): (&Body, FaceId), direction: DVec3) -> Option<[f64; 2]> {
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    for pole in body.poles_held(face) {
        low = low.min(direction.dot(pole));
        high = high.max(direction.dot(pole));
    }
    for coedge in body.face(face).loops.iter().flatten() {
        let edge = body.edge(coedge.edge);
        let curve = body.curve(edge.curve);
        let mut points = vec![curve.point(edge.from), curve.point(edge.to)];
        match curve {
            Curve::Line(_) => {}
            Curve::Circle(circle) => points.extend(turning(circle, direction, edge.from, edge.to)),
            Curve::Conic(conic) => points.extend(
                conic
                    .turning(direction, edge.from, edge.to)
                    .into_iter()
                    .map(|t| conic.point(t)),
            ),
            Curve::Meet(_) => return None,
        }
        for point in points {
            let along = direction.dot(point);
            low = low.min(along);
            high = high.max(along);
        }
    }
    low.is_finite().then_some([low, high])
}

/// The points of a circle, between two of its parameters, where it goes
/// furthest along `direction` or against it.
fn turning(circle: &Circle, direction: DVec3, from: f64, to: f64) -> Vec<DVec3> {
    let phase = direction.dot(circle.v).atan2(direction.dot(circle.u));
    let first = ((from - phase) / PI).ceil() as i64;
    let last = ((to - phase) / PI).floor() as i64;
    (first..=last)
        .map(|turn| circle.point(phase + turn as f64 * PI))
        .collect()
}

/// The distance between two stretches of line, each given by its two ends.
fn between_stretches([a, b]: [DVec3; 2], [c, d]: [DVec3; 2]) -> f64 {
    let (one, other, across) = (b - a, d - c, a - c);
    let (oo, tt) = (one.length_squared(), other.length_squared());
    let (ot, oa, ta) = (one.dot(other), one.dot(across), other.dot(across));
    let square = (oo * tt - ot * ot).max(0.0);
    let mut s = if square > 0.0 {
        ((ot * ta - tt * oa) / square).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let mut t = if tt > 0.0 { (ot * s + ta) / tt } else { 0.0 };
    if t < 0.0 {
        t = 0.0;
        s = if oo > 0.0 {
            (-oa / oo).clamp(0.0, 1.0)
        } else {
            0.0
        };
    } else if t > 1.0 {
        t = 1.0;
        s = if oo > 0.0 {
            ((ot - oa) / oo).clamp(0.0, 1.0)
        } else {
            0.0
        };
    }
    (a + one * s).distance(c + other * t)
}

#[cfg(test)]
mod tests;
