//! Replaying a history, and rewinding it.
//!
//! Checked from outside: every door these tests go through is a public one.

use cao_part::history::{Operation, PointRef};
use cao_part::{DimensionOutcome, History, PartState};
use cao_sketch::{DimensionTarget, LengthOutcome, SegmentId, WorkPlane};
use glam::DVec2;

fn chain_history() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::ZERO),
        end: PointRef::New(DVec2::new(2.0, 0.0)),
        construction: false,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        // Point 0 is the sketch origin, so the corner just drawn is 2.
        start: PointRef::Existing(cao_sketch::PointId(2)),
        end: PointRef::New(DVec2::new(2.0, 1.0)),
        construction: false,
    });
    history
}

#[test]
fn replaying_a_history_builds_the_drawing() {
    let state = PartState::rebuild(&chain_history());
    assert_eq!(state.sketches.len(), 1);
    assert_eq!(state.sketches[0].segments().len(), 2);
    assert_eq!(
        state.sketches[0].points().len(),
        4,
        "the origin, plus three drawn points with the corner shared"
    );
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
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0,
        placement: None,
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
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0,
        placement: None,
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
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0,
        placement: None,
    });

    let outcome = state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(1)),
        value: 100.0,
        placement: None,
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
            start: PointRef::New(DVec2::ZERO),
            end: PointRef::New(DVec2::X),
            construction: false,
        }),
        None
    );
    assert!(state.sketches.is_empty());
}
