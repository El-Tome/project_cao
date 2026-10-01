//! Two perpendicular walls decided to touch at a point, side by side.

use glam::DVec3;

use super::Wall;
use crate::brep::domain::Location;
use crate::brep::scale::Scale;
use crate::brep::topology::{Body, SurfaceId};

/// For each wall touching a perpendicular one at a point, where both hold a
/// face, that point: a sample of its circles at its angle is put square to
/// the axis from it, as from a vertex.
///
/// The kernel decides such a pair to touch when its axes stand the sum of
/// the radii apart within its tolerance, and leaves no curve and no vertex:
/// the exact walls may overlap there by up to that tolerance. The point
/// stands where the planes of the origin touch both walls, a step of either
/// grid, and each wall's ruling through it, drawn on its own wall, would
/// pass through the other's triangles. Put halfway between the two walls'
/// rulings there, both rulings lie in one plane and meet at the point, as
/// two walls touching exactly do.
pub(super) fn anchors(body: &Body, walls: &[Wall]) -> Vec<(SurfaceId, DVec3)> {
    let eps = body.scale().eps();
    let mut anchors = Vec::new();
    for (at, one) in walls.iter().enumerate() {
        for other in &walls[at + 1..] {
            let Some(point) = touching(one, other, eps) else {
                continue;
            };
            if holds(body, one, point, eps) && holds(body, other, point, eps) {
                anchors.extend([(one.0, point), (other.0, point)]);
            }
        }
    }
    anchors
}

/// The point two perpendicular walls touch at, side by side, halfway
/// between them along their common perpendicular: none unless their axes
/// stand the sum of their radii apart within `eps`.
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
    let toward = across * apart.signum();
    let on_one = one.origin + one.axis * one.axis.dot(between) + toward * one.radius;
    let on_other = other.origin - other.axis * other.axis.dot(between) - toward * other.radius;
    Some((on_one + on_other) / 2.0)
}

/// Whether a face of the wall holds `point`, inside or on its boundary.
fn holds(body: &Body, (id, cylinder): &Wall, point: DVec3, eps: f64) -> bool {
    let at = cylinder.parameters(point);
    body.face_ids().any(|face| {
        body.face(face).surface == *id
            && !matches!(body.locate(face, at, eps), Ok(Location::Outside))
    })
}
