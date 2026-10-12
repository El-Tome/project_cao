use glam::DVec2;

use super::*;
use crate::axis_angle::AxisToward;
use crate::constraints::Toward;
use crate::plane::WorkPlane;

fn between() -> DimensionTarget {
    DimensionTarget::AngleBetween {
        first: SegmentId(0),
        first_toward: Toward::End,
        second: SegmentId(1),
        second_toward: Toward::Start,
    }
}

fn to_the_axis(axis: SketchAxis) -> DimensionTarget {
    DimensionTarget::AxisAngle {
        segment: SegmentId(0),
        segment_toward: Toward::End,
        axis,
        axis_toward: AxisToward::Positive,
    }
}

#[test]
fn zero_and_a_half_turn_are_one_rule_and_a_right_angle_the_other() {
    for (value, rule) in [
        (
            0.0,
            Constraint::Parallel {
                first: SegmentId(0),
                second: SegmentId(1),
            },
        ),
        (
            180.0,
            Constraint::Parallel {
                first: SegmentId(0),
                second: SegmentId(1),
            },
        ),
        (
            90.0,
            Constraint::Perpendicular {
                first: SegmentId(0),
                second: SegmentId(1),
            },
        ),
    ] {
        assert_eq!(between().rule_at(value), Some(rule), "at {value}");
        assert_eq!(
            DimensionTarget::corner(SegmentId(0), SegmentId(1)).rule_at(value),
            Some(rule),
            "at a corner, at {value}",
        );
    }
}

#[test]
fn an_angle_to_an_axis_means_the_rule_to_that_axis() {
    assert_eq!(
        to_the_axis(SketchAxis::V).rule_at(180.0),
        Some(Constraint::AxisParallel {
            segment: SegmentId(0),
            axis: SketchAxis::V,
        }),
    );
    assert_eq!(
        to_the_axis(SketchAxis::U).rule_at(90.0),
        Some(Constraint::AxisPerpendicular {
            segment: SegmentId(0),
            axis: SketchAxis::U,
        }),
    );
}

#[test]
fn a_value_a_hair_off_or_another_dimension_means_no_rule() {
    for value in [0.01, 89.99, 179.99, 45.0, -90.0, 270.0, 360.0] {
        assert_eq!(between().rule_at(value), None, "at {value}");
    }
    assert_eq!(DimensionTarget::Length(SegmentId(0)).rule_at(0.0), None);
}

#[test]
fn an_angle_typed_while_drawing_turns_the_same_way_past_a_half_turn() {
    for value in [-90.0, 270.0, 90.0] {
        assert_eq!(
            ExactAngle::drawn(value),
            Some(ExactAngle::Square),
            "{value}"
        );
    }
    for value in [-180.0, 360.0, 0.0, 180.0] {
        assert_eq!(
            ExactAngle::drawn(value),
            Some(ExactAngle::Straight),
            "{value}"
        );
    }
    assert_eq!(ExactAngle::drawn(30.0), None);
}

/// A trait drawn at 30° from (2, 3), and the arm the line tool lays to read
/// it against, with the angle between the two.
fn a_trait_on_its_arm() -> (Sketch, DimensionTarget) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(DVec2::new(2.0, 3.0));
    let to = sketch
        .add_point(DVec2::new(2.0, 3.0) + DVec2::from_angle(std::f64::consts::FRAC_PI_6) * 10.0);
    let drawn = sketch.add_segment(from, to);
    let reaches = sketch.add_point(DVec2::new(12.0, 3.0));
    let arm = sketch.add_construction_segment(from, reaches);
    sketch.add_constraint(Constraint::AxisParallel {
        segment: arm,
        axis: SketchAxis::U,
    });
    let target = DimensionTarget::corner(arm, drawn);
    sketch.set_dimension(target, 30.0, false);
    (sketch, target)
}

#[test]
fn zero_typed_against_an_arm_is_the_rule_to_the_horizontal_and_the_arm_goes() {
    let (sketch, target) = a_trait_on_its_arm();

    let in_place = sketch.rule_in_place_of(target, 0.0).expect("a rule");

    assert_eq!(
        in_place,
        InPlace {
            rule: Constraint::AxisParallel {
                segment: SegmentId(0),
                axis: SketchAxis::U,
            },
            erased: vec![Element::Segment(SegmentId(1)), Element::Point(PointId(3))],
        },
    );
}

#[test]
fn a_construction_line_someone_leans_on_is_not_taken_for_an_arm() {
    let (mut sketch, target) = a_trait_on_its_arm();
    sketch.set_dimension(DimensionTarget::Length(SegmentId(1)), 10.0, false);

    let in_place = sketch.rule_in_place_of(target, 0.0).expect("a rule");

    assert_eq!(
        in_place,
        InPlace {
            rule: Constraint::Parallel {
                first: SegmentId(1),
                second: SegmentId(0),
            },
            erased: Vec::new(),
        },
    );
}

#[test]
fn an_arm_whose_far_end_carries_a_circle_is_not_taken_for_an_arm() {
    let (mut sketch, target) = a_trait_on_its_arm();
    sketch.add_circle(PointId(3), 2.0);

    let in_place = sketch.rule_in_place_of(target, 0.0).expect("a rule");

    assert!(in_place.erased.is_empty(), "{:?}", in_place.erased);
}
