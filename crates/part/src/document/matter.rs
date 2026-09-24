//! The matter of a part as its steps made it: which faces each one raised or
//! cut, for what names a step to show it where it stands.

use super::PartDocument;
use crate::history::Operation;

impl PartDocument {
    /// The faces of the part a step of matter made, by the number of its
    /// operation — nothing for any other step, and nothing when a geometry
    /// cached before the faces were noted leaves them unknown: better none
    /// than another step's.
    pub fn faces_made_by(&self, step: u32) -> Vec<usize> {
        let steps: Vec<u32> = self
            .history
            .replay_order()
            .into_iter()
            .filter(|(_, operation)| {
                matches!(
                    operation,
                    Operation::Extrude { .. } | Operation::Revolve { .. }
                )
            })
            .map(|(number, _)| number)
            .collect();
        if steps.len() != self.state.made.len() {
            return Vec::new();
        }
        steps
            .iter()
            .position(|number| *number == step)
            .map_or_else(Vec::new, |rank| self.state.faces_made(rank))
    }
}
