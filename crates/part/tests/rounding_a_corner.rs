//! What a part does with a fillet: the corner stays two whole traits and is
//! drawn round, the history says the same thing every time it is replayed,
//! and the radius is read in millimetres like every other length the user
//! types.

use cao_part::history::{Operation, PointRef};
use cao_part::{Outcome, PartState};
use cao_sketch::{Corner, CutCurve, CutShape, PointId, SegmentId, WorkPlane};
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
fn a_fillet_draws_a_curve_where_the_corner_was_and_leaves_both_traits_whole() {
    let mut state = replay(&a_right_angle());
    let before = state.sketches[0].clone();

    let said = state.apply(&Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(EAST, NORTH)],
        radius: 3.0.into(),
        construction: false,
    });

    let sketch = &state.sketches[0];
    assert_eq!(sketch.live_arcs().count(), 0, "nothing is laid but the cut");
    assert_eq!(sketch.segments().len(), before.segments().len());
    assert_eq!(sketch.endpoints(EAST), before.endpoints(EAST));
    assert_eq!(sketch.endpoints(NORTH), before.endpoints(NORTH));
    let drawn = sketch.drawn_cuts();
    assert_eq!(drawn.len(), 1);
    assert!(drawn[0].fits);
    assert!(matches!(drawn[0].curve, CutCurve::Arc(_)));
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
fn a_fillet_replayed_draws_the_same_curve() {
    let mut operations = a_right_angle();
    operations.push(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(EAST, NORTH)],
        radius: 3.0.into(),
        construction: false,
    });

    let curve = |state: &PartState| state.sketches[0].drawn_cuts();

    assert_eq!(curve(&replay(&operations)), curve(&replay(&operations)));
    assert_eq!(curve(&replay(&operations)).len(), 1);
}

#[test]
fn a_fillet_radius_is_measured_in_millimetres_like_every_other_length() {
    let mut state = replay(&a_right_angle());
    state.millimeters_per_unit = Some(2.0);

    state.apply(&Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(EAST, NORTH)],
        radius: 6.0.into(),
        construction: false,
    });

    let sketch = &state.sketches[0];
    let (_, cut) = sketch.live_corner_cuts().next().expect("the cut just laid");
    let CutShape::Fillet { radius } = cut.shape else {
        panic!("a fillet: {cut:?}");
    };
    assert!(
        (radius - 3.0).abs() <= TOLERANCE,
        "six millimetres at two millimetres to the unit is a radius of three units, got {radius}"
    );
}
