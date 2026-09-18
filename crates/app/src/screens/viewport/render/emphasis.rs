//! How a piece of the drawing is coloured once the user has something to do
//! with it: held by the selection, under the cursor, or already shown to the
//! rule being laid down.

use cao_prefs::theme::Theme;
use cao_sketch::{Element, RulePick, Selection, Sketch};

use super::super::SketchContext;
use super::tint_at;

/// How faint what a click has not laid yet is drawn: there, and plainly not
/// there yet.
const A_PROMISE: f32 = 0.55;

/// The colour of what a tool is only offering to lay.
pub(super) fn ghost(theme: &Theme) -> [f32; 4] {
    tint_at(theme.sketch_free, A_PROMISE)
}

/// The colour and width a piece of the drawing ends up with.
pub(super) fn mark(
    context: &SketchContext<'_>,
    theme: &Theme,
    what: Selection,
    laid: &[Element],
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
    let only_shown = matches!(what, Selection::Element(element) if laid.contains(&element));
    drawn_as(theme, picked, held, only_shown, color, width)
}

/// What a rule has already been shown comes first, and in a colour of its own:
/// the cursor sitting on it would take nothing, since it is already taken, and
/// borrowing the hover colour there would say the opposite.
fn drawn_as(
    theme: &Theme,
    picked_by_a_rule: bool,
    held_or_hovered: bool,
    only_shown: bool,
    color: [f32; 4],
    width: f32,
) -> ([f32; 4], f32) {
    if only_shown {
        return (ghost(theme), width);
    }
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
mod tests;
