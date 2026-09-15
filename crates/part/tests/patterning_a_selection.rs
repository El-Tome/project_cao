//! What a part does with a pattern: copies stand round a point of the drawing
//! or in rows square to a direction, and a replay rebuilds the same ones.

use cao_part::PartState;
use cao_part::history::{Operation, PointRef};
use cao_sketch::{ChosenAxis, Element, PointId, Repeats, SegmentId, SketchAxis, WorkPlane};
use glam::DVec2;

const TOLERANCE: f64 = 1e-9;
const CENTRE: PointId = PointId(1);

fn a_trait_beside_a_centre() -> Vec<Operation> {
    vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
        },
        Operation::AddPoint {
            sketch: 0,
            position: DVec2::ZERO,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(3.0, 0.0)),
            end: PointRef::New(DVec2::new(4.0, 0.0)),
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

fn a_ring_of(count: usize) -> Operation {
    Operation::CircularPattern {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        centre: CENTRE,
        degrees: 360.0 / count as f64,
        count,
    }
}

#[test]
fn a_ring_of_six_stands_six_traits_round_the_centre() {
    let mut state = replay(&a_trait_beside_a_centre());

    state.apply(&a_ring_of(6));

    let sketch = &state.sketches[0];
    assert_eq!(sketch.live_segments().count(), 6);
    for (id, _) in sketch.live_segments() {
        let (from, to) = sketch.endpoints(id);
        assert!((from.length() - 3.0).abs() <= TOLERANCE);
        assert!((to.length() - 4.0).abs() <= TOLERANCE);
    }
}

#[test]
fn a_ring_replayed_rebuilds_the_same_copies() {
    let mut operations = a_trait_beside_a_centre();
    operations.push(a_ring_of(6));

    let ends = |state: &PartState| {
        let sketch = &state.sketches[0];
        sketch
            .live_segments()
            .map(|(id, _)| sketch.endpoints(id))
            .collect::<Vec<_>>()
    };

    assert_eq!(ends(&replay(&operations)), ends(&replay(&operations)));
    assert_eq!(ends(&replay(&operations)).len(), 6);
}

#[test]
fn a_centre_the_drawing_does_not_have_lays_nothing() {
    let mut state = replay(&a_trait_beside_a_centre());

    let said = state.apply(&Operation::CircularPattern {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        centre: PointId(99),
        degrees: 60.0,
        count: 6,
    });

    assert_eq!(said, None);
    assert_eq!(state.sketches[0].live_segments().count(), 1);
}

fn a_grid_of(along: Repeats, across: Repeats) -> Operation {
    Operation::RectangularPattern {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        direction: ChosenAxis::Sketch(SketchAxis::U),
        along,
        across,
    }
}

#[test]
fn a_grid_of_three_by_two_stands_six_traits_in_two_rows() {
    let mut state = replay(&a_trait_beside_a_centre());

    state.apply(&a_grid_of(
        Repeats {
            step: 10.0,
            count: 3,
        },
        Repeats {
            step: 5.0,
            count: 2,
        },
    ));

    let sketch = &state.sketches[0];
    assert_eq!(sketch.live_segments().count(), 6);
    let rows = sketch
        .live_segments()
        .filter(|(id, _)| sketch.endpoints(*id).0.y > 4.9)
        .count();
    assert_eq!(
        rows, 3,
        "three of the six stand a step across the direction"
    );
}

#[test]
fn a_grid_replayed_rebuilds_the_same_copies() {
    let mut operations = a_trait_beside_a_centre();
    operations.push(a_grid_of(
        Repeats {
            step: 10.0,
            count: 3,
        },
        Repeats {
            step: 5.0,
            count: 2,
        },
    ));

    let ends = |state: &PartState| {
        let sketch = &state.sketches[0];
        sketch
            .live_segments()
            .map(|(id, _)| sketch.endpoints(id))
            .collect::<Vec<_>>()
    };

    assert_eq!(ends(&replay(&operations)), ends(&replay(&operations)));
    assert_eq!(ends(&replay(&operations)).len(), 6);
}

#[test]
fn a_step_is_typed_in_millimetres_and_laid_down_in_units() {
    let mut state = replay(&a_trait_beside_a_centre());
    state.millimeters_per_unit = Some(2.0);

    state.apply(&a_grid_of(
        Repeats {
            step: 10.0,
            count: 2,
        },
        Repeats {
            step: 10.0,
            count: 1,
        },
    ));

    let sketch = &state.sketches[0];
    let laid = sketch
        .live_segments()
        .map(|(id, _)| sketch.endpoints(id).0.x)
        .find(|x| *x > 3.5)
        .expect("a copy east of the original");
    assert!(
        (laid - 8.0).abs() <= TOLERANCE,
        "ten millimetres at two per unit is five units along, got {laid}"
    );
}
