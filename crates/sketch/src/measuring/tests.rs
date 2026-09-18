//! What sketch · measuring.rs is held to.

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
fn an_angle_between_two_traits_that_do_not_meet_is_told_apart_from_one_that_does() {
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

    assert_eq!(outcome, DimensionPick::TraitsDoNotTouch);
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
