//! A step of matter changed after it was raised: what it is replayed as.

use std::collections::BTreeMap;

use super::{History, Operation};

impl History {
    /// The latest change in effect of each step of matter changed since it
    /// was raised, by the number of the operation that raised it.
    pub(crate) fn changes_in_effect(&self) -> BTreeMap<u32, &Operation> {
        self.applied_operations()
            .iter()
            .filter_map(|operation| match operation {
                Operation::EditMatter { raised_by, to } => Some((*raised_by, &**to)),
                _ => None,
            })
            .collect()
    }

    /// The operations in effect, in the order they were done, each step of
    /// matter as it now stands: what compaction writes the part down from.
    pub(crate) fn applied_as_raised(&self) -> Vec<Operation> {
        let changed = self.changes_in_effect();
        self.numbers[..self.applied]
            .iter()
            .zip(self.applied_operations())
            .map(|(number, operation)| changed.get(number).copied().unwrap_or(operation))
            .cloned()
            .collect()
    }

    /// The operation of that number, as the step it opened is replayed now:
    /// its latest change in effect, or itself. Nothing while it is undone.
    pub fn raised_as(&self, number: u32) -> Option<&Operation> {
        let at = self.position_of(number).filter(|at| *at < self.applied)?;
        let changed = self.changes_in_effect().remove(&number);
        Some(changed.unwrap_or(&self.operations[at]))
    }
}
