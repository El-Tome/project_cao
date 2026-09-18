//! What app · screens/viewport/render/emphasis.rs is held to.

use super::*;
use cao_sketch::{Element, SegmentId, SketchAxis, WorkPlane};

#[test]
fn a_piece_the_click_has_not_laid_yet_is_drawn_faint_rather_than_as_itself() {
    let theme = Theme::default();

    let (shown, width) = drawn_as(&theme, false, false, true, PLAIN, 1.5);

    assert_eq!(
        shown,
        ghost(&theme),
        "what the click would lay reads as a promise, not as drawing already there"
    );
    assert_eq!(width, 1.5, "a promise is faint, not thick");
}
use glam::{DVec2, Vec3};

const TOLERANCE: f64 = 1e-9;
const PLAIN: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

#[test]
fn what_a_rule_has_been_shown_is_not_drawn_in_the_colour_the_cursor_paints_with() {
    let theme = Theme::default();

    let (picked, _) = drawn_as(&theme, true, true, false, PLAIN, 1.0);
    let (hovered, _) = drawn_as(&theme, false, true, false, PLAIN, 1.0);

    assert_eq!(picked, tint_at(theme.picked, 1.0));
    assert_ne!(
        picked, hovered,
        "an element the rule already holds reads as one the next click would take",
    );
}

#[test]
fn a_piece_of_the_drawing_no_one_is_dealing_with_keeps_the_colour_it_came_with() {
    let (color, width) = drawn_as(&Theme::default(), false, false, false, PLAIN, 1.5);

    assert_eq!((color, width), (PLAIN, 1.5));
}

#[test]
fn an_axis_shown_to_a_rule_is_drawn_out_past_both_ends_of_the_drawing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(-30.0, 5.0));
    let end = sketch.add_point(DVec2::new(30.0, 5.0));
    sketch.add_segment(start, end);

    let mut out = Vec::new();
    push_picked_axes(
        &mut out,
        &sketch,
        &Theme::default(),
        &[RulePick::Axis(SketchAxis::V)],
    );

    let places: Vec<DVec2> = out
        .iter()
        .map(|vertex| {
            sketch
                .plane
                .to_local(Vec3::from(vertex.position).as_dvec3())
        })
        .collect();
    assert_eq!(places.len(), 2, "an axis is one straight step, so two ends");
    assert!(
        places.iter().all(|place| place.x.abs() < TOLERANCE),
        "the vertical axis was drawn somewhere other than x = 0: {places:?}",
    );
    assert!(
        places.iter().any(|place| place.y <= -30.0) && places.iter().any(|place| place.y >= 30.0),
        "the axis stops short of the drawing it is meant to run past: {places:?}",
    );
}

#[test]
fn a_rule_shown_no_axis_draws_none() {
    let sketch = Sketch::new(WorkPlane::XY);
    let theme = Theme::default();

    let mut nothing = Vec::new();
    push_picked_axes(&mut nothing, &sketch, &theme, &[]);
    let mut a_trait = Vec::new();
    push_picked_axes(
        &mut a_trait,
        &sketch,
        &theme,
        &[RulePick::Element(Element::Segment(SegmentId(0)))],
    );

    assert!(nothing.is_empty());
    assert!(
        a_trait.is_empty(),
        "a trait shown to the rule brought an axis along with it",
    );
}
