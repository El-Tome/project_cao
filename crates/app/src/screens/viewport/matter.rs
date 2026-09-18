//! How much of the part a frame draws while a sketch is open.

use std::borrow::Cow;

use cao_solid::Mesh;
use glam::Vec3;

use crate::screens::sketch::SketchEditor;

/// The matter a frame draws: the whole body, or what is left of it behind the
/// plane of the sketch being edited.
///
/// Drawing the body whole while a sketch is open means drawing inside the part
/// without seeing what one draws. What lies behind the plane stays — that is
/// what one draws against, an edge to line up with, a hole to centre on.
///
/// The editor keeps its plane after the sketch is closed, so the view can still
/// be aligned with it; what says the cut is on is that a sketch is being edited.
pub(crate) fn shown_body<'a>(editor: &SketchEditor, body: &'a Mesh, eye: Vec3) -> Cow<'a, Mesh> {
    let cutting = editor.active_sketch().and(editor.plane);
    let Some(plane) = cutting else {
        return Cow::Borrowed(body);
    };
    let (towards, offset) = plane.near_side(eye.as_dvec3());
    Cow::Owned(body.behind(towards, offset))
}

#[cfg(test)]
mod tests;
