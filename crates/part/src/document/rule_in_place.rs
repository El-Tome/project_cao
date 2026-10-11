//! A rule laid where a dimension stood, as one step.

use cao_sketch::{DimensionTarget, InPlace, LengthOutcome};

use super::PartDocument;
use crate::history::Operation;

impl PartDocument {
    /// Erases the dimension on `target`, and whatever goes with it, and lays
    /// the rule in its place — one step, which one undo takes back whole.
    ///
    /// The erase comes first: laid beside the dimension it contradicts, the
    /// rule would be refused. A rule that cannot hold is refused like any
    /// other, and then nothing is recorded and the dimension stays: the two are
    /// tried on a copy before either is applied, since a refused rule leaves the
    /// history and would leave the erase standing in it alone.
    pub fn lay_in_place_of(
        &mut self,
        sketch: usize,
        target: DimensionTarget,
        in_place: InPlace,
    ) -> bool {
        let Some(drawn) = self.sketches().get(sketch) else {
            return false;
        };
        let scale = self.scale();
        let mut trial = drawn.clone();
        trial.erase_dimension(target);
        for element in &in_place.erased {
            trial.erase(*element);
        }
        trial.resolve(scale);
        if trial.lay_rule(in_place.rule, scale) == LengthOutcome::BestEffort {
            return false;
        }

        let opened = self.history.mark();
        self.apply(Operation::EraseMany {
            sketch,
            elements: in_place.erased,
            dimensions: vec![target],
            constraints: Vec::new(),
        });
        let laid = self.apply(Operation::Constrain {
            sketch,
            constraint: in_place.rule,
        });
        if laid == Some(crate::outcome::Outcome::RuleRefused) {
            self.undo();
            self.history.drop_what_was_undone();
            return false;
        }
        self.history.fold_into_one_gesture(opened);
        true
    }
}

#[cfg(test)]
mod tests;
