//! A size typed to lay something the drawing had no way of measuring — an
//! extrusion's depth, the step of a rectangular pattern — is the part's first
//! value when it comes first, and a unit is a millimetre from then on.
//!
//! Closes #440.
//! - a depth typed for an extrusion on a part with no scale sets the scale at a
//!   unit to the millimetre; a dimension typed afterwards on the drawing it
//!   stands on moves the drawing, and the plate keeps its depth —
//!   `a_depth_typed_before_any_value_fixes_a_unit_at_a_millimetre`,
//!   `a_dimension_typed_after_a_depth_moves_the_drawing_and_leaves_the_depth`
//! - the same for the step of a rectangular pattern typed on a part with no
//!   scale — `a_patterns_step_typed_before_any_value_fixes_a_unit_at_a_millimetre`
//! - a revolution, or a circular pattern's step, sets nothing —
//!   `a_revolution_leaves_the_part_without_a_scale`,
//!   `a_circular_patterns_angle_leaves_the_part_without_a_scale`
//! - replaying the history rebuilds the same part, scale included —
//!   `replaying_a_part_whose_depth_fixed_its_scale_rebuilds_it_the_same`

use cao_part::history::{ExtrusionMode, Operation, PointRef, RevolutionAxis};
use cao_part::{PartDocument, PartState};
use cao_sketch::{
    ChosenAxis, DimensionTarget, Element, Repeats, SegmentId, Sketch, SketchAxis, WorkPlane,
};
use glam::DVec2;

const TOLERANCE: f64 = 1e-6;

/// The side of the rectangle that was drawn 70 long.
const SIDE: DimensionTarget = DimensionTarget::Length(SegmentId(0));

fn at_nine() -> chrono::DateTime<chrono::Utc> {
    "2026-01-02T09:00:00Z".parse().expect("a date")
}

/// How thick the part's matter stands, in the units the drawing is in — which
/// is what a scale that moved shows: in millimetres a plate re-read at a new
/// scale measures what it always did, and on screen it has changed size.
fn depth_in_units(document: &PartDocument) -> f64 {
    let (min, max) = document.body().bounds().expect("a volume");
    max.z - min.z
}

/// A part holding one rectangle, 70 by 30 as it was drawn, and nothing said
/// about its size yet.
fn a_rectangle_nobody_has_measured() -> PartDocument {
    let mut document = PartDocument::new("Plate", at_nine());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(70.0, 30.0)),
        construction: false,
    });
    document
}

#[test]
fn a_depth_typed_before_any_value_fixes_a_unit_at_a_millimetre() {
    let mut document = a_rectangle_nobody_has_measured();
    let areas = document.areas_at(0, &[DVec2::new(35.0, 15.0)]);

    document.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: 20.0.into(),
        mode: ExtrusionMode::Add,
    });

    assert!(
        document.has_scale(),
        "the depth was the part's first value and left it with no scale",
    );
    assert!(
        (document.scale() - 1.0).abs() < 1e-12,
        "a unit came out worth {} millimetres",
        document.scale(),
    );
}

#[test]
fn a_dimension_typed_after_a_depth_moves_the_drawing_and_leaves_the_depth() {
    let mut document = a_rectangle_nobody_has_measured();
    let areas = document.areas_at(0, &[DVec2::new(35.0, 15.0)]);
    document.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: 20.0.into(),
        mode: ExtrusionMode::Add,
    });

    document.apply(Operation::SetDimension {
        sketch: 0,
        target: SIDE,
        value: 100.0.into(),
        placement: None,
    });

    let side = document.sketches()[0].segment_length(SegmentId(0));
    assert!(
        (side - 100.0).abs() < TOLERANCE,
        "the drawing did not give way: the side is {side} units across where it \
         was drawn 70 and 100 millimetres were typed, so the scale moved instead",
    );
    assert!(
        (depth_in_units(&document) - 20.0).abs() < TOLERANCE,
        "the plate gave way with the drawing: {} units deep, where it was raised 20",
        depth_in_units(&document),
    );
}

#[test]
fn a_patterns_step_typed_before_any_value_fixes_a_unit_at_a_millimetre() {
    let mut document = PartDocument::new("Grid", at_nine());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::ZERO),
        end: PointRef::New(DVec2::new(4.0, 0.0)),
        construction: false,
    });

    document.apply(Operation::RectangularPattern {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        direction: ChosenAxis::Sketch(SketchAxis::U),
        along: Repeats {
            step: 25.0,
            count: 3,
        }
        .into(),
        across: Repeats {
            step: 0.0,
            count: 1,
        }
        .into(),
    });

    assert!(
        document.has_scale(),
        "the step was the part's first value and left it with no scale",
    );
    assert!(
        (document.scale() - 1.0).abs() < 1e-12,
        "a unit came out worth {} millimetres",
        document.scale(),
    );
    let drawing = &document.sketches()[0];
    assert_eq!(drawing.live_segments().count(), 3, "three copies in a row");
    let apart = drawing.endpoints(SegmentId(1)).0.x - drawing.endpoints(SegmentId(0)).0.x;
    assert!(
        (apart.abs() - 25.0).abs() < TOLERANCE,
        "the copies stand {} units apart where 25 millimetres were typed",
        apart.abs(),
    );
}

#[test]
fn a_revolution_leaves_the_part_without_a_scale() {
    let mut document = a_rectangle_nobody_has_measured();
    let areas = document.areas_at(0, &[DVec2::new(35.0, 15.0)]);

    document.apply(Operation::Revolve {
        sketch: 0,
        areas,
        axis: RevolutionAxis::Sketch(SketchAxis::U),
        angle: 90.0.into(),
        mode: ExtrusionMode::Add,
    });

    assert!(
        !document.has_scale(),
        "an angle said how big the part is: a unit came out worth {} millimetres",
        document.scale(),
    );
    let replayed = PartState::rebuild(&document.history);
    assert!(
        !replayed.has_scale(),
        "the replay read the angle as the part's first value, at {} millimetres to the unit",
        replayed.scale(),
    );
}

#[test]
fn a_circular_patterns_angle_leaves_the_part_without_a_scale() {
    let mut document = a_rectangle_nobody_has_measured();

    document.apply(Operation::CircularPattern {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        centre: Sketch::ORIGIN,
        degrees: 90.0.into(),
        count: 4.0.into(),
    });

    assert!(
        !document.has_scale(),
        "an angle said how big the part is: a unit came out worth {} millimetres",
        document.scale(),
    );
    let replayed = PartState::rebuild(&document.history);
    assert!(
        !replayed.has_scale(),
        "the replay read the angle as the part's first value, at {} millimetres to the unit",
        replayed.scale(),
    );
}

#[test]
fn replaying_a_part_whose_depth_fixed_its_scale_rebuilds_it_the_same() {
    let mut document = a_rectangle_nobody_has_measured();
    let areas = document.areas_at(0, &[DVec2::new(35.0, 15.0)]);
    document.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: 20.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: SIDE,
        value: 100.0.into(),
        placement: None,
    });

    let replayed = PartState::rebuild(&document.history);

    assert!(
        (replayed.scale() - document.scale()).abs() < 1e-12,
        "the replay came out at {} millimetres to the unit where the part stands at {}",
        replayed.scale(),
        document.scale(),
    );
    let (min, max) = replayed.body.bounds().expect("a volume");
    assert!(
        (max.z - min.z - depth_in_units(&document)).abs() < TOLERANCE,
        "the replay raised {} units of matter where the part holds {}",
        max.z - min.z,
        depth_in_units(&document),
    );
    assert!(
        (replayed.sketches[0].segment_length(SegmentId(0)) - 100.0).abs() < TOLERANCE,
        "the replay drew the side {} units across",
        replayed.sketches[0].segment_length(SegmentId(0)),
    );
}
