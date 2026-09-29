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
