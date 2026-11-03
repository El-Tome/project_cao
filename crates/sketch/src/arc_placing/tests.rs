//! What sketch · arc_placing.rs is held to.
//!
//! Closes #483, for the rule that turns the hand's path into an arc.
//! - the cursor leaves the start going down: the arc turns clockwise. Going
//!   up: counter-clockwise — `a_hand_leaving_the_start_downwards_turns_the_arc_clockwise`,
//!   `a_hand_leaving_the_start_upwards_turns_the_arc_counter_clockwise`
//! - it keeps going as far round as the hand does: down past a quarter, on past
//!   the half, up the far side, three quarters of a circle, clockwise —
//!   `a_hand_going_three_quarters_round_downwards_draws_three_quarters_clockwise`
//! - winding back over the start and on the other way turns the arc the other
//!   way — `winding_back_over_the_start_and_on_turns_the_arc_the_other_way`
//! - past a full turn, the arc starts again from nothing, still in the same
//!   direction — `past_a_full_turn_the_arc_starts_again_from_nothing_the_same_way_round`,
//!   and coming back short of that turn closes it again rather than flipping
//!   it — `coming_back_short_of_a_full_turn_closes_the_arc_again_the_same_way_round`
//! - an angle typed in the field is laid out in the direction the hand has
//!   taken: gone down, typed `270`, three quarters clockwise; the value is
//!   always positive — `an_angle_typed_after_the_hand_went_down_is_laid_out_clockwise`
//! - nothing has moved yet: counter-clockwise, as today; the first movement
//!   decides after that — `an_angle_typed_before_the_hand_moved_is_laid_out_counter_clockwise`
//! - a clockwise arc is recorded as the sketch has always held arcs,
//!   counter-clockwise from `start` to `end` with its ends swapped —
//!   `a_hand_leaving_the_start_downwards_turns_the_arc_clockwise`
//! - the line the typed angle is measured from still starts from the start leg —
//!   `the_leg_an_angle_opens_from_points_where_an_angle_of_zero_would_land`
//! - a clockwise arc gets its angle dimension and its two construction radii —
//!   no test: held in `crates/app/src/screens/viewport/input/arcs/tests.rs`,
//!   where the tool lays them
//! - `ArcMode::ByEnds` and its tests, untouched — no test: the `ByEnds` tests
//!   above are left as they were, and still pass

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

fn turned_along(places: &[DVec2], path: &[DVec2]) -> f64 {
    path.iter()
        .fold(0.0, |turned, &cursor| turned_round(places, turned, cursor))
}

#[test]
fn a_hand_leaving_the_start_downwards_turns_the_arc_clockwise() {
    let centre = DVec2::ZERO;
    let start = DVec2::new(10.0, 0.0);
    let places = [centre, start];
    let down_to_the_south = [
        DVec2::new(10.0, -1.0),
        DVec2::new(9.0, -4.0),
        DVec2::new(7.0, -7.0),
        DVec2::new(4.0, -9.0),
        DVec2::new(0.0, -10.0),
    ];

    let turned = turned_along(&places, &down_to_the_south);
    let drawn = arc_turned(&places, turned, DVec2::new(0.0, -10.0))
        .expect("a centre, a start and a hand gone a quarter round make an arc");

    assert!(
        (sweep_of(drawn) - std::f64::consts::FRAC_PI_2).abs() < TOLERANCE,
        "the hand went a quarter round, and the arc runs {} degrees",
        sweep_of(drawn).to_degrees(),
    );
    assert!(
        drawn.start.distance(DVec2::new(0.0, -10.0)) < TOLERANCE
            && drawn.end.distance(start) < TOLERANCE,
        "a clockwise arc is held counter-clockwise with its ends swapped, \
         from {:?} to {:?}",
        drawn.start,
        drawn.end,
    );
}

#[test]
fn a_hand_leaving_the_start_upwards_turns_the_arc_counter_clockwise() {
    let centre = DVec2::ZERO;
    let start = DVec2::new(10.0, 0.0);
    let places = [centre, start];
    let up_to_the_north = [
        DVec2::new(10.0, 1.0),
        DVec2::new(9.0, 4.0),
        DVec2::new(7.0, 7.0),
        DVec2::new(4.0, 9.0),
        DVec2::new(0.0, 10.0),
    ];

    let turned = turned_along(&places, &up_to_the_north);
    let drawn = arc_turned(&places, turned, DVec2::new(0.0, 10.0))
        .expect("a centre, a start and a hand gone a quarter round make an arc");

    assert!(
        (sweep_of(drawn) - std::f64::consts::FRAC_PI_2).abs() < TOLERANCE,
        "the hand went a quarter round, and the arc runs {} degrees",
        sweep_of(drawn).to_degrees(),
    );
    assert!(
        drawn.start.distance(start) < TOLERANCE
            && drawn.end.distance(DVec2::new(0.0, 10.0)) < TOLERANCE,
        "a counter-clockwise arc runs from the start it was given, \
         from {:?} to {:?}",
        drawn.start,
        drawn.end,
    );
}

const ROUND: [DVec2; 2] = [DVec2::ZERO, DVec2::new(10.0, 0.0)];

fn at_degrees(degrees: f64) -> DVec2 {
    DVec2::from_angle(degrees.to_radians()) * 10.0
}

fn hand_going_round_through_degrees(stops: &[f64]) -> Vec<DVec2> {
    let mut path = Vec::new();
    let mut at: f64 = 0.0;
    for &stop in stops {
        while (stop - at).abs() > 1e-9 {
            at += (stop - at).clamp(-5.0, 5.0);
            path.push(at_degrees(at));
        }
    }
    path
}

fn arc_after(stops: &[f64]) -> ArcDraft {
    let path = hand_going_round_through_degrees(stops);
    let cursor = *path.last().expect("the hand went somewhere");
    arc_turned(&ROUND, turned_along(&ROUND, &path), cursor)
        .expect("a centre, a start and a hand gone round make an arc")
}

fn assert_runs(drawn: ArcDraft, from_degrees: f64, sweep_degrees: f64) {
    assert!(
        drawn.start.distance(at_degrees(from_degrees)) < TOLERANCE
            && (sweep_of(drawn).to_degrees() - sweep_degrees).abs() < 1e-6,
        "expected {sweep_degrees} degrees counter-clockwise from {from_degrees}, \
         got {} degrees from {:?} to {:?}",
        sweep_of(drawn).to_degrees(),
        drawn.start,
        drawn.end,
    );
}

#[test]
fn a_hand_going_three_quarters_round_downwards_draws_three_quarters_clockwise() {
    let drawn = arc_after(&[-270.0]);

    assert_runs(drawn, 90.0, 270.0);
    assert!(
        drawn.end.distance(ROUND[1]) < TOLERANCE,
        "the arc has to end on the start the hand left from, not at {:?}",
        drawn.end,
    );
}

#[test]
fn winding_back_over_the_start_and_on_turns_the_arc_the_other_way() {
    assert_runs(arc_after(&[-200.0, 45.0]), 0.0, 45.0);
    assert_runs(arc_after(&[200.0, -45.0]), -45.0, 45.0);
}

#[test]
fn past_a_full_turn_the_arc_starts_again_from_nothing_the_same_way_round() {
    assert_runs(arc_after(&[-370.0]), -10.0, 10.0);
    assert_runs(arc_after(&[370.0]), 0.0, 10.0);
}

#[test]
fn coming_back_short_of_a_full_turn_closes_the_arc_again_the_same_way_round() {
    assert_runs(arc_after(&[-370.0, -350.0]), 10.0, 350.0);
}

#[test]
fn an_angle_typed_before_the_hand_moved_is_laid_out_counter_clockwise() {
    let not_moved = turned_along(&ROUND, &[ROUND[1]]);
    let typed_at_once = aimed(ArcMode::ByCenter, &ROUND, ROUND[1], Some(90.0), 1.0);

    let drawn = arc_turned(&ROUND, not_moved, typed_at_once)
        .expect("a centre, a start and a typed angle make an arc");

    assert_runs(drawn, 0.0, 90.0);
}
