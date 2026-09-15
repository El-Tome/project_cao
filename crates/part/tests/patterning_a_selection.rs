//! What a part does with a circular pattern: copies stand round a point of the
//! drawing, and a replay rebuilds the same ring.

use cao_part::PartState;
use cao_part::history::{Operation, PointRef};
use cao_sketch::{Element, PointId, SegmentId, WorkPlane};
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
