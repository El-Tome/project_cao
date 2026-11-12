//! What a drawing is laid on, and whether the part still has it.
//!
//! A drawing on a plane of the origin is held by nothing and never moves. One
//! laid on a face of the part travels with that face: the design records which
//! face, and the plane is worked out again at every replay from the part as it
//! then stands.

use cao_sketch::WorkPlane;

use crate::history::FaceAnchor;
use crate::state::PartState;

impl PartState {
    /// Where a drawing about to be laid down actually sits.
    ///
    /// `recorded` is the plane it was drawn on the day it was made, kept as
    /// what to fall back on. A face that the part no longer has leaves the
    /// drawing there and marks it: catching the nearest face instead would
    /// move a drawing to somewhere nobody pointed at. The part shown with its
    /// volumes' states lays the drawing on the plane it has in the part,
    /// whatever the part shown makes of its face.
    pub(crate) fn plane_for(&mut self, recorded: WorkPlane, on: &Option<FaceAnchor>) -> WorkPlane {
        let Some(anchor) = on else {
            return recorded;
        };
        if let Some(plane) = self.plane_in_the_part(self.sketch_being_made()) {
            return plane;
        }
        if let Some(plane) = self.face_now(anchor, self.replaying) {
            return plane;
        }
        self.adrift.insert(self.sketch_being_made());
        recorded
    }

    /// The face of the part as it stands that the operation of number
    /// `recorded_by` named `face`.
    ///
    /// A face's number is handed out as the steps of matter replay, each
    /// numbering its own from where the one before it ended, so a step put in
    /// before a step of matter moves every number that step gives. The face was
    /// named against the steps made before the operation naming it, in the
    /// order of the part: found again among those, it is the face standing at
    /// the same place of the same step now. With no step put in, that is the
    /// number as it was named.
    fn face_named_by(&self, face: usize, recorded_by: u32) -> usize {
        let mut then = self.made.first().map_or(0, |(_, range)| range.start);
        for (number, now) in &self.made {
            if *number >= recorded_by {
                continue;
            }
            if face < then + now.len() {
                return now.start + face.saturating_sub(then);
            }
            then += now.len();
        }
        face
    }

    /// The plane the anchored face offers as the part stands now, the face
    /// found again as the operation of number `recorded_by` named it.
    pub(crate) fn face_now(&self, anchor: &FaceAnchor, recorded_by: u32) -> Option<WorkPlane> {
        let face = self
            .body
            .plane_of(self.face_named_by(anchor.face, recorded_by))?;
        Some(WorkPlane::from_face(&face.corners, face.normal, anchor.up))
    }
}
