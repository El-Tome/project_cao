//! Two perpendicular walls decided to touch at a point, side by side.

use std::f64::consts::TAU;

use glam::DVec3;

use super::super::sampling::divisions;
use super::{Wall, bears, wall_of};
use crate::brep::curve::Curve;
use crate::brep::domain::Location;
use crate::brep::scale::Scale;
use crate::brep::topology::{Body, SurfaceId};

/// For each wall touching a perpendicular one at a point, the step of its
/// grid through that point, with the stretch of height between the levels
/// of its circles and vertices round it: the circles bounding that stretch
/// withhold the step.
///
/// The kernel decides such a pair to touch when its axes stand the sum of
/// the radii apart within its tolerance, and leaves no curve and no vertex:
/// the exact walls may overlap there by up to that tolerance. The point
/// stands where the planes of the origin touch both walls, a step of either
/// grid, and each wall's ruling through it would pass through the other's
/// triangles. Withheld from both where both hold a face there, the chords
/// across sag apart from each other.
pub(super) fn withheld(
    body: &Body,
    walls: &[Wall],
    tolerance: f64,
) -> Vec<(SurfaceId, (usize, [f64; 2]))> {
    let eps = body.scale().eps();
    let mut withheld = Vec::new();
    for (at, one) in walls.iter().enumerate() {
        for other in &walls[at + 1..] {
            let Some(point) = touching(one, other, eps) else {
                continue;
            };
            let steps = [one, other].map(|wall| on_a_step(body, wall, point, tolerance, eps));
            if let [Some(first), Some(second)] = steps {
                withheld.extend([(one.0, first), (other.0, second)]);
            }
        }
    }
    withheld
}

/// The point two perpendicular walls touch at, side by side: none unless
/// their axes stand the sum of their radii apart within `eps`.
fn touching((_, one): &Wall, (_, other): &Wall, eps: f64) -> Option<DVec3> {
    if one.axis.dot(other.axis).abs() > Scale::RELATIVE {
        return None;
    }
    let across = one.axis.cross(other.axis).normalize();
    let between = other.origin - one.origin;
    let apart = across.dot(between);
    if (apart.abs() - one.radius - other.radius).abs() > eps {
        return None;
    }
    let foot = one.origin + one.axis * one.axis.dot(between);
    Some(foot + across * apart.signum() * one.radius)
}

/// The step of a wall's grid through `point`, with the levels of its
/// circles and vertices on either side of the point's height, when the
/// point stands on a step and inside a face of the wall.
fn on_a_step(
    body: &Body,
    wall: &Wall,
    point: DVec3,
    tolerance: f64,
    eps: f64,
) -> Option<(usize, [f64; 2])> {
    let (id, cylinder) = wall;
    let at = cylinder.parameters(point);
    let steps = divisions(cylinder.radius, tolerance);
    let step = TAU / steps as f64;
    let rank = (at.x / step).round();
    if (at.x - rank * step).abs() * cylinder.radius > eps {
        return None;
    }
    let holds = body.face_ids().any(|face| {
        body.face(face).surface == *id
            && !matches!(body.locate(face, at, eps), Ok(Location::Outside))
    });
    if !holds {
        return None;
    }
    let circles = body
        .edge_ids()
        .filter_map(|edge| match body.curve(body.edge(edge).curve) {
            Curve::Circle(circle) if wall_of(circle, std::slice::from_ref(wall), eps).is_some() => {
                Some(circle.center.dot(cylinder.axis))
            }
            _ => None,
        });
    let vertices = body
        .vertex_ids()
        .map(|vertex| body.vertex(vertex))
        .filter(|vertex| bears(body, vertex, wall))
        .map(|vertex| vertex.point.dot(cylinder.axis));
    let height = point.dot(cylinder.axis);
    let (mut below, mut above) = (f64::NEG_INFINITY, f64::INFINITY);
    for level in circles.chain(vertices) {
        if level <= height {
            below = below.max(level);
        } else {
            above = above.min(level);
        }
    }
    Some((rank.rem_euclid(steps as f64) as usize, [below, above]))
}
