//! The sections kept in a part: planes it is looked through, found again on the
//! part as it stands at every replay, and which drawings can be laid on.

use cao_sketch::{Sketch, WorkPlane};
use serde::{Deserialize, Serialize};

use crate::history::{FaceAnchor, Operation, SectionAsked, SideDrawing};
use crate::state::PartState;

/// A section as the part holds it now.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedSection {
    /// Its rank among the sections ever made, which deleting one leaves alone.
    pub id: usize,
    /// The plane it starts from: the face's as the part stands now, or the one
    /// it was made on when that face is gone.
    pub plane: WorkPlane,
    pub on: Option<FaceAnchor>,
    pub offset: f64,
    pub behind: SideDrawing,
    pub in_front: SideDrawing,
    /// Whether the face it was laid on is gone, which leaves it cutting nothing.
    pub broken: bool,
}

impl SavedSection {
    /// The plane the part is cut along, when the section still stands on
    /// something.
    pub fn cutting_plane(&self) -> Option<WorkPlane> {
        (!self.broken).then(|| WorkPlane {
            origin: self.plane.origin + self.plane.normal() * self.offset,
            ..self.plane
        })
    }
}

impl PartState {
    /// Keeps, changes or drops a section, or lays a drawing on one. None of it
    /// touches the matter.
    pub(crate) fn apply_to_sections(&mut self, operation: &Operation) {
        match operation {
            Operation::SaveSection(asked) => {
                let kept = self.section_from(self.sections.len(), asked);
                self.sections.push(Some(kept));
            }
            Operation::EditSection { section, to } => {
                let kept = self.section_from(*section, to);
                if let Some(slot @ Some(_)) = self.sections.get_mut(*section) {
                    *slot = Some(kept);
                }
            }
            Operation::DeleteSection { section } => {
                if let Some(slot) = self.sections.get_mut(*section) {
                    *slot = None;
                }
            }
            Operation::CreateSketchOnSection { section, plane } => {
                let cut = self
                    .sections
                    .get(*section)
                    .copied()
                    .flatten()
                    .and_then(|kept| kept.cutting_plane())
                    .map(|cut| facing_as(cut, plane));
                if cut.is_none() {
                    self.adrift.insert(self.sketches.len());
                }
                self.sketches.push(Sketch::new(cut.unwrap_or(*plane)));
            }
            _ => {}
        }
    }

    fn section_from(&self, id: usize, asked: &SectionAsked) -> SavedSection {
        let found = asked.on.as_ref().map(|anchor| self.face_now(anchor));
        SavedSection {
            id,
            plane: found.flatten().unwrap_or(asked.plane),
            on: asked.on,
            offset: asked.offset,
            behind: asked.behind,
            in_front: asked.in_front,
            broken: matches!(found, Some(None)),
        }
    }
}

/// The cut turned to face the way the drawing was started from, which is the
/// side it was seen from when it was laid.
fn facing_as(cut: WorkPlane, laid: &WorkPlane) -> WorkPlane {
    match cut.normal().dot(laid.normal()) < 0.0 {
        true => WorkPlane::from_normal(cut.origin, -cut.normal()),
        false => cut,
    }
}
