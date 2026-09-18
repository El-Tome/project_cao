//! What sketch · rule_intent.rs is held to.

use super::*;
use crate::plane::WorkPlane;

#[test]
fn the_order_of_the_clicks_does_not_change_the_rule() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(glam::DVec2::new(0.0, 0.0));
    let end = sketch.add_point(glam::DVec2::new(10.0, 0.0));
    let point = sketch.add_point(glam::DVec2::new(5.0, 5.0));
    let segment = sketch.add_segment(start, end);

    let forward = rule_intent(
        Rule::Coincident,
        &[
            RulePick::Element(Element::Point(point)),
            RulePick::Element(Element::Segment(segment)),
        ],
        &sketch,
    );
    let backward = rule_intent(
        Rule::Coincident,
        &[
            RulePick::Element(Element::Segment(segment)),
            RulePick::Element(Element::Point(point)),
        ],
        &sketch,
    );

    assert_eq!(
        forward, backward,
        "a point and a trait coincide either way round"
    );
    assert_eq!(
        forward,
        Some(RuleIntent::Constrain(Constraint::OnSegment {
            point,
            segment
        })),
    );
}

#[test]
fn two_points_asked_to_coincide_become_one() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let first = sketch.add_point(glam::DVec2::new(0.0, 0.0));
    let second = sketch.add_point(glam::DVec2::new(10.0, 10.0));

    let intent = rule_intent(
        Rule::Coincident,
        &[
            RulePick::Element(Element::Point(first)),
            RulePick::Element(Element::Point(second)),
        ],
        &sketch,
    );

    assert_eq!(
        intent,
        Some(RuleIntent::Merge {
            kept: first,
            dropped: second
        }),
    );
}

#[test]
fn the_origin_is_never_the_point_that_gives_way() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let held = sketch.add_point(glam::DVec2::new(10.0, 10.0));

    let intent = rule_intent(
        Rule::Coincident,
        &[
            RulePick::Element(Element::Point(held)),
            RulePick::Element(Element::Point(Sketch::ORIGIN)),
        ],
        &sketch,
    );

    assert_eq!(
        intent,
        Some(RuleIntent::Merge {
            kept: Sketch::ORIGIN,
            dropped: held
        }),
    );
}

#[test]
fn two_circles_asked_to_be_concentric_merge_their_centres() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let first_center = sketch.add_point(glam::DVec2::new(0.0, 0.0));
    let second_center = sketch.add_point(glam::DVec2::new(20.0, 0.0));
    let first = sketch.add_circle(first_center, 5.0);
    let second = sketch.add_circle(second_center, 8.0);

    let intent = rule_intent(
        Rule::Concentric,
        &[
            RulePick::Element(Element::Circle(first)),
            RulePick::Element(Element::Circle(second)),
        ],
        &sketch,
    );

    assert_eq!(
        intent,
        Some(RuleIntent::Merge {
            kept: first_center,
            dropped: second_center
        }),
    );
}

#[test]
fn two_arcs_shown_equal_ask_for_the_same_radius() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (centre_one, start_one, end_one) = (
        sketch.add_point(glam::DVec2::ZERO),
        sketch.add_point(glam::DVec2::new(10.0, 0.0)),
        sketch.add_point(glam::DVec2::new(0.0, 10.0)),
    );
    let one = sketch.add_arc(centre_one, start_one, end_one);
    let (centre_two, start_two, end_two) = (
        sketch.add_point(glam::DVec2::new(50.0, 0.0)),
        sketch.add_point(glam::DVec2::new(60.0, 0.0)),
        sketch.add_point(glam::DVec2::new(50.0, 10.0)),
    );
    let two = sketch.add_arc(centre_two, start_two, end_two);

    let intent = rule_intent(
        Rule::Equal,
        &[
            RulePick::Element(Element::Arc(one)),
            RulePick::Element(Element::Arc(two)),
        ],
        &sketch,
    );

    assert_eq!(
        intent,
        Some(RuleIntent::Constrain(Constraint::EqualRadiusArc {
            first: one,
            second: two,
        })),
    );
}

#[test]
fn an_arc_and_a_trait_shown_tangent_brush_each_other() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(glam::DVec2::new(-40.0, 20.0));
    let end = sketch.add_point(glam::DVec2::new(40.0, 20.0));
    let segment = sketch.add_segment(start, end);
    let (centre, arc_start, arc_end) = (
        sketch.add_point(glam::DVec2::ZERO),
        sketch.add_point(glam::DVec2::new(8.0, 0.0)),
        sketch.add_point(glam::DVec2::new(0.0, 8.0)),
    );
    let arc = sketch.add_arc(centre, arc_start, arc_end);

    let intent = rule_intent(
        Rule::Tangent,
        &[
            RulePick::Element(Element::Arc(arc)),
            RulePick::Element(Element::Segment(segment)),
        ],
        &sketch,
    );

    assert_eq!(
        intent,
        Some(RuleIntent::Constrain(Constraint::ArcTangent {
            arc,
            segment,
            at: None,
        })),
    );
}

#[test]
fn a_rule_shown_the_wrong_kinds_yields_nothing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let point = sketch.add_point(glam::DVec2::new(0.0, 0.0));

    let intent = rule_intent(
        Rule::Perpendicular,
        &[RulePick::Element(Element::Point(point))],
        &sketch,
    );

    assert_eq!(intent, None, "a right angle needs two traits, not a point");
}
