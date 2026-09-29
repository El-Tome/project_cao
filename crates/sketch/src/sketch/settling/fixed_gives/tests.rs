//! What sketch · settling/fixed_gives.rs is held to.
//!
//! Closes #453.
//! - a rectangle with its two bottom corners fixed takes a new width, the
//!   corner nearest the origin staying —
//!   `a_rectangle_fixed_by_its_bottom_corners_takes_a_new_width`
//! - with three corners fixed, the fewest fixed points move —
//!   `with_three_corners_fixed_the_fewest_fixed_points_move`
//! - two fixed points as near the origin as each other: the one fixed first
//!   stays — `of_two_fixed_points_as_near_the_origin_the_one_fixed_first_stays`
//! - a point that gave is still fixed, at its new place —
//!   `a_point_that_gave_is_still_fixed_where_it_now_stands`
//! - a value something free can answer moves no fixed point —
//!   `a_value_something_free_can_answer_moves_no_fixed_point`
//! - a value only the origin stands in the way of is still refused —
//!   `a_value_only_the_origin_stands_in_the_way_of_is_still_refused`
//! - a fixed trait laid collinear with a sketch axis comes onto the axis, and
//!   is still fixed there —
//!   `a_fixed_trait_laid_along_an_axis_comes_onto_it_and_stays_fixed`

use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
use crate::plane::WorkPlane;
use crate::sketch::{Element, LengthOutcome, PointId, SegmentId, Sketch};

const SCALE: f64 = 1.0;

/// A micrometre: the solver stops at a thousandth of a percent of the
/// drawing's size, far inside that.
const SETTLED: f64 = 1e-3;

/// A rectangle 100 wide and 50 high, three of its corners squared, laid to
/// the right of the origin and above it. Its corners run bottom left, bottom
/// right, top right, top left; its sides bottom, right, top, left.
fn rectangle(sketch: &mut Sketch) -> ([PointId; 4], [SegmentId; 4]) {
    let corners = [
        DVec2::new(20.0, 10.0),
        DVec2::new(120.0, 10.0),
        DVec2::new(120.0, 60.0),
        DVec2::new(20.0, 60.0),
    ]
    .map(|place| sketch.add_point(place));
    let sides: [SegmentId; 4] =
        std::array::from_fn(|side| sketch.add_segment(corners[side], corners[(side + 1) % 4]));
    for pair in 0..3 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[pair],
            second: sides[pair + 1],
        });
    }
    (corners, sides)
}

fn fix(sketch: &mut Sketch, point: PointId) {
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(point),
    });
}

fn type_value(sketch: &mut Sketch, target: DimensionTarget, value: f64) -> LengthOutcome {
    sketch.set_dimension(target, value, false);
    sketch.land_value(target, SCALE)
}

fn stayed(sketch: &Sketch, point: PointId, place: DVec2) -> bool {
    sketch.point(point).distance(place) < SETTLED
}

fn is_fixed(sketch: &Sketch, point: PointId) -> bool {
    sketch.points_that_stay()[point.0]
}

#[test]
fn a_rectangle_fixed_by_its_bottom_corners_takes_a_new_width() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let ([left, right, ..], [bottom, ..]) = rectangle(&mut sketch);
    fix(&mut sketch, right);
    fix(&mut sketch, left);

    let outcome = type_value(&mut sketch, DimensionTarget::Length(bottom), 150.0);

    assert_eq!(outcome, LengthOutcome::Exact, "the width is taken");
    assert!(
        (sketch.segment_length(bottom) - 150.0).abs() < SETTLED,
        "the rectangle is 150 wide, not {}",
        sketch.segment_length(bottom)
    );
    assert!(
        stayed(&sketch, left, DVec2::new(20.0, 10.0)),
        "the corner nearest the origin stays, not at {}",
        sketch.point(left)
    );
    assert!(
        stayed(&sketch, right, DVec2::new(170.0, 10.0)),
        "the other one moves along the bottom, to {}",
        sketch.point(right)
    );
}

#[test]
fn with_three_corners_fixed_the_fewest_fixed_points_move() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let ([bottom_left, bottom_right, top_right, top_left], [bottom, ..]) = rectangle(&mut sketch);
    fix(&mut sketch, bottom_left);
    fix(&mut sketch, bottom_right);
    fix(&mut sketch, top_right);

    let outcome = type_value(&mut sketch, DimensionTarget::Length(bottom), 150.0);

    assert_eq!(outcome, LengthOutcome::Exact, "the width is taken");
    assert!(
        stayed(&sketch, bottom_right, DVec2::new(120.0, 10.0))
            && stayed(&sketch, top_right, DVec2::new(120.0, 60.0)),
        "the two on the right stay: {} and {}",
        sketch.point(bottom_right),
        sketch.point(top_right)
    );
    assert!(
        stayed(&sketch, bottom_left, DVec2::new(-30.0, 10.0)),
        "the bottom left one moves alone, to {}",
        sketch.point(bottom_left)
    );
    assert!(
        (sketch.point(top_left).x + 30.0).abs() < SETTLED,
        "its free neighbour above follows, to {}",
        sketch.point(top_left)
    );
}

#[test]
fn of_two_fixed_points_as_near_the_origin_the_one_fixed_first_stays() {
    for first_fixed_on_the_right in [true, false] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let left = sketch.add_point(DVec2::new(-50.0, 20.0));
        let right = sketch.add_point(DVec2::new(50.0, 20.0));
        let line = sketch.add_segment(left, right);
        let (first, second) = match first_fixed_on_the_right {
            true => (right, left),
            false => (left, right),
        };
        fix(&mut sketch, first);
        fix(&mut sketch, second);
        let first_place = sketch.point(first);
        let second_place = sketch.point(second);

        let outcome = type_value(&mut sketch, DimensionTarget::Length(line), 150.0);

        assert_eq!(outcome, LengthOutcome::Exact, "the length is taken");
        assert!(
            stayed(&sketch, first, first_place),
            "the one fixed first stays, not at {}",
            sketch.point(first)
        );
        assert!(
            !stayed(&sketch, second, second_place),
            "the one fixed second gives"
        );
    }
}

#[test]
fn a_point_that_gave_is_still_fixed_where_it_now_stands() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let ([left, right, ..], [bottom, ..]) = rectangle(&mut sketch);
    fix(&mut sketch, left);
    fix(&mut sketch, right);
    type_value(&mut sketch, DimensionTarget::Length(bottom), 150.0);
    let gave_to = sketch.point(right);

    assert!(is_fixed(&sketch, right), "it is still fixed");
    let top_right = sketch.segments()[1].end;
    sketch.settle_around(top_right, DVec2::new(250.0, 90.0), SCALE);
    assert!(
        stayed(&sketch, right, gave_to),
        "and a drag does not move it from where it gave to, {}",
        sketch.point(right)
    );
}

#[test]
fn a_value_something_free_can_answer_moves_no_fixed_point() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let ([left, right, ..], [bottom, ..]) = rectangle(&mut sketch);
    fix(&mut sketch, left);
    fix(&mut sketch, right);
    let right_side = SegmentId(1);

    let outcome = type_value(&mut sketch, DimensionTarget::Length(right_side), 80.0);

    assert_eq!(outcome, LengthOutcome::Exact, "the height is taken");
    assert!(
        stayed(&sketch, left, DVec2::new(20.0, 10.0))
            && stayed(&sketch, right, DVec2::new(120.0, 10.0)),
        "the top corners answer it, no fixed one: {} and {}",
        sketch.point(left),
        sketch.point(right)
    );
    assert!((sketch.segment_length(bottom) - 100.0).abs() < SETTLED);
}

#[test]
fn a_value_only_the_origin_stands_in_the_way_of_is_still_refused() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let across = sketch.add_point(DVec2::new(60.0, 0.0));
    let up = sketch.add_point(DVec2::new(0.0, 80.0));
    let line = sketch.add_segment(across, up);
    sketch.add_constraint(Constraint::OnAxis {
        point: across,
        axis: SketchAxis::U,
    });
    sketch.add_constraint(Constraint::OnAxis {
        point: up,
        axis: SketchAxis::V,
    });
    sketch.set_dimension(DimensionTarget::Length(line), 100.0, false);
    let slope = sketch
        .angle_with_axis(line, SketchAxis::U)
        .expect("the trait leans");
    sketch.set_dimension(
        DimensionTarget::AxisAngle {
            segment: line,
            axis: SketchAxis::U,
        },
        slope,
        false,
    );
    fix(&mut sketch, across);
    fix(&mut sketch, up);
    assert_eq!(sketch.resolve(SCALE), LengthOutcome::Exact);

    let outcome = type_value(
        &mut sketch,
        DimensionTarget::Distance {
            from: Sketch::ORIGIN,
            to: across,
        },
        30.0,
    );

    assert_eq!(
        outcome,
        LengthOutcome::BestEffort,
        "the axes and the trait's values leave its ends one place each; only the origin could go"
    );
    assert_eq!(
        sketch.point(Sketch::ORIGIN),
        DVec2::ZERO,
        "the origin stays"
    );
}

#[test]
fn a_fixed_trait_laid_along_an_axis_comes_onto_it_and_stays_fixed() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(20.0, 30.0));
    let end = sketch.add_point(DVec2::new(120.0, 30.0));
    let line = sketch.add_segment(start, end);
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Segment(line),
    });

    let outcome = sketch.lay_rule(
        Constraint::AxisCollinear {
            segment: line,
            axis: SketchAxis::U,
        },
        SCALE,
    );

    assert_eq!(outcome, LengthOutcome::Exact, "the rule holds");
    assert!(
        sketch.point(start).y.abs() < SETTLED && sketch.point(end).y.abs() < SETTLED,
        "the trait is on the axis: {} to {}",
        sketch.point(start),
        sketch.point(end)
    );
    assert!(
        is_fixed(&sketch, start) && is_fixed(&sketch, end),
        "and still fixed there"
    );
}

#[test]
fn fixed_points_nothing_untrue_reaches_are_never_tried() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    for far in 0..12 {
        let apart = sketch.add_point(DVec2::new(500.0 + 10.0 * f64::from(far), 500.0));
        fix(&mut sketch, apart);
    }
    let start = sketch.add_point(DVec2::new(20.0, 30.0));
    let end = sketch.add_point(DVec2::new(120.0, 30.0));
    let line = sketch.add_segment(start, end);
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Segment(line),
    });

    let outcome = sketch.lay_rule(
        Constraint::AxisCollinear {
            segment: line,
            axis: SketchAxis::U,
        },
        SCALE,
    );

    assert_eq!(outcome, LengthOutcome::Exact, "the rule holds");
    assert!(
        sketch.point(start).y.abs() < SETTLED && sketch.point(end).y.abs() < SETTLED,
        "the trait is on the axis: {} to {}",
        sketch.point(start),
        sketch.point(end)
    );
}
