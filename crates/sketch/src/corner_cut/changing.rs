//! A cut changed in place: its value, which side a chamfer measures from,
//! whether it is drawn as construction, or the cut taken off. Each answers
//! whether there was a cut standing to change.

use super::{CornerCutId, CutShape};
use crate::chamfer::Chamfer;
use crate::erased::Erased;
use crate::sketch::Sketch;

impl Sketch {
    /// Gives every cut laid with this one a new shape: the corners a gesture
    /// rounded together change together.
    pub fn reshape_cuts(&mut self, cut: CornerCutId, shape: CutShape) -> bool {
        let group = self.cuts_grouped_with(cut);
        for id in &group {
            self.corner_cuts[id.0].shape = shape;
            self.keep_what_the_shape_offers(*id);
        }
        !group.is_empty()
    }

    /// Notes how the values of these cuts were written — a number or a
    /// formula of the part's variables — for the part to read back when they
    /// are opened again, as a dimension keeps its own. The drawing never
    /// reads it.
    pub fn write_cuts_as(&mut self, cuts: &[CornerCutId], written: [Option<String>; 2]) {
        for cut in cuts {
            if self.cuts_written.len() <= cut.0 {
                self.cuts_written.resize(cut.0 + 1, [None, None]);
            }
            self.cuts_written[cut.0] = written.clone();
        }
    }

    /// How the values of a cut were written, as noted.
    pub fn cut_written(&self, cut: CornerCutId) -> [Option<String>; 2] {
        self.cuts_written.get(cut.0).cloned().unwrap_or_default()
    }

    /// Takes a cut off its corner, which is sharp again. The cuts laid with
    /// it stay, and so does what hung on it, where it is.
    pub fn take_cut_off(&mut self, cut: CornerCutId) -> bool {
        if !self.stands_live(cut) {
            return false;
        }
        self.let_go_of_the_cut(cut);
        Erased::mark(&mut self.erased.cuts, cut.0);
        true
    }

    /// Swaps the side a chamfer measures from: the two distances trade
    /// sides, and a distance and an angle are read from the other side. A
    /// fillet and an equal chamfer read both sides alike, and are left as
    /// they are.
    pub fn swap_cut_sides(&mut self, cut: CornerCutId) -> bool {
        if !self.stands_live(cut) {
            return false;
        }
        let laid = &mut self.corner_cuts[cut.0];
        match laid.shape {
            CutShape::Chamfer(Chamfer::Sided { .. } | Chamfer::Angled { .. }) => {
                laid.sides.reverse();
                self.follow_the_sides_swapped(cut);
                true
            }
            CutShape::Fillet { .. } | CutShape::Chamfer(Chamfer::Equal(_)) => false,
        }
    }

    /// Draws one cut as construction, or as the outline. The cuts laid with
    /// it, and its sides, are left as they are.
    pub fn set_cut_construction(&mut self, cut: CornerCutId, construction: bool) -> bool {
        if !self.stands_live(cut) {
            return false;
        }
        self.corner_cuts[cut.0].construction = construction;
        true
    }

    fn stands_live(&self, cut: CornerCutId) -> bool {
        self.live_corner_cuts().any(|(id, _)| id == cut)
    }
}

#[cfg(test)]
mod tests;
