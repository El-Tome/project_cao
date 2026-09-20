//! What sketch · corner.rs is held to.

use glam::DVec2;

use super::*;
use crate::plane::WorkPlane;

const CORNER: DVec2 = DVec2::new(2.0, 1.0);

/// A right angle, one side running east and the other north.
fn a_right_angle() -> (Sketch, SegmentId, SegmentId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let pivot = sketch.add_point(CORNER);
    let east = sketch.add_point(CORNER + DVec2::new(10.0, 0.0));
    let north = sketch.add_point(CORNER + DVec2::new(0.0, 10.0));
    let along = sketch.add_segment(pivot, east);
    let up = sketch.add_segment(pivot, north);
    (sketch, along, up, pivot)
}

#[test]
fn a_point_where_two_traits_meet_is_a_corner_and_names_them_both() {
    let (sketch, along, up, pivot) = a_right_angle();

    assert_eq!(sketch.corner_at(pivot), Some((along, up)));
}

#[test]
fn a_point_where_a_third_trait_runs_in_is_no_corner_to_take() {
    let (mut sketch, _along, _up, pivot) = a_right_angle();
    let away = sketch.add_point(CORNER + DVec2::new(-10.0, -10.0));
    sketch.add_segment(pivot, away);

    assert_eq!(
        sketch.corner_at(pivot),
        None,
        "three traits leave no saying which two the cut is meant for"
    );
}

#[test]
fn an_end_of_a_single_trait_is_no_corner_to_take() {
    let (sketch, along, _up, _pivot) = a_right_angle();
    let lonely = sketch.segments()[along.0].end;

    assert_eq!(sketch.corner_at(lonely), None);
}

#[test]
fn a_point_a_curve_runs_into_is_no_corner_to_take() {
    let (mut sketch, _along, _up, pivot) = a_right_angle();
    let centre = sketch.add_point(CORNER + DVec2::new(-5.0, 0.0));
    let far = sketch.add_point(CORNER + DVec2::new(-10.0, 0.0));
    sketch.add_arc(centre, pivot, far);

    assert_eq!(
        sketch.corner_at(pivot),
        None,
        "a corner is still two straight traits, and a curve ending here is \
         neither one of them nor nothing"
    );
}

#[test]
fn a_trait_erased_no_longer_makes_a_corner_of_the_point_it_left() {
    let (mut sketch, _along, up, pivot) = a_right_angle();

    sketch.erase(crate::sketch::Element::Segment(up));

    assert_eq!(sketch.corner_at(pivot), None);
}
