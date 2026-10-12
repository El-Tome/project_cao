//! Whether two faces whose surfaces stand at a slant — a plane neither along
//! nor square to a cylinder's axis, two cylinders at a skew angle, a cone and
//! anything not of its axis — stand clear of each other, decided on the
//! faces themselves rather than on the boxes round them: the kernel builds
//! no curve such a pair shares, so the pair declines the operation unless
//! no place lies on both faces.
//!
//! A face reaches along any direction exactly as far as its boundary does: a
//! plane face is flat, and a point inside a wall lies on a ruling whose two
//! ends are on the boundary, between them along any direction. So the reach
//! of a face is read off its edges, exactly for lines and circles; on a cone
//! whose face holds its apex within it, as a whole point's does, a ruling
//! ends there, and the apex is read too. A face bounded by the curve two
//! cylinders meet along is never decided clear.

use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use super::operands::Operands;
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Surface};
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
                Surface::Plane(_) | Surface::Cylinder(_) => false,
            })
}

/// Whether a face stands nearer a cone's axis everywhere than the face on
/// the cone does anywhere: a pocket within a chamfered end.
fn in_the_hollow(cone: &Cone, nappe: (&Body, FaceId), face: (&Body, FaceId), room: f64) -> bool {
    match (radii(cone, nappe), farthest(face, cone.origin, cone.axis)) {
        (Some([inner, _]), Some(farthest)) => farthest + room < inner,
        _ => false,
    }
}

/// Whether a face stands beyond a plane touching the cone along one of its
/// rulings, on the side away from the solid the nappe bounds: a block beside
/// a point's slope, which no direction the two surfaces offer tells apart
/// from the ring the slope makes. The planes tried touch the cone where the
/// face's corners stand about the axis, and where their mean does.
fn beyond(cone: &Cone, face: (&Body, FaceId), room: f64) -> bool {
    let corners = corners(face);
    let mean = corners.iter().sum::<DVec3>() / corners.len().max(1) as f64;
    corners.iter().chain([&mean]).any(|&corner| {
        let theta = cone.parameters(corner).x;
        let outward = cone.normal(theta) * cone.ruling.x.signum();
        let touching = outward.dot(cone.point(DVec2::new(theta, 0.0)));
        reach(face, outward).is_some_and(|[low, _]| low > touching + room)
    })
}

/// The ends of a face's edges, and the centres of its circles.
fn corners((body, face): (&Body, FaceId)) -> Vec<DVec3> {
    body.face(face)
        .loops
        .iter()
        .flatten()
        .flat_map(|coedge| {
            let edge = body.edge(coedge.edge);
            let curve = body.curve(edge.curve);
            let centre = match curve {
                Curve::Circle(circle) => Some(circle.center),
                Curve::Line(_) | Curve::Meet(_) => None,
            };
            [curve.point(edge.from), curve.point(edge.to)]
                .into_iter()
                .chain(centre)
        })
        .collect()
}

/// How far from the line through `origin` along `axis` a face stands at
/// most, or more: read off its boundary, since the distance from a line
/// grows along any straight line, a circle taken as far as its centre
/// stands plus its radius; none where an edge runs along the curve two
/// cylinders meet along.
fn farthest((body, face): (&Body, FaceId), origin: DVec3, axis: DVec3) -> Option<f64> {
    let off = |point: DVec3| from_the_axis(point, origin, axis);
    let mut farthest = body.apex_held(face).map_or(0.0, off);
    for coedge in body.face(face).loops.iter().flatten() {
        let edge = body.edge(coedge.edge);
        let reached = match body.curve(edge.curve) {
            Curve::Line(line) => off(line.point(edge.from)).max(off(line.point(edge.to))),
            Curve::Circle(circle) => off(circle.center) + circle.radius,
            Curve::Meet(_) => return None,
        };
        farthest = farthest.max(reached);
    }
    Some(farthest)
}

/// The direction a surface stands square to: a plane's normal, a
/// cylinder's or a cone's axis.
fn offered(surface: &Surface) -> DVec3 {
    match surface {
        Surface::Plane(plane) => plane.normal,
        Surface::Cylinder(cylinder) => cylinder.axis,
        Surface::Cone(cone) => cone.axis,
    }
}

/// A face on a cylinder or a cone, seen about its axis: the stretch of the
/// axis it spans, and how near and how far from the axis it stands.
struct Wall {
    ends: [DVec3; 2],
    axis: DVec3,
    inner: f64,
    outer: f64,
}

impl Wall {
    fn of(surface: &Surface, face: (&Body, FaceId)) -> Option<Wall> {
        let (origin, axis, [inner, outer]) = match surface {
            Surface::Cylinder(cylinder) => (cylinder.origin, cylinder.axis, [cylinder.radius; 2]),
            Surface::Cone(cone) => (cone.origin, cone.axis, radii(cone, face)?),
            Surface::Plane(_) => return None,
        };
        let [low, high] = reach(face, axis)?;
        Some(Wall {
            ends: [low, high].map(|height| origin + axis * height),
            axis,
            inner,
            outer,
        })
    }

    /// Whether two walls stand apart side by side: the stretches of axis
    /// they span, each grown by how far its wall stands from it, do not
    /// meet.
    fn beside(&self, other: &Wall, room: f64) -> bool {
        between_stretches(self.ends, other.ends) > self.outer + other.outer + room
    }

    /// Whether this wall stands inside the hollow the other leaves about
    /// its axis: a bore within a countersink's narrow end.
    fn within(&self, other: &Wall, room: f64) -> bool {
        let off = |point: DVec3| from_the_axis(point, other.ends[0], other.axis);
        off(self.ends[0]).max(off(self.ends[1])) + self.outer < other.inner - room
    }
}

/// How near and how far from its axis a face on a cone stands: as its
/// boundary does, since the distance grows along each ruling, nought where
/// the face holds its apex; none where an edge runs along the curve two
/// cylinders meet along.
fn radii(cone: &Cone, (body, face): (&Body, FaceId)) -> Option<[f64; 2]> {
    let off = |point: DVec3| from_the_axis(point, cone.origin, cone.axis);
    let mut inner = if body.apex_held(face).is_some() {
        0.0
    } else {
        f64::INFINITY
    };
    let mut outer: f64 = 0.0;
    for coedge in body.face(face).loops.iter().flatten() {
        let edge = body.edge(coedge.edge);
        let (near, far) = match body.curve(edge.curve) {
            Curve::Line(line) => {
                let start = line.point(edge.from);
                let speed = line.direction - cone.axis * cone.axis.dot(line.direction);
                let radial = start - cone.origin;
                let radial = radial - cone.axis * cone.axis.dot(radial);
                let nearest = if speed.length_squared() > 0.0 {
                    (edge.from - radial.dot(speed) / speed.length_squared())
                        .clamp(edge.from, edge.to)
                } else {
                    edge.from
                };
                (
                    off(line.point(nearest)),
                    off(start).max(off(line.point(edge.to))),
                )
            }
            Curve::Circle(circle) => {
                let centre = off(circle.center);
                let square = circle.axis.cross(cone.axis).length() <= Scale::RELATIVE;
                let near = if square {
                    (circle.radius - centre).abs()
                } else {
                    0.0
                };
                (near, centre + circle.radius)
            }
            Curve::Meet(_) => return None,
        };
        inner = inner.min(near);
        outer = outer.max(far);
    }
    inner.is_finite().then_some([inner, outer])
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
/// edges and the apex it holds within it; none where an edge runs along the
/// curve two cylinders meet along.
fn reach((body, face): (&Body, FaceId), direction: DVec3) -> Option<[f64; 2]> {
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    if let Some(apex) = body.apex_held(face) {
        low = direction.dot(apex);
        high = low;
    }
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
