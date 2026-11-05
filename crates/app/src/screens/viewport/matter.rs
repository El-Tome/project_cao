//! How much of the part a frame draws while a sketch is open.

use std::borrow::Cow;
use std::cell::RefCell;

use cao_render::MatterInFront;
use cao_sketch::WorkPlane;
use cao_solid::Body;
use glam::{DVec3, Vec3};

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
    let Some((cut, see_through)) = Cut::asked(editor, mode, eye) else {
        return Drawn {
            opaque: Cow::Borrowed(body),
            see_through: None,
        };
    };
    Drawn {
        opaque: Cow::Owned(body.behind(cut.towards, cut.offset)),
        see_through: see_through.then(|| body.in_front(cut.towards, cut.offset)),
    }
}

/// The halves last cut, each handed to every frame until the part, the plane
/// or the side the eye is on changes: a cut costs the kernel a boolean, which
/// no frame can afford.
#[derive(Default)]
pub(crate) struct Sections {
    last: RefCell<Option<Section>>,
    #[cfg(test)]
    behind_cuts: std::cell::Cell<usize>,
    #[cfg(test)]
    front_cuts: std::cell::Cell<usize>,
}

struct Section {
    cut: Cut,
    body: Body,
    behind: Option<Body>,
    in_front: Option<Body>,
}

impl Sections {
    pub(crate) fn drawn<'a>(
        &self,
        editor: &SketchEditor,
        mode: ViewMode,
        body: &'a Body,
        eye: Vec3,
    ) -> Drawn<'a> {
        let Some((cut, see_through)) = Cut::asked(editor, mode, eye) else {
            return drawn(editor, mode, body, eye);
        };
        let mut last = self.last.borrow_mut();
        // Compared by value: the document keeps no revision of its body, and
        // reading two bodies through costs a fraction of cutting one.
        let mut section = match last.take() {
            Some(section) if section.cut == cut && section.body == *body => section,
            _ => Section {
                cut,
                body: body.clone(),
                behind: None,
                in_front: None,
            },
        };
        let behind = section
            .behind
            .get_or_insert_with(|| {
                #[cfg(test)]
                self.behind_cuts.set(self.behind_cuts.get() + 1);
                body.behind(cut.towards, cut.offset)
            })
            .clone();
        let in_front = see_through.then(|| {
            section
                .in_front
                .get_or_insert_with(|| {
                    #[cfg(test)]
                    self.front_cuts.set(self.front_cuts.get() + 1);
                    body.in_front(cut.towards, cut.offset)
                })
                .clone()
        });
        *last = Some(section);
        Drawn {
            opaque: Cow::Owned(behind),
            see_through: in_front,
        }
    }

    #[cfg(test)]
    pub(crate) fn behind_cuts(&self) -> usize {
        self.behind_cuts.get()
    }

    #[cfg(test)]
    pub(crate) fn front_cuts(&self) -> usize {
        self.front_cuts.get()
    }
}

/// Where the body is cut, and which side of the plane is kept.
#[derive(Clone, Copy, PartialEq)]
struct Cut {
    towards: DVec3,
    offset: f64,
}

impl Cut {
    /// The cut a frame asks for, if any, and whether what stands in front is
    /// handed over to be drawn see-through.
    fn asked(editor: &SketchEditor, mode: ViewMode, eye: Vec3) -> Option<(Cut, bool)> {
        let plane = sketch_plane_in_view(editor, mode)?;
        let (towards, offset) = plane.near_side(eye.as_dvec3());
        let see_through = match editor.matter_in_front {
            MatterInFront::Opaque => return None,
            MatterInFront::Hidden => false,
            MatterInFront::SeeThrough => true,
        };
        Some((Cut { towards, offset }, see_through))
    }
}

fn sketch_plane_in_view(editor: &SketchEditor, mode: ViewMode) -> Option<WorkPlane> {
    editor.active_sketch().and(editor.plane).filter(|plane| {
        matches!(mode, ViewMode::Plane(view) if view.normal().cross(plane.normal()).length() < 1e-9)
    })
}

#[cfg(test)]
mod tests;
