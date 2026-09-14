//! How a piece of the drawing is coloured once the user has something to do
//! with it: held by the selection, under the cursor, or already shown to the
//! rule being laid down.

use cao_prefs::theme::Theme;
use cao_sketch::{RulePick, Selection, Sketch};

use super::super::SketchContext;
use super::tint_at;

/// The colour and width a piece of the drawing ends up with.
pub(super) fn mark(
    context: &SketchContext<'_>,
    theme: &Theme,
    what: Selection,
    color: [f32; 4],
    width: f32,
) -> ([f32; 4], f32) {
    let picked = match what {
        Selection::Element(element) => context
            .editor
            .rule_picks()
            .contains(&RulePick::Element(element)),
        _ => false,
    };
    let held = context.editor.is_selected(what) || context.editor.hovered == Some(what);
    drawn_as(theme, picked, held, color, width)
}

/// What a rule has already been shown comes first, and in a colour of its own:
/// the cursor sitting on it would take nothing, since it is already taken, and
/// borrowing the hover colour there would say the opposite.
fn drawn_as(
    theme: &Theme,
    picked_by_a_rule: bool,
    held_or_hovered: bool,
    color: [f32; 4],
    width: f32,
) -> ([f32; 4], f32) {
    if picked_by_a_rule {
        return (tint_at(theme.picked, 1.0), width * 1.8);
    }
    if held_or_hovered {
        return (tint_at(theme.highlight, 1.0), width * 1.8);
    }
    (color, width)
}

/// A sketch axis shown to a rule, drawn past the whole drawing so that it reads
/// as an axis rather than as one more trait.
pub(super) fn push_picked_axes(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    picks: &[RulePick],
) {
    let reach = sketch
        .bounds()
        .map(|(min, max)| (max - min).length())
        .unwrap_or(1.0)
        .max(1.0);
    let color = tint_at(theme.picked, 1.0);

    for pick in picks {
        let RulePick::Axis(axis) = pick else {
            continue;
        };
        for end in [-reach, reach] {
            out.push(cao_render::Vertex::line(
                sketch.plane.to_world(axis.direction() * end).as_vec3(),
                color,
                2.5,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cao_sketch::{Element, SegmentId, SketchAxis, WorkPlane};
    use glam::{DVec2, Vec3};

    const TOLERANCE: f64 = 1e-9;
    const PLAIN: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

    #[test]
    fn what_a_rule_has_been_shown_is_not_drawn_in_the_colour_the_cursor_paints_with() {
        let theme = Theme::default();

        let (picked, _) = drawn_as(&theme, true, true, PLAIN, 1.0);
        let (hovered, _) = drawn_as(&theme, false, true, PLAIN, 1.0);

        assert_eq!(picked, tint_at(theme.picked, 1.0));
        assert_ne!(
            picked, hovered,
            "an element the rule already holds reads as one the next click would take",
        );
    }

    #[test]
    fn a_piece_of_the_drawing_no_one_is_dealing_with_keeps_the_colour_it_came_with() {
        let (color, width) = drawn_as(&Theme::default(), false, false, PLAIN, 1.5);

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
            places.iter().any(|place| place.y <= -30.0)
                && places.iter().any(|place| place.y >= 30.0),
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
}
