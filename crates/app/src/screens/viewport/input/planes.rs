//! Which plane a click lands on, before there is any sketch to draw in.

use cao_sketch::WorkPlane;
use glam::{DVec2, DVec3};

use crate::screens::sketch::PlaneChoice;

use super::{SketchContext, ViewportState, plane_half_size};

/// What is offered to sketch on under the cursor: a face of the part where
/// there is one, otherwise the nearest of the three planes of the origin.
///
/// The part comes first rather than whatever is nearest the camera. The three
/// planes are unbounded sheets running right through the part, so nearest-wins
/// would leave them covering the very faces one usually wants — while they
/// stay reachable everywhere the part is not.
pub(crate) fn plane_under(
    state: &ViewportState,
    context: &SketchContext<'_>,
    origin: DVec3,
    direction: DVec3,
) -> Option<PlaneChoice> {
    if let Some(hit) = context.document.body().ray_hit(origin, direction) {
        // The sketch's own origin lands where the world origin projects onto
        // the face, so that a drawing on a face is still measured from
        // somewhere the user can point at.
        let normal = hit.polygon.normal();
        let plane = WorkPlane::from_normal(normal * hit.polygon.plane_offset(), normal);
        return Some(PlaneChoice::Face(plane));
    }

    let half_size = plane_half_size(state);
    WorkPlane::ORIGIN_PLANES
        .iter()
        .enumerate()
        .filter_map(|(index, plane)| {
            let local = plane_hit(plane, origin, direction, half_size)?;
            Some(((plane.to_world(local) - origin).length(), index))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, index)| PlaneChoice::Origin(index))
}

/// Where a ray crosses a plane patch, in plane coordinates, if it lands inside
/// the square actually drawn.
fn plane_hit(plane: &WorkPlane, origin: DVec3, direction: DVec3, half_size: f64) -> Option<DVec2> {
    let hit = plane.ray_intersection(origin, direction)?;
    (hit.x.abs() <= half_size && hit.y.abs() <= half_size).then_some(hit)
}
