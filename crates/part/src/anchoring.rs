//! What a drawing is laid on, and whether the part still has it.
//!
//! A drawing on a plane of the origin is held by nothing and never moves. One
//! laid on a face of the part travels with that face: the design records which
//! face, and the plane is worked out again at every replay from the part as it
//! then stands.

use cao_sketch::WorkPlane;
use glam::DVec3;

use crate::history::FaceAnchor;
use crate::state::PartState;

impl PartState {
    /// Where a drawing about to be laid down actually sits.
    ///
    /// `recorded` is the plane it was drawn on the day it was made, kept as
    /// what to fall back on. A face that the part no longer has leaves the
    /// drawing there and marks it: catching the nearest face instead would
    /// move a drawing to somewhere nobody pointed at.
    pub(crate) fn plane_for(&mut self, recorded: WorkPlane, on: &Option<FaceAnchor>) -> WorkPlane {
        let Some(anchor) = on else {
            return recorded;
        };
        match self.face_now(anchor) {
            Some(plane) => plane,
            None => {
                self.adrift.insert(self.sketches.len());
                recorded
            }
        }
    }

    /// The plane the anchored face offers as the part stands now.
    fn face_now(&self, anchor: &FaceAnchor) -> Option<WorkPlane> {
        let normal = self.body.pieces_of(anchor.face).next()?.normal();
        let corners: Vec<DVec3> = self
            .body
            .pieces_of(anchor.face)
            .flat_map(|piece| piece.corners.iter().copied())
            .collect();
        Some(WorkPlane::from_face(&corners, normal, anchor.up))
    }
}
