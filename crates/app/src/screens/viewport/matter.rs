//! How much of the part a frame draws while a sketch is open.

use std::borrow::Cow;

use cao_render::MatterInFront;
use cao_sketch::WorkPlane;
use cao_solid::Body;
use glam::Vec3;

use super::ViewMode;
use crate::screens::sketch::SketchEditor;

/// The matter a frame draws: opaque, the whole body or what is left of it
/// behind the plane of the sketch being edited; see-through, what stands in
/// front of that plane, when that is how it is to be shown.
///
/// Drawing the body whole while a sketch is open means drawing inside the part
/// without seeing what one draws. What lies behind the plane stays — that is
/// what one draws against, an edge to line up with, a hole to centre on.
///
/// The cut holds only while the view sits on that plane or on one parallel to
/// it: orbited off it, or landed on a plane across it, the part is seen whole.
pub(crate) struct Drawn<'a> {
    pub(crate) opaque: Cow<'a, Body>,
    pub(crate) see_through: Option<Body>,
}

pub(crate) fn drawn<'a>(
    editor: &SketchEditor,
    mode: ViewMode,
    body: &'a Body,
    eye: Vec3,
) -> Drawn<'a> {
    let whole = Drawn {
        opaque: Cow::Borrowed(body),
        see_through: None,
    };
    let Some(plane) = sketch_plane_in_view(editor, mode) else {
        return whole;
    };
    let (towards, offset) = plane.near_side(eye.as_dvec3());
    match editor.matter_in_front {
        MatterInFront::Opaque => whole,
        MatterInFront::Hidden => Drawn {
            opaque: Cow::Owned(body.behind(towards, offset)),
            see_through: None,
        },
        MatterInFront::SeeThrough => {
            let (behind, in_front) = body.parted(towards, offset);
            Drawn {
                opaque: Cow::Owned(behind),
                see_through: Some(in_front),
            }
        }
    }
}

fn sketch_plane_in_view(editor: &SketchEditor, mode: ViewMode) -> Option<WorkPlane> {
    editor.active_sketch().and(editor.plane).filter(|plane| {
        matches!(mode, ViewMode::Plane(view) if view.normal().cross(plane.normal()).length() < 1e-9)
    })
}

#[cfg(test)]
mod tests;
