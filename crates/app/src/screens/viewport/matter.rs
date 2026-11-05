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
#[derive(Clone)]
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
    match Cut::asked(editor, mode, eye) {
        Some(cut) => cut.of(body),
        None => Drawn {
            opaque: Cow::Borrowed(body),
            see_through: None,
        },
    }
}

/// The last section cut, handed to every frame until what it was cut from
/// changes: the cut costs the kernel a boolean, which no frame can afford.
#[derive(Default)]
pub(crate) struct Sections {
    last: RefCell<Option<Section>>,
    #[cfg(test)]
    cuts: std::cell::Cell<usize>,
}

struct Section {
    cut: Cut,
    body: Body,
    drawn: Drawn<'static>,
}

impl Sections {
    pub(crate) fn drawn<'a>(
        &self,
        editor: &SketchEditor,
        mode: ViewMode,
        body: &'a Body,
        eye: Vec3,
    ) -> Drawn<'a> {
        let Some(cut) = Cut::asked(editor, mode, eye) else {
            return drawn(editor, mode, body, eye);
        };
        let mut last = self.last.borrow_mut();
        // Compared by value: the document keeps no revision of its body, and
        // reading two bodies through costs a fraction of cutting one.
        let section = match last.take() {
            Some(section) if section.cut == cut && section.body == *body => section,
            _ => {
                #[cfg(test)]
                self.cuts.set(self.cuts.get() + 1);
                Section {
                    cut,
                    body: body.clone(),
                    drawn: cut.of(body),
                }
            }
        };
        let drawn = section.drawn.clone();
        *last = Some(section);
        drawn
    }

    #[cfg(test)]
    pub(crate) fn cuts(&self) -> usize {
        self.cuts.get()
    }
}

/// Where the body is cut, which side is kept, and whether what stands in
/// front is handed over to be drawn see-through.
#[derive(Clone, Copy, PartialEq)]
struct Cut {
    towards: DVec3,
    offset: f64,
    see_through: bool,
}

impl Cut {
    fn asked(editor: &SketchEditor, mode: ViewMode, eye: Vec3) -> Option<Cut> {
        let plane = sketch_plane_in_view(editor, mode)?;
        let (towards, offset) = plane.near_side(eye.as_dvec3());
        let see_through = match editor.matter_in_front {
            MatterInFront::Opaque => return None,
            MatterInFront::Hidden => false,
            MatterInFront::SeeThrough => true,
        };
        Some(Cut {
            towards,
            offset,
            see_through,
        })
    }

    fn of<'a>(self, body: &Body) -> Drawn<'a> {
        if self.see_through {
            let (behind, in_front) = body.parted(self.towards, self.offset);
            Drawn {
                opaque: Cow::Owned(behind),
                see_through: Some(in_front),
            }
        } else {
            Drawn {
                opaque: Cow::Owned(body.behind(self.towards, self.offset)),
                see_through: None,
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
