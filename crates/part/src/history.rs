//! Everything done to a part, in order, and the cursor that says how much
//! of it is in effect.

use serde::{Deserialize, Serialize};

mod operation;
mod step;
pub use operation::{ExtrusionMode, FaceAnchor, Operation, PointRef, RevolutionAxis};
pub use step::{Step, StepKind};

/// Everything done to a part, in order, with a cursor separating what is
/// applied from what can be redone.
///
/// The part's geometry is not stored: it is rebuilt by replaying this list. So
/// undo, redo and "go back to this step" are all the same operation — moving
/// the cursor — and the redo tail survives being saved and reopened.
///
/// The list is also grouped into the major steps of the design — a sketch and
/// what was drawn on it, an extrusion — and that grouping is recorded rather
/// than worked out from the list: it is what the file is laid out by, one
/// folder per step, and what an edit of a past step will hang on.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct History {
    operations: Vec<Operation>,
    steps: Vec<Step>,
    applied: usize,
    last_operation_number: u32,
}

/// A design with its operations taken out: the ordered index of the steps,
/// the cursor, and the numbers already handed out.
///
/// This is what `design/history.json` holds, and it is enough on its own to
/// say what the part is made of and in what order — the operations of a step
/// are read from that step's folder, and only when they are wanted.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Index {
    pub steps: Vec<Step>,
    pub applied: usize,
    pub last_operation_number: u32,
}

impl History {
    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }

    /// The major steps of the design, in the order they were made.
    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    /// The operations recorded under one step, as the stretch of the list
    /// they occupy.
    pub(crate) fn operations_of(&self, step: usize) -> &[Operation] {
        let start: usize = self.steps[..step].iter().map(Step::len).sum();
        &self.operations[start..start + self.steps[step].len()]
    }

    /// A history with nothing in it, going on handing out the numbers this
    /// one left off at.
    ///
    /// What compaction writes: it rewrites the steps, and a number that named
    /// one of them must not come back naming another.
    pub(crate) fn following(&self) -> Self {
        Self {
            last_operation_number: self.last_operation_number,
            ..Self::default()
        }
    }

    /// The index alone, as it goes into the file.
    pub(crate) fn index(&self) -> Index {
        Index {
            steps: self.steps.clone(),
            applied: self.applied,
            last_operation_number: self.last_operation_number,
        }
    }

    /// Puts a history back together from its index and the operations of every
    /// step, run together in the order the index names them.
    ///
    /// Returns nothing when the two do not answer to each other — a step
    /// whose folder holds another number of operations than the index says,
    /// or a cursor past the end. The part then fails to open rather than
    /// opening on a design nobody can vouch for.
    pub(crate) fn restore(index: Index, operations: Vec<Operation>) -> Option<Self> {
        let counted: usize = index.steps.iter().map(Step::len).sum();
        if counted != operations.len() || index.applied > operations.len() {
            return None;
        }
        Some(Self {
            operations,
            steps: index.steps,
            applied: index.applied,
            last_operation_number: index.last_operation_number,
        })
    }

    /// How many operations are currently in effect.
    pub fn applied(&self) -> usize {
        self.applied
    }

    pub fn applied_operations(&self) -> &[Operation] {
        &self.operations[..self.applied]
    }

    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    pub fn can_undo(&self) -> bool {
        self.applied > 0
    }

    pub fn can_redo(&self) -> bool {
        self.applied < self.operations.len()
    }

    /// Records a new operation. Anything that had been undone is dropped: the
    /// part has taken a different turn, and keeping the old branch would leave
    /// a redo that no longer follows from what is on screen.
    pub fn push(&mut self, operation: Operation) {
        self.drop_what_was_undone();
        let opens = StepKind::opened_by(&operation);
        // An operation that opens no step names the sketch it belongs to, and
        // that sketch is the step still open. With none open it names a sketch
        // the part does not have: `PartState` makes nothing of it either, so
        // recording it would leave a step in the file with nothing to say what
        // it stands on. Nothing happened, and the two stay in step.
        if opens.is_none() && self.steps.is_empty() {
            return;
        }
        if let Some(kind) = opens {
            self.steps.push(Step::opened(kind));
        }
        self.last_operation_number += 1;
        let number = self.last_operation_number;
        if let Some(step) = self.steps.last_mut() {
            step.record(number);
        }
        self.operations.push(operation);
        self.applied = self.operations.len();
    }

    fn drop_what_was_undone(&mut self) {
        if self.applied == self.operations.len() {
            return;
        }
        self.operations.truncate(self.applied);
        let mut left = self.applied;
        self.steps.retain_mut(|step| {
            let kept = step.len().min(left);
            left -= kept;
            step.keep(kept);
            kept > 0
        });
    }

    pub fn undo(&mut self) -> bool {
        if !self.can_undo() {
            return false;
        }
        self.applied -= 1;
        true
    }

    pub fn redo(&mut self) -> bool {
        if !self.can_redo() {
            return false;
        }
        self.applied += 1;
        true
    }

    /// Moves the cursor anywhere in the list, which is how the history tree
    /// puts the part back the way it was at a given step.
    pub fn rewind_to(&mut self, applied: usize) {
        self.applied = applied.min(self.operations.len());
    }
}

#[cfg(test)]
mod tests;
