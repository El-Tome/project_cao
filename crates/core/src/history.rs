use cao_sketch::{PointId, SegmentId, WorkPlane};
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Which point an operation refers to.
///
/// Resolved when the user clicks, never re-derived on replay: snapping depends
/// on the zoom level at the time, so re-running it later could join different
/// points and rebuild a different drawing.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PointRef {
    Existing(PointId),
    New(Vec2),
}

/// One step of the part's history. Replaying the list from the start rebuilds
/// the whole part, which is what makes rolling back to any point possible.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    CreateSketch {
        plane: WorkPlane,
    },
    AddSegment {
        sketch: usize,
        start: PointRef,
        end: PointRef,
    },
    SetDimension {
        sketch: usize,
        segment: SegmentId,
        millimeters: f32,
    },
}

impl Operation {
    /// Short name for the history tree.
    pub fn label(&self) -> String {
        match self {
            Self::CreateSketch { plane } => format!("Esquisse — {}", plane.label()),
            Self::AddSegment { .. } => "Trait".to_string(),
            Self::SetDimension { millimeters, .. } => format!("Cote {millimeters} mm"),
        }
    }

    /// The line shown when a history entry is unfolded.
    pub fn detail(&self) -> String {
        match self {
            Self::CreateSketch { plane } => format!(
                "Plan d'origine ({:.0}, {:.0}, {:.0})",
                plane.normal().x,
                plane.normal().y,
                plane.normal().z
            ),
            Self::AddSegment { sketch, start, end } => {
                format!(
                    "Esquisse {sketch} · {} → {}",
                    point_label(start),
                    point_label(end)
                )
            }
            Self::SetDimension {
                sketch, segment, ..
            } => format!("Esquisse {sketch} · trait {}", segment.0),
        }
    }

    /// True for the operations that open a new feature in the tree, and under
    /// which the following ones are grouped.
    pub fn starts_feature(&self) -> bool {
        matches!(self, Self::CreateSketch { .. })
    }
}

fn point_label(point: &PointRef) -> String {
    match point {
        PointRef::Existing(id) => format!("point {}", id.0),
        PointRef::New(position) => format!("({:.1}, {:.1})", position.x, position.y),
    }
}

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

    /// Moves the cursor anywhere in the list, which is how the history tree
    /// puts the part back the way it was at a given step.
    pub fn rewind_to(&mut self, applied: usize) {
        self.applied = applied.min(self.operations.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn segment_op(sketch: usize) -> Operation {
        Operation::AddSegment {
            sketch,
            start: PointRef::New(Vec2::ZERO),
            end: PointRef::New(Vec2::X),
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
