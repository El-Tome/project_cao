//! How a drawing settles when it is pulled about.

use super::*;

#[test]
fn a_figure_adrift_is_never_settled_however_it_is_fixed() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(100.0, 100.0));
    let b = sketch.add_point(DVec2::new(200.0, 110.0));
    let c = sketch.add_point(DVec2::new(150.0, 190.0));
    for (start, end, length) in [(a, b, 100.0), (b, c, 95.0), (c, a, 90.0)] {
        let side = sketch.add_segment(start, end);
        sketch.set_dimension(DimensionTarget::Length(side), length, false);
    }
    sketch.resolve(1.0);

    // Drawn away from the origin and leaning, it can still be slid about
    // and turned: two ways of sliding, one of turning.
    assert_eq!(sketch.freedom(1.0).degrees_of_freedom, 3);

    // A Fixe holds it still while the drawing settles, but it anchors it to
    // nothing: the figure could be anywhere on the plane.
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(a),
    });
    assert_eq!(
        sketch.freedom(1.0).degrees_of_freedom,
        3,
        "pinning one corner ties nothing down"
    );
}

#[test]
fn a_shape_leaning_has_to_say_which_way_up_it_is() {
    // Two traits from the origin, at a right angle to each other and both
    // measured: everything about the shape is said except the way it lies.
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(80.0, 30.0));
    let far = sketch.add_point(DVec2::new(50.0, 110.0));
    let along = sketch.add_segment(Sketch::ORIGIN, corner);
    let across = sketch.add_segment(corner, far);
    sketch.set_dimension(DimensionTarget::Length(along), 90.0, false);
    sketch.set_dimension(DimensionTarget::Length(across), 60.0, false);
    sketch.set_dimension(
        DimensionTarget::Angle {
            first: along,
            second: across,
        },
        90.0,
        false,
    );
    sketch.resolve(1.0);
    assert_eq!(
        sketch.freedom(1.0).degrees_of_freedom,
        1,
        "which way up it lies is still to be said"
    );

    // Said with an angle against an axis, and there is nothing left.
    sketch.set_dimension(
        DimensionTarget::AxisAngle {
            segment: along,
            axis: crate::constraints::SketchAxis::U,
        },
        0.0,
        false,
    );
    sketch.resolve(1.0);
    assert!(sketch.is_fully_constrained(1.0), "and now it is laid down");
}

#[test]
fn a_drag_that_cannot_be_had_leaves_the_drawing_whole() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(-80.0, -50.0),
        DVec2::new(90.0, -40.0),
        DVec2::new(10.0, 80.0),
    ];
    let ids: Vec<_> = corners
        .iter()
        .map(|place| sketch.add_point(*place))
        .collect();
    let sides: Vec<_> = [(0, 1), (1, 2), (2, 0)]
        .iter()
        .map(|(a, b)| sketch.add_segment(ids[*a], ids[*b]))
        .collect();
    let (place, radius) = crate::construct::circle_touching_three(
        (corners[0], corners[1]),
        (corners[1], corners[2]),
        (corners[2], corners[0]),
    )
    .unwrap();
    let center = sketch.add_point(place);
    let circle = sketch.add_circle(center, radius);
    for segment in &sides {
        sketch.add_constraint(Constraint::Tangent {
            circle,
            segment: *segment,
            at: None,
        });
    }
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Circle(circle),
    });
    sketch.set_dimension(DimensionTarget::Diameter(circle), 100.0, false);
    sketch.resolve(1.0);

    // Dragged somewhere the two sides through it cannot follow while the
    // circle stays put and keeps its size.
    sketch.settle_around(ids[1], DVec2::new(900.0, -600.0), 1.0);

    let wanted = sketch.circle(circle).radius;
    assert!(
        (wanted - 50.0).abs() < 0.5,
        "the circle keeps its size: {wanted}"
    );
    for (rank, side) in sides.iter().enumerate() {
        let gap = sketch.point_to_segment(center, *side).unwrap();
        assert!(
            (gap - wanted).abs() < 0.5,
            "side {rank} still touches: {gap} instead of {wanted}"
        );
    }
}

#[test]
fn a_whole_figure_moved_at_once_keeps_its_shape() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(40.0, 40.0),
        DVec2::new(140.0, 40.0),
        DVec2::new(140.0, 110.0),
        DVec2::new(40.0, 110.0),
    ];
    let ids: Vec<_> = corners
        .iter()
        .map(|place| sketch.add_point(*place))
        .collect();
    let sides: Vec<_> = (0..4)
        .map(|rank| sketch.add_segment(ids[rank], ids[(rank + 1) % 4]))
        .collect();
    for pair in 0..3 {
        sketch.set_dimension(
            DimensionTarget::Angle {
                first: sides[pair],
                second: sides[pair + 1],
            },
            90.0,
            false,
        );
    }
    sketch.resolve(1.0);
    let before: Vec<f64> = sides.iter().map(|s| sketch.segment_length(*s)).collect();

    let step = DVec2::new(-70.0, 55.0);
    let dropped: Vec<(PointId, DVec2)> = ids
        .iter()
        .map(|point| (*point, sketch.point(*point) + step))
        .collect();
    sketch.settle_around_all(&dropped, 1.0);

    for (rank, point) in ids.iter().enumerate() {
        assert!(
            sketch.point(*point).distance(corners[rank] + step) < 1e-6,
            "corner {rank} was carried across as it was"
        );
    }
    for (rank, side) in sides.iter().enumerate() {
        assert!(
            (sketch.segment_length(*side) - before[rank]).abs() < 1e-6,
            "and side {rank} was not stretched"
        );
    }
}

#[test]
fn a_point_on_the_rim_resizes_the_circle_when_it_is_pulled() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::new(0.0, 0.0));
    let circle = sketch.add_circle(center, 10.0);
    let rim = sketch.add_point(DVec2::new(10.0, 0.0));
    sketch.add_constraint(Constraint::OnCircle { point: rim, circle });

    sketch.settle_around(rim, DVec2::new(30.0, 0.0), 1.0);

    assert!(
        sketch.point(rim).distance(DVec2::new(30.0, 0.0)) < 1e-9,
        "the dropped point does not move any more"
    );
    let round = sketch.circle(circle);
    let reach = sketch.point(rim).distance(sketch.point(round.center));
    assert!(
        (reach - round.radius).abs() < 1e-3,
        "the point stayed on the rim"
    );
    assert!(round.radius > 10.0, "the circle grew: {}", round.radius);
}

#[test]
fn a_circle_dragged_by_its_centre_stays_under_the_cursor() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(-60.0, 0.0));
    let end = sketch.add_point(DVec2::new(60.0, 0.0));
    let line = sketch.add_segment(start, end);
    let center = sketch.add_point(DVec2::new(0.0, 20.0));
    let circle = sketch.add_circle(center, 20.0);
    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment: line,
        at: None,
    });

    let dropped = DVec2::new(35.0, 45.0);
    sketch.settle_around(center, dropped, 1.0);

    assert!(
        sketch.point(center).distance(dropped) < 1e-9,
        "the centre slid under the cursor: {}",
        sketch.point(center)
    );
    let gap = sketch.point_to_segment(center, line).unwrap();
    assert!(
        (gap - sketch.circle(circle).radius).abs() < 1e-2,
        "and the tangency still holds"
    );
}

#[test]
fn a_line_pulls_along_its_whole_body() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let end = sketch.add_point(DVec2::new(100.0, 0.0));
    let side = sketch.add_segment(Sketch::ORIGIN, end);

    let (found, at) = sketch
        .nearest_on_segment(DVec2::new(30.0, 2.0), 5.0)
        .expect("the segment pulls");
    assert_eq!(found, side);
    assert!(at.distance(DVec2::new(30.0, 0.0)) < 1e-4, "{at:?}");

    assert_eq!(sketch.nearest_on_segment(DVec2::new(30.0, 40.0), 5.0), None);
    // Past the end, the pull stops at the end rather than off in space.
    let (_, beyond) = sketch
        .nearest_on_segment(DVec2::new(104.0, 0.0), 5.0)
        .expect("the end still pulls");
    assert!(beyond.distance(DVec2::new(100.0, 0.0)) < 1e-4);
}

#[test]
fn the_middle_of_a_line_is_its_own_catch() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let end = sketch.add_point(DVec2::new(100.0, 0.0));
    let side = sketch.add_segment(Sketch::ORIGIN, end);

    let (found, at) = sketch
        .nearest_midpoint(DVec2::new(48.0, 3.0), 5.0)
        .expect("the midpoint pulls");
    assert_eq!(found, side);
    assert_eq!(at, DVec2::new(50.0, 0.0));
    assert_eq!(sketch.nearest_midpoint(DVec2::new(20.0, 0.0), 5.0), None);
}

#[test]
fn the_origin_never_moves() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    sketch.move_point(Sketch::ORIGIN, DVec2::new(10.0, 10.0));
    assert_eq!(sketch.point(Sketch::ORIGIN), DVec2::ZERO);
}
