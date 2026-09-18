//! What sketch · rule_marks.rs is held to.

use super::*;
use crate::plane::WorkPlane;

#[test]
fn a_right_angle_writes_its_mark_in_the_corner() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let pivot = sketch.add_point(DVec2::new(0.0, 0.0));
    let a = sketch.add_point(DVec2::new(40.0, 0.0));
    let b = sketch.add_point(DVec2::new(0.0, 20.0));
    let first = sketch.add_segment(pivot, a);
    let second = sketch.add_segment(pivot, b);
    let constraint = Constraint::Perpendicular { first, second };
    sketch.add_constraint(constraint);

    let marks = sketch.rule_marks(constraint);
    assert_eq!(
        marks.len(),
        1,
        "a right angle writes one mark, in the corner"
    );
    let mark = marks[0];
    assert!(
        mark.x > 0.0 && mark.x < 20.0 && mark.y > 0.0 && mark.y < 20.0,
        "the mark should sit inside the corner, not on the traits: {mark}"
    );
}

#[test]
fn two_traits_held_square_without_touching_write_on_themselves() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(40.0, 0.0));
    let c = sketch.add_point(DVec2::new(0.0, 30.0));
    let d = sketch.add_point(DVec2::new(0.0, 70.0));
    let first = sketch.add_segment(a, b);
    let second = sketch.add_segment(c, d);
    let constraint = Constraint::Perpendicular { first, second };
    sketch.add_constraint(constraint);

    let marks = sketch.rule_marks(constraint);
    assert_eq!(
        marks,
        vec![DVec2::new(20.0, 0.0), DVec2::new(0.0, 50.0)],
        "with no shared corner, each trait carries its own mark, at its middle"
    );
}

#[test]
fn a_tangency_writes_where_the_circle_touches() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(-40.0, 0.0));
    let end = sketch.add_point(DVec2::new(40.0, 0.0));
    let line = sketch.add_segment(start, end);
    let center = sketch.add_point(DVec2::new(0.0, 25.0));
    let circle = sketch.add_circle(center, 25.0);

    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment: line,
        at: None,
    });
    let constraint = *sketch
        .constraints()
        .iter()
        .find(|rule| matches!(rule, Constraint::Tangent { .. }))
        .expect("the tangency was added");

    let marks = sketch.rule_marks(constraint);
    assert_eq!(
        marks,
        vec![DVec2::new(0.0, 0.0)],
        "the mark sits where the circle actually touches the line"
    );
}

#[test]
fn a_rule_whose_mark_is_far_from_the_cursor_is_not_picked() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(40.0, 0.0));
    let c = sketch.add_point(DVec2::new(0.0, 30.0));
    let d = sketch.add_point(DVec2::new(0.0, 70.0));
    let first = sketch.add_segment(a, b);
    let second = sketch.add_segment(c, d);
    sketch.add_constraint(Constraint::Parallel { first, second });

    assert_eq!(sketch.nearest_rule(DVec2::new(200.0, 200.0), 5.0), None);
    assert!(sketch.nearest_rule(DVec2::new(20.0, 1.0), 5.0).is_some());
}
