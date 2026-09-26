//! What a side or a curve pulled by the hand does.
//!
//! Closes #422.
//! - 9: a side pulled across travels, the traits joining it stretch, the
//!   opposite side stays —
//!   `a_side_pulled_across_travels_and_the_opposite_side_stays`,
//!   `a_right_triangle_grows_when_its_hypotenuse_is_pulled_out`,
//!   `a_typed_tail_does_not_take_the_place_of_the_opposite_side`,
//!   `a_d_shape_pulled_by_its_flat_keeps_its_curve`,
//!   `a_slot_pulled_by_its_flat_widens_and_its_other_flat_stays`,
//!   `an_ellipse_closed_by_a_chord_keeps_its_centre_when_the_chord_is_pulled`
//! - 10: slid along itself, the side turns its shape about the middle of the
//!   side opposite, the far corner when none stands opposite, or the point
//!   holding it, the place grabbed going round with the hand —
//!   `a_side_slid_along_itself_turns_the_shape_about_the_middle_of_its_opposite_side`,
//!   `a_triangle_slid_by_a_side_turns_about_its_far_corner`,
//!   `an_l_slid_by_one_arm_turns_about_the_far_end_of_the_other`, whatever
//!   else hangs off the shape or was drawn first —
//!   `a_tail_or_a_construction_line_off_the_far_corner_does_not_take_the_place_of_the_opposite_side`,
//!   `a_point_held_on_the_opposite_side_does_not_take_the_place_of_its_middle`,
//!   `a_house_slid_by_its_floor_turns_about_a_place_above_its_middle`,
//!   `a_regular_hexagon_slid_by_a_side_turns_about_the_middle_of_the_side_opposite`,
//!   `a_triangle_with_a_construction_height_still_turns_about_its_far_corner`,
//!   `a_shape_held_at_a_point_turns_about_it`, and a shape sharing only the
//!   origin with another turns alone —
//!   `a_shape_sharing_only_the_origin_with_another_turns_alone`,
//!   `a_curve_centred_on_the_origin_turns_without_what_else_stands_there`,
//!   `an_arc_centred_on_the_origin_turns_whole`
//! - 11: which of the two is read at every instant, a turn having to be plain:
//!   a pull straight across resizes wherever the side was pressed, a pull
//!   drifting sideways from the middle of a side resizes until the drift is
//!   twice the pull and near a corner a little drift still resizes, a slide
//!   along turns, and a turn stays a turn past a quarter —
//!   `more_across_than_along_resizes_more_along_than_across_turns`,
//!   `a_side_pulled_straight_across_resizes_wherever_it_was_pressed`,
//!   `a_pull_that_drifts_sideways_keeps_resizing_until_the_drift_is_twice_the_pull`,
//!   `a_pull_near_a_corner_drifting_a_little_still_resizes`,
//!   `a_side_slid_along_near_its_end_turns`, `a_turn_past_a_quarter_stays_a_turn`
//! - 12: a trait on its own travels whole wherever the hand takes it, typed or
//!   not, carrying what is held on it, as far as its rules let it —
//!   `a_lone_trait_travels_whole_wherever_the_hand_takes_it`,
//!   `a_lone_trait_carries_the_point_held_on_it`,
//!   `a_lone_trait_held_to_an_axis_travels_as_far_as_the_axis_lets_it`,
//!   `a_lone_slanted_trait_held_to_an_axis_travels_along_it_whole`,
//!   `a_lone_trait_kept_at_a_distance_from_the_origin_travels_whole_round_it`,
//!   and what a rule ties it to follows —
//!   `a_lone_trait_tied_to_another_by_a_rule_carries_it_along`;
//!   a shape whose pivot lies on
//!   the pressed side's line, a rule holds upright or two fixed points nail
//!   down has no along; a curve kept from turning is drawn to its size
//!   instead — `a_shape_whose_pivot_lies_on_the_pressed_side_has_no_along`,
//!   `a_shape_held_upright_by_a_rule_has_no_along`,
//!   `a_shape_with_an_axis_angle_or_two_fixed_points_has_nowhere_to_turn_about`,
//!   `an_arc_kept_from_turning_by_a_rule_is_drawn_to_its_size_instead`
//! - 13: a shape that cannot stretch across leaves the side where it is —
//!   `a_shape_that_cannot_stretch_across_leaves_its_side_where_it_is`,
//!   `a_side_that_cannot_reach_a_curve_at_one_end_leaves_the_drawing_as_it_was`
//! - 15: a curve pulled towards or away from its centre is drawn to its new
//!   size as before — `a_curve_pulled_out_is_drawn_to_its_new_size_as_before`,
//!   and a hand gone out more than half as far as round only resizes —
//!   `an_arc_pulled_out_more_than_half_as_far_as_round_is_only_drawn_to_its_size`
//! - 16: slid along, an arc turns about its centre keeping its sweep, an
//!   ellipse keeping its shape, both drawn to the size that keeps the place
//!   grabbed under the hand, and what is joined to either turns with it —
//!   `an_arc_slid_along_its_curve_turns_keeping_its_sweep_and_reaching_the_hand`,
//!   `an_ellipse_slid_along_its_curve_turns_and_grows_to_the_hand_keeping_its_shape`,
//!   `an_arc_slid_along_turns_what_is_joined_to_it_with_it`; a size a rule
//!   holds is kept, the curve only turning about its centre —
//!   `an_arc_of_typed_radius_slid_round_turns_at_its_radius_about_its_centre`,
//!   `an_ellipse_with_a_typed_axis_slid_round_turns_keeping_its_shape`,
//!   `a_curve_whose_size_is_held_writes_no_size_it_did_not_reach`; a curve
//!   tangent to others still reaches the hand —
//!   `a_slot_s_cap_slid_round_and_out_reaches_the_hand_about_its_centre`
//! - 17: a circle is always drawn to its new size —
//!   `a_circle_is_always_drawn_to_its_new_size`
//! - 18: while a shape turns, the end nearest the hand is pulled onto a grid
//!   point within reach — `a_turned_side_brings_its_corner_onto_the_grid`,
//!   `an_arc_turned_back_near_where_it_was_is_pulled_onto_the_grid`
//! - 14: a press on a side takes hold of it, and of the curve instead when the
//!   curve is nearer; an ellipse's axis and a side that cannot move are not
//!   taken, and a box is drawn there as before —
//!   `a_press_takes_hold_of_the_side_or_the_curve_it_is_nearer`,
//!   `an_ellipse_axis_and_a_side_that_cannot_move_are_not_taken_hold_of`
//! - 24: a side pulled past the opposite one goes through, the shape coming
//!   out the other way round —
//!   `a_side_pulled_past_the_opposite_side_turns_the_rectangle_inside_out`;
//!   not an arc, which would come out inside out —
//!   `a_slot_s_touch_point_slid_round_its_cap_leaves_the_other_cap_whole`

use glam::DVec2;

use super::*;
use crate::constraints::{Constraint, DimensionTarget};
use crate::length::LengthOutcome;
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

fn laid_curve(sketch: &mut Sketch, drag: &CurveDrag, curve: Curved) {
    match drag {
        CurveDrag::Resize { reach } => sketch.resize(curve, *reach, 1.0),
        CurveDrag::Along { turn, reach } => {
            let turned = sketch.turn_shape(&turn.points, turn.about, turn.angle, 1.0);
            match reach {
                Some(reach) => sketch.resize_in_place(curve, *reach, 1.0),
                None => turned,
            }
        }
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
fn a_lone_trait_travels_whole_wherever_the_hand_takes_it() {
    for typed in [false, true] {
        for (cursor, by) in [
            (DVec2::new(40.0, 25.0), DVec2::new(10.0, 15.0)),
            (DVec2::new(80.0, 12.0), DVec2::new(50.0, 2.0)),
            (DVec2::new(5.0, -30.0), DVec2::new(-25.0, -40.0)),
        ] {
            let mut sketch = Sketch::new(WorkPlane::XY);
            let from = sketch.add_point(DVec2::new(10.0, 10.0));
            let to = sketch.add_point(DVec2::new(60.0, 10.0));
            let line = sketch.add_segment(from, to);
            if typed {
                sketch.set_dimension(DimensionTarget::Length(line), 50.0, false);
            }

            let drag = sketch.side_drag(line, DVec2::new(30.0, 10.0), cursor, &no_grid(), 1.0);
            assert_eq!(drag, SideDrag::Across { by }, "typed: {typed}");
            laid(&mut sketch, &drag, line);

            assert_near(sketch.point(from), DVec2::new(10.0, 10.0) + by, "one end");
            assert_near(
                sketch.point(to),
                DVec2::new(60.0, 10.0) + by,
                "the other end",
            );
        }
    }
}

#[test]
fn a_lone_trait_carries_the_point_held_on_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(DVec2::new(10.0, 10.0));
    let to = sketch.add_point(DVec2::new(60.0, 10.0));
    let line = sketch.add_segment(from, to);
    let held = sketch.add_point(DVec2::new(45.0, 10.0));
    sketch.add_constraint(Constraint::OnSegment {
        point: held,
        segment: line,
    });

    let drag = sketch.side_drag(
        line,
        DVec2::new(30.0, 10.0),
        DVec2::new(40.0, 25.0),
        &no_grid(),
        1.0,
    );
    laid(&mut sketch, &drag, line);

    assert_near(sketch.point(from), DVec2::new(20.0, 25.0), "one end");
    assert_near(sketch.point(to), DVec2::new(70.0, 25.0), "the other end");
    assert_near(
        sketch.point(held),
        DVec2::new(55.0, 25.0),
        "the point held on it",
    );
}

#[test]
fn a_side_pulled_past_the_opposite_side_turns_the_rectangle_inside_out() {
    let (mut sketch, [a, b, c, d], sides) = rectangle();

    sketch.move_side(sides[2], DVec2::new(0.0, -80.0), 1.0);

    assert_near(sketch.point(a), DVec2::new(20.0, 20.0), "a bottom corner");
    assert_near(sketch.point(b), DVec2::new(120.0, 20.0), "the other");
    assert_near(sketch.point(c), DVec2::new(120.0, -10.0), "a top corner");
    assert_near(sketch.point(d), DVec2::new(20.0, -10.0), "the other");
}

#[test]
fn a_side_slid_along_itself_turns_the_shape_about_the_middle_of_its_opposite_side() {
    let (mut sketch, corners, sides) = rectangle();
    let left = sides[3];
    let pivot = DVec2::new(120.0, 45.0);

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
    assert_near(turn.about, pivot, "the middle of the right side");
    assert!(
        turn.angle < 0.0,
        "up the left side is clockwise: {}",
        turn.angle
    );
    let before: Vec<f64> = corners
        .iter()
        .map(|corner| sketch.point(*corner).distance(pivot))
        .collect();
    laid(&mut sketch, &drag, left);

    for (rank, corner) in corners.iter().enumerate() {
        let reach = sketch.point(*corner).distance(pivot);
        assert!(
            (reach - before[rank]).abs() < SETTLED,
            "corner {rank} kept its reach"
        );
    }
    let grabbed = DVec2::from_angle(turn.angle).rotate(DVec2::new(-100.0, 0.0));
    let wanted = (DVec2::new(19.0, 60.0) - pivot).normalize();
    assert!(
        grabbed.normalize().distance(wanted) < 1e-9,
        "the place grabbed went round with the hand"
    );
}

#[test]
fn a_triangle_slid_by_a_side_turns_about_its_far_corner() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(0.0, 0.0),
        DVec2::new(100.0, 0.0),
        DVec2::new(40.0, 60.0),
    ]
    .map(|place| sketch.add_point(place));
    let sides: [SegmentId; 3] =
        std::array::from_fn(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 3]));

    let drag = sketch.side_drag(
        sides[0],
        DVec2::new(50.0, 0.0),
        DVec2::new(65.0, -1.0),
        &no_grid(),
        1.0,
    );

    let SideDrag::Along(turn) = drag else {
        panic!("sliding the base along turns: {drag:?}");
    };
    assert_near(turn.about, DVec2::new(40.0, 60.0), "the far corner");
}

#[test]
fn an_l_slid_by_one_arm_turns_about_the_far_end_of_the_other() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let top = sketch.add_point(DVec2::new(10.0, 60.0));
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let far = sketch.add_point(DVec2::new(90.0, 10.0));
    let upright = sketch.add_segment(top, corner);
    let flat = sketch.add_segment(corner, far);
    sketch.add_constraint(Constraint::Perpendicular {
        first: upright,
        second: flat,
    });

    let drag = sketch.side_drag(
        upright,
        DVec2::new(10.0, 35.0),
        DVec2::new(9.0, 50.0),
        &no_grid(),
        1.0,
    );

    let SideDrag::Along(turn) = drag else {
        panic!("sliding the upright arm along turns: {drag:?}");
    };
    assert_near(
        turn.about,
        DVec2::new(90.0, 10.0),
        "the far end of the flat arm",
    );
}

#[test]
fn a_pull_that_drifts_sideways_keeps_resizing_until_the_drift_is_twice_the_pull() {
    let (sketch, _, sides) = rectangle();
    let pressed = DVec2::new(70.0, 70.0);

    let drifting = sketch.side_drag(sides[2], pressed, DVec2::new(85.0, 80.0), &no_grid(), 1.0);
    let sliding = sketch.side_drag(sides[2], pressed, DVec2::new(95.0, 80.0), &no_grid(), 1.0);

    assert_eq!(
        drifting,
        SideDrag::Across {
            by: DVec2::new(0.0, 10.0)
        },
        "a drift of one and a half times the pull still resizes"
    );
    assert!(
        matches!(sliding, SideDrag::Along(_)),
        "past twice the pull it turns: {sliding:?}"
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
    let centre = DVec2::new(70.0, 20.0);
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
fn an_arc_slid_along_its_curve_turns_keeping_its_sweep_and_reaching_the_hand() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let start = sketch.add_point(DVec2::new(90.0, 50.0));
    let end = sketch.add_point(DVec2::new(50.0, 90.0));
    let arc = sketch.add_arc(centre, start, end);
    let pressed = DVec2::new(50.0, 50.0) + DVec2::from_angle(0.8) * 40.0;
    let cursor = DVec2::new(50.0, 50.0) + DVec2::from_angle(1.1) * 45.0;

    let drag = sketch.curve_drag(Curved::Arc(arc), pressed, cursor, cursor, &no_grid(), 1.0);
    let CurveDrag::Along {
        reach: Some(reach), ..
    } = &drag
    else {
        panic!("sliding along the arc turns it and draws it out: {drag:?}");
    };
    assert!(
        (reach - 45.0).abs() < 1e-9,
        "as far out as the hand: {reach}"
    );
    let sweep = sketch.arc_sweep(arc);
    laid_curve(&mut sketch, &drag, Curved::Arc(arc));

    assert_near(sketch.point(centre), DVec2::new(50.0, 50.0), "the centre");
    assert!(
        (sketch.arc_radius(arc) - 45.0).abs() < SETTLED,
        "the radius: {}",
        sketch.arc_radius(arc)
    );
    assert!((sketch.arc_sweep(arc) - sweep).abs() < SETTLED, "the sweep");
    let went = (sketch.point(start) - DVec2::new(50.0, 50.0)).to_angle();
    assert!((went - 0.3).abs() < 1e-6, "the ends went round: {went}");
}

#[test]
fn an_arc_pulled_out_more_than_half_as_far_as_round_is_only_drawn_to_its_size() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let start = sketch.add_point(DVec2::new(90.0, 50.0));
    let end = sketch.add_point(DVec2::new(50.0, 90.0));
    let arc = sketch.add_arc(centre, start, end);
    let pressed = DVec2::new(50.0, 50.0) + DVec2::from_angle(0.8) * 40.0;
    let cursor = DVec2::new(50.0, 50.0) + DVec2::from_angle(0.9) * 43.0;

    let drag = sketch.curve_drag(Curved::Arc(arc), pressed, cursor, cursor, &no_grid(), 1.0);

    assert!(
        matches!(drag, CurveDrag::Resize { reach } if (reach - 43.0).abs() < 1e-9),
        "four round against three out is not yet a turn: {drag:?}"
    );
}

#[test]
fn an_ellipse_slid_along_its_curve_turns_and_grows_to_the_hand_keeping_its_shape() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 20.0));
    let west = sketch.add_point(DVec2::new(20.0, 20.0));
    let east = sketch.add_point(DVec2::new(80.0, 20.0));
    let south = sketch.add_point(DVec2::new(50.0, 10.0));
    let north = sketch.add_point(DVec2::new(50.0, 30.0));
    let id = sketch.add_ellipse(centre, [west, east], [south, north]);
    let pressed = DVec2::new(50.0, 30.0);
    let cursor = DVec2::new(40.0, 29.0);

    let drag = sketch.curve_drag(
        Curved::Ellipse(id),
        pressed,
        cursor,
        cursor,
        &no_grid(),
        1.0,
    );
    assert!(
        matches!(drag, CurveDrag::Along { .. }),
        "sliding along the ellipse turns it: {drag:?}"
    );
    laid_curve(&mut sketch, &drag, Curved::Ellipse(id));

    let drawn = sketch.ellipse_draft(id);
    assert_near(drawn.centre, DVec2::new(50.0, 20.0), "the centre");
    assert!(
        (drawn.first.length() / drawn.second - 3.0).abs() < SETTLED,
        "the shape: {} by {}",
        drawn.first.length(),
        drawn.second
    );
    assert_near(
        sketch.point(north),
        cursor,
        "the place grabbed, under the hand",
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
    let centre = DVec2::new(70.0, 20.0);
    let pressed = DVec2::new(70.0, 70.0);
    let near_square = centre + DVec2::from_angle(0.02).rotate(pressed - centre);

    let with_grid = sketch.side_drag(sides[2], pressed, near_square, &grid(10.0, 2.0), 1.0);
    let without = sketch.side_drag(sides[2], pressed, near_square, &no_grid(), 1.0);
    let out_of_reach = sketch.side_drag(sides[2], pressed, near_square, &grid(10.0, 0.1), 1.0);

    let turned = |drag: SideDrag| match drag {
        SideDrag::Along(turn) => turn.angle,
        other => panic!("a slide along turns: {other:?}"),
    };
    assert!(turned(with_grid).abs() < 1e-9, "brought back square");
    assert!((turned(without) - 0.02).abs() < 1e-9, "no grid, no pull");
    assert!(
        (turned(out_of_reach) - 0.02).abs() < 1e-9,
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

#[test]
fn a_side_pulled_straight_across_resizes_wherever_it_was_pressed() {
    let (sketch, _, sides) = rectangle();

    for x in [25.0, 70.0, 110.0, 118.0] {
        let drag = sketch.side_drag(
            sides[2],
            DVec2::new(x, 70.0),
            DVec2::new(x, 90.0),
            &no_grid(),
            1.0,
        );
        assert_eq!(
            drag,
            SideDrag::Across {
                by: DVec2::new(0.0, 20.0)
            },
            "pressed at {x}"
        );
    }
}

#[test]
fn a_side_slid_along_near_its_end_turns() {
    let (sketch, _, sides) = rectangle();

    let drag = sketch.side_drag(
        sides[2],
        DVec2::new(115.0, 70.0),
        DVec2::new(135.0, 70.0),
        &no_grid(),
        1.0,
    );

    assert!(matches!(drag, SideDrag::Along(_)), "{drag:?}");
}

#[test]
fn a_shape_sharing_only_the_origin_with_another_turns_alone() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        Sketch::ORIGIN,
        sketch.add_point(DVec2::new(100.0, 0.0)),
        sketch.add_point(DVec2::new(100.0, 50.0)),
        sketch.add_point(DVec2::new(0.0, 50.0)),
    ];
    let sides: [SegmentId; 4] =
        std::array::from_fn(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 4]));
    for corner in 0..3 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[corner],
            second: sides[corner + 1],
        });
    }
    let other = sketch.add_point(DVec2::new(-60.0, -40.0));
    sketch.add_segment(Sketch::ORIGIN, other);

    let drag = sketch.side_drag(
        sides[2],
        DVec2::new(20.0, 50.0),
        DVec2::new(0.0, 50.0),
        &no_grid(),
        1.0,
    );

    let SideDrag::Along(turn) = drag else {
        panic!("sliding the top along turns: {drag:?}");
    };
    assert!(
        !turn.points.contains(&other),
        "the other shape is not taken along"
    );
    assert_near(turn.about, DVec2::ZERO, "about the origin holding it");
}

#[test]
fn a_d_shape_pulled_by_its_flat_keeps_its_curve() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let half = (30.0_f64 * 30.0 - 15.0 * 15.0).sqrt();
    let left = sketch.add_point(DVec2::new(50.0 - half, 65.0));
    let right = sketch.add_point(DVec2::new(50.0 + half, 65.0));
    let arc = sketch.add_arc(centre, right, left);
    let flat = sketch.add_segment(left, right);

    sketch.move_side(flat, DVec2::new(0.0, 5.0), 1.0);

    assert_near(sketch.point(centre), DVec2::new(50.0, 50.0), "the centre");
    assert!(
        (sketch.arc_radius(arc) - 30.0).abs() < SETTLED,
        "the radius {}",
        sketch.arc_radius(arc)
    );
    assert!(
        (sketch.point(left).y - 70.0).abs() < SETTLED,
        "the flat went up: {}",
        sketch.point(left)
    );
    assert!(
        (sketch.point(left).distance(sketch.point(centre)) - 30.0).abs() < SETTLED,
        "on the curve"
    );
}

#[test]
fn a_typed_tail_does_not_take_the_place_of_the_opposite_side() {
    let (mut sketch, [a, b, c, d], sides) = rectangle();
    let end = sketch.add_point(DVec2::new(120.0, -80.0));
    let tail = sketch.add_segment(b, end);
    sketch.set_dimension(DimensionTarget::Length(tail), 100.0, false);

    sketch.move_side(sides[0], DVec2::new(0.0, -20.0), 1.0);

    assert_near(sketch.point(a), DVec2::new(20.0, 0.0), "a bottom corner");
    assert_near(
        sketch.point(b),
        DVec2::new(120.0, 0.0),
        "the other bottom corner",
    );
    assert_near(sketch.point(c), DVec2::new(120.0, 70.0), "a top corner");
    assert_near(
        sketch.point(d),
        DVec2::new(20.0, 70.0),
        "the other top corner",
    );
}

#[test]
fn a_shape_held_upright_by_a_rule_has_no_along() {
    let (mut sketch, _, sides) = rectangle();
    sketch.add_constraint(Constraint::AxisParallel {
        segment: sides[0],
        axis: crate::constraints::SketchAxis::U,
    });

    let drag = sketch.side_drag(
        sides[2],
        DVec2::new(70.0, 70.0),
        DVec2::new(100.0, 71.0),
        &no_grid(),
        1.0,
    );

    assert_eq!(
        drag,
        SideDrag::Across {
            by: DVec2::new(0.0, 1.0)
        }
    );
}

#[test]
fn an_arc_slid_along_turns_what_is_joined_to_it_with_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let start = sketch.add_point(DVec2::new(90.0, 50.0));
    let end = sketch.add_point(DVec2::new(50.0, 90.0));
    let arc = sketch.add_arc(centre, start, end);
    let tip = sketch.add_point(DVec2::new(130.0, 50.0));
    sketch.add_segment(start, tip);
    let pressed = DVec2::new(50.0, 50.0) + DVec2::from_angle(0.8) * 40.0;
    let cursor = DVec2::new(50.0, 50.0) + DVec2::from_angle(1.1) * 40.0;

    let drag = sketch.curve_drag(Curved::Arc(arc), pressed, cursor, cursor, &no_grid(), 1.0);
    assert!(
        matches!(drag, CurveDrag::Along { .. }),
        "sliding along the arc turns it: {drag:?}"
    );
    laid_curve(&mut sketch, &drag, Curved::Arc(arc));

    let went = DVec2::new(50.0, 50.0) + DVec2::from_angle(0.3).rotate(DVec2::new(80.0, 0.0));
    assert_near(
        sketch.point(tip),
        went,
        "the trait joined to it turned with it",
    );
}

/// A slot: two half-circles of radius 20 about (30, 30) and (130, 30), joined
/// by two flats they are tangent to.
fn slot() -> (Sketch, [PointId; 4], [SegmentId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let left = sketch.add_point(DVec2::new(30.0, 30.0));
    let right = sketch.add_point(DVec2::new(130.0, 30.0));
    let corners = [
        DVec2::new(30.0, 10.0),
        DVec2::new(130.0, 10.0),
        DVec2::new(130.0, 50.0),
        DVec2::new(30.0, 50.0),
    ]
    .map(|place| sketch.add_point(place));
    let bottom = sketch.add_segment(corners[0], corners[1]);
    let top = sketch.add_segment(corners[2], corners[3]);
    let east = sketch.add_arc(right, corners[1], corners[2]);
    let west = sketch.add_arc(left, corners[3], corners[0]);
    for (arc, segment) in [(east, bottom), (east, top), (west, bottom), (west, top)] {
        sketch.add_constraint(Constraint::ArcTangent {
            arc,
            segment,
            at: None,
        });
    }
    sketch.resolve(1.0);
    (sketch, corners, [bottom, top])
}

#[test]
fn a_slot_pulled_by_its_flat_widens_and_its_other_flat_stays() {
    let (mut sketch, corners, [_, top]) = slot();

    let outcome = sketch.move_side(top, DVec2::new(0.0, 10.0), 1.0);

    assert_eq!(outcome, LengthOutcome::Exact);
    assert!(
        (sketch.point(corners[2]).y - 60.0).abs() < SETTLED,
        "{}",
        sketch.point(corners[2])
    );
    assert!(
        (sketch.point(corners[0]).y - 10.0).abs() < SETTLED,
        "{}",
        sketch.point(corners[0])
    );
}

#[test]
fn a_side_that_cannot_reach_a_curve_at_one_end_leaves_the_drawing_as_it_was() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let big = sketch.add_point(DVec2::new(50.0, 50.0));
    let small = sketch.add_point(DVec2::new(150.0, 50.0));
    let from = sketch.add_point(DVec2::new(50.0 + (900.0_f64 - 25.0).sqrt(), 55.0));
    let to = sketch.add_point(DVec2::new(150.0 - (100.0_f64 - 25.0).sqrt(), 55.0));
    let big_end = sketch.add_point(DVec2::new(50.0, 80.0));
    let small_end = sketch.add_point(DVec2::new(150.0, 40.0));
    sketch.add_arc(big, from, big_end);
    sketch.add_arc(small, small_end, to);
    let side = sketch.add_segment(from, to);
    let before = sketch.points().to_vec();

    sketch.move_side(side, DVec2::new(0.0, 10.0), 1.0);

    assert_eq!(sketch.points(), before.as_slice());
}

#[test]
fn a_curve_centred_on_the_origin_turns_without_what_else_stands_there() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        Sketch::ORIGIN,
        sketch.add_point(DVec2::new(100.0, 0.0)),
        sketch.add_point(DVec2::new(100.0, 50.0)),
        sketch.add_point(DVec2::new(0.0, 50.0)),
    ];
    for rank in 0..4 {
        sketch.add_segment(corners[rank], corners[(rank + 1) % 4]);
    }
    let start = sketch.add_point(DVec2::new(-30.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, -30.0));
    let arc = sketch.add_arc(Sketch::ORIGIN, start, end);
    let pressed = DVec2::from_angle(-2.4) * 30.0;
    let cursor = DVec2::from_angle(-2.1) * 30.0;

    let CurveDrag::Along { turn, .. } =
        sketch.curve_drag(Curved::Arc(arc), pressed, cursor, cursor, &no_grid(), 1.0)
    else {
        panic!("sliding along the arc turns it");
    };

    assert!(
        !turn.points.contains(&corners[2]),
        "the rectangle stays out of it: {:?}",
        turn.points
    );
}

#[test]
fn a_shape_whose_pivot_lies_on_the_pressed_side_has_no_along() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        Sketch::ORIGIN,
        sketch.add_point(DVec2::new(100.0, 0.0)),
        sketch.add_point(DVec2::new(100.0, 50.0)),
        sketch.add_point(DVec2::new(0.0, 50.0)),
    ];
    let sides: [SegmentId; 4] =
        std::array::from_fn(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 4]));
    let other = sketch.add_point(DVec2::new(-60.0, -40.0));
    sketch.add_segment(Sketch::ORIGIN, other);

    let drag = sketch.side_drag(
        sides[0],
        DVec2::new(50.0, 0.0),
        DVec2::new(80.0, -1.0),
        &no_grid(),
        1.0,
    );

    assert_eq!(
        drag,
        SideDrag::Across {
            by: DVec2::new(0.0, -1.0)
        }
    );
    assert!(
        !sketch
            .shape_through(&[Sketch::ORIGIN, corners[1]])
            .contains(&other),
        "a shape walked from its free end stops at the origin"
    );
}

#[test]
fn an_arc_centred_on_the_origin_turns_whole() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(30.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, 30.0));
    let arc = sketch.add_arc(Sketch::ORIGIN, start, end);
    let pressed = DVec2::from_angle(0.8) * 30.0;
    let cursor = DVec2::from_angle(1.1) * 30.0;

    let CurveDrag::Along { turn, .. } =
        sketch.curve_drag(Curved::Arc(arc), pressed, cursor, cursor, &no_grid(), 1.0)
    else {
        panic!("sliding along the arc turns it");
    };

    assert!(
        turn.points.contains(&start) && turn.points.contains(&end),
        "{:?}",
        turn.points
    );
}

#[test]
fn an_arc_kept_from_turning_by_a_rule_is_drawn_to_its_size_instead() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let start = sketch.add_point(DVec2::new(90.0, 50.0));
    let end = sketch.add_point(DVec2::new(50.0, 90.0));
    let arc = sketch.add_arc(centre, start, end);
    let tip = sketch.add_point(DVec2::new(130.0, 50.0));
    let arm = sketch.add_segment(start, tip);
    sketch.add_constraint(Constraint::AxisParallel {
        segment: arm,
        axis: crate::constraints::SketchAxis::U,
    });
    let pressed = DVec2::new(50.0, 50.0) + DVec2::from_angle(0.8) * 40.0;
    let cursor = DVec2::new(50.0, 50.0) + DVec2::from_angle(1.1) * 40.0;

    let drag = sketch.curve_drag(Curved::Arc(arc), pressed, cursor, cursor, &no_grid(), 1.0);

    assert!(matches!(drag, CurveDrag::Resize { .. }), "{drag:?}");
}

#[test]
fn a_shape_with_an_axis_angle_or_two_fixed_points_has_nowhere_to_turn_about() {
    let (mut angled, _, sides) = rectangle();
    angled.set_dimension(
        DimensionTarget::AxisAngle {
            segment: sides[0],
            axis: crate::constraints::SketchAxis::U,
        },
        0.0,
        false,
    );
    let (mut nailed, corners, _) = rectangle();
    for corner in [corners[0], corners[2]] {
        nailed.add_constraint(Constraint::Fixed {
            element: crate::element::Element::Point(corner),
        });
    }

    for sketch in [&angled, &nailed] {
        let drag = sketch.side_drag(
            sides[2],
            DVec2::new(70.0, 70.0),
            DVec2::new(100.0, 71.0),
            &no_grid(),
            1.0,
        );
        assert!(matches!(drag, SideDrag::Across { .. }), "{drag:?}");
    }
}

#[test]
fn an_ellipse_closed_by_a_chord_keeps_its_centre_when_the_chord_is_pulled() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let west = sketch.add_point(DVec2::new(10.0, 50.0));
    let east = sketch.add_point(DVec2::new(90.0, 50.0));
    let south = sketch.add_point(DVec2::new(50.0, 30.0));
    let north = sketch.add_point(DVec2::new(50.0, 70.0));
    let id = sketch.add_ellipse(centre, [west, east], [south, north]);
    let across = 40.0 * (1.0 - 0.25_f64).sqrt();
    let from = sketch.add_point(DVec2::new(50.0 + across, 60.0));
    let to = sketch.add_point(DVec2::new(50.0 - across, 60.0));
    sketch.ellipses[id.0].drawn = Some((from, to));
    let chord = sketch.add_segment(to, from);
    sketch.resolve(1.0);

    sketch.move_side(chord, DVec2::new(0.0, -3.0), 1.0);

    assert_near(sketch.point(centre), DVec2::new(50.0, 50.0), "the centre");
    assert_near(
        sketch.point(north),
        DVec2::new(50.0, 70.0),
        "the top of the curve",
    );
}

#[test]
fn an_arc_turned_back_near_where_it_was_is_pulled_onto_the_grid() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let start = sketch.add_point(DVec2::new(90.0, 50.0));
    let end = sketch.add_point(DVec2::new(50.0, 90.0));
    let arc = sketch.add_arc(centre, start, end);
    let pressed = DVec2::new(50.0, 50.0) + DVec2::from_angle(0.8) * 40.0;
    let nearly = DVec2::new(50.0, 50.0) + DVec2::from_angle(0.82) * 40.0;

    let pulled = sketch.curve_drag(
        Curved::Arc(arc),
        pressed,
        nearly,
        nearly,
        &grid(10.0, 2.0),
        1.0,
    );
    let free = sketch.curve_drag(Curved::Arc(arc), pressed, nearly, nearly, &no_grid(), 1.0);

    let turned = |drag: CurveDrag| match drag {
        CurveDrag::Along { turn, reach } => {
            assert_eq!(reach, None, "its reach is its own");
            turn.angle
        }
        other => panic!("a slide round turns: {other:?}"),
    };
    assert!(
        turned(pulled).abs() < 1e-9,
        "its end brought back onto the grid"
    );
    assert!((turned(free) - 0.02).abs() < 1e-9, "no grid, no pull");
}

#[test]
fn a_lone_trait_held_to_an_axis_travels_as_far_as_the_axis_lets_it() {
    for (from, to, pressed, cursor, by) in [
        (
            DVec2::new(30.0, 0.0),
            DVec2::new(30.0, 50.0),
            DVec2::new(30.0, 25.0),
            DVec2::new(40.0, 26.0),
            DVec2::new(10.0, 0.0),
        ),
        (
            DVec2::new(30.0, 0.0),
            DVec2::new(80.0, 0.0),
            DVec2::new(50.0, 0.0),
            DVec2::new(60.0, 5.0),
            DVec2::new(10.0, 0.0),
        ),
    ] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let foot = sketch.add_point(from);
        let head = sketch.add_point(to);
        let line = sketch.add_segment(foot, head);
        sketch.add_constraint(Constraint::OnAxis {
            point: foot,
            axis: crate::constraints::SketchAxis::U,
        });

        let drag = sketch.side_drag(line, pressed, cursor, &no_grid(), 1.0);
        laid(&mut sketch, &drag, line);

        assert_near(sketch.point(foot), from + by, "the end held on the axis");
        assert_near(sketch.point(head), to + by, "the other end");
    }
}

#[test]
fn a_lone_trait_kept_at_a_distance_from_the_origin_travels_whole_round_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let foot = sketch.add_point(DVec2::new(30.0, 40.0));
    let head = sketch.add_point(DVec2::new(30.0, 90.0));
    let line = sketch.add_segment(foot, head);
    sketch.set_dimension(
        DimensionTarget::Distance {
            from: Sketch::ORIGIN,
            to: foot,
        },
        50.0,
        false,
    );

    let drag = sketch.side_drag(
        line,
        DVec2::new(30.0, 65.0),
        DVec2::new(40.0, 66.0),
        &no_grid(),
        1.0,
    );
    let SideDrag::Across { by } = drag else {
        panic!("a lone trait travels: {drag:?}");
    };

    assert_eq!(sketch.move_side(line, by, 1.0), LengthOutcome::Exact);
    assert!(
        sketch.point(foot).x > 32.0,
        "it went the hand's way: {}",
        sketch.point(foot)
    );
    assert!(
        (sketch.point(foot).length() - 50.0).abs() < SETTLED,
        "at its distance from the origin"
    );
    assert_near(
        sketch.point(head) - sketch.point(foot),
        DVec2::new(0.0, 50.0),
        "whole, as long and as upright as it was",
    );
}

#[test]
fn a_tail_or_a_construction_line_off_the_far_corner_does_not_take_the_place_of_the_opposite_side() {
    for construction in [false, true] {
        let (mut sketch, [_, b, ..], sides) = rectangle();
        match construction {
            false => {
                let tip = sketch.add_point(DVec2::new(170.0, -30.0));
                sketch.add_segment(b, tip);
            }
            true => {
                let tip = sketch.add_point(DVec2::new(200.0, 20.0));
                sketch.add_construction_segment(b, tip);
            }
        }

        let drag = sketch.side_drag(
            sides[3],
            DVec2::new(20.0, 45.0),
            DVec2::new(19.0, 60.0),
            &no_grid(),
            1.0,
        );

        let SideDrag::Along(turn) = drag else {
            panic!("sliding the left side along turns: {drag:?}");
        };
        assert_near(
            turn.about,
            DVec2::new(120.0, 45.0),
            "the middle of the right side",
        );
    }
}

#[test]
fn a_point_held_on_the_opposite_side_does_not_take_the_place_of_its_middle() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let held = sketch.add_point(DVec2::new(110.0, 70.0));
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
    sketch.add_constraint(Constraint::OnSegment {
        point: held,
        segment: sides[2],
    });

    let drag = sketch.side_drag(
        sides[0],
        DVec2::new(70.0, 20.0),
        DVec2::new(85.0, 19.0),
        &no_grid(),
        1.0,
    );

    let SideDrag::Along(turn) = drag else {
        panic!("sliding the bottom along turns: {drag:?}");
    };
    assert_near(turn.about, DVec2::new(70.0, 70.0), "the middle of the top");
}

#[test]
fn a_house_slid_by_its_floor_turns_about_a_place_above_its_middle() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(0.0, 0.0),
        DVec2::new(100.0, 0.0),
        DVec2::new(100.0, 60.0),
        DVec2::new(50.0, 100.0),
        DVec2::new(0.0, 60.0),
    ]
    .map(|place| sketch.add_point(place));
    let sides: [SegmentId; 5] =
        std::array::from_fn(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 5]));

    let drag = sketch.side_drag(
        sides[0],
        DVec2::new(50.0, 0.0),
        DVec2::new(65.0, -1.0),
        &no_grid(),
        1.0,
    );

    let SideDrag::Along(turn) = drag else {
        panic!("sliding the floor along turns: {drag:?}");
    };
    assert!(
        (turn.about.x - 50.0).abs() < 1e-9,
        "a shape the same both sides turns about its middle: {}",
        turn.about
    );
}

#[test]
fn a_pull_near_a_corner_drifting_a_little_still_resizes() {
    let (sketch, _, sides) = rectangle();

    let drag = sketch.side_drag(
        sides[2],
        DVec2::new(30.0, 70.0),
        DVec2::new(34.0, 80.0),
        &no_grid(),
        1.0,
    );

    assert_eq!(
        drag,
        SideDrag::Across {
            by: DVec2::new(0.0, 10.0)
        }
    );
}

#[test]
fn a_slot_s_touch_point_slid_round_its_cap_leaves_the_other_cap_whole() {
    let (mut sketch, corners, _) = slot();
    let pull = sketch.pull(corners[3], 1.0);
    let landing = sketch.slide(corners[3], DVec2::new(50.0, 30.0));

    sketch.settle_pulled(&pull, landing, 1.0);

    for arc in [crate::arc::ArcId(0), crate::arc::ArcId(1)] {
        assert!(
            sketch.arc_radius(arc) > 15.0,
            "a cap kept its size: {}",
            sketch.arc_radius(arc)
        );
    }
}

#[test]
fn an_arc_of_typed_radius_slid_round_turns_at_its_radius_about_its_centre() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let start = sketch.add_point(DVec2::new(90.0, 50.0));
    let end = sketch.add_point(DVec2::new(50.0, 90.0));
    let arc = sketch.add_arc(centre, start, end);
    sketch.set_dimension(DimensionTarget::ArcRadius(arc), 40.0, false);
    let pressed = DVec2::new(50.0, 50.0) + DVec2::from_angle(0.8) * 40.0;
    let cursor = DVec2::new(50.0, 50.0) + DVec2::from_angle(1.1) * 45.0;

    let drag = sketch.curve_drag(Curved::Arc(arc), pressed, cursor, cursor, &no_grid(), 1.0);
    assert!(matches!(drag, CurveDrag::Along { .. }), "{drag:?}");
    laid_curve(&mut sketch, &drag, Curved::Arc(arc));

    assert_near(sketch.point(centre), DVec2::new(50.0, 50.0), "the centre");
    assert!(
        (sketch.arc_radius(arc) - 40.0).abs() < SETTLED,
        "the radius typed: {}",
        sketch.arc_radius(arc)
    );
    let went = (sketch.point(start) - DVec2::new(50.0, 50.0)).to_angle();
    assert!((went - 0.3).abs() < 1e-6, "it turned: {went}");
}

#[test]
fn an_ellipse_with_a_typed_axis_slid_round_turns_keeping_its_shape() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 20.0));
    let west = sketch.add_point(DVec2::new(20.0, 20.0));
    let east = sketch.add_point(DVec2::new(80.0, 20.0));
    let south = sketch.add_point(DVec2::new(50.0, 10.0));
    let north = sketch.add_point(DVec2::new(50.0, 30.0));
    let id = sketch.add_ellipse(centre, [west, east], [south, north]);
    let first = sketch.ellipses()[id.0].first;
    sketch.set_dimension(DimensionTarget::Length(first), 60.0, false);

    let drag = sketch.curve_drag(
        Curved::Ellipse(id),
        DVec2::new(50.0, 30.0),
        DVec2::new(40.0, 29.0),
        DVec2::new(40.0, 29.0),
        &no_grid(),
        1.0,
    );
    laid_curve(&mut sketch, &drag, Curved::Ellipse(id));

    let drawn = sketch.ellipse_draft(id);
    assert!(
        (drawn.first.length() - 30.0).abs() < SETTLED && (drawn.second - 10.0).abs() < SETTLED,
        "its shape and size: {} by {}",
        drawn.first.length(),
        drawn.second
    );
    assert!(
        drawn.first.to_angle() > 0.1,
        "and it turned: {}",
        drawn.first
    );
}

#[test]
fn a_regular_hexagon_slid_by_a_side_turns_about_the_middle_of_the_side_opposite() {
    for (start, way) in [(0.0_f64, 1.0_f64), (0.0, -1.0), (120.0, -1.0), (300.0, 1.0)] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let places: Vec<DVec2> = (0..6)
            .map(|rank| {
                let degrees = 240.0 + start + way * 60.0 * rank as f64;
                DVec2::new(100.0, 100.0) + DVec2::from_angle(degrees.to_radians()) * 40.0
            })
            .collect();
        let corners: Vec<PointId> = places
            .iter()
            .map(|place| sketch.add_point(*place))
            .collect();
        let sides: Vec<SegmentId> = (0..6)
            .map(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 6]))
            .collect();
        let bottom = sides
            .iter()
            .copied()
            .find(|side| {
                let (from, to) = sketch.endpoints(*side);
                from.y < 70.0 && to.y < 70.0
            })
            .expect("a flat bottom");
        let floor = sketch.endpoints(bottom).0.y;

        let drag = sketch.side_drag(
            bottom,
            DVec2::new(100.0, floor),
            DVec2::new(115.0, floor - 1.0),
            &no_grid(),
            1.0,
        );

        let SideDrag::Along(turn) = drag else {
            panic!("sliding the bottom along turns: {drag:?}");
        };
        assert_near(
            turn.about,
            DVec2::new(100.0, 100.0 + 40.0 * 60f64.to_radians().sin()),
            "the middle of the top side",
        );
    }
}

#[test]
fn a_triangle_with_a_construction_height_still_turns_about_its_far_corner() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(10.0, 10.0),
        DVec2::new(110.0, 10.0),
        DVec2::new(50.0, 70.0),
    ]
    .map(|place| sketch.add_point(place));
    let sides: [SegmentId; 3] =
        std::array::from_fn(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 3]));
    let foot = sketch.add_point(DVec2::new(50.0, 10.0));
    sketch.add_constraint(Constraint::OnSegment {
        point: foot,
        segment: sides[0],
    });
    let height = sketch.add_construction_segment(foot, corners[2]);
    sketch.add_constraint(Constraint::Perpendicular {
        first: sides[0],
        second: height,
    });

    let drag = sketch.side_drag(
        sides[0],
        DVec2::new(60.0, 10.0),
        DVec2::new(75.0, 9.0),
        &no_grid(),
        1.0,
    );

    let SideDrag::Along(turn) = drag else {
        panic!("sliding the base along turns: {drag:?}");
    };
    assert_near(turn.about, DVec2::new(50.0, 70.0), "the far corner");
}

#[test]
fn a_lone_trait_tied_to_another_by_a_rule_carries_it_along() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(DVec2::new(10.0, 10.0));
    let to = sketch.add_point(DVec2::new(60.0, 10.0));
    let line = sketch.add_segment(from, to);
    let other_from = sketch.add_point(DVec2::new(80.0, 10.0));
    let other_to = sketch.add_point(DVec2::new(130.0, 10.0));
    let other = sketch.add_segment(other_from, other_to);
    sketch.add_constraint(Constraint::Collinear {
        first: line,
        second: other,
    });

    let drag = sketch.side_drag(
        line,
        DVec2::new(30.0, 10.0),
        DVec2::new(30.0, 30.0),
        &no_grid(),
        1.0,
    );
    laid(&mut sketch, &drag, line);

    assert_near(sketch.point(from), DVec2::new(10.0, 30.0), "one end");
    assert_near(sketch.point(to), DVec2::new(60.0, 30.0), "the other end");
    assert!(
        (sketch.point(other_from).y - 30.0).abs() < SETTLED
            && (sketch.point(other_to).y - 30.0).abs() < SETTLED,
        "the trait on its line came with it"
    );
}

#[test]
fn a_lone_slanted_trait_held_to_an_axis_travels_along_it_whole() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let foot = sketch.add_point(DVec2::new(30.0, 0.0));
    let head = sketch.add_point(DVec2::new(60.0, 40.0));
    let line = sketch.add_segment(foot, head);
    sketch.add_constraint(Constraint::OnAxis {
        point: foot,
        axis: crate::constraints::SketchAxis::U,
    });

    let drag = sketch.side_drag(
        line,
        DVec2::new(45.0, 20.0),
        DVec2::new(65.0, 22.0),
        &no_grid(),
        1.0,
    );
    laid(&mut sketch, &drag, line);

    assert_near(
        sketch.point(foot),
        DVec2::new(50.0, 0.0),
        "the foot, on the axis",
    );
    assert_near(
        sketch.point(head),
        DVec2::new(80.0, 40.0),
        "the head, the same way",
    );
}

#[test]
fn a_slot_s_cap_slid_round_and_out_reaches_the_hand_about_its_centre() {
    let (mut sketch, _, _) = slot();
    let cap = Curved::Arc(crate::arc::ArcId(0));
    let cursor = DVec2::new(130.0, 30.0) + DVec2::from_angle(0.5) * 24.0;

    let drag = sketch.curve_drag(
        cap,
        DVec2::new(150.0, 30.0),
        cursor,
        cursor,
        &no_grid(),
        1.0,
    );
    laid_curve(&mut sketch, &drag, cap);

    assert!(
        (sketch.arc_radius(crate::arc::ArcId(0)) - 24.0).abs() < SETTLED,
        "the cap reaches the hand: {}",
        sketch.arc_radius(crate::arc::ArcId(0))
    );
    assert_near(
        sketch.point(sketch.arc(crate::arc::ArcId(0)).center),
        DVec2::new(130.0, 30.0),
        "about its centre",
    );
}

#[test]
fn a_curve_whose_size_is_held_writes_no_size_it_did_not_reach() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(30.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, 30.0));
    let arc = sketch.add_arc(Sketch::ORIGIN, start, end);
    sketch.set_dimension(DimensionTarget::ArcRadius(arc), 30.0, false);

    let outcome = sketch.resize_in_place(Curved::Arc(arc), 34.0, 1.0);

    assert_eq!(outcome, LengthOutcome::BestEffort);
    assert!(
        (sketch.arc_radius(arc) - 30.0).abs() < 1e-9,
        "given back: {}",
        sketch.arc_radius(arc)
    );
}
