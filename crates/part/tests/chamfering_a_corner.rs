//! What a part does with a chamfer: the corner is cut back, the history says
//! the same thing every time it is replayed, and the distances are read in
//! millimetres like every other length the user types.

use cao_part::history::{Operation, PointRef};
use cao_part::{Outcome, PartState};
use cao_sketch::{Chamfer, PointId, SegmentId, WorkPlane};
use glam::DVec2;

const CORNER: DVec2 = DVec2::new(2.0, 1.0);
const EAST: SegmentId = SegmentId(0);
const NORTH: SegmentId = SegmentId(1);
const TOLERANCE: f64 = 1e-9;

fn a_right_angle() -> Vec<Operation> {
    vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
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

fn ends(state: &PartState) -> Vec<(DVec2, DVec2)> {
    let sketch = &state.sketches[0];
    sketch
        .live_segments()
        .map(|(id, _)| sketch.endpoints(id))
        .collect()
}

#[test]
fn a_chamfer_leaves_three_traits_where_a_corner_had_two() {
    let mut state = replay(&a_right_angle());

    let said = state.apply(&Operation::Chamfer {
        sketch: 0,
        first: EAST,
        second: NORTH,
        mode: Chamfer::Equal(3.0),
    });

    assert_eq!(state.sketches[0].live_segments().count(), 3);
    assert_eq!(
        said,
        Some(Outcome::Cut {
            rules: 0,
            values: 0
        }),
        "nothing spoke of either side, so the cut cost nothing"
    );
}

#[test]
fn a_chamfer_replayed_rebuilds_the_corner_it_cut() {
    let mut operations = a_right_angle();
    operations.push(Operation::Chamfer {
        sketch: 0,
        first: EAST,
        second: NORTH,
        mode: Chamfer::Sided {
            first: 2.0,
            second: 6.0,
        },
    });

    assert_eq!(ends(&replay(&operations)), ends(&replay(&operations)));
    assert_eq!(ends(&replay(&operations)).len(), 3);
}

#[test]
fn a_chamfer_is_measured_in_millimetres_like_every_other_length() {
    let mut state = replay(&a_right_angle());
    state.millimeters_per_unit = Some(2.0);

    state.apply(&Operation::Chamfer {
        sketch: 0,
        first: EAST,
        second: NORTH,
        mode: Chamfer::Equal(6.0),
    });

    let sketch = &state.sketches[0];
    let reached = sketch
        .live_points()
        .any(|(_, place)| place.distance(CORNER + DVec2::new(3.0, 0.0)) <= TOLERANCE);
    assert!(
        reached,
        "six millimetres at two millimetres to the unit is three units along the side"
    );
}
