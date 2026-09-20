//! What app · screens/viewport/input/landing.rs is held to.

use super::*;
use cao_sketch::{CircleId, SegmentId, WorkPlane};

/// A trait with a circle crossing it, and nothing else.
fn a_trait_and_a_circle() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(0.0, 20.0));
    let east = sketch.add_point(DVec2::new(40.0, 20.0));
    sketch.add_segment(west, east);
    let centre = sketch.add_point(DVec2::new(10.0, 20.0));
    sketch.add_circle(centre, 5.0);
    sketch
}

#[test]
fn a_point_born_beside_the_drawing_is_held_by_nothing() {
    let sketch = a_trait_and_a_circle();

    assert_eq!(
        born_at(&sketch, DVec2::new(30.0, 35.0)),
        PointRef::New(DVec2::new(30.0, 35.0)),
    );
}

#[test]
fn a_point_born_on_a_trait_is_held_on_it() {
    let sketch = a_trait_and_a_circle();

    assert_eq!(
        born_at(&sketch, DVec2::new(30.0, 20.0)),
        PointRef::Held {
            at: DVec2::new(30.0, 20.0),
            on: vec![Support::Segment(SegmentId(0))],
        },
    );
}

#[test]
fn a_point_born_where_a_trait_and_a_circle_cross_is_held_on_both() {
    let sketch = a_trait_and_a_circle();

    assert_eq!(
        born_at(&sketch, DVec2::new(15.0, 20.0)),
        PointRef::Held {
            at: DVec2::new(15.0, 20.0),
            on: vec![Support::Segment(SegmentId(0)), Support::Circle(CircleId(0))],
        },
    );
}

#[test]
fn a_place_three_curves_run_through_holds_a_point_by_two_of_them() {
    let mut sketch = a_trait_and_a_circle();
    // A second trait down the middle of the first, through the same crossing.
    let south = sketch.add_point(DVec2::new(15.0, 0.0));
    let north = sketch.add_point(DVec2::new(15.0, 40.0));
    sketch.add_segment(south, north);

    let on = landed_on(&sketch, DVec2::new(15.0, 20.0));

    assert_eq!(on.len(), AT_A_CROSSING, "{on:?}");
}
