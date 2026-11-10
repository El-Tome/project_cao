//! Which plane a click lands on, before there is any sketch to draw in, and
//! what the pointer does to the section view: a face clicked to cut along,
//! the handle pulled along the normal.

use cao_part::history::{FaceAnchor, Operation};
use cao_render::camera::view_angles_towards;
use cao_sketch::WorkPlane;
use cao_solid::Body;
use glam::{DVec2, DVec3};

use crate::screens::SketchContext;
use crate::screens::sketch::PlaneChoice;

use super::{PICK_PIXELS, ViewScale, ViewportState, plane_half_size, to_ndc};
#[cfg(test)]
use crate::screens::viewport::matter::shown;
use crate::screens::viewport::matter::{drawn_under, seen};
use crate::screens::viewport::navigation::drag_asked;
use crate::screens::viewport::{DEFAULT_SKETCH_RADIUS, section::Section};

/// The step where the cursor is offering planes to draw on, and a click takes
/// one: a fresh sketch on it, and the view swung round to face it.
///
/// Returns true when a sketch was started; the section view, if on, is turned
/// off, the sketch's own cut taking over.
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
    // The cut of a kept section switched on is a plane the drawing hangs on
    // and follows; the section view's own cut is taken once and stays put.
    let laid = match (choice, state.section.is_none()) {
        (PlaneChoice::Section(_), true) => state
            .kept_sections
            .cutting()
            .map(|section| Operation::CreateSketchOnSection { section, plane }),
        _ => None,
    };
    context
        .document
        .apply(laid.unwrap_or(Operation::CreateSketch { plane, on }));
    let sketch = context.document.sketches().len() - 1;
    let plane = context.document.sketches()[sketch].plane;
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
    state.section = None;
    true
}

/// While the section waits for a face, a click on a flat one makes its plane
/// the cutting plane, facing out of the part, at a distance of nothing.
pub(super) fn choose_a_cutting_face(
    state: &mut ViewportState,
    body: &Body,
    clicked: bool,
    origin: DVec3,
    direction: DVec3,
) -> bool {
    if !clicked || !state.section().is_some_and(Section::is_waiting_for_a_face) {
        return false;
    }
    let offered =
        what_the_part_offers(body, origin, direction, |normal| once_facing(state, normal));
    let (Some(PlaneChoice::Face { plane, .. }), Some(section)) = (offered, state.section.as_mut())
    else {
        return false;
    };
    section.choose_face(plane);
    true
}

/// Whether the gesture this frame belongs to the section view: a click while
/// it waits for a face, or a drag of the primary button, navigating nothing,
/// that took hold of its handle. A sketch being edited takes every gesture.
pub(crate) fn point_the_section(
    ui: &egui::Ui,
    state: &mut ViewportState,
    response: &egui::Response,
    (rect, scale): (egui::Rect, ViewScale),
    context: &SketchContext<'_>,
) -> bool {
    if context.editor.active_sketch().is_some() {
        return false;
    }
    let camera = state.camera;
    let ray = |at| {
        let (origin, direction) = camera.ray(to_ndc(at, rect), rect.width() / rect.height());
        (origin.as_dvec3(), direction.as_dvec3())
    };
    let body = context.document.body();
    if state.section().is_some_and(Section::is_waiting_for_a_face) {
        let Some(at) = response
            .interact_pointer_pos()
            .filter(|_| response.clicked())
        else {
            return false;
        };
        let (origin, direction) = ray(at);
        let Some(drawn) = seen(state, context.editor, context.document) else {
            return true;
        };
        let met = drawn_under(&drawn, origin, direction);
        let piece = met.map_or(&*drawn.opaque, |(piece, _)| piece);
        if choose_a_cutting_face(state, piece, true, origin, direction) {
            anchor_the_cut(state, piece, body, (origin, direction));
        }
        return true;
    }
    let navigating = ui.input(|input| drag_asked(input, &state.config).is_some());
    let Some(section) = state.section.as_mut() else {
        return false;
    };
    if response.drag_started_by(egui::PointerButton::Primary)
        && !navigating
        && let Some(at) = ui.input(|input| input.pointer.press_origin())
    {
        let (origin, direction) = ray(at);
        section.grab(body, origin, direction, scale.world_size_of(PICK_PIXELS));
    }
    if !section.is_held() {
        return false;
    }
    match response.interact_pointer_pos() {
        Some(at) if response.dragged_by(egui::PointerButton::Primary) => {
            let (origin, direction) = ray(at);
            section.pull(origin, direction);
        }
        _ => section.let_go(),
    }
    true
}

/// The part's own face the section was just laid on, kept with it so that a
/// section kept in the part follows that face.
fn anchor_the_cut(state: &mut ViewportState, piece: &Body, part: &Body, ray: (DVec3, DVec3)) {
    let Some(hit) = piece.ray_hit(ray.0, ray.1) else {
        return;
    };
    let up = once_facing(state, hit.normal);
    if let Some(section) = state.section.as_mut() {
        section.on = own_face(piece, part, hit.face).map(|face| FaceAnchor { face, up });
    }
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
    if let Some(choice) = what_is_drawn_offers(state, context, origin, direction) {
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

/// What the part as drawn offers under the ray. A face the cut left in pieces
/// is offered as the part's own face it was parted from; a face the part does
/// not have at all is the cut itself, offered as its plane, turned the way the
/// face met looks.
fn what_is_drawn_offers(
    state: &ViewportState,
    context: &SketchContext<'_>,
    origin: DVec3,
    direction: DVec3,
) -> Option<PlaneChoice> {
    let part = context.document.body();
    let drawn = seen(state, context.editor, context.document)?;
    let (piece, hit) = drawn_under(&drawn, origin, direction)?;
    let up = |normal| once_facing(state, normal);
    match own_face(piece, part, hit.face) {
        Some(face) => offered_by(part, face, hit.normal, up),
        None => {
            let cut = state.section_shown()?.cutting_plane();
            Some(PlaneChoice::Section(
                match hit.normal.dot(cut.normal()) > 0.0 {
                    true => cut,
                    false => WorkPlane::from_normal(cut.origin, -cut.normal()),
                },
            ))
        }
    }
}

/// The part's own face a face of `drawn` stands for, or `None` for one the
/// part does not have.
fn own_face(drawn: &Body, part: &Body, mut face: usize) -> Option<usize> {
    while !part.has_face(face) {
        face = drawn.parted_from(face)?;
    }
    Some(face)
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
    body: &Body,
    origin: DVec3,
    direction: DVec3,
    screen_up: impl Fn(DVec3) -> DVec3,
) -> Option<PlaneChoice> {
    let hit = body.ray_hit(origin, direction)?;
    offered_by(body, hit.face, hit.normal, screen_up)
}

fn offered_by(
    body: &Body,
    face: usize,
    normal: DVec3,
    screen_up: impl Fn(DVec3) -> DVec3,
) -> Option<PlaneChoice> {
    if !body.is_flat(face) {
        return Some(PlaneChoice::Curved(face));
    }
    // The corners of the whole face, not of the piece the ray met: a flat face
    // is stored as several pieces, and the corner a drawing is read from has
    // to be one of the face's own.
    let corners = body.plane_of(face)?.corners;
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
