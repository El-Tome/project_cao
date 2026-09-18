//! What a rule with no value holds.

use super::*;

#[test]
fn a_right_angle_can_be_asked_for_without_a_value() {
    let (mut sketch, first, second) = corner();
    sketch.add_constraint(Constraint::Perpendicular { first, second });
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    let square = direction(&sketch, first).dot(direction(&sketch, second));
    assert!(square.abs() < 1e-6, "dot product = {square}");
}

#[test]
fn two_traits_can_be_told_to_keep_the_same_direction() {
    let (mut sketch, first, second) = corner();
    sketch.add_constraint(Constraint::Parallel { first, second });
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    let across = direction(&sketch, first).perp_dot(direction(&sketch, second));
    assert!(across.abs() < 1e-6, "cross product = {across}");
}

#[test]
fn two_traits_can_be_told_to_have_the_same_length() {
    let (mut sketch, first, second) = corner();
    sketch.add_constraint(Constraint::Equal { first, second });
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    let gap = sketch.segment_length(first) - sketch.segment_length(second);
    assert!(gap.abs() < 1e-4, "length gap = {gap}");
}

#[test]
fn a_point_can_be_held_on_a_line() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(100.0, 20.0));
    let line = sketch.add_segment(start, end);
    let floating = sketch.add_point(DVec2::new(40.0, 50.0));

    sketch.add_constraint(Constraint::OnSegment {
        point: floating,
        segment: line,
    });
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    let gap = sketch.point_to_segment(floating, line).unwrap();
    assert!(gap < 1e-6, "the point is {gap} away from the line");
}

#[test]
fn a_point_can_be_held_halfway_along_a_trait() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(80.0, 40.0));
    let line = sketch.add_segment(start, end);
    let floating = sketch.add_point(DVec2::new(10.0, 60.0));

    sketch.add_constraint(Constraint::Midpoint {
        point: floating,
        segment: line,
    });
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    let middle = (sketch.point(start) + sketch.point(end)) * 0.5;
    assert!(sketch.point(floating).distance(middle) < 1e-6);
}

#[test]
fn a_circle_brushes_a_line_drawn_the_other_way_round_too() {
    // The trait runs right to left, which puts the circle on the negative
    // side of it. Growing the circle then closes the gap the other way; a
    // size correction taken with the wrong sign sends the radius running.
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(40.0, 0.0));
    let end = sketch.add_point(DVec2::new(-40.0, 0.0));
    let line = sketch.add_segment(start, end);
    let center = sketch.add_point(DVec2::new(0.0, 30.0));
    let circle = sketch.add_circle(center, 12.0);
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Segment(line),
    });
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Circle(circle),
    });
    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment: line,
        at: None,
    });

    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);
    let round = sketch.circle(circle);
    assert!(
        (round.radius - 30.0).abs() < 1e-3,
        "the radius reached the line: {}",
        round.radius
    );
}

#[test]
fn a_tangency_keeps_a_point_where_the_two_touch() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(-40.0, 0.0));
    let end = sketch.add_point(DVec2::new(40.0, 0.0));
    let line = sketch.add_segment(start, end);
    let center = sketch.add_point(DVec2::new(0.0, 30.0));
    let circle = sketch.add_circle(center, 25.0);

    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment: line,
        at: None,
    });
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    let Some(Constraint::Tangent {
        at: Some(touch), ..
    }) = sketch
        .constraints()
        .iter()
        .find(|rule| matches!(rule, Constraint::Tangent { .. }))
        .copied()
    else {
        panic!("the tangency has no contact point");
    };
    let contact = sketch.point(touch);
    let foot = sketch.foot_on_segment(center, line).unwrap();
    assert!(contact.distance(foot) < 1e-3, "the contact slid: {contact}");

    // The line moved out from under it: the contact follows, it does not
    // stay behind on the old spot.
    sketch.settle_around(end, DVec2::new(40.0, 40.0), 1.0);
    let foot = sketch
        .foot_on_segment(sketch.circle(circle).center, line)
        .unwrap();
    assert!(sketch.point(touch).distance(foot) < 1e-2);
}

#[test]
fn a_circle_told_to_brush_a_line_takes_the_size_that_touches() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(100.0, 0.0));
    let line = sketch.add_segment(start, end);
    let center = sketch.add_point(DVec2::new(50.0, 30.0));
    let circle = sketch.add_circle(center, 20.0);

    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment: line,
        at: None,
    });
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    // Nothing says how big it is, so the circle meets the line by moving
    // and by growing at once — whichever costs least.
    let gap = sketch.point_to_segment(center, line).unwrap();
    assert!(
        (gap - sketch.circle(circle).radius).abs() < 1e-6,
        "the circle does not touch: {gap} against {}",
        sketch.circle(circle).radius
    );
    assert!(sketch.circle(circle).radius > 20.0, "it did grow");
}

#[test]
fn a_circle_of_a_said_size_moves_to_keep_touching() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(100.0, 0.0));
    let line = sketch.add_segment(start, end);
    let center = sketch.add_point(DVec2::new(50.0, 30.0));
    let circle = sketch.add_circle(center, 20.0);

    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment: line,
        at: None,
    });
    sketch.set_dimension(DimensionTarget::Radius(circle), 20.0, false);
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    // The size is the decision now, so the circle comes down to the line.
    let held = sketch.circle(circle).radius;
    assert!((held - 20.0).abs() < 1e-3, "radius = {held}");
    let gap = sketch.point_to_segment(center, line).unwrap();
    assert!((gap - 20.0).abs() < 1e-3, "distance to the centre = {gap}");
}

#[test]
fn an_inscribed_circle_follows_the_triangle_it_sits_in() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(200.0, 0.0));
    let c = sketch.add_point(DVec2::new(100.0, 150.0));
    let sides = [
        sketch.add_segment(a, b),
        sketch.add_segment(b, c),
        sketch.add_segment(c, a),
    ];
    let (place, radius) = crate::construct::circle_touching_three(
        (sketch.point(a), sketch.point(b)),
        (sketch.point(b), sketch.point(c)),
        (sketch.point(c), sketch.point(a)),
    )
    .unwrap();
    let center = sketch.add_point(place);
    let circle = sketch.add_circle(center, radius);
    for segment in sides {
        sketch.add_constraint(Constraint::Tangent {
            circle,
            segment,
            at: None,
        });
    }

    sketch.set_dimension(DimensionTarget::Length(sides[0]), 300.0, false);
    sketch.resolve(1.0);

    let held = sketch.circle(circle).radius;
    for segment in sides {
        let gap = sketch.point_to_segment(center, segment).unwrap();
        assert!(
            (gap - held).abs() < 0.05,
            "the circle is {gap} from a side for a radius of {held}"
        );
    }
    assert!((sketch.segment_length(sides[0]) - 300.0).abs() < 0.05);
}

#[test]
fn two_traits_can_be_laid_on_the_same_line() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(50.0, 0.0));
    let c = sketch.add_point(DVec2::new(70.0, 18.0));
    let d = sketch.add_point(DVec2::new(120.0, 30.0));
    let first = sketch.add_segment(a, b);
    let second = sketch.add_segment(c, d);

    sketch.add_constraint(Constraint::Collinear { first, second });
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    for point in [c, d] {
        let gap = sketch.point_to_segment(point, first).unwrap();
        assert!(gap < 1e-3, "one end is {gap} away from the other line");
    }
}

#[test]
fn the_second_trait_takes_the_length_of_the_first() {
    let (mut sketch, first, second) = corner();
    let wanted = sketch.segment_length(first);
    sketch.add_constraint(Constraint::Equal { first, second });
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    assert!(
        (sketch.segment_length(first) - wanted).abs() < 1e-9,
        "the first segment moved"
    );
    assert!((sketch.segment_length(second) - wanted).abs() < 1e-4);
}

#[test]
fn a_trait_can_be_laid_on_an_axis_of_the_sketch() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(10.0, 12.0));
    let end = sketch.add_point(DVec2::new(70.0, 40.0));
    let line = sketch.add_segment(start, end);

    sketch.add_constraint(Constraint::AxisCollinear {
        segment: line,
        axis: SketchAxis::U,
    });
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    assert!(sketch.point(start).y.abs() < 1e-6);
    assert!(sketch.point(end).y.abs() < 1e-6);
}

#[test]
fn fixing_a_trait_holds_both_of_its_ends() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(10.0, 10.0));
    let end = sketch.add_point(DVec2::new(60.0, 10.0));
    let line = sketch.add_segment(start, end);
    let far = sketch.add_point(DVec2::new(60.0, 60.0));
    let other = sketch.add_segment(end, far);

    sketch.add_constraint(Constraint::Fixed {
        element: Element::Segment(line),
    });
    sketch.set_dimension(DimensionTarget::Length(other), 90.0, false);
    sketch.resolve(1.0);

    assert!(sketch.point(start).distance(DVec2::new(10.0, 10.0)) < 1e-9);
    assert!(sketch.point(end).distance(DVec2::new(60.0, 10.0)) < 1e-9);
    assert!((sketch.segment_length(other) - 90.0).abs() < 1e-4);
}

#[test]
fn a_fixed_point_stays_where_it_was_put() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(10.0, 10.0));
    let end = sketch.add_point(DVec2::new(60.0, 10.0));
    let line = sketch.add_segment(start, end);

    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(start),
    });
    sketch.set_dimension(DimensionTarget::Length(line), 90.0, false);
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    assert!(sketch.point(start).distance(DVec2::new(10.0, 10.0)) < 1e-9);
    assert!((sketch.segment_length(line) - 90.0).abs() < 1e-4);
}

#[test]
fn a_rule_goes_when_what_it_spoke_of_goes() {
    let (mut sketch, first, second) = corner();
    sketch.add_constraint(Constraint::Perpendicular { first, second });
    assert_eq!(sketch.constraints().len(), 1);

    sketch.erase(Element::Segment(second));
    assert!(sketch.constraints().is_empty());
}
