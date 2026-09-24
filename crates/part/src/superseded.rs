//! A value on a drawing taken away, or typed again, stops following the
//! variables from that moment: it is worked out against the table as it stood
//! then. Replayed against the table as it stands now, the shape it set before
//! it went would go on moving with a variable nothing on the drawing shows.
//!
//! A value a cut drops is not followed here: the drawing says how many it
//! dropped, not which, and such a value goes on following its variable.

use std::collections::HashMap;

use cao_sketch::DimensionTarget;

use crate::history::{History, Operation};
use crate::state::PartState;
use crate::variables::Variables;

impl PartState {
    /// Finds every value set on a drawing and later taken away or typed
    /// again, and what the variables came to at that moment.
    pub(crate) fn read_superseded(&mut self, history: &History) {
        let mut setters: HashMap<(usize, DimensionTarget), u32> = HashMap::new();
        let mut gone: HashMap<(u32, DimensionTarget), u32> = HashMap::new();
        for (number, operation) in history.replay_order() {
            walk(number, operation, &mut setters, &mut gone);
        }
        self.superseded = gone
            .into_iter()
            .map(|(set, at)| {
                let mut then = Variables::default();
                for change in history.variable_changes_before(at) {
                    then.change(change);
                }
                (set, then.values())
            })
            .collect();
    }

    /// What the variables come to for the value on this target the operation
    /// being replayed sets: as they stood when it was taken away, or as they
    /// stand now.
    pub(crate) fn values_for(&self, target: DimensionTarget) -> &[f64] {
        self.superseded
            .get(&(self.replaying, target))
            .map_or(&self.values, |then| then)
    }
}

fn walk(
    number: u32,
    operation: &Operation,
    setters: &mut HashMap<(usize, DimensionTarget), u32>,
    gone: &mut HashMap<(u32, DimensionTarget), u32>,
) {
    match operation {
        Operation::Gesture(done) => {
            for one in done {
                walk(number, one, setters, gone);
            }
        }
        Operation::SetDimension { sketch, target, .. } => {
            if let Some(before) = setters.insert((*sketch, *target), number) {
                gone.insert((before, *target), number);
            }
        }
        Operation::EraseMany {
            sketch, dimensions, ..
        } => {
            for target in dimensions {
                if let Some(before) = setters.remove(&(*sketch, *target)) {
                    gone.insert((before, *target), number);
                }
            }
        }
        _ => {}
    }
}
