//! The changes made to the part's variables, as the history keeps them: under
//! no step, since the variables are a table of the whole part.

use super::{History, Operation};
use crate::variables::VariableChange;

impl History {
    /// Every change made to the part's variables, undone ones included, in
    /// the order they were made — what the part file keeps of the table.
    pub(crate) fn table_operations(&self) -> Vec<Operation> {
        self.variables
            .iter()
            .filter_map(|number| self.at(*number).cloned())
            .collect()
    }

    /// The changes to the variables in effect that were made before the
    /// operation of that number: the table as it stood then.
    pub(crate) fn variable_changes_before(&self, number: u32) -> Vec<&VariableChange> {
        self.numbers[..self.applied]
            .iter()
            .zip(&self.operations)
            .filter(|(given, _)| **given < number)
            .filter_map(|(_, operation)| match operation {
                Operation::Variable(change) => Some(change),
                _ => None,
            })
            .collect()
    }

    /// The changes to the variables in effect, in the order they were made:
    /// what the table is played from before any step is.
    pub fn variable_changes(&self) -> Vec<&VariableChange> {
        self.operations[..self.applied]
            .iter()
            .filter_map(|operation| match operation {
                Operation::Variable(change) => Some(change),
                _ => None,
            })
            .collect()
    }
}
