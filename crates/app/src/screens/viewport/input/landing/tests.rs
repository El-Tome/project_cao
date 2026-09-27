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

#[test]
fn an_end_of_a_trait_dropped_back_along_it_is_held_by_nothing() {
    let sketch = a_trait_and_a_circle();
    let far = sketch.segments()[0].end;

    let held = dropped_on(&sketch, far, DVec2::new(30.0, 20.0));

    assert!(
        held.is_empty(),
        "a trait runs through its own ends whatever they do, so landing on it \
         holds that end to nothing: {held:?}",
    );
}

#[test]
fn a_corner_dropped_where_a_trait_stood_before_the_drag_moved_it_is_held_by_nothing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(20.0, 20.0),
        DVec2::new(120.0, 20.0),
        DVec2::new(120.0, 70.0),
        DVec2::new(20.0, 70.0),
    ]
    .map(|place| sketch.add_point(place));
    let sides: [SegmentId; 4] =
        std::array::from_fn(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 4]));
    for corner in 0..3 {
        sketch.add_constraint(cao_sketch::Constraint::Perpendicular {
            first: sides[corner],
            second: sides[corner + 1],
        });
    }
    let tip = sketch.add_point(DVec2::new(140.0, 60.0));
    sketch.add_segment(corners[1], tip);
    let landing = DVec2::new(130.0, 40.0);
    let mut settled = sketch.clone();
    let pull = settled.pull(corners[2], 1.0);
    settled.settle_pulled(&pull, landing, 1.0);

    assert!(
        !dropped_on(&sketch, corners[2], landing).is_empty(),
        "the tail ran through the place before the drag"
    );
    assert!(
        held_at_drop(&sketch, &settled, corners[2], landing).is_empty(),
        "the drag took it away, and the drop is held by nothing"
    );
}

#[test]
fn a_point_dropped_on_a_trait_the_drag_left_in_place_is_held_on_it() {
    let mut sketch = a_trait_and_a_circle();
    let lone = sketch.add_point(DVec2::new(30.0, 40.0));
    let landing = DVec2::new(30.0, 20.0);
    let mut settled = sketch.clone();
    let pull = settled.pull(lone, 1.0);
    settled.settle_pulled(&pull, landing, 1.0);

    let held = held_at_drop(&sketch, &settled, lone, landing);

    assert_eq!(held, vec![Support::Segment(SegmentId(0))]);
}

#[test]
fn the_middle_of_a_trait_carried_sideways_is_not_held_on_it_again() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(10.0, 10.0));
    let east = sketch.add_point(DVec2::new(50.0, 10.0));
    let line = sketch.add_segment(west, east);
    let middle = sketch.add_point(DVec2::new(30.0, 10.0));
    sketch.add_constraint(cao_sketch::Constraint::Midpoint {
        point: middle,
        segment: line,
    });
    let landing = DVec2::new(30.0, 40.0);
    let mut settled = sketch.clone();
    let pull = settled.pull(middle, 1.0);
    settled.settle_pulled(&pull, landing, 1.0);

    assert!(
        held_at_drop(&sketch, &settled, middle, landing).is_empty(),
        "it stands on its trait by its own rule, not by where it was dropped"
    );
}
