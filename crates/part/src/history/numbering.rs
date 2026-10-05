//! The numbers operations are given: one each, in the order they are
//! recorded, never handed out twice.

use super::{History, Operation};

impl History {
    /// The number the next operation recorded will be given, which is what
    /// names it while it is applied, before it is recorded.
    pub(crate) fn next_number(&self) -> u32 {
        self.last_operation_number + 1
    }

    /// Puts an operation at the end of the list under the next number, and
    /// says which.
    pub(super) fn number(&mut self, operation: Operation) -> u32 {
        self.last_operation_number += 1;
        let number = self.last_operation_number;
        self.operations.push(operation);
        self.numbers.push(number);
        self.applied = self.operations.len();
        number
    }
}
