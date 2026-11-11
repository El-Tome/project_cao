//! The sections kept in a part: planes it is looked through, found again on the
//! part as it stands, and which drawings can be laid on.
//!
//! A drawing laid on one, `Operation::CreateSketchOnSection`, follows its cut,
//! and keeps the cut as it was when laid to fall back on when the section cuts
//! nothing.

use cao_sketch::{Sketch, WorkPlane};
use glam::DVec3;
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
    pub tilt_about_u_degrees: f64,
    pub tilt_about_v_degrees: f64,
    /// The point its plane is tilted about, where it was kept on that plane.
    pub pivot: DVec3,
    /// Whether the face it was laid on is gone, which leaves it cutting nothing.
    pub broken: bool,
}

impl SavedSection {
    /// The plane the part is cut along, when the section still stands on
    /// something: the one it starts from, tilted, moved along its own normal.
    pub fn cutting_plane(&self) -> Option<WorkPlane> {
        let tilts = (self.tilt_about_u_degrees, self.tilt_about_v_degrees);
        let tilted = self.plane.tilted(tilts, self.pivot);
        (!self.broken).then(|| WorkPlane {
            origin: tilted.origin + tilted.normal() * self.offset,
            ..tilted
        })
    }
}

/// The point a section's plane is tilted about, chosen with the plane: the
/// middle of the part, brought onto `plane` when that plane is a face's.
pub fn pivot_of(plane: WorkPlane, on_a_face: bool, middle: DVec3) -> DVec3 {
    let normal = plane.normal();
    match on_a_face {
        true => middle - normal * normal.dot(middle - plane.origin),
        false => middle,
    }
}

/// A pivot as a section keeps it: along `plane`'s `u`, its `v` and its normal,
/// so that it follows a face the plane is taken off.
pub fn pivot_kept_on(plane: WorkPlane, pivot: DVec3) -> DVec3 {
    let along = plane.to_local(pivot);
    along.extend(plane.normal().dot(pivot - plane.origin))
}

fn pivot_found_on(plane: WorkPlane, kept: DVec3) -> DVec3 {
    plane.to_world(kept.truncate()) + plane.normal() * kept.z
}

/// A section as the history leaves it: what it was last asked to be, its face
/// not yet found. The face is looked for where the section is used — on the
/// part as it stands for the tree, on the part a drawing is laid on for that
/// drawing — since the section's own step replays before what was raised after
/// it.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub(crate) struct Kept {
    asked: SectionAsked,
    /// Gone from the tree and changed no more, but the drawings laid on it
    /// still stand where it puts them.
    deleted: bool,
}

impl PartState {
    /// Keeps, changes or drops a section, or lays a drawing on one. None of it
    /// touches the matter.
    pub(crate) fn apply_to_sections(&mut self, operation: &Operation) {
        match operation {
            Operation::SaveSection(asked) => self.sections.push(Kept {
                asked: *asked,
                deleted: false,
            }),
            Operation::EditSection { section, to } => {
                if let Some(kept) = self.sections.get_mut(*section).filter(|kept| !kept.deleted) {
                    kept.asked = *to;
                }
            }
            Operation::DeleteSection { section } => {
                if let Some(kept) = self.sections.get_mut(*section) {
                    kept.deleted = true;
                }
            }
            Operation::CreateSketchOnSection { section, plane } => {
                // The part shown lays it where it is in the part, whatever its
                // section cuts of the part shown.
                let cut = self
                    .plane_in_the_part(self.sketch_being_made())
                    .or_else(|| {
                        self.sections
                            .get(*section)
                            .and_then(|kept| self.found(*section, kept).cutting_plane())
                            .map(|cut| facing_as(cut, plane))
                    });
                if cut.is_none() {
                    self.adrift.insert(self.sketch_being_made());
                }
                self.lay_sketch(Sketch::new(cut.unwrap_or(*plane)));
            }
            _ => {}
        }
    }

    /// The sections still kept, each found on the part as it stands.
    pub(crate) fn sections_standing(&self) -> Vec<SavedSection> {
        self.sections
            .iter()
            .enumerate()
            .filter(|(_, kept)| !kept.deleted)
            .map(|(id, kept)| self.found(id, kept))
            .collect()
    }

    fn found(&self, id: usize, kept: &Kept) -> SavedSection {
        let asked = &kept.asked;
        let face = asked.on.as_ref().map(|anchor| self.face_now(anchor));
        let plane = face.flatten().unwrap_or(asked.plane);
        SavedSection {
            id,
            plane,
            on: asked.on,
            offset: asked.offset,
            behind: asked.behind,
            in_front: asked.in_front,
            tilt_about_u_degrees: asked.tilt_about_u_degrees,
            tilt_about_v_degrees: asked.tilt_about_v_degrees,
            pivot: pivot_found_on(plane, asked.pivot),
            broken: matches!(face, Some(None)),
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
