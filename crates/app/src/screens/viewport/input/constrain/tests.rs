//! Closes #463.
//! - « Parallèle » and « Perpendiculaire » take a sketch axis as one of their
//!   two picks — `parallel_and_perpendicular_can_be_pointed_at_an_axis`; in
//!   both orders, which is `rule_intent`'s to read — no test here:
//!   `cao_sketch`'s `a_trait_square_to_an_axis.rs` holds it

use cao_sketch::{SketchAxis, WorkPlane};

use super::*;

const SNAP: f64 = 2.0;

#[test]
fn parallel_and_perpendicular_can_be_pointed_at_an_axis() {
    let sketch = Sketch::new(WorkPlane::XY);
    for rule in [Rule::Parallel, Rule::Perpendicular, Rule::Collinear] {
        assert_eq!(
            nearest_rule_pick(&sketch, DVec2::new(40.0, 0.5), SNAP, rule),
            Some(RulePick::Axis(SketchAxis::U)),
            "{rule:?} on the horizontal"
        );
        assert_eq!(
            nearest_rule_pick(&sketch, DVec2::new(-0.5, 40.0), SNAP, rule),
            Some(RulePick::Axis(SketchAxis::V)),
            "{rule:?} on the vertical"
        );
    }
    assert_eq!(
        nearest_rule_pick(&sketch, DVec2::new(40.0, 0.5), SNAP, Rule::Equal),
        None,
        "an axis has no length to be equal to"
    );
}
