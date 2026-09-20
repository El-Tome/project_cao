//! What a part hands back when a cut takes a stretch out of a curve.
//!
//! The same reckoning as a trait's, read through the door the canvas uses: a
//! sweep measures a curve neither piece runs any more, a reach measures both.

use cao_part::history::{Operation, PointRef};
use cao_part::{Outcome, PartState};
use cao_sketch::{ArcId, DimensionTarget, PointId, WorkPlane};
use glam::DVec2;

const CUT: ArcId = ArcId(0);
const REACH: f64 = 10.0;

fn on_the_rim(degrees: f64) -> DVec2 {
    DVec2::from_angle(degrees.to_radians()) * REACH
}

/// A half turn from due east round to due west, measured both ways, with two
/// points sitting on it a third and two thirds of the way along.
fn a_curve_measured_twice() -> PartState {
    let mut state = PartState::default();
    state.apply(&Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    state.apply(&Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        start: PointRef::New(on_the_rim(0.0)),
        end: PointRef::New(on_the_rim(180.0)),
        construction: false,
    });
    state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::ArcRadius(CUT),
        value: REACH,
        placement: None,
    });
    state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::ArcSweep(CUT),
        value: 180.0,
        placement: None,
    });
    for degrees in [60.0, 120.0] {
        state.apply(&Operation::AddPoint {
            sketch: 0,
            position: on_the_rim(degrees),
            on: Vec::new(),
        });
    }
    state
}

#[test]
fn a_cut_on_a_curve_says_how_much_of_what_was_typed_it_could_not_carry_over() {
    let mut state = a_curve_measured_twice();

    let outcome = state.apply(&Operation::TrimArc {
        sketch: 0,
        arc: CUT,
        from: PointId(4),
        to: PointId(5),
    });

    assert_eq!(
        outcome,
        Some(Outcome::Cut {
            rules: 0,
            values: 1
        }),
        "the sweep went; the reach followed both pieces",
    );
    assert_eq!(state.sketches[0].live_arcs().count(), 2);
}

#[test]
fn a_cut_on_a_curve_that_cannot_be_made_says_nothing() {
    let mut state = a_curve_measured_twice();

    let outcome = state.apply(&Operation::TrimArc {
        sketch: 0,
        arc: CUT,
        from: PointId(99),
        to: PointId(5),
    });

    assert_eq!(outcome, None);
}
