//! How much of the part a frame draws while a sketch is open.

use std::borrow::Cow;

use cao_render::MatterInFront;
use cao_sketch::WorkPlane;
use cao_solid::Body;
use glam::Vec3;

use super::ViewMode;
use crate::screens::sketch::SketchEditor;

/// The matter a frame draws opaque: the whole body, or what is left of it
/// behind the plane of the sketch being edited.
///
/// Drawing the body whole while a sketch is open means drawing inside the part
/// without seeing what one draws. What lies behind the plane stays — that is
/// what one draws against, an edge to line up with, a hole to centre on.
///
/// The cut holds only while the view sits square to that plane: orbited off it,
/// or landed on another plane, the part is seen whole.
pub(crate) fn shown_body<'a>(
    editor: &SketchEditor,
    mode: ViewMode,
    body: &'a Body,
    eye: Vec3,
) -> Cow<'a, Body> {
    match sketch_plane_in_view(editor, mode) {
        Some(plane) if editor.matter_in_front != MatterInFront::Opaque => {
            let (towards, offset) = plane.near_side(eye.as_dvec3());
            Cow::Owned(body.behind(towards, offset))
        }
        _ => Cow::Borrowed(body),
    }
}

/// The matter in front of the sketch plane, when it is to be drawn
/// see-through rather than opaque or not at all.
pub(crate) fn see_through_body(
    editor: &SketchEditor,
    mode: ViewMode,
    body: &Body,
    eye: Vec3,
) -> Option<Body> {
    let plane = sketch_plane_in_view(editor, mode)
        .filter(|_| editor.matter_in_front == MatterInFront::SeeThrough)?;
    let (towards, offset) = plane.near_side(eye.as_dvec3());
    Some(body.behind(-towards, -offset))
}

fn sketch_plane_in_view(editor: &SketchEditor, mode: ViewMode) -> Option<WorkPlane> {
    editor.active_sketch().and(editor.plane).filter(|plane| {
        matches!(mode, ViewMode::Plane(view) if view.normal().cross(plane.normal()).length() < 1e-9)
    })
}

#[cfg(test)]
mod tests;
