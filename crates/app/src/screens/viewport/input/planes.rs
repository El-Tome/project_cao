//! Which plane a click lands on, before there is any sketch to draw in.

use cao_part::history::Operation;
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
    context.document.apply(Operation::CreateSketch { plane });
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
    if let Some(choice) = what_the_part_offers(context.document.body(), origin, direction) {
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
fn what_the_part_offers(body: &Mesh, origin: DVec3, direction: DVec3) -> Option<PlaneChoice> {
    let hit = body.ray_hit(origin, direction)?;
    let face = hit.polygon.face;
    if !body.is_flat(face) {
        return Some(PlaneChoice::Curved(face));
    }
    // The sketch's own origin lands where the world origin projects onto the
    // face, so that a drawing on a face is still measured from somewhere the
    // user can point at.
    let normal = hit.polygon.normal();
    let plane = WorkPlane::from_normal(normal * hit.polygon.plane_offset(), normal);
    Some(PlaneChoice::Face { plane, face })
}

/// Where a ray crosses a plane patch, in plane coordinates, if it lands inside
/// the square actually drawn.
fn plane_hit(plane: &WorkPlane, origin: DVec3, direction: DVec3, half_size: f64) -> Option<DVec2> {
    let hit = plane.ray_intersection(origin, direction)?;
    (hit.x.abs() <= half_size && hit.y.abs() <= half_size).then_some(hit)
}

#[cfg(test)]
mod tests;
