//! The matter of a part as its steps made it: which faces each one raised or
//! cut, for what names a step to show it where it stands.

use cao_solid::Declined;

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

    /// Whether the kernel declined the step of matter of that number: the
    /// part stands as it did before it, and the step is broken.
    pub fn is_declined(&self, step: u32) -> bool {
        self.state.declined.contains_key(&step)
    }

    /// Why the kernel declined the step of matter of that number, for a
    /// developer to read: nothing on screen says it.
    pub fn declined_because(&self, step: u32) -> Option<Declined> {
        self.state.declined.get(&step).copied()
    }
}
