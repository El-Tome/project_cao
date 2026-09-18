//! What sketch · solver/arc_solver.rs is held to.

use glam::DVec2;

use super::*;
use crate::element::Element;
use crate::plane::WorkPlane;
use crate::sketch::Sketch;

#[test]
fn a_radius_dimension_on_an_arc_holds_while_the_drawing_moves() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(10.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, 10.0));
    let arc = sketch.add_arc(Sketch::ORIGIN, start, end);

    sketch.set_dimension(DimensionTarget::ArcRadius(arc), 25.0, false);
    sketch.resolve(1.0);

    assert!(
        (sketch.arc_radius(arc) - 25.0).abs() < 1e-2,
        "radius = {}",
        sketch.arc_radius(arc),
    );
}

#[test]
fn a_swept_angle_dimension_on_an_arc_holds() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let start = sketch.add_point(DVec2::new(10.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, 10.0));
    let arc = sketch.add_arc(centre, start, end);

    sketch.set_dimension(DimensionTarget::ArcSweep(arc), 150.0, false);
    sketch.resolve(1.0);

    let swept = sketch.arc_sweep(arc).to_degrees();
    assert!((swept - 150.0).abs() < 1e-2, "sweep = {swept}");
}

#[test]
fn an_arc_told_tangent_to_a_line_settles_onto_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let left = sketch.add_point(DVec2::new(-40.0, 20.0));
    let right = sketch.add_point(DVec2::new(40.0, 20.0));
    let line = sketch.add_segment(left, right);
    let start = sketch.add_point(DVec2::new(8.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, 8.0));
    let arc = sketch.add_arc(Sketch::ORIGIN, start, end);
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Segment(line),
    });

    sketch.add_constraint(Constraint::ArcTangent {
        arc,
        segment: line,
        at: None,
    });
    sketch.resolve(1.0);

    let gap = sketch.point_to_segment(Sketch::ORIGIN, line).unwrap();
    assert!(
        (gap - sketch.arc_radius(arc)).abs() < 1e-2,
        "gap {gap} against radius {}",
        sketch.arc_radius(arc),
    );
}

#[test]
fn two_arcs_told_equal_radius_settle_to_the_same_reach() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre_one = sketch.add_point(DVec2::new(-50.0, 0.0));
    let start_one = sketch.add_point(DVec2::new(-40.0, 0.0));
    let end_one = sketch.add_point(DVec2::new(-50.0, 10.0));
    let one = sketch.add_arc(centre_one, start_one, end_one);
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(centre_one),
    });

    let centre_two = sketch.add_point(DVec2::new(50.0, 0.0));
    let start_two = sketch.add_point(DVec2::new(80.0, 0.0));
    let end_two = sketch.add_point(DVec2::new(50.0, 30.0));
    let two = sketch.add_arc(centre_two, start_two, end_two);
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(centre_two),
    });

    sketch.add_constraint(Constraint::EqualRadiusArc {
        first: one,
        second: two,
    });
    sketch.resolve(1.0);

    assert!(
        (sketch.arc_radius(one) - sketch.arc_radius(two)).abs() < 1e-2,
        "one = {}, two = {}",
        sketch.arc_radius(one),
        sketch.arc_radius(two),
    );
}
