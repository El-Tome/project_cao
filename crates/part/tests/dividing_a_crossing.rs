//! What a part does with a division: the traits are cut where they cross, and
//! the history says the same thing every time it is replayed.

use cao_part::history::{Operation, PointRef};
use cao_part::{Outcome, PartState};
use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};
use glam::DVec2;

const ACROSS: SegmentId = SegmentId(0);
const UP: SegmentId = SegmentId(1);
const CROSSING: DVec2 = DVec2::new(5.0, 0.0);

fn drawing_two_traits_crossing() -> Vec<Operation> {
    vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(0.0, 0.0)),
            end: PointRef::New(DVec2::new(10.0, 0.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(5.0, -5.0)),
            end: PointRef::New(DVec2::new(5.0, 5.0)),
            construction: false,
        },
    ]
}

fn replay(operations: &[Operation]) -> PartState {
    let mut state = PartState::default();
    for operation in operations {
        state.apply(operation);
    }
    state
}

#[test]
fn dividing_a_crossing_leaves_four_traits_where_there_were_two() {
    let mut state = replay(&drawing_two_traits_crossing());

    let said = state.apply(&Operation::Split {
        sketch: 0,
        segments: vec![ACROSS, UP],
        at: CROSSING,
    });

    let sketch = &state.sketches[0];
    assert_eq!(sketch.live_segments().count(), 4);
    assert_eq!(
        said,
        Some(Outcome::Cut {
            rules: 0,
            values: 0
        }),
        "nothing spoke of either trait, so the division cost nothing"
    );
}

#[test]
fn a_division_replayed_rebuilds_the_drawing_it_left_behind() {
    let mut operations = drawing_two_traits_crossing();
    operations.push(Operation::Split {
        sketch: 0,
        segments: vec![ACROSS, UP],
        at: CROSSING,
    });

    let once = replay(&operations);
    let twice = replay(&operations);

    let ends = |state: &PartState| {
        state.sketches[0]
            .live_segments()
            .map(|(id, _)| state.sketches[0].endpoints(id))
            .collect::<Vec<_>>()
    };
    assert_eq!(ends(&once), ends(&twice));
    assert_eq!(ends(&once).len(), 4);
}

#[test]
fn the_pieces_stand_exactly_where_the_traits_ran() {
    let mut state = replay(&drawing_two_traits_crossing());

    state.apply(&Operation::Split {
        sketch: 0,
        segments: vec![ACROSS, UP],
        at: CROSSING,
    });

    let sketch = &state.sketches[0];
    let mut ends: Vec<(f64, f64)> = sketch
        .live_segments()
        .flat_map(|(id, _)| {
            let (from, to) = sketch.endpoints(id);
            [(from.x, from.y), (to.x, to.y)]
        })
        .collect();
    ends.sort_by(|left, right| left.partial_cmp(right).expect("no place is nowhere"));
    ends.dedup();

    assert_eq!(
        ends,
        vec![(0.0, 0.0), (5.0, -5.0), (5.0, 0.0), (5.0, 5.0), (10.0, 0.0),],
        "the four ends the traits had, and the crossing they now stop at"
    );
}

#[test]
fn a_division_says_what_the_traits_it_cut_took_with_them() {
    let mut operations = drawing_two_traits_crossing();
    operations.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(ACROSS),
        value: 10.0,
        placement: None,
    });
    let mut state = replay(&operations);

    let said = state.apply(&Operation::Split {
        sketch: 0,
        segments: vec![ACROSS, UP],
        at: CROSSING,
    });

    assert_eq!(
        said,
        Some(Outcome::Cut {
            rules: 0,
            values: 1
        }),
        "the length measured the whole trait, and neither piece is it"
    );
}
