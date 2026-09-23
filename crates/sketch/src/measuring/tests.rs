//! What sketch · measuring.rs is held to.
//!
//! Closes #405.
//! - a point on the trait, then the trait, lays no dimension and is refused as
//!   already on it — `a_point_on_the_trait_then_the_trait_is_refused_as_already_on_it`
//! - one of the trait's own ends, then the trait, is refused the same way —
//!   `an_end_of_the_trait_then_the_trait_is_refused_as_already_on_it`
//! - a point on the trait's prolongation, then the trait, lays no dimension and
//!   is refused as in line with it —
//!   `a_point_on_the_prolongation_of_the_trait_is_refused_as_in_line_with_it`
//! - a point off the line, then the trait, still lays the distance square to it
//!   — `a_point_off_the_line_then_the_trait_still_measures_the_distance_to_it`
//! - a refused click enters nothing in the history — no test: a refusal is a
//!   `DimensionPick` that carries no target, and the canvas records an
//!   operation only from a `Target`; the sentence each refusal is said with
//!   lives in the interface's language file, not here

use super::*;
use crate::plane::WorkPlane;

#[test]
fn a_point_to_point_click_waits_for_the_second_point_then_measures_between_them() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(10.0, 10.0));
    let b = sketch.add_point(DVec2::new(30.0, 10.0));

    let (picks, first_pick) = measure_pick(
        &sketch,
        DimensionMode::PointToPoint,
        DimensionPicks::default(),
        sketch.point(a),
        1.0,
    );
    assert_eq!(first_pick, DimensionPick::WaitingForSecondPoint);

    let (_, second_pick) = measure_pick(
        &sketch,
        DimensionMode::PointToPoint,
        picks,
        sketch.point(b),
        1.0,
    );
    assert_eq!(
        second_pick,
        DimensionPick::Target(DimensionTarget::Distance { from: a, to: b }),
    );
}

#[test]
fn an_angle_between_two_parallel_traits_is_refused_for_being_parallel() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(40.0, 0.0));
    let first = sketch.add_segment(a, b);
    let apart_from = sketch.add_point(DVec2::new(0.0, 40.0));
    let apart_to = sketch.add_point(DVec2::new(40.0, 40.0));
    let _stray = sketch.add_segment(apart_from, apart_to);

    let picks = DimensionPicks {
        first_angle_segment: Some(first),
        ..DimensionPicks::default()
    };
    let (_, outcome) = measure_pick(
        &sketch,
        DimensionMode::Angle,
        picks,
        DVec2::new(20.0, 40.0),
        1.0,
    );

    assert!(
        matches!(outcome, DimensionPick::TraitsAreParallel { .. }),
        "two traits that run the same way open no angle, got {outcome:?}",
    );
}

#[test]
fn nothing_under_the_cursor_leaves_what_was_already_picked_untouched() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let point = sketch.add_point(DVec2::new(0.0, 0.0));
    let picks = DimensionPicks {
        first_point: Some(point),
        ..DimensionPicks::default()
    };

    let (kept, outcome) = measure_pick(
        &sketch,
        DimensionMode::Auto,
        picks,
        DVec2::new(500.0, 500.0),
        1.0,
    );

    assert_eq!(outcome, DimensionPick::Nothing);
    assert_eq!(
        kept, picks,
        "a click on empty ground says nothing to measure, but does not forget the point already picked",
    );
}

#[test]
fn a_click_on_an_arc_measures_its_radius() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let start = sketch.add_point(DVec2::new(10.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, 10.0));
    let arc = sketch.add_arc(centre, start, end);

    let (_, pick) = measure_pick(
        &sketch,
        DimensionMode::Auto,
        DimensionPicks::default(),
        DVec2::new(
            10.0 * std::f64::consts::FRAC_1_SQRT_2,
            10.0 * std::f64::consts::FRAC_1_SQRT_2,
        ),
        1.0,
    );

    assert_eq!(pick, DimensionPick::Target(DimensionTarget::ArcRadius(arc)));
}

/// A trait lying along the horizontal from x = 10 to x = 30, and whatever point
/// the caller lays, taken as the first half of a distance.
fn a_point_picked_then_the_trait(place: DVec2) -> DimensionPick {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(10.0, 5.0));
    let end = sketch.add_point(DVec2::new(30.0, 5.0));
    sketch.add_segment(start, end);
    let point = match place {
        _ if place == DVec2::new(10.0, 5.0) => start,
        _ => sketch.add_point(place),
    };

    let (picks, _) = measure_pick(
        &sketch,
        DimensionMode::Auto,
        DimensionPicks::default(),
        sketch.point(point),
        0.5,
    );
    let (_, pick) = measure_pick(
        &sketch,
        DimensionMode::Auto,
        picks,
        DVec2::new(25.0, 5.0),
        0.5,
    );
    pick
}

#[test]
fn a_point_on_the_trait_then_the_trait_is_refused_as_already_on_it() {
    assert_eq!(
        a_point_picked_then_the_trait(DVec2::new(20.0, 5.0)),
        DimensionPick::PointAlreadyOnTheTrait,
        "a point sitting on the trait is no distance from it at all",
    );
}

#[test]
fn an_end_of_the_trait_then_the_trait_is_refused_as_already_on_it() {
    assert_eq!(
        a_point_picked_then_the_trait(DVec2::new(10.0, 5.0)),
        DimensionPick::PointAlreadyOnTheTrait,
        "the trait's own end is no distance from the trait either",
    );
}

#[test]
fn a_point_on_the_prolongation_of_the_trait_is_refused_as_in_line_with_it() {
    assert_eq!(
        a_point_picked_then_the_trait(DVec2::new(40.0, 5.0)),
        DimensionPick::PointInLineWithTheTrait,
        "a point beyond the trait's end, on its line, is no distance from that line",
    );
}

#[test]
fn a_point_off_the_line_then_the_trait_still_measures_the_distance_to_it() {
    let pick = a_point_picked_then_the_trait(DVec2::new(20.0, 15.0));

    assert!(
        matches!(
            pick,
            DimensionPick::Target(DimensionTarget::PointToSegment { .. })
        ),
        "a point standing off the line has a distance to it, and the tool lays it: {pick:?}",
    );
}

#[test]
fn two_parallel_traits_name_themselves_so_the_gap_between_them_can_be_read() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low_left = sketch.add_point(DVec2::new(0.0, 0.0));
    let low_right = sketch.add_point(DVec2::new(40.0, 0.0));
    let first = sketch.add_segment(low_left, low_right);
    let high_left = sketch.add_point(DVec2::new(0.0, 40.0));
    let high_right = sketch.add_point(DVec2::new(40.0, 40.0));
    let second = sketch.add_segment(high_left, high_right);

    let picks = DimensionPicks {
        first_angle_segment: Some(first),
        ..DimensionPicks::default()
    };
    let (_, outcome) = measure_pick(
        &sketch,
        DimensionMode::Angle,
        picks,
        DVec2::new(20.0, 40.0),
        1.0,
    );

    assert_eq!(
        outcome,
        DimensionPick::TraitsAreParallel { first, second },
        "an angle is nothing here, but the gap is the question, and reading it needs both traits named",
    );
}
