//! What a typed angle leaves in the drawing: a horizontal arm to read it
//! against, and the reading between the two.
//!
//! Closes #314.
//! - the arm runs along the horizontal, towards +X, and is held there —
//!   `the_arm_runs_towards_plus_x_whichever_way_the_trait_goes`,
//!   `an_arm_held_parallel_to_the_horizontal_axis_stays_horizontal`
//! - its length at birth is the trait's own width, or the least that can be
//!   seen — `an_arm_is_as_wide_as_the_trait_unless_that_is_too_little_to_see`
//! - a trait square to an origin axis it starts on is held on that axis, with
//!   no arm at all — `a_trait_square_to_an_axis_it_starts_on_leans_on_that_axis`,
//!   and off the axes a right angle is an angle like any other —
//!   `a_trait_square_to_an_axis_it_does_not_start_on_gets_an_arm_like_any_other`
//! - holding the arm's length lets the sketch end up fully constrained —
//!   `holding_the_arms_length_is_what_pins_the_drawing_down`
//! - a line drawn with a typed length only, or with no typed value, is
//!   untouched — no test: `line_dimensions` no longer writes an angle and is
//!   otherwise as it was, held by `shape_dimensions/tests.rs`
//! - erasing either of the two takes the reading and leaves the other standing
//!   — `erasing_either_of_the_two_takes_the_reading_and_leaves_the_other`
//! - dimensioning an angle by hand between two traits still works — no test:
//!   nothing here touches the dimension tool, and `DimensionTarget::AxisAngle`
//!   is still what an axis picked by hand writes
//!
//! What the tool lays with all this is answered where it lays it, in
//! `crates/app/src/screens/viewport/input/angle_arm/`.

use cao_sketch::{
    AngleArm, Constraint, DimensionTarget, Element, PointId, SegmentId, Sketch, SketchAxis,
    WorkPlane, angle_arm,
};
use glam::DVec2;

const TOLERANCE: f64 = 1e-6;

#[test]
fn an_arm_held_parallel_to_the_horizontal_axis_stays_horizontal() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(3.0, 4.0));
    let end = sketch.add_point(DVec2::new(9.0, 6.5));
    let arm = sketch.add_construction_segment(start, end);
    sketch.add_constraint(Constraint::AxisParallel {
        segment: arm,
        axis: SketchAxis::U,
    });

    sketch.solve(1.0);

    let (from, to) = sketch.endpoints(arm);
    assert!(
        (from.y - to.y).abs() < TOLERANCE,
        "the arm runs from {from} to {to}, which is not horizontal",
    );
    assert!(
        (from - DVec2::new(3.0, 4.0)).length() > TOLERANCE
            || (to - DVec2::new(9.0, 6.5)).length() > TOLERANCE,
        "the rule moved nothing at all, so it is holding nothing",
    );
}

/// A trait drawn from `start` towards `degrees`, ten units long.
fn a_trait_drawn_at(start: DVec2, degrees: f64) -> (Sketch, SegmentId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(start);
    let to = sketch.add_point(start + DVec2::from_angle(degrees.to_radians()) * 10.0);
    let segment = sketch.add_segment(from, to);
    (sketch, segment, from)
}

#[test]
fn the_arm_runs_towards_plus_x_whichever_way_the_trait_goes() {
    let start = DVec2::new(2.0, 3.0);
    let (sketch, segment, from) = a_trait_drawn_at(start, 150.0);

    let Some(AngleArm::Arm(reaches)) = angle_arm(&sketch, segment, from, 1.0) else {
        panic!("a trait leaning nowhere near an axis is read against an arm");
    };

    assert!(
        reaches.x > start.x && (reaches.y - start.y).abs() < TOLERANCE,
        "the arm runs from {start} to {reaches}, which is not east along the horizontal",
    );
}

#[test]
fn a_trait_square_to_an_axis_it_starts_on_leans_on_that_axis() {
    for (degrees, axis) in [
        (0.0, SketchAxis::U),
        (180.0, SketchAxis::U),
        (90.0, SketchAxis::V),
        (270.0, SketchAxis::V),
    ] {
        let start = match axis {
            SketchAxis::U => DVec2::new(4.0, 0.0),
            SketchAxis::V => DVec2::new(0.0, 4.0),
        };
        let (sketch, segment, from) = a_trait_drawn_at(start, degrees);

        assert_eq!(
            angle_arm(&sketch, segment, from, 1.0),
            Some(AngleArm::Axis(axis)),
            "a trait at {degrees}° from a point on that axis needs no arm of its own",
        );
    }
}

#[test]
fn a_trait_square_to_an_axis_it_does_not_start_on_gets_an_arm_like_any_other() {
    let (sketch, segment, from) = a_trait_drawn_at(DVec2::new(4.0, 2.0), 90.0);

    assert!(
        matches!(
            angle_arm(&sketch, segment, from, 1.0),
            Some(AngleArm::Arm(_))
        ),
        "off the axes, a right angle is an angle like any other",
    );
}

#[test]
fn an_arm_is_as_wide_as_the_trait_unless_that_is_too_little_to_see() {
    let start = DVec2::new(2.0, 2.0);
    let (wide, segment, from) = a_trait_drawn_at(start, 30.0);
    let least = 3.0;

    let Some(AngleArm::Arm(reaches)) = angle_arm(&wide, segment, from, least) else {
        panic!("a trait at 30° is read against an arm");
    };
    let (_, end) = wide.endpoints(segment);
    assert!(
        (reaches.x - start.x - (end.x - start.x).abs()).abs() < TOLERANCE,
        "the arm is {} wide where the trait spans {}",
        reaches.x - start.x,
        (end.x - start.x).abs(),
    );

    let (narrow, segment, from) = a_trait_drawn_at(start, 89.0);
    let Some(AngleArm::Arm(reaches)) = angle_arm(&narrow, segment, from, least) else {
        panic!("a trait at 89° is read against an arm");
    };
    assert!(
        (reaches.x - start.x - least).abs() < TOLERANCE,
        "a trait barely wider than nothing was given an arm {} long",
        reaches.x - start.x,
    );
}

#[test]
fn holding_the_arms_length_is_what_pins_the_drawing_down() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let reaching = DVec2::from_angle(150_f64.to_radians()) * 10.0;
    let end = sketch.add_point(reaching);
    let drawn = sketch.add_segment(Sketch::ORIGIN, end);
    let reach = reaching.x.abs();
    let far = sketch.add_point(DVec2::new(reach, 0.0));
    let arm = sketch.add_construction_segment(Sketch::ORIGIN, far);

    sketch.add_constraint(Constraint::AxisParallel {
        segment: arm,
        axis: SketchAxis::U,
    });
    sketch.set_dimension(
        DimensionTarget::Angle {
            first: arm,
            second: drawn,
        },
        150.0,
        false,
    );
    sketch.set_dimension(DimensionTarget::Length(drawn), 10.0, false);

    assert!(
        !sketch.is_fully_constrained(1.0),
        "the arm is free to be any length, and the drawing is said to be pinned down",
    );

    sketch.set_dimension(DimensionTarget::Length(arm), reach, false);

    assert!(
        sketch.is_fully_constrained(1.0),
        "holding the arm's length left something free",
    );
}

#[test]
fn erasing_either_of_the_two_takes_the_reading_and_leaves_the_other() {
    let laid = || {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let end = sketch.add_point(DVec2::from_angle(150_f64.to_radians()) * 10.0);
        let drawn = sketch.add_segment(Sketch::ORIGIN, end);
        let far = sketch.add_point(DVec2::new(9.0, 0.0));
        let arm = sketch.add_construction_segment(Sketch::ORIGIN, far);
        sketch.set_dimension(
            DimensionTarget::Angle {
                first: arm,
                second: drawn,
            },
            150.0,
            false,
        );
        (sketch, drawn, arm)
    };

    let (mut sketch, drawn, arm) = laid();
    sketch.erase(Element::Segment(arm));
    assert!(
        sketch.dimensions().is_empty(),
        "the reading measures an arm that is gone",
    );
    assert!(
        !sketch.is_erased_segment(drawn),
        "erasing the arm took the trait with it",
    );

    let (mut sketch, drawn, arm) = laid();
    sketch.erase(Element::Segment(drawn));
    assert!(
        sketch.dimensions().is_empty(),
        "the reading measures a trait that is gone",
    );
    assert!(
        !sketch.is_erased_segment(arm),
        "geometry nobody selected was erased along with the trait",
    );
}
