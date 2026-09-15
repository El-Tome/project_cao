//! What a part does with a mirror: a second copy appears the other side of the
//! axis, the original stays, and a replay rebuilds the same thing.

use cao_part::PartState;
use cao_part::history::{Operation, PointRef};
use cao_sketch::{Element, MirrorAxis, PointId, SegmentId, SketchAxis, WorkPlane};
use glam::DVec2;

const TOLERANCE: f64 = 1e-9;

fn a_trait_east_of_the_axis() -> Vec<Operation> {
    vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(2.0, 1.0)),
            end: PointRef::New(DVec2::new(6.0, 1.0)),
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

fn mirrored() -> Operation {
    Operation::Mirror {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        axis: MirrorAxis::Sketch(SketchAxis::V),
    }
}

#[test]
fn a_mirrored_trait_stands_the_other_side_of_the_axis_with_the_first_still_there() {
    let mut state = replay(&a_trait_east_of_the_axis());

    state.apply(&mirrored());

    let sketch = &state.sketches[0];
    assert_eq!(
        sketch.live_segments().count(),
        2,
        "the original and its copy"
    );
    let mut ends: Vec<f64> = sketch
        .live_points()
        .map(|(_, at)| at.x)
        .filter(|x| x.abs() > TOLERANCE)
        .collect();
    ends.sort_by(f64::total_cmp);
    assert_eq!(ends, vec![-6.0, -2.0, 2.0, 6.0]);
}

#[test]
fn a_mirror_replayed_rebuilds_the_copy_it_laid() {
    let mut operations = a_trait_east_of_the_axis();
    operations.push(mirrored());

    let ends = |state: &PartState| {
        let sketch = &state.sketches[0];
        sketch
            .live_segments()
            .map(|(id, _)| sketch.endpoints(id))
            .collect::<Vec<_>>()
    };

    assert_eq!(ends(&replay(&operations)), ends(&replay(&operations)));
    assert_eq!(ends(&replay(&operations)).len(), 2);
}

#[test]
fn a_mirror_across_a_trait_the_drawing_does_not_have_does_nothing() {
    let mut state = replay(&a_trait_east_of_the_axis());

    let said = state.apply(&Operation::Mirror {
        sketch: 0,
        elements: vec![Element::Point(PointId(1))],
        axis: MirrorAxis::Trait(SegmentId(9)),
    });

    assert_eq!(said, None);
    assert_eq!(state.sketches[0].live_segments().count(), 1);
}
