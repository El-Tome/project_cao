//! How much of the part a frame draws while a sketch is open or a section is
//! on, and where a ray meets what it draws.

use std::borrow::Cow;
use std::cell::RefCell;

use cao_part::PartDocument;
use cao_render::MatterInFront;
use cao_sketch::WorkPlane;
use cao_solid::{Body, FaceHit};
use glam::{DVec3, Vec3};

use super::section::Section;
use super::visibility::FaceModes;
use super::{ViewMode, ViewportState, to_ndc};
use crate::screens::sketch::SketchEditor;

/// The matter a frame draws: opaque, the whole body, a side of a cut or
/// nothing; see-through, what of it is to be shown that way. While a sketch
/// is edited, what is opaque is what lies behind its plane, and see-through
/// what stands in front of it, when that is how it is to be shown.
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

/// What a frame draws of the part: the sketch's own cut while a sketch is
/// edited, the section's while one is on, the part whole otherwise.
pub(crate) fn shown<'a>(state: &ViewportState, editor: &SketchEditor, body: &'a Body) -> Drawn<'a> {
    match state.section_shown() {
        Some(section) if editor.active_sketch().is_none() => {
            state.sections.sectioned(section, body)
        }
        _ => state
            .sections
            .drawn(editor, state.mode, body, state.camera.eye()),
    }
}

/// The piece of what is drawn, opaque or see-through, the ray meets first,
/// and where. No invisible face is met: the ray goes on through it.
pub(super) fn drawn_under<'a>(
    faces: &FaceModes,
    drawn: &'a Drawn<'_>,
    origin: DVec3,
    direction: DVec3,
) -> Option<(&'a Body, FaceHit)> {
    std::iter::once(&*drawn.opaque)
        .chain(drawn.see_through.as_ref())
        .filter_map(|piece| {
            let shown = |face| faces.shows(piece, face);
            Some((piece, piece.ray_hit_among(origin, direction, &shown)?))
        })
        .min_by(|(_, a), (_, b)| a.distance.total_cmp(&b.distance))
}

/// What a frame draws of the part, nothing while the body is hidden: worked
/// out once a frame and handed both to the painting and to the point under the
/// pointer, since a second call would compare the whole body with the one the
/// section's halves were cut from, and copy those halves again.
pub(crate) fn seen<'a>(
    state: &ViewportState,
    editor: &SketchEditor,
    body: &'a Body,
) -> Option<Drawn<'a>> {
    (!state.shows_nothing()).then(|| shown(state, editor, body))
}

impl ViewportState {
    /// Where the ray through a place of `canvas` first meets what is drawn.
    pub(super) fn point_of_the_part_at(
        &self,
        document: &PartDocument,
        drawn: &Drawn<'_>,
        at: egui::Pos2,
        canvas: egui::Rect,
    ) -> Option<DVec3> {
        let (origin, direction) = self.camera.ray(to_ndc(at, canvas), canvas.aspect_ratio());
        let faces = self.face_modes(document);
        self.point_met(&faces, drawn, origin.as_dvec3(), direction.as_dvec3())
    }

    fn point_met(
        &self,
        faces: &FaceModes,
        drawn: &Drawn<'_>,
        origin: DVec3,
        direction: DVec3,
    ) -> Option<DVec3> {
        let (_, hit) = drawn_under(faces, drawn, origin, direction)?;
        Some(origin + direction * hit.distance)
    }
}

/// The halves last cut, each handed to every frame until the part, the plane
/// or the side the eye is on changes: a cut costs the kernel a boolean, which
/// no frame can afford.
#[derive(Default)]
pub(crate) struct Sections {
    last: RefCell<Option<Halves>>,
    #[cfg(test)]
    behind_cuts: std::cell::Cell<usize>,
    #[cfg(test)]
    front_cuts: std::cell::Cell<usize>,
}

struct Halves {
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
        let (behind, in_front) = self.halves(cut, body, true, see_through);
        Drawn {
            opaque: Cow::Owned(behind.unwrap_or_default()),
            see_through: in_front,
        }
    }

    /// Each side of the section drawn as it is set; both sides alike, the
    /// part is drawn whole and nothing is cut.
    pub(crate) fn sectioned<'a>(&self, section: &Section, body: &'a Body) -> Drawn<'a> {
        let parts: Vec<(Cow<'a, Body>, MatterInFront)> = if section.behind == section.in_front {
            vec![(Cow::Borrowed(body), section.behind)]
        } else {
            let tilted = section.tilted();
            let towards = tilted.normal();
            let cut = Cut {
                towards,
                offset: towards.dot(tilted.origin) + section.offset,
            };
            let shown = |state| state != MatterInFront::Hidden;
            let (behind, in_front) =
                self.halves(cut, body, shown(section.behind), shown(section.in_front));
            [(behind, section.behind), (in_front, section.in_front)]
                .into_iter()
                .filter_map(|(half, state)| Some((Cow::Owned(half?), state)))
                .collect()
        };
        let mut drawn = Drawn {
            opaque: Cow::Owned(Body::default()),
            see_through: None,
        };
        for (part, state) in parts {
            match state {
                MatterInFront::Opaque => drawn.opaque = part,
                MatterInFront::SeeThrough => drawn.see_through = Some(part.into_owned()),
                MatterInFront::Hidden => {}
            }
        }
        drawn
    }

    /// The halves of `body` asked for, each cut only the first time it is.
    fn halves(
        &self,
        cut: Cut,
        body: &Body,
        behind: bool,
        in_front: bool,
    ) -> (Option<Body>, Option<Body>) {
        let mut last = self.last.borrow_mut();
        // Compared by value: the document keeps no revision of its body, and
        // reading two bodies through costs a fraction of cutting one.
        let mut halves = match last.take() {
            Some(halves) if halves.cut == cut && halves.body == *body => halves,
            _ => Halves {
                cut,
                body: body.clone(),
                behind: None,
                in_front: None,
            },
        };
        let behind = behind.then(|| {
            halves
                .behind
                .get_or_insert_with(|| {
                    #[cfg(test)]
                    self.behind_cuts.set(self.behind_cuts.get() + 1);
                    body.behind(cut.towards, cut.offset)
                })
                .clone()
        });
        let in_front = in_front.then(|| {
            halves
                .in_front
                .get_or_insert_with(|| {
                    #[cfg(test)]
                    self.front_cuts.set(self.front_cuts.get() + 1);
                    body.in_front(cut.towards, cut.offset)
                })
                .clone()
        });
        *last = Some(halves);
        (behind, in_front)
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
