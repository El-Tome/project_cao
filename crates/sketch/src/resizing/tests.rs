use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget};
use crate::plane::WorkPlane;
use crate::resizing::Curved;
use crate::sketch::{Element, Sketch};

#[test]
fn a_circle_drawn_to_a_new_size_keeps_the_centre_it_had() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 30.0));
    let round = sketch.add_circle(centre, 10.0);

    sketch.resize_circle(round, 16.0, 1.0);

    assert!(
        (sketch.circle(round).radius - 16.0).abs() < 1e-9,
        "the circle stands {} out",
        sketch.circle(round).radius,
    );
    let stayed = sketch.point(centre);
    assert!(
        stayed.distance(DVec2::new(20.0, 30.0)) < 1e-9,
        "and its centre stayed where it was, not at {stayed}",
    );
}

#[test]
fn a_point_held_on_a_circle_follows_it_to_its_new_size() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 30.0));
    let round = sketch.add_circle(centre, 10.0);
    let on_rim = sketch.add_point(DVec2::new(30.0, 30.0));
    sketch.add_constraint(Constraint::OnCircle {
        point: on_rim,
        circle: round,
    });

    sketch.resize_circle(round, 16.0, 1.0);

    let reach = sketch.point(on_rim).distance(sketch.point(centre));
    assert!(
        (reach - 16.0).abs() < 1e-6,
        "the point held on the rim stands {reach} out, and the rim 16",
    );
}

#[test]
fn a_size_a_typed_value_forbids_leaves_the_circle_as_it_was() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 30.0));
    let round = sketch.add_circle(centre, 10.0);
    sketch.set_dimension(DimensionTarget::Diameter(round), 20.0, false);

    sketch.resize_circle(round, 16.0, 1.0);

    assert!(
        (sketch.circle(round).radius - 10.0).abs() < 1e-6,
        "the diameter was typed, and it is what the circle answers to: {}",
        sketch.circle(round).radius,
    );
}

#[test]
fn an_arc_drawn_to_a_new_size_keeps_the_sweep_it_was_drawn_with() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 30.0));
    let start = sketch.add_point(DVec2::new(30.0, 30.0));
    let end = sketch.add_point(DVec2::new(20.0, 40.0));
    let bend = sketch.add_arc(centre, start, end);
    let sweep = sketch.arc_sweep(bend);

    sketch.resize_arc(bend, 16.0, 1.0);

    let reach = sketch.arc_radius(bend);
    assert!(
        (reach - 16.0).abs() < 1e-6,
        "the curve stands {reach} out from its centre",
    );
    let now = sketch.arc_sweep(bend);
    assert!(
        (now - sweep).abs() < 1e-6,
        "and it still runs {sweep} round, not {now}",
    );
    let stayed = sketch.point(centre);
    assert!(
        stayed.distance(DVec2::new(20.0, 30.0)) < 1e-9,
        "about the centre it had, not {stayed}",
    );
}

#[test]
fn a_press_on_a_circles_outline_takes_hold_of_the_circle() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 30.0));
    let round = sketch.add_circle(centre, 10.0);

    assert_eq!(
        sketch.curve_at(DVec2::new(30.2, 30.0), 1.0),
        Some(Curved::Circle(round)),
    );
}

#[test]
fn a_press_inside_a_circle_takes_hold_of_no_curve() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 30.0));
    sketch.add_circle(centre, 10.0);

    assert_eq!(sketch.curve_at(DVec2::new(24.0, 30.0), 1.0), None);
}

#[test]
fn the_nearer_of_a_circle_and_an_arc_is_the_one_taken_hold_of() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 30.0));
    sketch.add_circle(centre, 10.0);
    let bend_centre = sketch.add_point(DVec2::new(20.0, 30.0));
    let start = sketch.add_point(DVec2::new(30.5, 30.0));
    let end = sketch.add_point(DVec2::new(20.0, 40.5));
    let bend = sketch.add_arc(bend_centre, start, end);

    assert_eq!(
        sketch.curve_at(DVec2::new(30.4, 30.0), 1.0),
        Some(Curved::Arc(bend)),
        "the arc runs a tenth nearer the press than the circle does",
    );
}

#[test]
fn an_arc_drawn_to_a_size_its_rules_forbid_is_left_as_it_was() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(20.0, 30.0));
    let start = sketch.add_point(DVec2::new(30.0, 30.0));
    let end = sketch.add_point(DVec2::new(20.0, 40.0));
    let bend = sketch.add_arc(centre, start, end);
    sketch.set_dimension(DimensionTarget::ArcRadius(bend), 10.0, false);
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Arc(bend),
    });
    let before = sketch.points().to_vec();

    sketch.resize_arc(bend, 16.0, 1.0);

    assert!(
        sketch
            .points()
            .iter()
            .zip(&before)
            .all(|(now, was)| now.distance(*was) < 1e-9),
        "nothing moved: {:?}",
        sketch.points()
    );
    assert!(
        (sketch.arc_radius(bend) - 10.0).abs() < 1e-6,
        "the radius typed is still true: {}",
        sketch.arc_radius(bend)
    );
}

#[test]
fn an_ellipse_drawn_to_a_new_size_reaches_it_about_its_centre() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let first = [DVec2::new(30.0, 50.0), DVec2::new(70.0, 50.0)].map(|at| sketch.add_point(at));
    let second = [DVec2::new(50.0, 40.0), DVec2::new(50.0, 60.0)].map(|at| sketch.add_point(at));
    let oval = sketch.add_ellipse(centre, first, second);

    sketch.resize_ellipse(oval, 30.0, 1.0);

    let reach = sketch.point(first[1]).distance(sketch.point(centre));
    assert!(
        (reach - 30.0).abs() < 1e-6,
        "its first axis reaches {reach}"
    );
    assert!(
        sketch.point(centre).distance(DVec2::new(50.0, 50.0)) < 1e-9,
        "about the centre it had, not {}",
        sketch.point(centre)
    );
}

#[test]
fn an_ellipse_drawn_to_a_size_its_rules_forbid_is_left_as_it_was() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let first = [DVec2::new(30.0, 50.0), DVec2::new(70.0, 50.0)].map(|at| sketch.add_point(at));
    let second = [DVec2::new(50.0, 40.0), DVec2::new(50.0, 60.0)].map(|at| sketch.add_point(at));
    let oval = sketch.add_ellipse(centre, first, second);
    sketch.set_dimension(
        DimensionTarget::Distance {
            from: centre,
            to: first[1],
        },
        20.0,
        false,
    );
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Ellipse(oval),
    });
    let before = sketch.points().to_vec();

    sketch.resize_ellipse(oval, 30.0, 1.0);

    assert!(
        sketch
            .points()
            .iter()
            .zip(&before)
            .all(|(now, was)| now.distance(*was) < 1e-9),
        "nothing moved: {:?}",
        sketch.points()
    );
}
