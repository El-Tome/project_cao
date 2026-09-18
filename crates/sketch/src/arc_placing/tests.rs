//! What sketch · arc_placing.rs is held to.

use super::*;
use crate::arcing::sweep_of;

const TOLERANCE: f64 = 1e-9;

#[test]
fn the_cursor_says_how_far_round_the_curve_runs_and_never_how_wide_it_is() {
    let centre = DVec2::ZERO;
    let start = DVec2::new(10.0, 0.0);
    let far_off_to_the_north = DVec2::new(0.0, 400.0);

    let drawn = arc_from(ArcMode::ByCenter, &[centre, start], far_off_to_the_north)
        .expect("a centre, a first end and a direction make an arc");

    assert_eq!(drawn.start, start);
    assert!(
        (drawn.end.distance(DVec2::new(0.0, 10.0))) < TOLERANCE,
        "the far end landed at {:?}, off the circle the near one fixed",
        drawn.end,
    );
}

#[test]
fn a_curve_running_nowhere_is_no_arc() {
    let centre = DVec2::ZERO;
    let start = DVec2::new(10.0, 0.0);

    assert_eq!(
        arc_from(ArcMode::ByCenter, &[centre, start], start),
        None,
        "the cursor is back where the curve starts, so there is no curve",
    );
    assert_eq!(
        arc_from(ArcMode::ByCenter, &[centre, centre], DVec2::new(0.0, 5.0)),
        None,
        "a curve no distance from its centre is a point",
    );
    assert_eq!(
        arc_from(ArcMode::ByCenter, &[centre], DVec2::new(0.0, 5.0)),
        None,
        "the centre alone says nothing about how wide the curve is",
    );
}

#[test]
fn an_arc_by_its_ends_passes_through_the_point_it_was_bent_to() {
    let (a, b) = (DVec2::new(-10.0, 0.0), DVec2::new(10.0, 0.0));
    let bent_up = DVec2::new(0.0, 10.0);

    let drawn = arc_from(ArcMode::ByEnds, &[a, b], bent_up)
        .expect("two ends and a point between them make an arc");

    // The walk always turns counter-clockwise, so passing through the top
    // on the way from one end to the other starts at the right-hand one.
    assert_eq!(drawn.start, b);
    assert_eq!(drawn.end, a);
    let radius = drawn.centre.distance(a);
    assert!(
        (drawn.centre.distance(bent_up) - radius).abs() < TOLERANCE,
        "the curve does not run through the point it was bent to: centre = {:?}",
        drawn.centre,
    );
}

#[test]
fn an_arc_by_its_ends_bent_the_other_way_swaps_which_end_it_starts_from() {
    let (a, b) = (DVec2::new(-10.0, 0.0), DVec2::new(10.0, 0.0));
    let bent_down = DVec2::new(0.0, -10.0);

    let drawn = arc_from(ArcMode::ByEnds, &[a, b], bent_down)
        .expect("two ends and a point between them make an arc");

    assert_eq!(
        drawn.start, a,
        "the walk still turns counter-clockwise, so bending the other way starts from the other end",
    );
    assert_eq!(drawn.end, b);
}

#[test]
fn three_points_on_a_line_make_no_arc_by_its_ends() {
    let (a, b) = (DVec2::new(0.0, 0.0), DVec2::new(20.0, 0.0));

    assert_eq!(
        arc_from(ArcMode::ByEnds, &[a, b], DVec2::new(10.0, 0.0)),
        None,
        "a point between the ends and in line with them bends no curve",
    );
}

#[test]
fn a_radius_typed_while_placing_the_second_place_holds_however_the_cursor_turns() {
    let centre = DVec2::ZERO;
    let cursor = DVec2::new(3.0, 4.0);

    let placed = aimed(ArcMode::ByCenter, &[centre], cursor, Some(100.0), 2.0);

    assert!(
        (centre.distance(placed) - 50.0).abs() < TOLERANCE,
        "100 mm at a scale of 2 mm per unit is 50 units: got {}",
        centre.distance(placed),
    );
    assert!(
        (placed - centre)
            .normalize()
            .distance((cursor - centre).normalize())
            < TOLERANCE,
        "the direction still follows the cursor: got {placed:?}",
    );
}

#[test]
fn a_distance_typed_while_placing_the_second_end_of_a_by_ends_arc_holds_too() {
    let first_end = DVec2::new(5.0, 0.0);
    let cursor = DVec2::new(5.0, 100.0);

    let placed = aimed(ArcMode::ByEnds, &[first_end], cursor, Some(30.0), 1.0);

    assert!((first_end.distance(placed) - 30.0).abs() < TOLERANCE);
}

#[test]
fn an_angle_typed_while_placing_the_third_place_turns_the_start_direction_by_that_many_degrees() {
    let centre = DVec2::ZERO;
    let start = DVec2::new(10.0, 0.0);

    let placed = aimed(
        ArcMode::ByCenter,
        &[centre, start],
        DVec2::new(-1.0, -1.0),
        Some(90.0),
        1.0,
    );

    assert!(
        placed.distance(DVec2::new(0.0, 10.0)) < TOLERANCE,
        "90 degrees from the start turns straight up regardless of where the cursor is: got {placed:?}",
    );
}

#[test]
fn a_radius_typed_while_bending_a_by_ends_arc_holds_however_near_the_cursor_stays() {
    let (a, b) = (DVec2::new(-30.0, 0.0), DVec2::new(30.0, 0.0));
    let barely_above_the_chord = DVec2::new(0.0, 1.0);

    let bent = aimed(
        ArcMode::ByEnds,
        &[a, b],
        barely_above_the_chord,
        Some(100.0),
        2.0,
    );
    let drawn = arc_from(ArcMode::ByEnds, &[a, b], bent).expect("a radius that reaches both ends");

    assert!(
        (drawn.centre.distance(drawn.start) - 50.0).abs() < TOLERANCE,
        "100 mm at a scale of 2 mm per unit is 50 units: the curve came out at {}",
        drawn.centre.distance(drawn.start),
    );
    assert!(
        bent.y > 0.0,
        "the cursor still says which side it bends to: got {bent:?}",
    );
}

#[test]
fn a_cursor_dragged_well_clear_of_the_chord_bends_that_radius_the_long_way_round() {
    let (a, b) = (DVec2::new(-30.0, 0.0), DVec2::new(30.0, 0.0));
    let well_above_the_chord = DVec2::new(0.0, 100.0);

    let bent = aimed(
        ArcMode::ByEnds,
        &[a, b],
        well_above_the_chord,
        Some(50.0),
        1.0,
    );
    let drawn = arc_from(ArcMode::ByEnds, &[a, b], bent).expect("a radius that reaches both ends");

    assert!(
        (drawn.centre.distance(drawn.start) - 50.0).abs() < TOLERANCE,
        "the curve came out at {}, not the 50 that was typed",
        drawn.centre.distance(drawn.start),
    );
    assert!(
        sweep_of(drawn) > std::f64::consts::PI,
        "a cursor that far out asks for the major arc: it runs {} radians",
        sweep_of(drawn),
    );
}

#[test]
fn a_radius_too_short_to_reach_both_ends_is_refused_rather_than_stretched_to_fit() {
    let (a, b) = (DVec2::new(-30.0, 0.0), DVec2::new(30.0, 0.0));
    let cursor = DVec2::new(0.0, 10.0);

    assert_eq!(
        aimed(ArcMode::ByEnds, &[a, b], cursor, Some(10.0), 1.0),
        cursor,
        "no curve 10 wide reaches ends 60 apart, so the cursor goes on deciding",
    );
}

#[test]
fn the_leg_an_angle_opens_from_points_where_an_angle_of_zero_would_land() {
    let centre = DVec2::new(4.0, -2.0);
    let start = DVec2::new(4.0, 8.0);

    let (from, to) = angle_reference(ArcMode::ByCenter, &[centre, start])
        .expect("a by-centre arc reads its third click as an angle");
    let no_angle_at_all = aimed(
        ArcMode::ByCenter,
        &[centre, start],
        DVec2::new(-50.0, -50.0),
        Some(0.0),
        1.0,
    );

    assert_eq!(from, centre, "the angle opens from the centre");
    assert!(
        (to - from)
            .normalize()
            .distance((no_angle_at_all - from).normalize())
            < TOLERANCE,
        "the leg runs towards {to:?} while an angle of zero lands at {no_angle_at_all:?}",
    );
}

#[test]
fn an_arc_placed_by_its_ends_measures_no_angle_and_so_opens_from_nothing() {
    let (a, b) = (DVec2::new(-10.0, 0.0), DVec2::new(10.0, 0.0));

    assert_eq!(angle_reference(ArcMode::ByEnds, &[a, b]), None);
    assert_eq!(
        angle_reference(ArcMode::ByCenter, &[a]),
        None,
        "a centre on its own is not yet measuring anything",
    );
}

#[test]
fn leaving_the_field_empty_leaves_the_cursor_deciding_everything() {
    let cursor = DVec2::new(7.0, 8.0);

    assert_eq!(
        aimed(ArcMode::ByCenter, &[DVec2::ZERO], cursor, None, 1.0),
        cursor
    );
}
