use cao_sketch::{Constraint, DimensionTarget, Element, PointId, SegmentId, SketchAxis, WorkPlane};
use glam::DVec2;
use serde::{Deserialize, Serialize};

/// Which point an operation refers to.
///
/// Resolved when the user clicks, never re-derived on replay: snapping depends
/// on the zoom level at the time, so re-running it later could join different
/// points and rebuild a different drawing.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PointRef {
    Existing(PointId),
    New(DVec2),
}

/// What an extrusion does to the part.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtrusionMode {
    /// Adds the prism to the part.
    Add,
    /// Takes the prism out of it.
    Cut,
}

/// What a face is swept around.
///
/// Either one of the sketch's own axes, or a line the user drew. A drawn line
/// is named by its rank in the sketch, which is stable: segments are only ever
/// appended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevolutionAxis {
    Sketch(SketchAxis),
    Segment(SegmentId),
}

/// One step of the part's history. Replaying the list from the start rebuilds
/// the whole part, which is what makes rolling back to any point possible.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    CreateSketch {
        plane: WorkPlane,
    },
    AddPoint {
        sketch: usize,
        position: DVec2,
    },
    AddSegment {
        sketch: usize,
        start: PointRef,
        end: PointRef,
    },
    /// Four corners and four sides in one step, so the history reads as one
    /// rectangle rather than four unrelated lines.
    AddRectangle {
        sketch: usize,
        corner: PointRef,
        opposite: PointRef,
    },
    AddCircle {
        sketch: usize,
        center: PointRef,
        radius: f64,
        /// The places clicked on the rim, kept as real points held on the
        /// circle. They are what a circle drawn by its points can be grabbed
        /// by: dragging one resizes the circle rather than leaving a stray
        /// point behind.
        #[serde(default)]
        rim: Vec<PointRef>,
    },
    /// Flags one or several shapes as construction geometry, or takes the flag
    /// back off — several at once, so a rectangle's four sides flag together
    /// in the one step that drew them, rather than one that undo could split.
    /// Sketch, elements, whether they are now construction.
    SetConstruction(usize, Vec<Element>, bool),
    /// Dragging a point to a new place.
    MovePoint {
        sketch: usize,
        point: PointId,
        position: DVec2,
    },
    /// Dragging a whole selection: every point named moves by the same step,
    /// so the shapes travel together instead of being pulled apart.
    MoveMany {
        sketch: usize,
        points: Vec<PointId>,
        by: DVec2,
    },
    /// Dragging an annotation away from where it sits by default.
    MoveDimension {
        sketch: usize,
        target: DimensionTarget,
        offset: DVec2,
    },
    SetDimension {
        sketch: usize,
        target: DimensionTarget,
        /// Millimetres for a length or radius, degrees for an angle.
        value: f64,
        /// Where the annotation goes, in sketch units. Carried by the same
        /// step rather than a `MoveDimension` of its own: a dimension put down
        /// somewhere is one action, and reading "Cote 60 mm" then "Cote
        /// déplacée" for every single click says nothing extra.
        #[serde(default)]
        placement: Option<DVec2>,
    },
    /// Turns closed areas of a sketch into matter, or takes matter away.
    Extrude {
        sketch: usize,
        /// One position inside each chosen area, in the sketch's own
        /// coordinates.
        ///
        /// The areas are named by a point rather than by their rank: a rank
        /// would move the moment another shape is drawn, and the extrusion
        /// would silently start applying to a different part of the drawing.
        picks: Vec<DVec2>,
        /// Millimetres. Negative goes the other way along the plane.
        distance: f64,
        mode: ExtrusionMode,
    },
    /// Makes two points one, once they have been laid on top of each other.
    ///
    /// Recorded rather than worked out again on replay: which points are close
    /// enough depends on the zoom at the time, so re-deriving it later could
    /// join a different pair — or none.
    MergePoints {
        sketch: usize,
        kept: PointId,
        dropped: PointId,
    },
    /// Lays down a rule with no value: perpendicular, parallel, equal…
    Constrain {
        sketch: usize,
        constraint: Constraint,
    },
    /// Deletes everything that was selected, in one step.
    EraseMany {
        sketch: usize,
        elements: Vec<Element>,
        dimensions: Vec<DimensionTarget>,
        #[serde(default)]
        constraints: Vec<Constraint>,
    },
    /// Sweeps closed areas of a sketch around an axis lying in its plane.
    Revolve {
        sketch: usize,
        picks: Vec<DVec2>,
        axis: RevolutionAxis,
        /// Degrees. Negative turns the other way.
        angle: f64,
        mode: ExtrusionMode,
    },
}

impl Operation {
    /// True for the operations that open a new feature in the tree, and under
    /// which the following ones are grouped.
    pub(crate) fn starts_feature(&self) -> bool {
        matches!(
            self,
            Self::CreateSketch { .. } | Self::Extrude { .. } | Self::Revolve { .. }
        )
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
    use super::*;

    fn segment_op(sketch: usize) -> Operation {
        Operation::AddSegment {
            sketch,
            start: PointRef::New(DVec2::ZERO),
            end: PointRef::New(DVec2::X),
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
