//! What sketch · crossing.rs is held to.

use super::*;

const TOLERANCE: f64 = 1e-9;

#[test]
fn two_segments_that_cross_meet_part_way_along_each_of_them() {
    let found = where_segments_cross(
        DVec2::new(0.0, 0.0),
        DVec2::new(10.0, 0.0),
        DVec2::new(2.0, -1.0),
        DVec2::new(2.0, 3.0),
    );
    let Some((along_a, along_b)) = found else {
        panic!("the two segments cross at (2, 0) and nothing was reported");
    };
    assert!(
        (along_a - 0.2).abs() < TOLERANCE,
        "a fifth of the way along the first, got {along_a}",
    );
    assert!(
        (along_b - 0.25).abs() < TOLERANCE,
        "a quarter of the way along the second, got {along_b}",
    );
}

#[test]
fn a_segment_that_only_ends_on_another_is_touching_it_not_crossing_it() {
    assert_eq!(
        where_segments_cross(
            DVec2::new(0.0, 0.0),
            DVec2::new(10.0, 0.0),
            DVec2::new(4.0, 0.0),
            DVec2::new(4.0, 6.0),
        ),
        None,
    );
}

#[test]
fn two_segments_along_the_same_line_never_cross() {
    assert_eq!(
        where_segments_cross(
            DVec2::new(0.0, 0.0),
            DVec2::new(10.0, 0.0),
            DVec2::new(3.0, 0.0),
            DVec2::new(13.0, 0.0),
        ),
        None,
    );
}

fn upper_half_circle(radius: f64) -> ArcDraft {
    ArcDraft {
        centre: DVec2::ZERO,
        start: DVec2::new(radius, 0.0),
        end: DVec2::new(-radius, 0.0),
    }
}

#[test]
fn a_straight_run_through_a_curve_cuts_it_twice() {
    let found = where_segment_crosses_arc(
        DVec2::new(-10.0, 3.0),
        DVec2::new(10.0, 3.0),
        upper_half_circle(5.0),
    );
    assert_eq!(found.len(), 2, "a chord of the bulge, got {found:?}");

    let (along_a, round_arc) = found[0];
    assert!(
        (along_a - 0.3).abs() < TOLERANCE,
        "the first meeting is three tenths along the run, got {along_a}",
    );
    let expected = (std::f64::consts::PI - 3.0f64.atan2(4.0)) / std::f64::consts::PI;
    assert!(
        (round_arc - expected).abs() < TOLERANCE,
        "and {expected} of the way round the curve, got {round_arc}",
    );

    assert!(
        (found[1].0 - 0.7).abs() < TOLERANCE,
        "the second is seven tenths along the run, got {}",
        found[1].0,
    );
}

#[test]
fn a_chord_passing_below_a_bulge_never_meets_it() {
    assert!(
        where_segment_crosses_arc(
            DVec2::new(-10.0, -3.0),
            DVec2::new(10.0, -3.0),
            upper_half_circle(5.0),
        )
        .is_empty(),
        "the run cuts the circle where the arc does not go",
    );
}

#[test]
fn a_straight_run_through_a_whole_circle_keeps_both_of_its_meetings() {
    let mut found = where_segment_crosses_circle(
        DVec2::new(-10.0, 3.0),
        DVec2::new(10.0, 3.0),
        DVec2::ZERO,
        5.0,
    );
    assert_eq!(found.len(), 2, "a chord of the circle, got {found:?}");

    let turn = std::f64::consts::TAU;
    for (expected_along, expected_round) in [
        (0.3, (3.0f64).atan2(-4.0) / turn),
        (0.7, (3.0f64).atan2(4.0) / turn),
    ] {
        let (along, round) = found.remove(0);
        assert!(
            (along - expected_along).abs() < TOLERANCE,
            "{expected_along} of the way along the run, got {along}",
        );
        assert!(
            (round - expected_round).abs() < TOLERANCE,
            "and {expected_round} of the way round the circle, got {round}",
        );
    }
}

#[test]
fn two_whole_circles_meet_twice_wherever_they_overlap() {
    let found = where_circles_cross(DVec2::ZERO, 5.0, DVec2::new(6.0, 0.0), 5.0);
    assert_eq!(
        found.len(),
        2,
        "the two circles meet at (3, 4) and (3, -4): {found:?}"
    );

    let turn = std::f64::consts::TAU;
    let (round_near, round_far) = found[0];
    assert!(
        (round_near - (4.0f64).atan2(3.0) / turn).abs() < TOLERANCE,
        "at (3, 4) the first has turned {round_near}",
    );
    assert!(
        (round_far - (4.0f64).atan2(-3.0) / turn).abs() < TOLERANCE,
        "and the second {round_far}",
    );
}

#[test]
fn two_circles_that_miss_each_other_never_meet() {
    assert!(where_circles_cross(DVec2::ZERO, 1.0, DVec2::new(9.0, 0.0), 2.0).is_empty());
    assert!(
        where_circles_cross(DVec2::ZERO, 5.0, DVec2::ZERO, 2.0).is_empty(),
        "one inside the other, sharing a centre",
    );
}

#[test]
fn a_curve_crossing_a_whole_circle_reports_only_the_meetings_it_runs_over() {
    let found = where_arc_crosses_circle(upper_half_circle(5.0), DVec2::new(6.0, 0.0), 5.0);
    assert_eq!(
        found.len(),
        1,
        "the circles meet at (3, 4) and (3, -4), and the arc runs over the first only: {found:?}",
    );

    let (round_arc, round_circle) = found[0];
    let expected = (4.0f64).atan2(3.0) / std::f64::consts::PI;
    assert!(
        (round_arc - expected).abs() < TOLERANCE,
        "{expected} of the way round the arc, got {round_arc}",
    );
    let expected = (4.0f64).atan2(-3.0) / std::f64::consts::TAU;
    assert!(
        (round_circle - expected).abs() < TOLERANCE,
        "and {expected} of the way round the circle, got {round_circle}",
    );
}

#[test]
fn two_curves_meet_only_where_both_of_them_run() {
    let mut shifted = upper_half_circle(5.0);
    shifted.centre = DVec2::new(6.0, 0.0);
    shifted.start = DVec2::new(11.0, 0.0);
    shifted.end = DVec2::new(1.0, 0.0);

    let found = where_arcs_cross(upper_half_circle(5.0), shifted);
    assert_eq!(
        found.len(),
        1,
        "the circles meet at (3, 4) and (3, -4), and only the first is on both arcs: {found:?}",
    );

    let (round_first, round_second) = found[0];
    let expected = 4.0f64.atan2(3.0) / std::f64::consts::PI;
    assert!(
        (round_first - expected).abs() < TOLERANCE,
        "{expected} of the way round the first, got {round_first}",
    );
    let expected = 4.0f64.atan2(-3.0) / std::f64::consts::PI;
    assert!(
        (round_second - expected).abs() < TOLERANCE,
        "{expected} of the way round the second, got {round_second}",
    );
}
