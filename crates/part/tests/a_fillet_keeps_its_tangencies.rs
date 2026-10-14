//! Closes #590.
//! - rounding several corners of one outline in one gesture leaves every
//!   fillet tangent to both its sides —
//!   `every_corner_rounded_in_one_gesture_keeps_both_its_tangencies`
//! - the same when the corners are rounded one after the other —
//!   `corners_rounded_one_after_the_other_keep_both_their_tangencies`
//! - a chamfer or a fillet cut on the corner next to a fillet leaves that
//!   fillet tangent to both its sides —
//!   `a_chamfer_beside_a_fillet_leaves_it_tangent_to_both_its_sides`
//! - a trim that takes the stretch an arc's contact stood on still drops the
//!   tangency, and counts it in what the cut cost — no test: it is the
//!   trim's own rule, held beside the trim in the sketch crate by the test
//!   saying a trim taking the contact of an arc drops its tangency

use cao_part::PartState;
use cao_part::history::{ChamferAsked, Operation, PointRef};
use cao_sketch::{ArcId, Constraint, Corner, PointId, Sketch, WorkPlane};
use glam::DVec2;

const LOW: DVec2 = DVec2::new(2.0, 1.0);
const HIGH: DVec2 = DVec2::new(102.0, 61.0);
const RADIUS: f64 = 10.0;

fn replay(operations: &[Operation]) -> PartState {
    let mut state = PartState::default();
    for operation in operations {
        state.apply(operation);
    }
    state
}

/// A rectangle standing clear of the origin, and its four corners.
fn a_rectangle() -> (Vec<Operation>, [PointId; 4]) {
    let operations = vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(LOW),
            opposite: PointRef::New(HIGH),
            construction: false,
        },
    ];
    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let at = |place: DVec2| {
        sketch
            .live_points()
            .find(|(_, point)| point.distance(place) < 1e-9)
            .map(|(id, _)| id)
            .expect("a corner of the rectangle")
    };
    let corners = [
        at(LOW),
        at(DVec2::new(HIGH.x, LOW.y)),
        at(HIGH),
        at(DVec2::new(LOW.x, HIGH.y)),
    ];
    (operations, corners)
}

fn fillet(corners: &[PointId]) -> Operation {
    Operation::Fillet {
        sketch: 0,
        corners: corners.iter().copied().map(Corner::At).collect(),
        radius: RADIUS.into(),
    }
}

/// The traits a fillet is held tangent to.
fn tangent_to(sketch: &Sketch, arc: ArcId) -> Vec<cao_sketch::SegmentId> {
    sketch
        .constraints()
        .iter()
        .filter_map(|rule| match rule {
            Constraint::ArcTangent {
                arc: on, segment, ..
            } if *on == arc => Some(*segment),
            _ => None,
        })
        .collect()
}

/// Every fillet held tangent to two live traits, each of which runs into one
/// of its ends.
fn assert_every_fillet_tangent_to_both_its_sides(state: &PartState, fillets: usize, case: &str) {
    let sketch = &state.sketches[0];
    assert_eq!(
        sketch.live_arcs().count(),
        fillets,
        "{case}: one fillet to each corner rounded"
    );
    for (arc, curve) in sketch.live_arcs() {
        let sides = tangent_to(sketch, arc);
        assert_eq!(
            sides.len(),
            2,
            "{case}: the fillet at {:?} is held tangent to {sides:?}",
            sketch.point(curve.center)
        );
        for side in sides {
            assert!(
                !sketch.is_erased_segment(side),
                "{case}: a tangency names a trait that is gone"
            );
            let line = sketch.segments()[side.0];
            assert!(
                [curve.start, curve.end]
                    .iter()
                    .any(|end| [line.start, line.end].contains(end)),
                "{case}: the trait a fillet is tangent to runs into one of its ends"
            );
        }
    }
}

#[test]
fn every_corner_rounded_in_one_gesture_keeps_both_its_tangencies() {
    let (mut operations, corners) = a_rectangle();

    operations.push(fillet(&corners));

    assert_every_fillet_tangent_to_both_its_sides(&replay(&operations), 4, "four in one gesture");
}

#[test]
fn corners_rounded_one_after_the_other_keep_both_their_tangencies() {
    let (mut operations, corners) = a_rectangle();

    for corner in corners {
        operations.push(fillet(&[corner]));
    }

    assert_every_fillet_tangent_to_both_its_sides(&replay(&operations), 4, "one after the other");
}

#[test]
fn a_chamfer_beside_a_fillet_leaves_it_tangent_to_both_its_sides() {
    let (mut operations, corners) = a_rectangle();
    operations.push(fillet(&[corners[0]]));

    operations.push(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::At(corners[1])],
        mode: ChamferAsked::Equal(5.0.into()),
    });

    assert_every_fillet_tangent_to_both_its_sides(&replay(&operations), 1, "a chamfer beside it");
}
