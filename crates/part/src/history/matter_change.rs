//! A step of matter changed after it was raised: what it is replayed as.

use std::collections::BTreeMap;

use super::{History, Operation};

impl History {
    /// The latest change in effect of each step of matter changed since it
    /// was raised, by the number of the operation that raised it. A change
    /// that would turn the step into another shape, which a file can carry,
    /// is no change of it.
    pub fn changes_in_effect(&self) -> BTreeMap<u32, &Operation> {
        self.applied_operations()
            .iter()
            .filter_map(|operation| match operation {
                Operation::EditMatter { raised_by, to } => Some((*raised_by, &**to)),
                _ => None,
            })
            .filter(|(raised_by, to)| self.stands_for(*raised_by, to))
            .collect()
    }

    /// Whether an operation is one to record: anything but a change naming no
    /// step of matter in effect, or turning it into another shape.
    pub(super) fn takes(&self, operation: &Operation) -> bool {
        let Operation::EditMatter { raised_by, to } = operation else {
            return true;
        };
        let in_effect = self
            .position_of(*raised_by)
            .is_some_and(|at| at < self.applied);
        in_effect && self.stands_for(*raised_by, to)
    }

    /// Whether `to` can stand for the operation of that number: a step of
    /// matter, raised the same way.
    fn stands_for(&self, raised_by: u32, to: &Operation) -> bool {
        matches!(
            (self.at(raised_by), to),
            (Some(Operation::Extrude { .. }), Operation::Extrude { .. })
                | (Some(Operation::Revolve { .. }), Operation::Revolve { .. })
        )
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
