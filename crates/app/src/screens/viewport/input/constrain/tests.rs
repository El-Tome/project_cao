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

#[test]
fn a_coincidence_points_at_a_cut_s_point_and_another_rule_at_the_side_under_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let right = sketch.add_point(DVec2::new(60.0, 10.0));
    let up = sketch.add_point(DVec2::new(10.0, 50.0));
    let side = sketch.add_segment(corner, right);
    let other = sketch.add_segment(corner, up);
    let cut = sketch
        .cut_corner(
            side,
            other,
            cao_sketch::CutShape::Fillet { radius: 5.0 },
            false,
            0,
        )
        .expect("the two traits meet");

    let tangent_point = DVec2::new(15.0, 10.0);
    assert_eq!(
        nearest_rule_pick(&sketch, tangent_point, SNAP, Rule::Coincident),
        Some(RulePick::CutPoint(cut, cao_sketch::CutPoint::First)),
    );
    assert_eq!(
        nearest_rule_pick(&sketch, tangent_point, SNAP, Rule::Parallel),
        Some(RulePick::Element(Element::Segment(side))),
    );
}

#[test]
fn the_tangent_tool_points_at_a_fillet_s_arc_rather_than_the_side_under_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let right = sketch.add_point(DVec2::new(60.0, 10.0));
    let up = sketch.add_point(DVec2::new(10.0, 50.0));
    let side = sketch.add_segment(corner, right);
    let other = sketch.add_segment(corner, up);
    let cut = sketch
        .cut_corner(
            side,
            other,
            cao_sketch::CutShape::Fillet { radius: 5.0 },
            false,
            0,
        )
        .expect("the two traits meet");
    let on_the_arc = sketch.cut_middle(cut).expect("a fillet draws an arc");
    assert!(
        sketch.nearest_segment(on_the_arc, SNAP).is_some(),
        "the arc's middle is within reach of a side"
    );

    assert_eq!(
        nearest_rule_pick(&sketch, on_the_arc, SNAP, Rule::Tangent),
        Some(RulePick::CutCurve(cut)),
    );
    assert_eq!(
        nearest_rule_pick(&sketch, DVec2::new(30.0, 10.5), SNAP, Rule::Tangent),
        Some(RulePick::Element(Element::Segment(side))),
        "away from the arc, the side is what is under the hand"
    );
}
