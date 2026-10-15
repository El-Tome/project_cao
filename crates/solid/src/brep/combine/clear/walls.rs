//! A face on a wall, seen about its axis: the stretch of the axis it spans,
//! and how near and how far from the axis it stands.

use glam::DVec3;

use super::{between_stretches, boxed, from_the_axis, reach};
use crate::brep::conic::Round;
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Surface};
use crate::brep::topology::{Body, FaceId};

pub(super) struct Wall {
    ends: [DVec3; 2],
    axis: DVec3,
    inner: f64,
    outer: f64,
}

impl Wall {
    pub(super) fn of(surface: &Surface, face: (&Body, FaceId)) -> Option<Wall> {
        let (origin, axis, [inner, outer]) = match surface {
            Surface::Cylinder(cylinder) => (cylinder.origin, cylinder.axis, [cylinder.radius; 2]),
            Surface::Cone(cone) => (cone.origin, cone.axis, radii(cone, face)?),
            Surface::Elliptic(wall) => (wall.origin, wall.axis, [wall.minor, wall.major]),
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
    pub(super) fn beside(&self, other: &Wall, room: f64) -> bool {
        between_stretches(self.ends, other.ends) > self.outer + other.outer + room
    }

    /// Whether this wall stands inside the hollow the other leaves about
    /// its axis: a bore within a countersink's narrow end.
    pub(super) fn within(&self, other: &Wall, room: f64) -> bool {
        let off = |point: DVec3| from_the_axis(point, other.ends[0], other.axis);
        off(self.ends[0]).max(off(self.ends[1])) + self.outer < other.inner - room
    }
}

/// How near and how far from its axis a face on a cone stands: as its
/// boundary does, since the distance grows along each ruling, nought where
/// the face holds its apex or a conic bounds it, a conic of the cone as far
/// out as it is widest and any other as its box's farthest corner; none
/// where an edge runs along the curve two cylinders meet along.
pub(super) fn radii(cone: &Cone, (body, face): (&Body, FaceId)) -> Option<[f64; 2]> {
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
            Curve::Conic(conic) => (
                0.0,
                if conic.round == Round::Cone(*cone) {
                    conic.widest(edge.from, edge.to)
                } else {
                    boxed(conic, edge.from, edge.to, off)
                },
            ),
            Curve::Meet(_) => return None,
        };
        inner = inner.min(near);
        outer = outer.max(far);
    }
    inner.is_finite().then_some([inner, outer])
}
