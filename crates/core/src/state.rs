use cao_sketch::{LengthOutcome, SegmentId, Sketch};

use crate::history::{History, Operation, PointRef};

/// What applying a typed length did.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DimensionOutcome {
    /// The part had no dimension yet, so this one set its scale instead of
    /// moving anything: the drawing keeps its shape and gains a real size.
    ScaleDefined { millimeters_per_unit: f32 },
    /// The scale was already fixed, so the geometry moved to match.
    Geometry(LengthOutcome),
}

/// The geometry of a part at a given point in its history.
///
/// Never saved: it is rebuilt by replaying the history, which is what keeps
/// "go back to this step" honest — there is no second copy that could drift
/// away from the list of operations.
#[derive(Debug, Clone, Default)]
pub struct PartState {
    /// Millimetres one world unit is worth, undefined until the first
    /// dimension is typed.
    pub millimeters_per_unit: Option<f32>,
    pub sketches: Vec<Sketch>,
}

impl PartState {
    pub fn rebuild(history: &History) -> Self {
        let mut state = Self::default();
        for operation in history.applied_operations() {
            state.apply(operation);
        }
        state
    }

    pub fn scale(&self) -> f32 {
        self.millimeters_per_unit.unwrap_or(1.0)
    }

    pub fn has_scale(&self) -> bool {
        self.millimeters_per_unit.is_some()
    }

    pub fn to_millimeters(&self, units: f32) -> f32 {
        units * self.scale()
    }

    /// Runs one operation. This is the only place geometry is produced, so a
    /// replay and a live edit can never disagree.
    pub fn apply(&mut self, operation: &Operation) -> Option<DimensionOutcome> {
        match operation {
            Operation::CreateSketch { plane } => {
                self.sketches.push(Sketch::new(*plane));
                None
            }
            Operation::AddSegment { sketch, start, end } => {
                let sketch = self.sketches.get_mut(*sketch)?;
                let start = resolve(sketch, start);
                let end = resolve(sketch, end);
                if start != end {
                    sketch.add_segment(start, end);
                }
                None
            }
            Operation::SetDimension {
                sketch,
                segment,
                millimeters,
            } => self.apply_dimension(*sketch, *segment, *millimeters),
        }
    }

    /// Applies a length typed by the user, in millimetres.
    ///
    /// The very first one defines what the drawing measures: nothing moves, the
    /// part simply learns how many millimetres a world unit is worth. Every
    /// later one is a constraint, and the geometry gives way instead.
    fn apply_dimension(
        &mut self,
        sketch: usize,
        segment: SegmentId,
        millimeters: f32,
    ) -> Option<DimensionOutcome> {
        if millimeters <= 0.0 {
            return None;
        }
        let length_in_units = self.sketches.get(sketch)?.segment_length(segment);
        if length_in_units < 1e-6 {
            return None;
        }

        if !self.has_scale() {
            self.sketches[sketch].set_dimension(segment, millimeters);
            let millimeters_per_unit = millimeters / length_in_units;
            self.millimeters_per_unit = Some(millimeters_per_unit);
            return Some(DimensionOutcome::ScaleDefined {
                millimeters_per_unit,
            });
        }

        let target_units = millimeters / self.scale();
        let sketch = &mut self.sketches[sketch];
        sketch.set_dimension(segment, millimeters);
        Some(DimensionOutcome::Geometry(
            sketch.set_segment_length(segment, target_units),
        ))
    }
}

fn resolve(sketch: &mut Sketch, point: &PointRef) -> cao_sketch::PointId {
    match point {
        PointRef::Existing(id) => *id,
        PointRef::New(position) => sketch.add_point(*position),
    }
}

#[cfg(test)]
mod tests {
    use cao_sketch::WorkPlane;
    use glam::Vec2;

    use super::*;

    fn chain_history() -> History {
        let mut history = History::default();
        history.push(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        history.push(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(Vec2::ZERO),
            end: PointRef::New(Vec2::new(2.0, 0.0)),
        });
        history.push(Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(cao_sketch::PointId(1)),
            end: PointRef::New(Vec2::new(2.0, 1.0)),
        });
        history
    }

    #[test]
    fn replaying_a_history_builds_the_drawing() {
        let state = PartState::rebuild(&chain_history());
        assert_eq!(state.sketches.len(), 1);
        assert_eq!(state.sketches[0].segments().len(), 2);
        assert_eq!(state.sketches[0].points().len(), 3, "the corner is shared");
    }

    /// Rewinding must give exactly the state that existed at that step: this is
    /// what both undo and the history tree rely on.
    #[test]
    fn rewinding_reproduces_the_earlier_state() {
        let mut history = chain_history();
        let after_first_segment = {
            let mut shorter = history.clone();
            shorter.rewind_to(2);
            PartState::rebuild(&shorter)
        };

        history.undo();
        let undone = PartState::rebuild(&history);

        assert_eq!(undone.sketches[0].segments().len(), 1);
        assert_eq!(
            undone.sketches[0].points().len(),
            after_first_segment.sketches[0].points().len()
        );
        assert_eq!(
            undone.sketches[0].points(),
            after_first_segment.sketches[0].points()
        );
    }

    /// A live edit and a replay must produce the same thing, or the drawing
    /// would silently change the next time the part is opened.
    #[test]
    fn applying_live_matches_replaying() {
        let mut history = chain_history();
        history.push(Operation::SetDimension {
            sketch: 0,
            segment: SegmentId(0),
            millimeters: 100.0,
        });

        let mut live = PartState::default();
        for operation in history.applied_operations() {
            live.apply(operation);
        }
        let replayed = PartState::rebuild(&history);

        assert_eq!(live.millimeters_per_unit, replayed.millimeters_per_unit);
        assert_eq!(live.sketches[0].points(), replayed.sketches[0].points());
    }

    #[test]
    fn the_first_dimension_sets_the_scale_without_moving_anything() {
        let mut state = PartState::rebuild(&chain_history());
        let before = state.sketches[0].points().to_vec();

        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            segment: SegmentId(0),
            millimeters: 100.0,
        });

        assert_eq!(
            outcome,
            Some(DimensionOutcome::ScaleDefined {
                millimeters_per_unit: 50.0
            })
        );
        assert_eq!(state.sketches[0].points(), before.as_slice());
    }

    #[test]
    fn later_dimensions_move_the_geometry() {
        let mut state = PartState::rebuild(&chain_history());
        state.apply(&Operation::SetDimension {
            sketch: 0,
            segment: SegmentId(0),
            millimeters: 100.0,
        });

        let outcome = state.apply(&Operation::SetDimension {
            sketch: 0,
            segment: SegmentId(1),
            millimeters: 100.0,
        });

        assert_eq!(
            outcome,
            Some(DimensionOutcome::Geometry(LengthOutcome::Exact))
        );
        let length = state.to_millimeters(state.sketches[0].segment_length(SegmentId(1)));
        assert!((length - 100.0).abs() < 1e-2, "got {length} mm");
    }

    #[test]
    fn an_operation_on_a_missing_sketch_is_ignored() {
        let mut state = PartState::default();
        assert_eq!(
            state.apply(&Operation::AddSegment {
                sketch: 3,
                start: PointRef::New(Vec2::ZERO),
                end: PointRef::New(Vec2::X),
            }),
            None
        );
        assert!(state.sketches.is_empty());
    }
}
