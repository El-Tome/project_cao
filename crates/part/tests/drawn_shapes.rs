//! The shapes a single step of the history draws.
//!
//! Checked from outside: every door these tests go through is a public one.

use cao_part::history::{Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{ArcId, SegmentId, WorkPlane};
use glam::DVec2;

fn a_quarter_turn(construction: bool) -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        start: PointRef::New(DVec2::new(10.0, 0.0)),
        end: PointRef::New(DVec2::new(0.0, 10.0)),
        construction,
    });
    history
}

#[test]
fn an_arc_is_rebuilt_from_its_step_and_undo_takes_it_back_off() {
    let mut history = a_quarter_turn(false);

    let state = PartState::rebuild(&history);
    assert_eq!(state.sketches[0].arcs().len(), 1);
    // The origin, plus the centre and the two ends.
    assert_eq!(state.sketches[0].points().len(), 4);
    let sweep = state.sketches[0].arc_sweep(ArcId(0));
    assert!(
        (sweep - std::f64::consts::FRAC_PI_2).abs() < 1e-6,
        "a quarter turn expected, got {sweep}",
    );

    history.undo();
    assert!(PartState::rebuild(&history).sketches[0].arcs().is_empty());
}

#[test]
fn a_guide_arc_is_rebuilt_as_a_guide() {
    let drawn = PartState::rebuild(&a_quarter_turn(false));
    let guide = PartState::rebuild(&a_quarter_turn(true));

    assert!(!drawn.sketches[0].arc(ArcId(0)).construction);
    assert!(guide.sketches[0].arc(ArcId(0)).construction);
}

#[test]
fn a_rectangle_is_one_step_with_four_sides() {
    let mut state = PartState::default();
    state.apply(&Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    state.apply(&Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });

    let sketch = &state.sketches[0];
    // The origin, plus the four corners.
    assert_eq!(sketch.points().len(), 5);
    assert_eq!(sketch.segments().len(), 4);
    assert!((sketch.segment_length(SegmentId(0)) - 40.0).abs() < 1e-4);
    assert!((sketch.segment_length(SegmentId(1)) - 20.0).abs() < 1e-4);
}

#[test]
fn a_construction_rectangle_flags_all_four_sides_in_the_one_step_that_drew_them() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: true,
    });

    let state = PartState::rebuild(&history);
    let sketch = &state.sketches[0];
    assert!(sketch.segments().iter().all(|segment| segment.construction));

    history.undo();
    let after_undo = PartState::rebuild(&history);
    assert!(after_undo.sketches[0].segments().is_empty());
}

#[test]
fn a_symmetric_segment_is_one_step_holding_its_middle() {
    let mut state = PartState::default();
    state.apply(&Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    state.apply(&Operation::AddSymmetricSegment {
        sketch: 0,
        middle: PointRef::New(DVec2::new(10.0, 0.0)),
        end: PointRef::New(DVec2::new(40.0, 0.0)),
        construction: false,
    });

    let sketch = &state.sketches[0];
    // The origin, the middle, the end, and its mirror image.
    assert_eq!(sketch.points().len(), 4);
    assert_eq!(sketch.segments().len(), 1);
    assert!(
        (sketch.segment_length(SegmentId(0)) - 60.0).abs() < 1e-4,
        "30 each way"
    );
    assert!(
        sketch.constraints().iter().any(|constraint| matches!(
            constraint,
            cao_sketch::Constraint::Midpoint { segment, .. } if *segment == SegmentId(0)
        )),
        "the middle point is held at the segment's midpoint",
    );
}
