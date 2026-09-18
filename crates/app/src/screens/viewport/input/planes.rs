//! Which plane a click lands on, before there is any sketch to draw in.

use cao_part::history::{FaceAnchor, Operation};
use cao_render::camera::view_angles_towards;
use cao_sketch::WorkPlane;
use cao_solid::Mesh;
use glam::{DVec2, DVec3};

use crate::screens::sketch::PlaneChoice;

use super::{DEFAULT_SKETCH_RADIUS, SketchContext, ViewportState, plane_half_size};

/// The step where the cursor is offering planes to draw on, and a click takes
/// one: a fresh sketch on it, and the view swung round to face it.
///
/// Returns true when a sketch was started.
pub(super) fn choose_a_plane(
    state: &mut ViewportState,
    context: &mut SketchContext<'_>,
    clicked: bool,
    origin: DVec3,
    direction: DVec3,
) -> bool {
    context.editor.hovered_plane = plane_under(state, context, origin, direction);
    context.editor.message = Some(match context.editor.hovered_plane {
        Some(PlaneChoice::Curved(_)) => context.lang.t("sketch.no_drawing_on_a_curve"),
        _ => context.lang.t("sketch.choose_a_plane"),
    });

    let (true, Some(choice)) = (clicked, context.editor.hovered_plane) else {
        return false;
    };
    // A curved face starts nothing, and the click does not fall through to
    // whatever lies behind it: starting a drawing on a plane hidden inside the
    // part would be worse than starting none.
    let Some(plane) = choice.plane() else {
        return false;
    };
    // A drawing laid on a face travels with it: the design records which face,
    // and the replay works the plane out again from the part as it then
    // stands. One laid on a plane of the origin is held by nothing.
    let on = match choice {
        PlaneChoice::Face { face, .. } => Some(FaceAnchor {
            face,
            up: once_facing(state, plane.normal()),
        }),
        _ => None,
    };
    context
        .document
        .apply(Operation::CreateSketch { plane, on });
    let sketch = context.document.sketches().len() - 1;
    context.editor.begin_editing(sketch, plane);
    // A fresh sketch has nothing to frame yet, so we show a patch of plane big
    // enough to draw in, centred where the click landed. On a face of the part
    // that matters: the plane's own origin is the world origin projected onto
    // it, which can be nowhere near the face.
    let center = plane
        .ray_intersection(origin, direction)
        .map(|local| plane.to_world(local))
        .unwrap_or(plane.origin);
    state.look_at_plane(plane, center, DEFAULT_SKETCH_RADIUS);
    true
}

/// What is offered to sketch on under the cursor: a face of the part where
/// there is one, otherwise the nearest of the three planes of the origin.
///
/// The part comes first rather than whatever is nearest the camera. The three
/// planes are unbounded sheets running right through the part, so nearest-wins
/// would leave them covering the very faces one usually wants — while they
/// stay reachable everywhere the part is not.
fn plane_under(
    state: &ViewportState,
    context: &SketchContext<'_>,
    origin: DVec3,
    direction: DVec3,
) -> Option<PlaneChoice> {
    let offered = what_the_part_offers(context.document.body(), origin, direction, |normal| {
        once_facing(state, normal)
    });
    if let Some(choice) = offered {
        return Some(choice);
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

/// What the face under the ray offers a drawing: its plane when the face is
/// flat, a refusal when it is not.
///
/// A face is a whole stretch of surface, however many flats it is stored as,
/// so a cylinder's wall answers once and answers no — rather than handing back
/// a plane tangent to whichever facet the ray happened to meet, at an angle
/// that depends on how finely the wall was cut.
///
/// `screen_up` says which way is up on screen once the view has swung round to
/// face a normal, which is what decides how the face will read.
fn what_the_part_offers(
    body: &Mesh,
    origin: DVec3,
    direction: DVec3,
    screen_up: impl Fn(DVec3) -> DVec3,
) -> Option<PlaneChoice> {
    let hit = body.ray_hit(origin, direction)?;
    let face = hit.polygon.face;
    if !body.is_flat(face) {
        return Some(PlaneChoice::Curved(face));
    }
    // The corners of the whole face, not of the piece the ray met: a flat face
    // is stored as several polygons, and the corner a drawing is read from has
    // to be one of the face's own.
    let corners: Vec<DVec3> = body
        .pieces_of(face)
        .flat_map(|piece| piece.corners.iter().copied())
        .collect();
    let normal = hit.polygon.normal();
    let plane = WorkPlane::from_face(&corners, normal, screen_up(normal));
    Some(PlaneChoice::Face { plane, face })
}

/// Which way is up on screen once the view has swung round to face `normal`.
///
/// The swing is what decides how the face will read, and it is settled by the
/// normal alone — so the answer is the camera the view is heading for, not the
/// oblique one the click came from.
fn once_facing(state: &ViewportState, normal: DVec3) -> DVec3 {
    let mut facing = state.camera;
    let (yaw, pitch) = view_angles_towards(normal.as_vec3());
    facing.set_view_angles(yaw, pitch);
    facing.up().as_dvec3()
}

/// Where a ray crosses a plane patch, in plane coordinates, if it lands inside
/// the square actually drawn.
fn plane_hit(plane: &WorkPlane, origin: DVec3, direction: DVec3, half_size: f64) -> Option<DVec2> {
    let hit = plane.ray_intersection(origin, direction)?;
    (hit.x.abs() <= half_size && hit.y.abs() <= half_size).then_some(hit)
}

#[cfg(test)]
mod tests;
