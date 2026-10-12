//! A line against an elliptic wall (#571): a quadratic, solved in the wall's
//! section squashed onto the unit circle, where the line passes nearest the
//! axis halfway between its two roots. How far it stands off the wall is read
//! square to the axis, as far as its offset from the axis exceeds the wall's
//! reach along the line's normal: the gap to the wall's tangent parallel to
//! it, which keeps its digits where the roots run together. The touch is
//! decided on that gap as `crossing.rs` decides it on a cylinder (decision
//! 4): within the tolerance outside, the line touches the wall once; within
//! it inside, it crosses twice, unless its roots stand within the tolerance
//! of each other or a surface it lies on was decided to touch the wall there.

use glam::{DVec2, DVec3};

use super::super::parallel;
use super::{Solved, Touches, beside, touches};
use crate::brep::curve::Line;
use crate::brep::scale::Scale;
use crate::brep::surface::EllipticCylinder;

pub(super) fn line_and_elliptic(
    line: &Line,
    wall: &EllipticCylinder,
    scale: Scale,
    touching: &Touches,
) -> Solved {
    if parallel(line.direction, wall.axis, scale) {
        return beside(wall.distance(line.origin), scale);
    }
    let flat = |v: DVec3| DVec2::new(v.dot(wall.u), v.dot(wall.v));
    let (from, towards) = (flat(line.origin - wall.origin), flat(line.direction));
    let normal = towards.perp().normalize();
    let reach = (wall.major * normal.x).hypot(wall.minor * normal.y);
    let gap = normal.dot(from).abs() - reach;
    if gap > scale.eps() {
        return Solved::At(Vec::new());
    }
    let squashed = |v: DVec2| DVec2::new(v.x / wall.major, v.y / wall.minor);
    let (at, pace) = (squashed(from), squashed(towards));
    let speed = pace.length_squared();
    let closest = -at.dot(pace) / speed;
    let depth = -gap / reach;
    let half = (depth * (2.0 - depth)).max(0.0).sqrt() / speed.sqrt();
    if touches(gap, half, touching.here(line.point(closest), scale), scale) {
        return Solved::At(vec![(closest, true)]);
    }
    Solved::At(vec![(closest - half, false), (closest + half, false)])
}
