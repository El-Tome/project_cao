//! Whether two faces whose surfaces stand at a slant — a plane neither along
//! nor square to a cylinder's axis, two cylinders at a skew angle — stand
//! clear of each other, decided on the faces themselves rather than on the
//! boxes round them: the kernel builds no curve such a pair shares, so the
//! pair declines the operation unless no place lies on both faces.
//!
//! A face reaches along any direction exactly as far as its boundary does: a
//! plane face is flat, and a point inside a wall lies on a ruling whose two
//! ends are on the boundary, between them along any direction. So the reach
//! of a face is read off its edges, exactly for lines and circles. A face
//! bounded by the curve two cylinders meet along is never decided clear.

use std::f64::consts::PI;

use glam::DVec3;

use super::operands::Operands;
use crate::brep::curve::{Circle, Curve};
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::{Body, FaceId, SurfaceId};

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
/// the walls span stand further apart than their radii.
///
/// A bore drilled through a hexagonal bar, past its far side, is told apart
/// from that side's slanted neighbours only across its own axis, in the plane
/// of the hexagon: square to the bore and to the neighbours' long edges.
fn apart(one: (&Body, FaceId), other: (&Body, FaceId), room: f64) -> bool {
    let [first, second] = [one, other].map(|(body, face)| *body.surface(body.face(face).surface));
    let mut directions = vec![offered(&first), offered(&second)];
    directions.push(directions[0].cross(directions[1]));
    for (wall, edged) in [(&first, other), (&second, one)] {
        if let Surface::Cylinder(cylinder) = wall {
            directions.extend(straight_edges(edged).map(|along| cylinder.axis.cross(along)));
        }
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
    separated
        || match (first, second) {
            (Surface::Cylinder(first), Surface::Cylinder(second)) => {
                walls_apart((&first, one), (&second, other), room)
            }
            _ => false,
        }
}

/// The direction a surface stands square to: a plane's normal, a
/// cylinder's axis.
fn offered(surface: &Surface) -> DVec3 {
    match surface {
        Surface::Plane(plane) => plane.normal,
        Surface::Cylinder(cylinder) => cylinder.axis,
    }
}

/// Whether two walls stand apart: the stretches of axis they span, each
/// grown by its radius, do not meet.
fn walls_apart(
    (first, one): (&Cylinder, (&Body, FaceId)),
    (second, other): (&Cylinder, (&Body, FaceId)),
    room: f64,
) -> bool {
    let stretch = |cylinder: &Cylinder, face| {
        reach(face, cylinder.axis)
            .map(|[low, high]| [low, high].map(|height| cylinder.origin + cylinder.axis * height))
    };
    match (stretch(first, one), stretch(second, other)) {
        (Some(one), Some(other)) => {
            between_stretches(one, other) > first.radius + second.radius + room
        }
        _ => false,
    }
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
/// edges; none where an edge runs along the curve two cylinders meet along.
fn reach((body, face): (&Body, FaceId), direction: DVec3) -> Option<[f64; 2]> {
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    for coedge in body.face(face).loops.iter().flatten() {
        let edge = body.edge(coedge.edge);
        let curve = body.curve(edge.curve);
        let mut points = vec![curve.point(edge.from), curve.point(edge.to)];
        match curve {
            Curve::Line(_) => {}
            Curve::Circle(circle) => points.extend(turning(circle, direction, edge.from, edge.to)),
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
