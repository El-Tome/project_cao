//! An edge of the part chamfered or rounded by the kernel, at the size asked
//! or at the most its two faces give.

use cao_solid::EdgeName;
use serde::{Deserialize, Serialize};

use crate::formula::Formula;
use crate::history::BlendShape;
use crate::outcome::Outcome;
use crate::state::PartState;

/// Closer than this to the size asked, the size taken is the same: the rest
/// is rounding.
const ONE_PLACE: f64 = 1e-9;

/// A chamfer or a fillet the faces of its edge could not give the size it
/// asked, laid at the most they give. In millimetres.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reduced {
    pub asked: f64,
    pub taken: f64,
}

impl PartState {
    /// Chamfers or rounds an edge, noting the face it lays as a step of matter
    /// notes its own, so that the faces after it are found again by their
    /// numbers whatever becomes of it.
    pub(crate) fn blend(
        &mut self,
        edge: &EdgeName,
        shape: BlendShape,
        size: &Formula,
    ) -> Option<Outcome> {
        let mut outcome = None;
        self.raising(|state| outcome = state.blended(edge, shape, size));
        outcome
    }

    fn blended(&mut self, edge: &EdgeName, shape: BlendShape, size: &Formula) -> Option<Outcome> {
        let Some(size) = self.size(size, |size| size >= 1e-6) else {
            self.body.count_past_cut(1);
            return None;
        };
        let edge = self.edge_named_by(edge, self.replaying);
        let scale = self.scale();
        let blended = match shape {
            BlendShape::Chamfer => self
                .body
                .chamfer(&edge, size / scale)
                .map(|cut| (cut.body, cut.taken)),
            BlendShape::Fillet => self
                .body
                .fillet(&edge, size / scale)
                .map(|round| (round.body, round.taken)),
        };
        match blended {
            Ok((body, taken)) => {
                self.body = body;
                let taken = taken * scale;
                if size - taken > ONE_PLACE * size.max(1.0) {
                    self.reduced
                        .insert(self.replaying, Reduced { asked: size, taken });
                }
                None
            }
            Err(reason) => {
                self.declined_with(reason, |body| body.count_past_cut(1));
                Some(Outcome::BlendDeclined(reason))
            }
        }
    }

    /// The edge the operation of number `recorded_by` named, its faces found
    /// again as the part now numbers them, as a drawing's face is.
    fn edge_named_by(&self, edge: &EdgeName, recorded_by: u32) -> EdgeName {
        let side = |numbers: &Vec<u32>| -> Vec<u32> {
            numbers
                .iter()
                .map(|&number| self.face_named_by(number as usize, recorded_by) as u32)
                .collect()
        };
        EdgeName {
            between: edge.between.each_ref().map(side),
            ends: edge
                .ends
                .each_ref()
                .map(|faces| faces.iter().map(side).collect()),
            ..edge.clone()
        }
    }
}
