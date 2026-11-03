//! What app · screens/viewport/input/landing.rs is held to.
//!
//! Closes #615.
//! - a point dropped on the middle mark of a trait carries the midpoint rule on
//!   that trait — `a_free_point_dropped_at_the_middle_of_a_trait_is_held_at_its_middle`;
//!   that it then follows the middle is held in `cao_part`, by
//!   `a_point_dropped_at_the_middle_follows_it_when_the_trait_stretches`
//! - that holds for the end of another trait —
//!   `the_end_of_another_trait_dropped_at_a_middle_is_held_there`
//! - for a point already on that trait —
//!   `a_point_slid_along_its_trait_to_the_middle_is_held_at_its_middle`
//! - and for a point held elsewhere —
//!   `a_point_held_elsewhere_dropped_at_a_middle_takes_the_middle_too`
//! - the midpoint rule replaces the hold on the trait — no test: held
//!   `holding_a_point_at_the_middle_takes_the_place_of_its_hold_on_the_trait`
//!   holds it in `cao_sketch`, and `cao_part` replays through it
//! - a point dropped anywhere else along a trait is held on it —
//!   `a_point_dropped_on_a_trait_the_drag_left_in_place_is_held_on_it`
//! - a drop where the middle of one trait crosses another is at that middle and
//!   on the other — `a_drop_where_a_middle_crosses_another_trait_is_at_that_middle_and_on_the_other`
//! - the drop and its rule are one step of the history — no test: held by
//!   `one_undo_takes_back_the_drop_and_its_midpoint_rule`, in `cao_part`
//! - a point dragged with the let-go key held behaves as it does today — no
//!   test: `dragging.rs` still asks for no hold while that key is down, and is
//!   not touched
//! - the « 1/2 » mark shows on the point straight away — no test:
//!   `rule_marks` already marks a midpoint rule, and the canvas has no net

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

#[test]
fn a_point_born_at_the_middle_of_a_trait_is_held_at_its_middle() {
    let sketch = a_trait_and_a_circle();

    assert_eq!(
        born_at(&sketch, DVec2::new(20.0, 20.0)),
        PointRef::Held {
            at: DVec2::new(20.0, 20.0),
            on: vec![Support::Middle(SegmentId(0))],
        },
    );
}

/// A trait lying flat from (10, 10) to (50, 10), its middle at (30, 10).
fn a_flat_trait() -> (Sketch, SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(10.0, 10.0));
    let east = sketch.add_point(DVec2::new(50.0, 10.0));
    let side = sketch.add_segment(west, east);
    (sketch, side)
}

const THE_MIDDLE: DVec2 = DVec2::new(30.0, 10.0);

fn dropped_at_the_middle(sketch: &Sketch, point: PointId) -> Vec<Support> {
    let mut settled = sketch.clone();
    let pull = settled.pull(point, 1.0);
    settled.settle_pulled(&pull, THE_MIDDLE, 1.0);
    assert!(
        settled.point(point).distance(THE_MIDDLE) < 1e-9,
        "the drag takes the point to the middle"
    );
    held_at_drop(sketch, &settled, point, THE_MIDDLE)
}

#[test]
fn a_free_point_dropped_at_the_middle_of_a_trait_is_held_at_its_middle() {
    let (mut sketch, side) = a_flat_trait();
    let lone = sketch.add_point(DVec2::new(30.0, 40.0));

    assert_eq!(
        dropped_at_the_middle(&sketch, lone),
        vec![Support::Middle(side)]
    );
}

#[test]
fn the_end_of_another_trait_dropped_at_a_middle_is_held_there() {
    let (mut sketch, side) = a_flat_trait();
    let foot = sketch.add_point(DVec2::new(30.0, 40.0));
    let head = sketch.add_point(DVec2::new(60.0, 60.0));
    sketch.add_segment(foot, head);

    assert_eq!(
        dropped_at_the_middle(&sketch, foot),
        vec![Support::Middle(side)]
    );
}

#[test]
fn a_point_slid_along_its_trait_to_the_middle_is_held_at_its_middle() {
    let (mut sketch, side) = a_flat_trait();
    let on_it = sketch.add_point(DVec2::new(20.0, 10.0));
    sketch.add_constraint(Support::Segment(side).holding(on_it));

    assert_eq!(
        dropped_at_the_middle(&sketch, on_it),
        vec![Support::Middle(side)]
    );
}

#[test]
fn a_point_held_elsewhere_dropped_at_a_middle_takes_the_middle_too() {
    let (mut sketch, side) = a_flat_trait();
    let centre = sketch.add_point(DVec2::new(30.0, 30.0));
    let round = sketch.add_circle(centre, 20.0);
    let on_the_round = sketch.add_point(DVec2::new(50.0, 30.0));
    sketch.add_constraint(Support::Circle(round).holding(on_the_round));

    assert_eq!(
        dropped_at_the_middle(&sketch, on_the_round),
        vec![Support::Middle(side)],
        "what already held it stays as it was, and is not laid a second time"
    );
}

#[test]
fn a_drop_where_a_middle_crosses_another_trait_is_at_that_middle_and_on_the_other() {
    let (mut sketch, side) = a_flat_trait();
    let below = sketch.add_point(DVec2::new(30.0, 0.0));
    let above = sketch.add_point(DVec2::new(30.0, 40.0));
    let upright = sketch.add_segment(below, above);
    let lone = sketch.add_point(DVec2::new(45.0, 40.0));

    assert_eq!(
        dropped_at_the_middle(&sketch, lone),
        vec![Support::Middle(side), Support::Segment(upright)]
    );
}
