//! Everything done to a part, in order, and the cursor that says how much
//! of it is in effect.

use serde::{Deserialize, Serialize};

mod operation;
pub use operation::{ExtrusionMode, Operation, PointRef, RevolutionAxis};

/// Everything done to a part, in order, with a cursor separating what is
/// applied from what can be redone.
///
/// The part's geometry is not stored: it is rebuilt by replaying this list. So
/// undo, redo and "go back to this step" are all the same operation — moving
/// the cursor — and the redo tail survives being saved and reopened.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct History {
    operations: Vec<Operation>,
    applied: usize,
}

impl History {
    pub fn operations(&self) -> &[Operation] {
        &self.operations
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
        self.operations.truncate(self.applied);
        self.operations.push(operation);
        self.applied = self.operations.len();
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

    /// Rewrites every operation in place. Only meant for bringing an older
    /// file up to date; nothing else should reach past the cursor.
    pub fn map_operations(&mut self, mut change: impl FnMut(&mut Operation)) {
        for operation in &mut self.operations {
            change(operation);
        }
    }

    /// Moves the cursor anywhere in the list, which is how the history tree
    /// puts the part back the way it was at a given step.
    pub fn rewind_to(&mut self, applied: usize) {
        self.applied = applied.min(self.operations.len());
    }
}

#[cfg(test)]
mod tests {
    use cao_sketch::WorkPlane;
    use glam::DVec2;

    use super::*;

    fn segment_op(sketch: usize) -> Operation {
        Operation::AddSegment {
            sketch,
            start: PointRef::New(DVec2::ZERO),
            end: PointRef::New(DVec2::X),
            construction: false,
        }
    }

    #[test]
    fn undo_and_redo_walk_the_list() {
        let mut history = History::default();
        history.push(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        history.push(segment_op(0));
        assert_eq!(history.applied(), 2);
        assert!(!history.can_redo());

        assert!(history.undo());
        assert_eq!(history.applied(), 1);
        assert!(history.can_redo());
        assert_eq!(history.operations().len(), 2, "the tail is kept for redo");

        assert!(history.redo());
        assert_eq!(history.applied(), 2);
        assert!(!history.redo());
    }

    #[test]
    fn undoing_past_the_start_does_nothing() {
        let mut history = History::default();
        assert!(!history.undo());
        assert!(!history.can_undo());
    }

    /// Drawing something new after an undo abandons the branch that was undone.
    #[test]
    fn a_new_operation_drops_what_was_undone() {
        let mut history = History::default();
        history.push(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        history.push(segment_op(0));
        history.undo();

        history.push(segment_op(1));

        assert_eq!(history.operations().len(), 2);
        assert_eq!(history.applied(), 2);
        assert!(!history.can_redo());
        assert_eq!(history.operations()[1], segment_op(1));
    }

    #[test]
    fn rewinding_keeps_everything_for_redo() {
        let mut history = History::default();
        for _ in 0..5 {
            history.push(segment_op(0));
        }

        history.rewind_to(2);

        assert_eq!(history.applied(), 2);
        assert_eq!(history.applied_operations().len(), 2);
        assert_eq!(history.operations().len(), 5);
        assert!(history.can_redo());
    }

    #[test]
    fn rewinding_past_the_end_is_clamped() {
        let mut history = History::default();
        history.push(segment_op(0));
        history.rewind_to(99);
        assert_eq!(history.applied(), 1);
    }
}
