//! Two perpendicular walls decided to touch at a point, side by side.

use glam::DVec3;

use super::{Wall, bears};
use crate::brep::domain::Location;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, SurfaceId, Vertex};

/// Under this share of the tolerance, a vertex stands on a wall.
const ROUNDING: f64 = 1e-3;

/// For each wall touching a perpendicular one at a point, where both hold a
/// face, that point: a sample of its circles at its angle is put square to
/// the axis from it, as from a vertex. With it, for each wall, the vertices
/// lying on both walls that stand off that one by more than rounding, which
/// the point stands for there: decided on both within the tolerance, such a
/// vertex may stand a hair off one, and that wall's samples put square to
/// its axis from it would lean its ruling a hair over the other wall's
/// faces — a pin touching a wedge of a hundredth of a degree, whose two
/// edges cross the pin's ruling 8.7e-5 apart, one 7.6e-9 outside the pin
/// (5550629812). A vertex standing on a wall anchors its samples there
/// still: a post's floor 1e-7 under the point it touches a lying cylinder
/// at puts a corner on both, and the cylinder's samples stand at it, where
/// the floor's line leaves the cylinder (5551116747). Nor does a vertex on
/// a plane holding the wall's ruling through it, whatever it stands off the
/// wall: a face of that plane ends there, and the ruling anchored at the
/// point would run through it — a cylinder 1e-7 too wide to touch a post,
/// ending square to its axis through the post's, whose end face the post's
/// ruling crossed 5e-8 short of the rim's corner (5564306306).
///
/// The kernel decides such a pair to touch when its axes stand the sum of
/// the radii apart within its tolerance, and leaves no curve and no vertex:
/// the exact walls may overlap there by up to that tolerance. The point
/// stands where the planes of the origin touch both walls, a step of either
/// grid, and each wall's ruling through it, drawn on its own wall, would
/// pass through the other's triangles. Put halfway between the two walls'
/// rulings there, both rulings lie in one plane and meet at the point, as
/// two walls touching exactly do.
pub(super) fn anchors(body: &Body, walls: &[Wall]) -> Vec<(SurfaceId, DVec3, Vec<DVec3>)> {
    let eps = body.scale().eps();
    let mut anchors = Vec::new();
    for (at, one) in walls.iter().enumerate() {
        for other in &walls[at + 1..] {
            let Some(point) = touching(one, other, eps) else {
                continue;
            };
            if holds(body, one, point, eps) && holds(body, other, point, eps) {
                for wall in [one, other] {
                    let off: Vec<DVec3> = body
                        .vertex_ids()
                        .map(|id| body.vertex(id))
                        .filter(|vertex| bears(body, vertex, one) && bears(body, vertex, other))
                        .filter(|vertex| !holds_its_ruling(body, vertex, wall))
                        .map(|vertex| vertex.point)
                        .filter(|at| wall.1.distance(*at).abs() > eps * ROUNDING)
                        .collect();
                    anchors.push((wall.0, point, off));
                }
            }
        }
    }
    anchors
}

/// Whether the vertex lies on a plane holding the wall's ruling through it:
/// a face there ends at the vertex, and the ruling moved off it in that
/// plane runs through the face.
fn holds_its_ruling(body: &Body, vertex: &Vertex, (_, cylinder): &Wall) -> bool {
    vertex
        .on
        .iter()
        .filter_map(|id| body.surfaces.get(id.0 as usize))
        .any(|surface| match surface {
            Surface::Plane(plane) => plane.normal.dot(cylinder.axis).abs() <= Scale::RELATIVE,
            Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => false,
        })
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
