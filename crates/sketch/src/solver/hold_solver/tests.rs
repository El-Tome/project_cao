//! What sketch · solver/hold_solver.rs is held to.
//!
//! Closes #479.
//! - a free point laid at the middle of a free trait comes to it, and the
//!   trait stays where it is — `a_point_laid_at_the_middle_comes_to_the_trait`
//! - a point already at the middle stays there when the trait's length is
//!   retyped — `a_point_at_the_middle_stays_there_when_the_length_changes`

use glam::DVec2;

use crate::LengthOutcome;
use crate::constraints::{Constraint, DimensionTarget};
use crate::plane::WorkPlane;
use crate::sketch::Sketch;

const SCALE: f64 = 1.0;

/// A micrometre: the solver stops at a thousandth of a percent of the
/// drawing's size, far inside that.
const SETTLED: f64 = 1e-3;

#[test]
fn a_point_laid_at_the_middle_comes_to_the_trait() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(100.0, 100.0));
    let end = sketch.add_point(DVec2::new(200.0, 100.0));
    let segment = sketch.add_segment(start, end);
    let point = sketch.add_point(DVec2::new(150.0, 150.0));

    let outcome = sketch.lay_rule(Constraint::Midpoint { point, segment }, SCALE);

    assert_eq!(outcome, LengthOutcome::Exact);
    for (at, expected, what) in [
        (start, DVec2::new(100.0, 100.0), "the trait's start"),
        (end, DVec2::new(200.0, 100.0), "the trait's end"),
        (point, DVec2::new(150.0, 100.0), "the point"),
    ] {
        assert!(
            sketch.point(at).distance(expected) < SETTLED,
            "{what} was to be at {expected}, it is at {}",
            sketch.point(at),
        );
    }
}

#[test]
fn a_point_at_the_middle_stays_there_when_the_length_changes() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(100.0, 100.0));
    let end = sketch.add_point(DVec2::new(200.0, 100.0));
    let segment = sketch.add_segment(start, end);
    let point = sketch.add_point(DVec2::new(150.0, 100.0));
    sketch.add_constraint(Constraint::Midpoint { point, segment });
    sketch.set_dimension(DimensionTarget::Length(segment), 100.0, false);

    sketch.set_dimension(DimensionTarget::Length(segment), 160.0, false);
    assert_eq!(
        sketch.land_value(DimensionTarget::Length(segment), SCALE),
        LengthOutcome::Exact,
    );

    let middle = (sketch.point(start) + sketch.point(end)) / 2.0;
    assert!(
        sketch.point(point).distance(middle) < SETTLED,
        "the point was to stay at the middle {middle}, it is at {}",
        sketch.point(point),
    );
    assert!((sketch.segment_length(segment) - 160.0).abs() < SETTLED);
}

#[test]
fn two_traits_laid_on_one_line_ask_that_each_end_of_the_second_stands_on_the_first() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(10.0, 10.0));
    let end = sketch.add_point(DVec2::new(50.0, 10.0));
    let first = sketch.add_segment(start, end);
    let near = sketch.add_point(DVec2::new(20.0, 13.0));
    let far = sketch.add_point(DVec2::new(60.0, 15.0));
    let second = sketch.add_segment(near, far);
    assert!(sketch.equations(SCALE).is_empty());

    sketch.add_constraint(Constraint::Collinear { first, second });

    let rows = sketch.equations(SCALE);
    let expected = [
        (
            3.0,
            [
                (start, DVec2::new(0.0, -0.75)),
                (end, DVec2::new(0.0, -0.25)),
                (near, DVec2::new(0.0, 1.0)),
            ],
        ),
        (
            5.0,
            [
                (start, DVec2::new(0.0, 0.25)),
                (end, DVec2::new(0.0, -1.25)),
                (far, DVec2::new(0.0, 1.0)),
            ],
        ),
    ];
    assert_eq!(rows.len(), expected.len(), "one row per end of the second");
    for (rank, (row, (error, leans))) in rows.iter().zip(expected).enumerate() {
        let mut wanted = vec![0.0; row.gradient.len()];
        for (point, slope) in leans {
            wanted[point.0 * 2] = slope.x;
            wanted[point.0 * 2 + 1] = slope.y;
        }
        assert!(!row.angular, "row {rank} is a length");
        assert!(
            (row.error - error).abs() < 1e-12,
            "row {rank} is {} off, not {error}",
            row.error,
        );
        for (variable, (is, was)) in row.gradient.iter().zip(&wanted).enumerate() {
            assert!(
                (is - was).abs() < 1e-12,
                "row {rank}, variable {variable}: {is}, not {was}",
            );
        }
    }
}
