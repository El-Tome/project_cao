//! What one gesture of the tool in hand leaves in the history.
//!
//! Closes #314.
//! - undo removes everything the gesture laid in one step —
//!   `one_undo_takes_back_everything_one_gesture_laid`
//! - what the gesture lays is what its operations lay, in order —
//!   `a_gesture_lays_what_its_operations_lay`
//! - compacting the history still works —
//!   `compacting_a_gesture_keeps_everything_it_laid`
//! - a gesture is folded at its close, since its later operations are written
//!   against what its earlier ones laid —
//!   `everything_recorded_after_a_mark_becomes_one_gesture`

use cao_part::history::{Operation, PointRef};
use cao_part::{History, PartDocument, PartState};
use cao_sketch::{DimensionTarget, PointId, SegmentId, WorkPlane};
use glam::DVec2;

const START: DVec2 = DVec2::ZERO;
const END: DVec2 = DVec2::new(10.0, 5.0);

/// A trait and, beside it, the horizontal arm its angle is read against —
/// laid as one gesture, the way the line tool lays them.
fn a_trait_leaning_on_an_arm() -> Operation {
    Operation::Gesture(vec![
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(START),
            end: PointRef::New(END),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(1)),
            end: PointRef::New(DVec2::new(10.0, 0.0)),
            construction: true,
        },
        Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Angle {
                first: SegmentId(1),
                second: SegmentId(0),
            },
            value: 26.565.into(),
            placement: None,
        },
    ])
}

fn a_sketch_with_the_gesture() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(a_trait_leaning_on_an_arm());
    history
}

#[test]
fn a_gesture_lays_what_its_operations_lay() {
    let state = PartState::rebuild(&a_sketch_with_the_gesture());
    let sketch = &state.sketches[0];

    assert_eq!(
        sketch.live_segments().count(),
        2,
        "the trait and the arm it leans on are both drawn",
    );
    assert_eq!(
        sketch.dimensions().len(),
        1,
        "the angle between the two is written down",
    );
}

#[test]
fn one_undo_takes_back_everything_one_gesture_laid() {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(a_trait_leaning_on_an_arm());

    assert!(document.undo(), "there was a gesture to take back");

    let sketch = &document.sketches()[0];
    assert_eq!(
        sketch.live_segments().count(),
        0,
        "one undo left a piece of the gesture behind",
    );
    assert!(
        sketch.dimensions().is_empty(),
        "one undo left the angle behind, measuring a trait that is gone",
    );
}

#[test]
fn compacting_a_gesture_keeps_everything_it_laid() {
    let compacted = PartState::rebuild(&cao_part::compact(&a_sketch_with_the_gesture()));
    let sketch = &compacted.sketches[0];

    assert_eq!(
        sketch.live_segments().count(),
        2,
        "compaction lost a piece of the gesture",
    );
    assert_eq!(
        sketch.dimensions().len(),
        1,
        "compaction lost the angle the gesture wrote down",
    );
}

#[test]
fn everything_recorded_after_a_mark_becomes_one_gesture() {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });

    let opened = document.history.mark();
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(START),
        end: PointRef::New(END),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 11.18.into(),
        placement: None,
    });
    document.history.fold_into_one_gesture(opened);

    assert!(document.undo(), "there was a gesture to take back");
    assert_eq!(
        document.sketches()[0].live_segments().count(),
        0,
        "the trait was laid before the value, so one undo left it standing",
    );
    assert_eq!(
        document.history.operations().len(),
        2,
        "the drawing was opened and one gesture was made, and the list says otherwise",
    );
}
