//! What sketch · circling.rs is held to.

use super::*;
use crate::plane::WorkPlane;

#[test]
fn a_three_point_circle_waits_for_two_places_before_it_draws() {
    let sketch = Sketch::new(WorkPlane::XY);
    let cursor = DVec2::new(10.0, 10.0);

    assert_eq!(
        circle_progress(CircleMode::ThreePoints, &[], &[], &sketch, cursor, 1.0),
        CircleProgress::AddPoint(cursor),
        "the first of three places only waits for the next",
    );
    assert_eq!(
        circle_progress(
            CircleMode::ThreePoints,
            &[DVec2::new(0.0, 0.0)],
            &[],
            &sketch,
            cursor,
            1.0,
        ),
        CircleProgress::AddPoint(cursor),
        "the second of three places still only waits",
    );
    assert_eq!(
        circle_progress(
            CircleMode::ThreePoints,
            &[DVec2::new(0.0, 0.0), DVec2::new(20.0, 0.0)],
            &[],
            &sketch,
            cursor,
            1.0,
        ),
        CircleProgress::Ready,
        "the third click has enough to settle a circle",
    );
}

#[test]
fn a_construction_built_from_traits_asks_for_one_when_none_is_under_the_cursor() {
    let sketch = Sketch::new(WorkPlane::XY);

    assert_eq!(
        circle_progress(
            CircleMode::ThreeTangents,
            &[],
            &[],
            &sketch,
            DVec2::new(10.0, 10.0),
            1.0,
        ),
        CircleProgress::NeedsSegment,
    );
}

#[test]
fn a_trait_already_picked_is_told_apart_from_one_freshly_found() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(20.0, 0.0));
    let segment = sketch.add_segment(a, b);
    let midpoint = DVec2::new(10.0, 0.0);

    assert_eq!(
        circle_progress(
            CircleMode::TwoTangents,
            &[],
            &[segment],
            &sketch,
            midpoint,
            1.0
        ),
        CircleProgress::AlreadyPicked,
    );
}

#[test]
fn a_two_point_construction_is_ready_as_soon_as_it_has_its_first_point() {
    let sketch = Sketch::new(WorkPlane::XY);

    assert_eq!(
        circle_progress(
            CircleMode::TwoPoints,
            &[DVec2::new(0.0, 0.0)],
            &[],
            &sketch,
            DVec2::new(10.0, 10.0),
            1.0,
        ),
        CircleProgress::Ready,
    );
}

#[test]
fn a_diameter_too_small_to_reach_both_points_is_held_at_the_smallest_that_does() {
    let (first, second) = (DVec2::new(-50.0, 0.0), DVec2::new(50.0, 0.0));
    let places = [first, second];

    let squeezed = circle_from(
        CircleMode::ThreePoints,
        &places,
        &[],
        DVec2::new(0.0, 10.0),
        Some(20.0),
        1.0,
    )
    .expect("two points and a cursor make a circle");

    assert!(
        (squeezed.radius - 50.0).abs() < 1e-9,
        "typing 150 goes through 1 and 15 on the way, and a circle that \
         vanishes at the first keystroke takes the field with it: got {}",
        squeezed.radius,
    );

    let roomy = circle_from(
        CircleMode::ThreePoints,
        &places,
        &[],
        DVec2::new(0.0, 10.0),
        Some(200.0),
        1.0,
    )
    .expect("two points and a cursor make a circle");

    assert!(
        (roomy.radius - 100.0).abs() < 1e-9,
        "a size that does reach both points is taken as typed: got {}",
        roomy.radius,
    );
}
