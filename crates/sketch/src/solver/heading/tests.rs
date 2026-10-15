//! What sketch · solver/heading.rs is held to.

use super::*;
use crate::WorkPlane;

/// Above the tolerance, as a creeping sweep's error is.
const OFF: f64 = 1e-3;
const SIZE: f64 = 10.0;

/// The origin and one point to the right of it.
fn a_point_alone() -> (Sketch, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let point = sketch.add_point(DVec2::new(1.0, 0.0));
    (sketch, point)
}

/// A pass settling `point` towards `x` along the first axis, by `pace` of
/// what is left each time, read as a sweep reads it.
fn a_pass(
    heading: &mut Heading,
    sketch: &mut Sketch,
    point: PointId,
    x: f64,
    pace: f64,
    worst: f64,
) -> bool {
    heading.starting(sketch);
    let at = sketch.point(point);
    sketch.place_point(point, DVec2::new(x + pace * (at.x - x), at.y));
    let pinned = vec![false; sketch.points().len()];
    heading.passed(sketch, worst, true, &pinned, SIZE)
}

/// One pass of a map that settles slowly one way and fast the others: it
/// moves a point a little less than the time before, the same way.
fn pass(at: &[f64]) -> Vec<f64> {
    let slow = [0.998, 0.9, 0.5];
    let settled = [3.0, -2.0, 7.0];
    (0..3)
        .map(|axis| {
            settled[axis] + slow[axis] * (at[axis] - settled[axis]) + 0.001 * at[(axis + 1) % 3]
        })
        .collect()
}

fn where_it_settles() -> Vec<f64> {
    let mut at = vec![0.0; 3];
    for _ in 0..200_000 {
        at = pass(&at);
    }
    at
}

#[test]
fn passes_creeping_towards_a_place_are_mixed_into_that_place() {
    let mut passes = VecDeque::new();
    let mut at = vec![10.0, 10.0, 10.0];
    for _ in 0..4 {
        let to = pass(&at);
        passes.push_back(Pass {
            from: at,
            to: to.clone(),
        });
        at = to;
    }

    let mixed = mixed(&passes).expect("enough passes to mix");
    let settled = where_it_settles();
    assert!(
        farthest(&mixed, &settled, 0) < 1e-9,
        "mixed {mixed:?}, settled {settled:?}"
    );
    assert!(
        farthest(&at, &settled, 0) > 1.0,
        "the passes alone are still far"
    );
}

#[test]
fn one_pass_is_not_enough_to_say_where_they_go() {
    let passes = VecDeque::from([Pass {
        from: vec![0.0],
        to: vec![1.0],
    }]);

    assert!(mixed(&passes).is_none());
}

#[test]
fn a_difference_saying_nothing_new_is_given_no_weight() {
    let one = vec![1.0, 2.0, 0.0];
    let twice = vec![2.0, 4.0, 0.0];

    let weights = least_squares(&[one.clone(), twice], &one);

    assert!((weights[0] - 1.0).abs() < 1e-12, "{weights:?}");
    assert_eq!(weights[1], 0.0, "a column left out weighs nothing at all");
}

#[test]
fn a_mix_carrying_a_point_past_a_tenth_of_the_drawing_puts_the_drawing_back() {
    let (mut sketch, point) = a_point_alone();
    let mut heading = Heading::new();
    let far = 1000.0;
    a_pass(&mut heading, &mut sketch, point, far, 0.999, OFF);
    let began = sketch.point(point);

    let put_back = (0..3).any(|_| a_pass(&mut heading, &mut sketch, point, far, 0.999, OFF));

    assert!(
        put_back,
        "where the passes are heading is past a tenth of the drawing"
    );
    assert_eq!(sketch.point(point), began);
    a_pass(&mut heading, &mut sketch, point, far, 0.999, OFF);
    let as_it_comes = far + 0.999 * (began.x - far);
    assert!(
        (sketch.point(point).x - as_it_comes).abs() < 1e-12,
        "once given up, the passes are taken as they come"
    );
}

#[test]
fn a_mix_near_at_hand_is_gone_to() {
    let (mut sketch, point) = a_point_alone();
    let mut heading = Heading::new();
    for _ in 0..3 {
        a_pass(&mut heading, &mut sketch, point, 1.5, 0.9, OFF);
    }

    assert!(
        (sketch.point(point).x - 1.5).abs() < 1e-12,
        "{}",
        sketch.point(point)
    );
}

#[test]
fn passes_holding_the_rules_again_are_no_longer_mixed() {
    let (mut sketch, point) = a_point_alone();
    let mut heading = Heading::new();
    a_pass(&mut heading, &mut sketch, point, 1.5, 0.9, OFF);
    a_pass(&mut heading, &mut sketch, point, 1.5, 0.9, OFF);
    let before = sketch.point(point).x;

    a_pass(&mut heading, &mut sketch, point, 1.5, 0.9, TOLERANCE / 2.0);
    a_pass(&mut heading, &mut sketch, point, 1.5, 0.9, TOLERANCE / 2.0);

    let as_they_come = 1.5 + 0.9 * 0.9 * (before - 1.5);
    assert!((sketch.point(point).x - as_they_come).abs() < 1e-12);
}

#[test]
fn a_pinned_point_is_left_where_its_pass_left_it() {
    let (mut sketch, point) = a_point_alone();
    let mut heading = Heading::new();
    let mut pinned = vec![false; sketch.points().len()];
    pinned[point.0] = true;
    for _ in 0..4 {
        heading.starting(&sketch);
        let at = sketch.point(point);
        sketch.place_point(point, DVec2::new(1.5 + 0.9 * (at.x - 1.5), at.y));
        let left = sketch.point(point);
        heading.passed(&mut sketch, OFF, true, &pinned, SIZE);

        assert_eq!(sketch.point(point), left);
    }
}
