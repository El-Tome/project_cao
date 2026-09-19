//! What the part holding a drawing's points is held to.
//!
//! Closes #359.
//! - a drawing its own rules already determine keeps its shape and every
//!   hold is let go — `a_drawing_that_stands_on_its_own_lets_go`
//! - a hold nothing contradicts is kept — `a_hold_that_costs_nothing_stands`
//! - a hold the rules cannot honour gives —
//!   `a_hold_the_rules_cannot_honour_gives`

use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
use crate::plane::WorkPlane;

use super::*;

/// A trait out from the drawing's own origin, with its far end held by the
/// part.
fn a_trait_held_at_its_far_end() -> (Sketch, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let far = sketch.add_point(DVec2::new(40.0, 0.0));
    sketch.add_segment(Sketch::ORIGIN, far);
    sketch.hold_on_the_part(far);
    (sketch, far)
}

#[test]
fn a_hold_that_costs_nothing_stands() {
    let (mut sketch, far) = a_trait_held_at_its_far_end();

    let (_, gave) = sketch.settle_with_the_part(1.0);

    assert!(!gave);
    assert!(
        sketch.is_anchored(far),
        "nothing asks it to be anywhere else"
    );
}

#[test]
fn a_drawing_that_stands_on_its_own_lets_go() {
    let (mut sketch, far) = a_trait_held_at_its_far_end();
    // Flat along the drawing's own axis and forty long: between them the far
    // end has nothing left to decide, and they put it exactly where the hold
    // already has it.
    sketch.add_constraint(Constraint::AxisCollinear {
        segment: crate::sketch::SegmentId(0),
        axis: SketchAxis::U,
    });
    sketch.set_dimension(
        DimensionTarget::Length(crate::sketch::SegmentId(0)),
        40.0,
        false,
    );
    assert!(
        sketch.freedom_alone(1.0).fully_constrained(),
        "the drawing has to stand up without the hold, or this proves \
         nothing: {:?}",
        sketch.freedom_alone(1.0),
    );

    let (_, gave) = sketch.settle_with_the_part(1.0);

    assert!(gave);
    assert!(
        !sketch.is_anchored(far),
        "a drawing its own rules settle is dragged about by a hold rather \
         than held by one",
    );
}

#[test]
fn a_hold_the_rules_cannot_honour_gives() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let far = sketch.add_point(DVec2::new(40.0, 20.0));
    sketch.add_segment(Sketch::ORIGIN, far);
    sketch.hold_on_the_part(far);
    // Held twenty off the axis, and asked to lie flat on it.
    sketch.add_constraint(Constraint::AxisCollinear {
        segment: crate::sketch::SegmentId(0),
        axis: SketchAxis::U,
    });

    let (_, gave) = sketch.settle_with_the_part(1.0);

    assert!(gave);
    assert!(!sketch.is_anchored(far));
    assert!(
        sketch.point(far).y.abs() < 1e-3,
        "the rule is what the drawing keeps: {:?}",
        sketch.point(far),
    );
}
