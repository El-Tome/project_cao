//! A fillet or a chamfer changed in place after it was laid: its values, the
//! side a chamfer measures from, whether it is construction, or the cut taken
//! off.

use cao_sketch::{CutShape, LengthOutcome, Sketch};

use crate::broken::Broken;
use crate::cutting::in_units;
use crate::dimensioning::DimensionOutcome;
use crate::history::{CutAsked, CutChange};
use crate::outcome::Outcome;
use crate::state::PartState;

impl PartState {
    /// Applies one change to a cut. A cut no longer there to change — taken
    /// off since, or its corner gone — breaks the step, as a value naming a
    /// trait that is gone does.
    pub(crate) fn change_cut(&mut self, change: &CutChange) -> Option<Outcome> {
        let before = self.before_the_cuts_move(change.sketch());
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
        self.settle_what_hangs_on_cuts(change.sketch(), before)
    }

    /// The drawing as it stands before its cuts move, when something hangs
    /// on them: what a change they cannot follow goes back to.
    pub(crate) fn before_the_cuts_move(&self, sketch: usize) -> Option<Sketch> {
        self.sketches
            .get(sketch)
            .filter(|drawing| drawing.anything_hangs_on_a_cut())
            .cloned()
    }

    /// Settles what hangs on the cuts of a drawing, after they moved. A
    /// drawing nothing hangs on is left exactly as the cuts left it. One where
    /// what hangs cannot follow goes back to `before`, refused as a value
    /// that cannot be held is.
    pub(crate) fn settle_what_hangs_on_cuts(
        &mut self,
        sketch: usize,
        before: Option<Sketch>,
    ) -> Option<Outcome> {
        let before = before?;
        let scale = self.scale();
        let drawing = self.sketches.get_mut(sketch)?;
        if drawing.resolve(scale) == LengthOutcome::Exact {
            return None;
        }
        *drawing = before;
        Some(Outcome::Dimension(DimensionOutcome::Geometry(
            LengthOutcome::BestEffort,
        )))
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
