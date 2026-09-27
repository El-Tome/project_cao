//! What sketch · tool.rs is held to.

use super::*;
use crate::sketch::Element;

#[test]
fn escape_undoes_the_shape_in_progress_before_it_gives_the_tool_back() {
    let mid_rectangle = ToolState::Rectangle {
        start: DVec2::new(1.0, 2.0),
    };
    assert!(
        mid_rectangle.is_busy(),
        "a rectangle with one corner placed is a shape to lose, not a tool to give up"
    );

    assert!(
        !ToolState::None.is_busy(),
        "nothing recorded leaves the tool itself as the only thing escape can give back"
    );
}

#[test]
fn a_selection_held_by_the_select_tool_is_also_something_escape_would_undo_first() {
    let mut holding = SelectState::default();
    holding
        .held
        .push(Selection::Element(Element::Point(PointId(0))));

    assert!(ToolState::Select(Box::new(holding)).is_busy());
    assert!(!ToolState::Select(Box::default()).is_busy());
}

#[test]
fn a_circle_or_a_rule_half_picked_is_busy_even_though_the_old_flat_check_missed_it() {
    // The flat `SketchEditor` this replaces checked `chain`, `pending_start`,
    // `placing`, `selected`, `selection`, and the three dimension picks for
    // whether Escape should hand the tool back — but never `circle_points`,
    // `circle_segments`, `rule_picks` or `band`. A circle two clicks into a
    // three-point construction, or a rule with one trait already picked,
    // was silently kicked back to Select. A variant that only exists once
    // something is recorded cannot make that mistake.
    let half_circle = ToolState::Circle {
        points: vec![DVec2::new(1.0, 1.0)],
        segments: Vec::new(),
    };
    let half_rule = ToolState::Constrain {
        picks: vec![RulePick::Element(Element::Point(PointId(0)))],
    };

    assert!(half_circle.is_busy());
    assert!(half_rule.is_busy());
}

#[test]
fn a_rule_half_laid_down_hands_back_what_it_has_been_pointed_at() {
    let trait_picked = RulePick::Element(Element::Segment(SegmentId(2)));
    let half_rule = ToolState::Constrain {
        picks: vec![trait_picked],
    };

    assert_eq!(half_rule.rule_picks(), [trait_picked]);
    assert!(
        ToolState::None.rule_picks().is_empty(),
        "a tool laying no rule down has been shown nothing to draw differently"
    );
}

#[test]
fn a_press_that_took_hold_of_nothing_is_what_pulls_a_box() {
    let mut pressed = SelectState::default();
    assert!(pressed.grabbed_nothing());

    pressed.dragged_point = Some(PointId(3));
    assert!(!pressed.grabbed_nothing(), "a point was taken");

    let group = SelectState {
        dragged_group: vec![PointId(1), PointId(2)],
        ..SelectState::default()
    };
    assert!(!group.grabbed_nothing(), "a selection was taken");

    let side = SelectState {
        dragged_side: Some(SegmentId(0)),
        ..SelectState::default()
    };
    assert!(!side.grabbed_nothing(), "a side was taken");
}
