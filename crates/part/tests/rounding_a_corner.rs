//! What a part does with a fillet: the corner becomes a curve, the history says
//! the same thing every time it is replayed, and the radius is read in
//! millimetres like every other length the user types.

use cao_part::history::{Operation, PointRef};
use cao_part::{Outcome, PartState};
use cao_sketch::{PointId, SegmentId, WorkPlane};
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

#[test]
fn a_fillet_leaves_a_curve_between_two_traits_where_a_corner_had_none() {
    let mut state = replay(&a_right_angle());

    let said = state.apply(&Operation::Fillet {
        sketch: 0,
        first: EAST,
        second: NORTH,
        radius: 3.0,
    });

    let sketch = &state.sketches[0];
    assert_eq!(sketch.live_arcs().count(), 1);
    assert_eq!(sketch.live_segments().count(), 2);
    assert_eq!(
        said,
        Some(Outcome::Cut {
            rules: 0,
            values: 0
        })
    );
}

#[test]
fn a_fillet_replayed_rebuilds_the_curve_it_laid() {
    let mut operations = a_right_angle();
    operations.push(Operation::Fillet {
        sketch: 0,
        first: EAST,
        second: NORTH,
        radius: 3.0,
    });

    let curve = |state: &PartState| {
        let sketch = &state.sketches[0];
        sketch
            .live_arcs()
            .map(|(id, _)| {
                let draft = sketch.arc_draft(id);
                (draft.centre, draft.start, draft.end)
            })
            .collect::<Vec<_>>()
    };

    assert_eq!(curve(&replay(&operations)), curve(&replay(&operations)));
    assert_eq!(curve(&replay(&operations)).len(), 1);
}

#[test]
fn a_fillet_radius_is_measured_in_millimetres_like_every_other_length() {
    let mut state = replay(&a_right_angle());
    state.millimeters_per_unit = Some(2.0);

    state.apply(&Operation::Fillet {
        sketch: 0,
        first: EAST,
        second: NORTH,
        radius: 6.0,
    });

    let sketch = &state.sketches[0];
    let (id, _) = sketch.live_arcs().next().expect("the curve just laid");
    let reach = sketch.arc_radius(id);
    assert!(
        (reach - 3.0).abs() <= TOLERANCE,
        "six millimetres at two millimetres to the unit is a radius of three units, got {reach}"
    );
}
