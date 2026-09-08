//! What a drawing says about itself follows every edit made to it.
//!
//! The verdict is read once per state of the drawing and kept until that state
//! changes, so each of these makes a change of one kind and asks again.

use cao_sketch::{DimensionTarget, Sketch, WorkPlane};
use glam::DVec2;

const SCALE: f64 = 1.0;

#[test]
fn a_length_added_after_the_drawing_was_read_settles_what_it_holds() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let end = sketch.add_point(DVec2::new(20.0, 0.0));
    let segment = sketch.add_segment(Sketch::ORIGIN, end);

    assert!(!sketch.settled_points(SCALE)[end.0]);

    sketch.set_dimension(DimensionTarget::Length(segment), 20.0, false);

    assert!(sketch.settled_points(SCALE)[end.0]);
}

#[test]
fn erasing_the_length_that_held_a_point_lets_it_move_again() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let end = sketch.add_point(DVec2::new(20.0, 0.0));
    let segment = sketch.add_segment(Sketch::ORIGIN, end);
    sketch.set_dimension(DimensionTarget::Length(segment), 20.0, false);

    assert!(sketch.settled_points(SCALE)[end.0]);

    sketch.erase_dimension(DimensionTarget::Length(segment));

    assert!(!sketch.settled_points(SCALE)[end.0]);
}

#[test]
fn a_point_drawn_after_the_drawing_was_read_is_read_too() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let end = sketch.add_point(DVec2::new(20.0, 0.0));
    let segment = sketch.add_segment(Sketch::ORIGIN, end);
    sketch.set_dimension(DimensionTarget::Length(segment), 20.0, false);

    assert_eq!(sketch.settled_points(SCALE).len(), 2);

    let loose = sketch.add_point(DVec2::new(90.0, 90.0));

    let settled = sketch.settled_points(SCALE);
    assert_eq!(settled.len(), 3);
    assert!(!settled[loose.0]);
}

#[test]
fn a_trait_turned_onto_an_axis_settles_without_a_value_being_added() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let diagonal = 28.284_271_247_461_902;
    let end = sketch.add_point(DVec2::new(20.0, 20.0));
    let segment = sketch.add_segment(Sketch::ORIGIN, end);
    sketch.set_dimension(DimensionTarget::Length(segment), diagonal, false);

    assert!(
        !sketch.settled_points(SCALE)[end.0],
        "a shape leaning at some other angle says nothing about which way up it is",
    );

    sketch.move_point(end, DVec2::new(diagonal, 0.0));

    assert!(
        sketch.settled_points(SCALE)[end.0],
        "laid along an axis it says so, and the length then leaves it nowhere to go",
    );
}
