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
    pub(crate) fn blend(
        &mut self,
        edge: &EdgeName,
        shape: BlendShape,
        size: &Formula,
    ) -> Option<Outcome> {
        let size = self.size(size, |size| size >= 1e-6)?;
        let scale = self.scale();
        let blended = match shape {
            BlendShape::Chamfer => self
                .body
                .chamfer(edge, size / scale)
                .map(|cut| (cut.body, cut.taken)),
            BlendShape::Fillet => self
                .body
                .fillet(edge, size / scale)
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
                self.declined_with(reason, |_| {});
                Some(Outcome::BlendDeclined(reason))
            }
        }
    }
}
