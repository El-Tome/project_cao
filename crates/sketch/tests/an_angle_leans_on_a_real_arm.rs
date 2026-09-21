//! What a typed angle leaves in the drawing: a horizontal arm to read it
//! against, and the reading between the two.
//!
//! Closes #314.
//! - the arm is held parallel to the horizontal axis, wherever it is —
//!   `an_arm_held_parallel_to_the_horizontal_axis_stays_horizontal`

use cao_sketch::{Constraint, Sketch, SketchAxis, WorkPlane};
use glam::DVec2;

const TOLERANCE: f64 = 1e-6;

#[test]
fn an_arm_held_parallel_to_the_horizontal_axis_stays_horizontal() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(3.0, 4.0));
    let end = sketch.add_point(DVec2::new(9.0, 6.5));
    let arm = sketch.add_construction_segment(start, end);
    sketch.add_constraint(Constraint::AxisParallel {
        segment: arm,
        axis: SketchAxis::U,
    });

    sketch.solve(1.0);

    let (from, to) = sketch.endpoints(arm);
    assert!(
        (from.y - to.y).abs() < TOLERANCE,
        "the arm runs from {from} to {to}, which is not horizontal",
    );
    assert!(
        (from - DVec2::new(3.0, 4.0)).length() > TOLERANCE
            || (to - DVec2::new(9.0, 6.5)).length() > TOLERANCE,
        "the rule moved nothing at all, so it is holding nothing",
    );
}
