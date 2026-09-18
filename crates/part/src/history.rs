//! Everything done to a part, in order, and the cursor that says how much
//! of it is in effect.

use std::collections::BTreeSet;

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
    /// The number each operation was given, in the same order. Kept beside the
    /// list rather than worked out from a position: undoing and pushing again
    /// leaves a gap in the numbers, and the steps name operations by number.
    numbers: Vec<u32>,
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

    /// The operations recorded under one step, in the order they were done.
    ///
    /// They are no longer a stretch of the list: an operation belongs to the
    /// step it edits, and one of those can be typed at any time.
    pub(crate) fn operations_of(&self, step: usize) -> Vec<Operation> {
        self.steps[step]
            .operations()
            .iter()
            .filter_map(|number| self.at(*number).cloned())
            .collect()
    }

    fn at(&self, number: u32) -> Option<&Operation> {
        let at = self.numbers.iter().position(|given| *given == number)?;
        self.operations.get(at)
    }

    /// Whether the operation just recorded landed in the step the design ends
    /// on.
    ///
    /// When it did not, it is replayed before everything raised after it, and
    /// applying it to the part as it stands would change the drawing and leave
    /// the matter behind.
    pub fn last_is_at_the_end(&self) -> bool {
        match (self.steps.last(), self.numbers.last()) {
            (Some(step), Some(newest)) => step.operations().last() == Some(newest),
            _ => true,
        }
    }

    /// The operations in the order they are replayed: each step whole, in the
    /// order the steps were made, rather than in the order things were typed.
    ///
    /// That is what makes an extrusion rebuild when the sketch it stands on is
    /// edited long afterwards. Undo walks the other order, which is the one
    /// the list is kept in.
    pub fn replay_order(&self) -> Vec<&Operation> {
        let in_effect: BTreeSet<u32> = self.numbers[..self.applied].iter().copied().collect();
        self.steps
            .iter()
            .flat_map(|step| step.operations())
            .filter(|number| in_effect.contains(number))
            .filter_map(|number| self.at(*number))
            .collect()
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
        // Each step's operations come back from its own folder, so what is
        // handed over is grouped by step. The list itself is kept in the order
        // things were done, which is what the numbers say and what undo walks.
        let mut taken = operations.into_iter();
        let mut numbered: Vec<(u32, Operation)> = Vec::with_capacity(counted);
        for step in &index.steps {
            for number in step.operations() {
                numbered.push((*number, taken.next()?));
            }
        }
        numbered.sort_by_key(|(number, _)| *number);
        let (numbers, operations) = numbered.into_iter().unzip();
        let mut history = Self {
            operations,
            numbers,
            steps: index.steps,
            applied: index.applied,
            last_operation_number: index.last_operation_number,
        };
        history.regroup();
        Some(history)
    }

    /// Puts every operation back under the step it edits.
    ///
    /// Which step an operation belongs to is worked out from the operation
    /// itself, so it is never read back wrong: a design written before that
    /// was decided comes back grouped the way it is replayed, rather than the
    /// way it happened to be filed.
    fn regroup(&mut self) {
        let kinds: Vec<StepKind> = self.steps.iter().map(Step::kind).collect();
        let mut steps: Vec<Step> = kinds.iter().map(|kind| Step::opened(*kind)).collect();
        let mut opened = 0;
        for (number, operation) in self.numbers.iter().zip(&self.operations) {
            let owner = match StepKind::opened_by(operation) {
                Some(_) => {
                    opened += 1;
                    opened - 1
                }
                None => {
                    let Some(step) = operation
                        .edits()
                        .and_then(|sketch| step_of(&kinds, sketch))
                        .filter(|step| *step < opened)
                    else {
                        continue;
                    };
                    step
                }
            };
            if let Some(step) = steps.get_mut(owner) {
                step.record(*number);
            }
        }
        self.steps = steps;
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
        let owner = match opens {
            Some(kind) => {
                self.steps.push(Step::opened(kind));
                self.steps.len() - 1
            }
            None => match operation
                .edits()
                .and_then(|sketch| self.step_of_sketch(sketch))
            {
                Some(step) => step,
                None => return,
            },
        };
        self.last_operation_number += 1;
        let number = self.last_operation_number;
        self.steps[owner].record(number);
        self.operations.push(operation);
        self.numbers.push(number);
        self.applied = self.operations.len();
    }

    /// Which step a sketch is: the rank-th step that opened a sketch.
    fn step_of_sketch(&self, sketch: usize) -> Option<usize> {
        step_of(
            &self.steps.iter().map(Step::kind).collect::<Vec<_>>(),
            sketch,
        )
    }

    fn drop_what_was_undone(&mut self) {
        if self.applied == self.operations.len() {
            return;
        }
        self.operations.truncate(self.applied);
        self.numbers.truncate(self.applied);
        let left: BTreeSet<u32> = self.numbers.iter().copied().collect();
        self.steps.retain_mut(|step| {
            step.keep_only(&left);
            !step.is_empty()
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

/// Which step a sketch is: the rank-th step that opened a sketch.
fn step_of(kinds: &[StepKind], sketch: usize) -> Option<usize> {
    kinds
        .iter()
        .enumerate()
        .filter(|(_, kind)| **kind == StepKind::Sketch)
        .map(|(rank, _)| rank)
        .nth(sketch)
}

#[cfg(test)]
mod tests;
