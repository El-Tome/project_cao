//! What part · dimensioning.rs is held to.

use cao_sketch::{CircleId, DimensionTarget, SegmentId, WorkPlane};
use glam::DVec2;

use super::*;
use crate::history::{Operation, PointRef};
use crate::outcome::Outcome;
use crate::state::PartState;

#[test]
fn a_circle_takes_its_radius_from_the_scale() {
    let mut state = PartState::default();
    state.apply(&Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    state.apply(&Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        radius: 4.0,
        rim: Vec::new(),
        construction: false,
    });

    // First value in the part: it sets the scale rather than resizing.
    let outcome = state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Radius(CircleId(0)),
        value: 20.0,
        placement: None,
    });
    assert_eq!(
        outcome,
        Some(Outcome::Dimension(DimensionOutcome::ScaleDefined {
            millimeters_per_unit: 5.0
        }))
    );
    assert!((state.sketches[0].circle(CircleId(0)).radius - 4.0).abs() < 1e-4);
}

/// An angle cannot set the scale: degrees say nothing about size.
#[test]
fn an_angle_never_defines_the_scale() {
    let mut state = PartState::default();
    state.apply(&Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    state.apply(&Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::ZERO),
        end: PointRef::New(DVec2::new(10.0, 0.0)),
        construction: false,
    });
    state.apply(&Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(cao_sketch::PointId(1)),
        end: PointRef::New(DVec2::new(0.0, 10.0)),
        construction: false,
    });

    let outcome = state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Angle {
            first: SegmentId(0),
            second: SegmentId(1),
        },
        value: 45.0,
        placement: None,
    });

    assert!(matches!(
        outcome,
        Some(Outcome::Dimension(DimensionOutcome::Geometry(_)))
    ));
    assert!(!state.has_scale());
    let measured = state.sketches[0]
        .angle_between(SegmentId(0), SegmentId(1))
        .expect("the segments meet");
    assert!((measured - 45.0).abs() < 1e-2, "got {measured}°");
}

/// A rectangle with one corner on the sketch origin, nothing about it said
/// yet.
fn a_rectangle_on_the_origin() -> PartState {
    let mut state = PartState::default();
    state.apply(&Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    state.apply(&Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
        opposite: PointRef::New(DVec2::new(70.0, 30.0)),
        construction: false,
    });
    state
}

/// Two sides, the three right angles a rectangle needs, and the direction
/// of one side against an axis — everything it takes to leave nothing
/// about it still to determine.
fn give_it_everything_it_needs(state: &mut PartState) {
    for (target, value) in [
        (DimensionTarget::Length(SegmentId(0)), 70.0),
        (DimensionTarget::Length(SegmentId(1)), 30.0),
        (
            DimensionTarget::Angle {
                first: SegmentId(0),
                second: SegmentId(1),
            },
            90.0,
        ),
        (
            DimensionTarget::Angle {
                first: SegmentId(1),
                second: SegmentId(2),
            },
            90.0,
        ),
        (
            DimensionTarget::Angle {
                first: SegmentId(2),
                second: SegmentId(3),
            },
            90.0,
        ),
        (
            DimensionTarget::AxisAngle {
                segment: SegmentId(0),
                axis: cao_sketch::SketchAxis::U,
            },
            0.0,
        ),
    ] {
        state.apply(&Operation::SetDimension {
            sketch: 0,
            target,
            value,
            placement: None,
        });
    }
}

/// The rectangle from the report: it must end up fully constrained — which
/// needs the angle taken against an axis, since nothing else stops it
/// turning about its corner.
#[test]
fn a_rectangle_on_the_axes_can_be_fully_constrained() {
    let mut state = a_rectangle_on_the_origin();
    assert!(
        !state.sketches[0].is_fully_constrained(1.0),
        "nothing given yet"
    );

    give_it_everything_it_needs(&mut state);

    let sketch = &state.sketches[0];
    assert_eq!(
        sketch.freedom(state.scale()).degrees_of_freedom,
        0,
        "the rectangle should have nothing left to determine"
    );
    assert!(sketch.is_fully_constrained(state.scale()));
}

/// Once it is settled, a further angle on the same rectangle adds nothing.
#[test]
fn a_further_angle_on_a_settled_rectangle_is_redundant() {
    let mut state = a_rectangle_on_the_origin();
    give_it_everything_it_needs(&mut state);

    // The fourth corner follows from the other three.
    let outcome = state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Angle {
            first: SegmentId(3),
            second: SegmentId(0),
        },
        value: 90.0,
        placement: None,
    });

    assert_eq!(
        outcome,
        Some(Outcome::Dimension(DimensionOutcome::Reference))
    );
}

/// A value on an already-settled shape becomes a readout, and the readout
/// shows what the geometry measures rather than what was typed.
#[test]
fn a_redundant_dimension_becomes_a_readout() {
    let mut state = PartState::default();
    state.apply(&Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    // Hung off the sketch origin, so only the far point can still move.
    state.apply(&Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
        end: PointRef::New(DVec2::new(10.0, 0.0)),
        construction: false,
    });
    state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0,
        placement: None,
    });
    state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::AxisAngle {
            segment: SegmentId(0),
            axis: cao_sketch::SketchAxis::U,
        },
        value: 0.0,
        placement: None,
    });

    // Changing a value that already drives something is not redundant, so
    // the redundant one has to be a target that has never been set.
    state.apply(&Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
        end: PointRef::Existing(cao_sketch::PointId(1)),
        construction: false,
    });

    let outcome = state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(1)),
        value: 999.0,
        placement: None,
    });

    assert_eq!(
        outcome,
        Some(Outcome::Dimension(DimensionOutcome::Reference))
    );
    let stored = state.sketches[0]
        .dimension_of(DimensionTarget::Length(SegmentId(1)))
        .expect("a readout was placed");
    assert!(stored.driven);
    assert!(
        (stored.value - 100.0).abs() < 1e-2,
        "a readout shows the measurement, not the typed value: {}",
        stored.value
    );
}
