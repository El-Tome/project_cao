//! What a part does with a chamfer: the corner stays two whole traits and is
//! drawn cut, the history says the same thing every time it is replayed, and
//! the distances are read in millimetres like every other length the user
//! types.

use cao_part::history::{Operation, PointRef};
use cao_part::{Outcome, PartState};
use cao_sketch::{Chamfer, Corner, CutCurve, DrawnCut, PointId, SegmentId, WorkPlane};
use glam::DVec2;

const CORNER: DVec2 = DVec2::new(2.0, 1.0);
const EAST: SegmentId = SegmentId(0);
const NORTH: SegmentId = SegmentId(1);
const TOLERANCE: f64 = 1e-9;

fn a_right_angle() -> Vec<Operation> {
    vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(CORNER),
            end: PointRef::New(CORNER + DVec2::new(10.0, 0.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(1)),
            end: PointRef::New(CORNER + DVec2::new(0.0, 10.0)),
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

/// Where each side is drawn from and to, then where the cut runs.
fn drawn(state: &PartState) -> (Vec<Option<(DVec2, DVec2)>>, Vec<DrawnCut>) {
    let sketch = &state.sketches[0];
    let sides = sketch
        .live_segments()
        .map(|(id, _)| sketch.visible_part(id))
        .collect();
    (sides, sketch.drawn_cuts())
}

#[test]
fn a_chamfer_draws_a_cut_across_the_corner_and_leaves_both_traits_whole() {
    let mut state = replay(&a_right_angle());

    let said = state.apply(&Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(EAST, NORTH)],
        mode: Chamfer::Equal(3.0).into(),
        construction: false,
    });

    let sketch = &state.sketches[0];
    assert_eq!(
        sketch.live_segments().count(),
        2,
        "nothing is laid but the cut"
    );
    let (sides, cuts) = drawn(&state);
    assert_eq!(cuts.len(), 1);
    assert!(matches!(cuts[0].curve, CutCurve::Straight { .. }));
    assert_eq!(
        sides[0],
        Some((
            CORNER + DVec2::new(3.0, 0.0),
            CORNER + DVec2::new(10.0, 0.0)
        )),
        "the side is drawn from where the cut meets it"
    );
    assert_eq!(
        said,
        Some(Outcome::Cut {
            rules: 0,
            values: 0,
            refused: 0
        }),
        "nothing spoke of either side, so the cut cost nothing"
    );
}

#[test]
fn a_chamfer_replayed_draws_the_same_cut() {
    let mut operations = a_right_angle();
    operations.push(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(EAST, NORTH)],
        mode: (Chamfer::Sided {
            first: 2.0,
            second: 6.0,
        })
        .into(),
        construction: false,
    });

    assert_eq!(drawn(&replay(&operations)), drawn(&replay(&operations)));
    assert_eq!(drawn(&replay(&operations)).1.len(), 1);
}

#[test]
fn a_chamfer_is_measured_in_millimetres_like_every_other_length() {
    let mut state = replay(&a_right_angle());
    state.millimeters_per_unit = Some(2.0);

    state.apply(&Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(EAST, NORTH)],
        mode: Chamfer::Equal(6.0).into(),
        construction: false,
    });

    let (_, cuts) = drawn(&state);
    assert!(
        cuts[0].touches[0].distance(CORNER + DVec2::new(3.0, 0.0)) <= TOLERANCE,
        "six millimetres at two millimetres to the unit is three units along the side: {:?}",
        cuts[0]
    );
}
