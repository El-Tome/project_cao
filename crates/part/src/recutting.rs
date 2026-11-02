//! A fillet or a chamfer changed in place after it was laid: its values, the
//! side a chamfer measures from, whether it is construction, or the cut taken
//! off.

use cao_sketch::CutShape;

use crate::broken::Broken;
use crate::cutting::in_units;
use crate::history::{CutAsked, CutChange};
use crate::outcome::Outcome;
use crate::state::PartState;

impl PartState {
    /// Applies one change to a cut. A cut no longer there to change — taken
    /// off since, or its corner gone — breaks the step, as a value naming a
    /// trait that is gone does.
    pub(crate) fn change_cut(&mut self, change: &CutChange) -> Option<Outcome> {
        let changed = match change {
            CutChange::Reshape { sketch, cut, value } => {
                let shape = self.shape_asked(value)?;
                let drawing = self.sketches.get_mut(*sketch)?;
                let reshaped = drawing.reshape_cuts(*cut, shape);
                let group = drawing.cuts_grouped_with(*cut);
                drawing.write_cuts_as(&group, value.written());
                reshaped
            }
            CutChange::TakeOff { sketch, cuts } => {
                let drawing = self.sketches.get_mut(*sketch)?;
                let mut taken = false;
                for cut in cuts {
                    taken |= drawing.take_cut_off(*cut);
                }
                taken
            }
            CutChange::SwapSides { sketch, cut } => {
                self.sketches.get_mut(*sketch)?.swap_cut_sides(*cut)
            }
            CutChange::Construction {
                sketch,
                cut,
                construction,
            } => self
                .sketches
                .get_mut(*sketch)?
                .set_cut_construction(*cut, *construction),
        };
        if !changed {
            self.broke(Broken::Operation(self.replaying));
        }
        None
    }

    /// The shape the values asked come to, in the drawing's units — nothing,
    /// and the step broken, when one of them comes to no number.
    fn shape_asked(&mut self, value: &CutAsked) -> Option<CutShape> {
        let scale = self.scale();
        match value {
            CutAsked::Fillet(radius) => {
                let radius = self.size(radius, |_| true)?;
                Some(CutShape::Fillet {
                    radius: radius / scale,
                })
            }
            CutAsked::Chamfer(mode) => {
                let Some(chamfer) = mode.worked_out(&self.values) else {
                    self.broke(Broken::Operation(self.replaying));
                    return None;
                };
                Some(CutShape::Chamfer(in_units(chamfer, scale)))
            }
        }
    }
}
