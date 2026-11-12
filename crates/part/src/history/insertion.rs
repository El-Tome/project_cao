//! A step put in among the steps of the part, right after the key step it is
//! read through, rather than after the last.
//!
//! The operation opening it is numbered last, as everything recorded is, so
//! undo takes it back first: only its place among the steps is not the end.

use super::{History, Operation, StepKind};

impl History {
    /// Records an operation opening a step of that kind, the step put in right
    /// after the `key_step`th key step in effect — at the end past the last.
    /// What was undone is dropped first, as for anything recorded.
    pub(crate) fn insert_after(&mut self, key_step: usize, kind: StepKind, operation: Operation) {
        self.drop_what_was_undone();
        let at = self
            .place_of_key_step(key_step)
            .map_or(self.steps.len(), |read| read + 1);
        let mut step = self.open(kind);
        step.record(self.number(operation));
        self.steps.insert(at, step);
    }

    /// The numbers of the operations that opened the sketches, in the order
    /// they were made: a sketch's rank here is what every operation names it
    /// by.
    pub(crate) fn sketches_made(&self) -> Vec<u32> {
        StepKind::Sketch
            .in_the_order_made(&self.steps)
            .into_iter()
            .map(|(number, _)| number)
            .collect()
    }
}
