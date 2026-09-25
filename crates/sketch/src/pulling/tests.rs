//! What a side or a curve pulled by the hand does.
//!
//! Closes #422.
//! - 9: a side pulled across travels, the traits joining it stretch, the
//!   opposite side stays; a trait on its own travels whole —
//!   `a_side_pulled_across_travels_and_the_opposite_side_stays`,
//!   `a_right_triangle_grows_when_its_hypotenuse_is_pulled_out`,
//!   `a_lone_trait_pulled_across_travels_whole`
//! - 10: slid along itself, the side turns its shape about the shape's centre,
//!   or about the point holding it, the place grabbed going round with the
//!   hand — `a_side_slid_along_itself_turns_the_shape_about_its_centre`,
//!   `a_shape_held_at_a_point_turns_about_it`
//! - 11: which of the two is read at every instant, about the place the shape
//!   turns on — `more_across_than_along_resizes_more_along_than_across_turns`,
//!   `a_turn_past_a_quarter_stays_a_turn`
//! - 12: a trait on its own has no along — `a_lone_trait_has_no_along`
//! - 13: a shape that cannot stretch across leaves the side where it is —
//!   `a_shape_that_cannot_stretch_across_leaves_its_side_where_it_is`
//! - 15: a curve pulled towards or away from its centre is drawn to its new
//!   size as before — `a_curve_pulled_out_is_drawn_to_its_new_size_as_before`
//! - 16: slid along, an arc turns keeping its radius and its sweep, an ellipse
//!   keeping its axes — `an_arc_slid_along_its_curve_turns_keeping_its_radius_and_sweep`,
//!   `an_ellipse_slid_along_its_curve_turns_keeping_its_axes`
//! - 17: a circle is always drawn to its new size —
//!   `a_circle_is_always_drawn_to_its_new_size`
//! - 18: while a shape turns, the end nearest the hand is pulled onto a grid
//!   point within reach — `a_turned_side_brings_its_corner_onto_the_grid`
//! - 14: a press on a side takes hold of it, and of the curve instead when the
//!   curve is nearer; an ellipse's axis and a side that cannot move are not
//!   taken, and a box is drawn there as before —
//!   `a_press_takes_hold_of_the_side_or_the_curve_it_is_nearer`,
//!   `an_ellipse_axis_and_a_side_that_cannot_move_are_not_taken_hold_of`

use glam::DVec2;

use super::*;
use crate::constraints::{Constraint, DimensionTarget};
use crate::plane::WorkPlane;

/// Close enough for points the solver placed: it stops at a hundred-thousandth
/// of the drawing's size, and these drawings are about a hundred across.
const SETTLED: f64 = 1e-3;

fn no_grid() -> SnapSettings {
    SnapSettings {
        point_reach: 1.0,
        curve_reach: 1.0,
        grid_step: None,
        grid_reach: 0.0,
    }
}

fn grid(step: f64, reach: f64) -> SnapSettings {
    SnapSettings {
        grid_step: Some(step),
        grid_reach: reach,
        ..no_grid()
    }
}

/// A rectangle as the tool lays it, 100 wide and 50 high, its corners
/// counter-clockwise from the bottom left.
fn rectangle() -> (Sketch, [PointId; 4], [SegmentId; 4]) {
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
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[corner],
            second: sides[corner + 1],
        });
    }
    (sketch, corners, sides)
}

fn assert_near(found: DVec2, wanted: DVec2, what: &str) {
    assert!(
        found.distance(wanted) < SETTLED,
        "{what}: wanted {wanted}, found {found}"
    );
}

fn laid(sketch: &mut Sketch, drag: &SideDrag, side: SegmentId) {
    match drag {
        SideDrag::Across { by } => sketch.move_side(side, *by, 1.0),
        SideDrag::Along(turn) => sketch.turn_shape(&turn.points, turn.about, turn.angle, 1.0),
    };
}

#[test]
fn a_side_pulled_across_travels_and_the_opposite_side_stays() {
    let (mut sketch, [a, b, c, d], sides) = rectangle();
    let top = sides[2];

    let drag = sketch.side_drag(
        top,
        DVec2::new(70.0, 70.0),
        DVec2::new(72.0, 90.0),
        &no_grid(),
        1.0,
    );
    assert_eq!(
        drag,
        SideDrag::Across {
            by: DVec2::new(0.0, 20.0)
        }
    );
    laid(&mut sketch, &drag, top);

    assert_near(sketch.point(a), DVec2::new(20.0, 20.0), "a bottom corner");
    assert_near(
        sketch.point(b),
        DVec2::new(120.0, 20.0),
        "the other bottom corner",
    );
    assert_near(sketch.point(c), DVec2::new(120.0, 90.0), "a top corner");
    assert_near(
        sketch.point(d),
        DVec2::new(20.0, 90.0),
        "the other top corner",
    );
}

#[test]
fn a_right_triangle_grows_when_its_hypotenuse_is_pulled_out() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let right = sketch.add_point(DVec2::new(10.0, 10.0));
    let far = sketch.add_point(DVec2::new(50.0, 10.0));
    let top = sketch.add_point(DVec2::new(10.0, 40.0));
    let base = sketch.add_segment(right, far);
    let side = sketch.add_segment(right, top);
    let hypotenuse = sketch.add_segment(far, top);
    sketch.add_constraint(Constraint::Perpendicular {
        first: base,
        second: side,
    });
    let normal = (sketch.point(top) - sketch.point(far)).normalize().perp();
    let outwards = if normal.dot(DVec2::ONE) > 0.0 {
        normal
    } else {
        -normal
    };

    sketch.move_side(hypotenuse, outwards * 10.0, 1.0);

    assert_near(
        sketch.point(right),
        DVec2::new(10.0, 10.0),
        "the right angle",
    );
    assert!(
        (sketch.point(far).y - 10.0).abs() < SETTLED,
        "the base stays level"
    );
    assert!(
        (sketch.point(top).x - 10.0).abs() < SETTLED,
        "the side stays upright"
    );
    assert!(
        sketch.point(far).x > 60.0,
        "and the triangle grew: {}",
        sketch.point(far)
    );
}

#[test]
fn a_lone_trait_pulled_across_travels_whole() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(DVec2::new(10.0, 10.0));
    let to = sketch.add_point(DVec2::new(60.0, 10.0));
    let line = sketch.add_segment(from, to);

    let drag = sketch.side_drag(
        line,
        DVec2::new(30.0, 10.0),
        DVec2::new(40.0, 25.0),
        &no_grid(),
        1.0,
    );
    laid(&mut sketch, &drag, line);

    assert_near(sketch.point(from), DVec2::new(10.0, 25.0), "one end");
    assert_near(sketch.point(to), DVec2::new(60.0, 25.0), "the other end");
}

#[test]
fn a_side_slid_along_itself_turns_the_shape_about_its_centre() {
    let (mut sketch, corners, sides) = rectangle();
    let left = sides[3];

    let drag = sketch.side_drag(
        left,
        DVec2::new(20.0, 45.0),
        DVec2::new(19.0, 60.0),
        &no_grid(),
        1.0,
    );
    let SideDrag::Along(turn) = &drag else {
        panic!("sliding a side along itself turns: {drag:?}");
    };
    assert_near(
        turn.about,
        DVec2::new(70.0, 45.0),
        "the centre it turns about",
    );
    assert!(
        turn.angle < 0.0,
        "up the left side is clockwise: {}",
        turn.angle
    );
    let before: Vec<f64> = corners
        .iter()
        .map(|corner| sketch.point(*corner).distance(turn.about))
        .collect();
    laid(&mut sketch, &drag, left);

    for (rank, corner) in corners.iter().enumerate() {
        let reach = sketch.point(*corner).distance(DVec2::new(70.0, 45.0));
        assert!(
            (reach - before[rank]).abs() < SETTLED,
            "corner {rank} kept its reach"
        );
    }
    let grabbed = DVec2::from_angle(turn.angle).rotate(DVec2::new(-50.0, 0.0));
    let wanted = (DVec2::new(19.0, 60.0) - DVec2::new(70.0, 45.0)).normalize();
    assert!(
        grabbed.normalize().distance(wanted) < 1e-9,
        "the place grabbed went round with the hand"
    );
}

#[test]
fn a_shape_held_at_a_point_turns_about_it() {
    let (mut sketch, [a, ..], sides) = rectangle();
    sketch.add_constraint(Constraint::Fixed {
        element: crate::element::Element::Point(a),
    });

    let drag = sketch.side_drag(
        sides[2],
        DVec2::new(70.0, 70.0),
        DVec2::new(40.0, 72.0),
        &no_grid(),
        1.0,
    );

    let SideDrag::Along(turn) = drag else {
        panic!("sliding the top along turns: {drag:?}");
    };
    assert_near(turn.about, DVec2::new(20.0, 20.0), "the corner holding it");
}

#[test]
fn more_across_than_along_resizes_more_along_than_across_turns() {
    let (sketch, _, sides) = rectangle();
    let top = sides[2];
    let pressed = DVec2::new(70.0, 70.0);

    let mostly_up = sketch.side_drag(top, pressed, DVec2::new(75.0, 80.0), &no_grid(), 1.0);
    let mostly_sideways = sketch.side_drag(top, pressed, DVec2::new(80.0, 73.0), &no_grid(), 1.0);

    assert!(
        matches!(mostly_up, SideDrag::Across { .. }),
        "{mostly_up:?}"
    );
    assert!(
        matches!(mostly_sideways, SideDrag::Along(_)),
        "{mostly_sideways:?}"
    );
}

#[test]
fn a_turn_past_a_quarter_stays_a_turn() {
    let (sketch, _, sides) = rectangle();
    let centre = DVec2::new(70.0, 45.0);
    let pressed = DVec2::new(70.0, 70.0);

    for degrees in [60.0_f64, 100.0, 150.0] {
        let cursor = centre + DVec2::from_angle(degrees.to_radians()).rotate(pressed - centre);
        let drag = sketch.side_drag(sides[2], pressed, cursor, &no_grid(), 1.0);
        let SideDrag::Along(turn) = drag else {
            panic!("{degrees}° round is still a turn: {drag:?}");
        };
        assert!(
            (turn.angle.to_degrees() - degrees).abs() < 1e-6,
            "{}",
            turn.angle.to_degrees()
        );
    }
}

#[test]
fn a_lone_trait_has_no_along() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(DVec2::new(10.0, 10.0));
    let to = sketch.add_point(DVec2::new(60.0, 10.0));
    let line = sketch.add_segment(from, to);

    let drag = sketch.side_drag(
        line,
        DVec2::new(30.0, 10.0),
        DVec2::new(80.0, 12.0),
        &no_grid(),
        1.0,
    );

    assert_eq!(
        drag,
        SideDrag::Across {
            by: DVec2::new(0.0, 2.0)
        }
    );
}

#[test]
fn a_shape_that_cannot_stretch_across_leaves_its_side_where_it_is() {
    let (mut sketch, corners, sides) = rectangle();
    let height = sketch.segment_length(sides[1]);
    sketch.set_dimension(DimensionTarget::Length(sides[1]), height, false);
    let before = corners.map(|corner| sketch.point(corner));

    sketch.move_side(sides[2], DVec2::new(0.0, 20.0), 1.0);

    for (rank, corner) in corners.iter().enumerate() {
        assert_near(sketch.point(*corner), before[rank], "a corner");
    }
}

#[test]
fn a_curve_pulled_out_is_drawn_to_its_new_size_as_before() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let start = sketch.add_point(DVec2::new(90.0, 50.0));
    let end = sketch.add_point(DVec2::new(50.0, 90.0));
    let arc = sketch.add_arc(centre, start, end);
    let pressed = DVec2::new(50.0, 50.0) + DVec2::from_angle(0.8) * 40.0;
    let cursor = DVec2::new(50.0, 50.0) + DVec2::from_angle(0.82) * 55.0;

    let drag = sketch.curve_drag(Curved::Arc(arc), pressed, cursor, cursor, &no_grid(), 1.0);

    assert_eq!(
        drag,
        CurveDrag::Resize {
            reach: sketch.reach_through(Curved::Arc(arc), cursor)
        }
    );
}

#[test]
fn an_arc_slid_along_its_curve_turns_keeping_its_radius_and_sweep() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let start = sketch.add_point(DVec2::new(90.0, 50.0));
    let end = sketch.add_point(DVec2::new(50.0, 90.0));
    let arc = sketch.add_arc(centre, start, end);
    let pressed = DVec2::new(50.0, 50.0) + DVec2::from_angle(0.8) * 40.0;
    let cursor = DVec2::new(50.0, 50.0) + DVec2::from_angle(1.1) * 41.0;

    let CurveDrag::Along(turn) =
        sketch.curve_drag(Curved::Arc(arc), pressed, cursor, cursor, &no_grid(), 1.0)
    else {
        panic!("sliding along the arc turns it");
    };
    let sweep = sketch.arc_sweep(arc);
    sketch.turn_shape(&turn.points, turn.about, turn.angle, 1.0);

    assert_near(sketch.point(centre), DVec2::new(50.0, 50.0), "the centre");
    assert!(
        (sketch.arc_radius(arc) - 40.0).abs() < SETTLED,
        "the radius"
    );
    assert!((sketch.arc_sweep(arc) - sweep).abs() < SETTLED, "the sweep");
    let went = (sketch.point(start) - DVec2::new(50.0, 50.0)).to_angle();
    assert!((went - 0.3).abs() < 1e-6, "the ends went round: {went}");
}

#[test]
fn an_ellipse_slid_along_its_curve_turns_keeping_its_axes() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 20.0));
    let west = sketch.add_point(DVec2::new(20.0, 20.0));
    let east = sketch.add_point(DVec2::new(80.0, 20.0));
    let south = sketch.add_point(DVec2::new(50.0, 10.0));
    let north = sketch.add_point(DVec2::new(50.0, 30.0));
    let id = sketch.add_ellipse(centre, [west, east], [south, north]);
    let pressed = DVec2::new(50.0, 30.0);
    let cursor = DVec2::new(40.0, 29.0);

    let CurveDrag::Along(turn) = sketch.curve_drag(
        Curved::Ellipse(id),
        pressed,
        cursor,
        cursor,
        &no_grid(),
        1.0,
    ) else {
        panic!("sliding along the ellipse turns it");
    };
    sketch.turn_shape(&turn.points, turn.about, turn.angle, 1.0);

    let drawn = sketch.ellipse_draft(id);
    assert_near(drawn.centre, DVec2::new(50.0, 20.0), "the centre");
    assert!(
        (drawn.first.length() - 30.0).abs() < SETTLED,
        "the first axis"
    );
    assert!((drawn.second - 10.0).abs() < SETTLED, "the second axis");
    assert!(
        drawn.first.to_angle() > 0.1,
        "and it turned: {}",
        drawn.first
    );
}

#[test]
fn a_circle_is_always_drawn_to_its_new_size() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let circle = sketch.add_circle(centre, 20.0);
    let pressed = DVec2::new(70.0, 50.0);
    let round = DVec2::new(50.0, 70.0);

    let drag = sketch.curve_drag(
        Curved::Circle(circle),
        pressed,
        round,
        round,
        &no_grid(),
        1.0,
    );

    assert_eq!(drag, CurveDrag::Resize { reach: 20.0 });
}

#[test]
fn a_turned_side_brings_its_corner_onto_the_grid() {
    let (sketch, _, sides) = rectangle();
    let centre = DVec2::new(70.0, 45.0);
    let pressed = DVec2::new(70.0, 70.0);
    let near_square = centre + DVec2::from_angle(0.03).rotate(pressed - centre);

    let with_grid = sketch.side_drag(sides[2], pressed, near_square, &grid(10.0, 2.0), 1.0);
    let without = sketch.side_drag(sides[2], pressed, near_square, &no_grid(), 1.0);
    let out_of_reach = sketch.side_drag(sides[2], pressed, near_square, &grid(10.0, 0.1), 1.0);

    let turned = |drag: SideDrag| match drag {
        SideDrag::Along(turn) => turn.angle,
        other => panic!("a slide along turns: {other:?}"),
    };
    assert!(turned(with_grid).abs() < 1e-9, "brought back square");
    assert!((turned(without) - 0.03).abs() < 1e-9, "no grid, no pull");
    assert!(
        (turned(out_of_reach) - 0.03).abs() < 1e-9,
        "nothing within reach, no pull"
    );
}

#[test]
fn a_press_takes_hold_of_the_side_or_the_curve_it_is_nearer() {
    let (mut sketch, _, sides) = rectangle();
    let centre = sketch.add_point(DVec2::new(70.0, 80.0));
    let circle = sketch.add_circle(centre, 8.0);

    assert_eq!(
        sketch.pulled_at(DVec2::new(40.0, 71.0), 5.0, 1.0),
        Some(Pulled::Side(sides[2]))
    );
    assert_eq!(
        sketch.pulled_at(DVec2::new(70.0, 73.0), 5.0, 1.0),
        Some(Pulled::Curve(Curved::Circle(circle)))
    );
    assert_eq!(sketch.pulled_at(DVec2::new(70.0, 45.0), 5.0, 1.0), None);
}

#[test]
fn an_ellipse_axis_and_a_side_that_cannot_move_are_not_taken_hold_of() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 20.0));
    let west = sketch.add_point(DVec2::new(20.0, 20.0));
    let east = sketch.add_point(DVec2::new(80.0, 20.0));
    let south = sketch.add_point(DVec2::new(50.0, 10.0));
    let north = sketch.add_point(DVec2::new(50.0, 30.0));
    sketch.add_ellipse(centre, [west, east], [south, north]);
    let far = sketch.add_point(DVec2::new(100.0, 0.0));
    let nailed = sketch.add_segment(Sketch::ORIGIN, far);
    sketch.add_constraint(Constraint::Fixed {
        element: crate::element::Element::Segment(nailed),
    });

    assert_eq!(sketch.pulled_at(DVec2::new(35.0, 20.0), 2.0, 1.0), None);
    assert_eq!(sketch.pulled_at(DVec2::new(60.0, 0.5), 2.0, 1.0), None);
}
