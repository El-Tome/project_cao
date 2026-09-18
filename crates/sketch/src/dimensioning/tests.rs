//! What sketch · dimensioning.rs is held to.

use super::*;
use crate::WorkPlane;

fn trait_from(start: DVec2, end: DVec2) -> (Sketch, DimensionTarget, PointId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(start);
    let to = sketch.add_point(end);
    let segment = sketch.add_segment(from, to);
    (sketch, DimensionTarget::Length(segment), from, to)
}

fn leaning(degrees: f64) -> (Sketch, DimensionTarget, PointId, PointId) {
    let start = DVec2::new(10.0, 10.0);
    let span = DVec2::from_angle(degrees.to_radians()) * 40.0;
    trait_from(start, start + span)
}

#[test]
fn a_trait_square_on_an_axis_offers_only_its_length() {
    let (sketch, length, ..) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(50.0, 10.0));

    assert_eq!(sketch.oriented(length, DVec2::new(30.0, 60.0)), length);
    assert_eq!(sketch.oriented(length, DVec2::new(70.0, 10.0)), length);
}

#[test]
fn a_trait_square_on_the_other_axis_offers_only_its_length_too() {
    let (sketch, length, ..) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(10.0, 50.0));

    assert!(!sketch.is_slanted(length));
    assert_eq!(sketch.oriented(length, DVec2::new(60.0, 30.0)), length);
}

#[test]
fn a_trait_drawn_right_to_left_is_no_more_slanted_than_the_same_one_drawn_the_other_way() {
    let (sketch, length, ..) = trait_from(DVec2::new(50.0, 10.0), DVec2::new(10.0, 10.0));

    assert!(!sketch.is_slanted(length));
}

#[test]
fn a_trait_a_hair_off_an_axis_is_not_yet_worth_two_more_readings() {
    let (sketch, length, ..) = leaning(0.4);

    assert!(!sketch.is_slanted(length));
    assert_eq!(sketch.oriented(length, DVec2::new(30.0, 60.0)), length);
}

#[test]
fn a_trait_just_past_the_threshold_offers_its_width_and_its_height() {
    let (sketch, length, ..) = leaning(0.6);

    assert!(sketch.is_slanted(length));
}

#[test]
fn a_cursor_above_a_slanted_trait_asks_for_its_width() {
    let (sketch, length, from, to) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(50.0, 40.0));

    assert_eq!(
        sketch.oriented(length, DVec2::new(30.0, 60.0)),
        DimensionTarget::Projected {
            from,
            to,
            axis: SketchAxis::U,
        },
    );
}

#[test]
fn a_cursor_beside_a_slanted_trait_asks_for_its_height() {
    let (sketch, length, from, to) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(50.0, 40.0));

    assert_eq!(
        sketch.oriented(length, DVec2::new(70.0, 25.0)),
        DimensionTarget::Projected {
            from,
            to,
            axis: SketchAxis::V,
        },
    );
}

#[test]
fn a_cursor_inside_the_box_of_the_ends_asks_for_the_length() {
    let (sketch, length, ..) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(50.0, 40.0));

    assert_eq!(sketch.oriented(length, DVec2::new(30.0, 25.0)), length);
    assert_eq!(sketch.oriented(length, DVec2::new(70.0, 60.0)), length);
}

#[test]
fn a_width_is_named_the_same_way_round_whichever_end_was_drawn_first() {
    let (start, end) = (DVec2::new(10.0, 10.0), DVec2::new(50.0, 40.0));
    let cursor = DVec2::new(30.0, 60.0);
    let (drawn_up, up, ..) = trait_from(start, end);

    let mut drawn_down = Sketch::new(WorkPlane::XY);
    let first = drawn_down.add_point(start);
    let second = drawn_down.add_point(end);
    let down = DimensionTarget::Length(drawn_down.add_segment(second, first));

    assert_eq!(
        drawn_up.oriented(up, cursor),
        drawn_down.oriented(down, cursor),
    );
}

#[test]
fn a_reading_already_asked_for_is_not_asked_for_a_second_time() {
    let (sketch, _, from, to) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(50.0, 40.0));
    let width = DimensionTarget::Projected {
        from,
        to,
        axis: SketchAxis::U,
    };

    assert_eq!(sketch.oriented(width, DVec2::new(30.0, 60.0)), width);
}

#[test]
fn a_segment_touches_its_own_ends_and_nothing_else() {
    let (mut sketch, length, from, to) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(50.0, 40.0));
    let DimensionTarget::Length(segment) = length else {
        unreachable!()
    };
    let apart = sketch.add_point(DVec2::new(0.0, 80.0));

    assert!(sketch.segment_touches(segment, from));
    assert!(sketch.segment_touches(segment, to));
    assert!(!sketch.segment_touches(segment, apart));
}

#[test]
fn the_horizontal_axis_is_under_a_cursor_on_it() {
    assert_eq!(axis_under(DVec2::new(30.0, 0.2), 0.5), Some(SketchAxis::U));
}

#[test]
fn the_vertical_axis_is_under_a_cursor_on_it() {
    assert_eq!(axis_under(DVec2::new(0.2, 30.0), 0.5), Some(SketchAxis::V));
}

#[test]
fn a_cursor_off_both_axes_is_on_neither() {
    assert_eq!(axis_under(DVec2::new(30.0, 30.0), 0.5), None);
}

fn right_angle_pair() -> (Sketch, SegmentId, SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(40.0, 0.0));
    let c = sketch.add_point(DVec2::new(40.0, 40.0));
    let first = sketch.add_segment(a, b);
    let second = sketch.add_segment(b, c);
    (sketch, first, second)
}

#[test]
fn a_length_clicked_twice_is_read_as_an_angle_when_a_second_trait_is_under_the_cursor() {
    let (sketch, first, second) = right_angle_pair();
    let target = DimensionTarget::Length(first);

    let refined = sketch
        .refine(target, DVec2::new(40.0, 20.0), 1.0)
        .expect("a second trait under the cursor refines the reading");

    assert_eq!(
        refined,
        DimensionTarget::Angle { first, second }.normalised()
    );
}

#[test]
fn a_length_clicked_with_nothing_else_under_the_cursor_stays_a_length() {
    let (sketch, first, _second) = right_angle_pair();

    assert_eq!(
        sketch.refine(
            DimensionTarget::Length(first),
            DVec2::new(500.0, 500.0),
            1.0
        ),
        None,
    );
}

#[test]
fn a_diameter_taken_back_to_the_centre_is_read_as_a_radius() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::new(100.0, 100.0));
    let circle = sketch.add_circle(center, 10.0);

    assert_eq!(
        sketch.refine(
            DimensionTarget::Diameter(circle),
            DVec2::new(100.05, 100.0),
            1.0
        ),
        Some(DimensionTarget::Radius(circle)),
    );
}

#[test]
fn a_radius_taken_on_to_one_of_the_arcs_own_ends_is_read_as_the_sweep() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let start = sketch.add_point(DVec2::new(10.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, 10.0));
    let arc = sketch.add_arc(centre, start, end);

    assert_eq!(
        sketch.refine(DimensionTarget::ArcRadius(arc), DVec2::new(10.05, 0.0), 1.0),
        Some(DimensionTarget::ArcSweep(arc)),
    );
    assert_eq!(
        sketch.refine(
            DimensionTarget::ArcRadius(arc),
            DVec2::new(500.0, 500.0),
            1.0
        ),
        None,
        "a click nowhere near either end asks for nothing more",
    );
}
