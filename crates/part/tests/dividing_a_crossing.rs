//! What a part does with a division: the traits are cut where they cross, and
//! the history says the same thing every time it is replayed.

use cao_part::history::{Operation, PointRef};
use cao_part::{Outcome, PartState};
use cao_sketch::{ArcId, DimensionTarget, SegmentId, WorkPlane};
use glam::DVec2;

const ACROSS: SegmentId = SegmentId(0);
const UP: SegmentId = SegmentId(1);
const CROSSING: DVec2 = DVec2::new(5.0, 0.0);

fn drawing_two_traits_crossing() -> Vec<Operation> {
    vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
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
        arcs: Vec::new(),
        at: CROSSING,
    });

    let sketch = &state.sketches[0];
    assert_eq!(sketch.live_segments().count(), 4);
    assert_eq!(
        said,
        Some(Outcome::Cut {
            rules: 0,
            values: 0,
            refused: 0
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
        arcs: Vec::new(),
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
        arcs: Vec::new(),
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
        arcs: Vec::new(),
        at: CROSSING,
    });

    assert_eq!(
        said,
        Some(Outcome::Cut {
            rules: 0,
            values: 1,
            refused: 0
        }),
        "the length measured the whole trait, and neither piece is it"
    );
}

/// A quarter arc from due east to due north around the origin, with a trait
/// running up across it at x = 3.
fn drawing_a_trait_across_an_arc() -> Vec<Operation> {
    vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddArc {
            sketch: 0,
            center: PointRef::New(DVec2::new(0.0, 0.0)),
            start: PointRef::New(DVec2::new(5.0, 0.0)),
            end: PointRef::New(DVec2::new(0.0, 5.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(3.0, -1.0)),
            end: PointRef::New(DVec2::new(3.0, 10.0)),
            construction: false,
        },
    ]
}

const ON_THE_ARC: DVec2 = DVec2::new(3.0, 4.0);

#[test]
fn dividing_an_arc_leaves_two_curves_where_there_was_one() {
    let mut state = replay(&drawing_a_trait_across_an_arc());

    let said = state.apply(&Operation::Split {
        sketch: 0,
        segments: vec![SegmentId(0)],
        arcs: vec![ArcId(0)],
        at: ON_THE_ARC,
    });

    let sketch = &state.sketches[0];
    assert_eq!(sketch.live_arcs().count(), 2, "the curve was cut in two");
    assert_eq!(sketch.live_segments().count(), 2, "and so was the trait");
    assert_eq!(
        said,
        Some(Outcome::Cut {
            rules: 0,
            values: 0,
            refused: 0
        })
    );
}

#[test]
fn an_arc_divided_and_replayed_comes_back_the_same() {
    let mut operations = drawing_a_trait_across_an_arc();
    operations.push(Operation::Split {
        sketch: 0,
        segments: vec![SegmentId(0)],
        arcs: vec![ArcId(0)],
        at: ON_THE_ARC,
    });

    let places = |state: &PartState| {
        let sketch = &state.sketches[0];
        sketch
            .live_arcs()
            .map(|(id, _)| {
                let draft = sketch.arc_draft(id);
                (draft.centre, draft.start, draft.end)
            })
            .collect::<Vec<_>>()
    };

    assert_eq!(places(&replay(&operations)), places(&replay(&operations)));
    assert_eq!(places(&replay(&operations)).len(), 2);
}

#[test]
fn a_division_recorded_before_arcs_could_be_cut_still_replays() {
    let written_by_an_older_version = r#"{
        "Split": { "sketch": 0, "segments": [0, 1], "at": [5.0, 0.0] }
    }"#;

    let operation: Operation = serde_json::from_str(written_by_an_older_version)
        .expect("an operation a past release wrote");

    let mut state = replay(&drawing_two_traits_crossing());
    state.apply(&operation);

    assert_eq!(
        state.sketches[0].live_segments().count(),
        4,
        "a division with no curves named divides the traits, as it always did"
    );
}
