//! What the ellipse tool shows before its curve is drawn: the width and the
//! angle of the first axis while its end is picked, and the width of the
//! second while it is.

use cao_sketch::{EllipseDraft, EllipseMode, Sketch, half_between};
use glam::DVec2;

use super::curves::{places_of, push_ellipse_at};
use super::marks::push_point_marker;
use super::preview::push_preview_line;
use crate::screens::viewport::input::{ellipse_aimed_at, ellipse_preview};
use crate::screens::viewport::{SketchContext, ViewScale};

pub(crate) fn live_fields(
    context: &SketchContext<'_>,
    cursor: DVec2,
) -> Option<([&'static str; 2], [f64; 2])> {
    let scale = context.document.scale();
    // Placed from its two ends, both figures are read straight off what is
    // being drawn: the gap between the two clicks, then the rise. From the
    // centre, each click gives half of what the axis measures across.
    let whole = match context.editor.ellipse_mode {
        EllipseMode::ByCentre => 2.0,
        EllipseMode::ByEnds => 1.0,
    };
    match context.editor.ellipse_places() {
        [first] => {
            let span = ellipse_aimed_at(context, cursor) - *first;
            Some((
                ["mm", "°"],
                [span.length() * whole * scale, span.to_angle().to_degrees()],
            ))
        }
        // Shown whether or not the places make an ellipse just now: a field
        // that disappears while being typed into cannot be typed into.
        [_, _] => {
            let across = ellipse_preview(context, cursor).map_or(0.0, |drawn| drawn.second);
            Some((["mm", ""], [across * whole * scale, 0.0]))
        }
        _ => None,
    }
}

/// The curve a click right now would draw with its two axes, or — while only
/// the first place is given — the axis the next click fixes.
pub(crate) fn push_preview(
    out: &mut Vec<cao_render::Vertex>,
    context: &SketchContext<'_>,
    sketch: &Sketch,
    cursor: DVec2,
    preview: [f32; 4],
    scale: ViewScale,
) {
    let marker = scale.world_size_of(3.0);
    let mode = context.editor.ellipse_mode;
    let aimed = ellipse_aimed_at(context, cursor);
    match (
        ellipse_preview(context, cursor),
        context.editor.ellipse_places(),
    ) {
        (Some(drawn), _) => {
            push_ellipse_at(
                out,
                sketch,
                run_of(drawn, mode, aimed),
                preview,
                1.5,
                context.editor.construction,
                scale,
            );
            for reach in [drawn.first, drawn.second_axis()] {
                push_preview_line(
                    out,
                    sketch,
                    drawn.centre - reach,
                    drawn.centre + reach,
                    preview,
                    true,
                    scale,
                );
                push_point_marker(out, sketch, drawn.centre + reach, marker, preview, 1.5);
            }
            push_point_marker(out, sketch, drawn.centre, marker, preview, 1.5);
        }
        // One place given: from the centre, the whole first axis is shown, the
        // click being one of its ends. From the two ends, what is shown is the
        // axis itself, running from the end already clicked to the cursor.
        (None, [held, ..]) => {
            let (from, to) = match mode {
                EllipseMode::ByCentre => (*held - (aimed - *held), aimed),
                EllipseMode::ByEnds => (*held, aimed),
            };
            push_preview_line(out, sketch, from, to, preview, true, scale);
            push_point_marker(out, sketch, to, marker, preview, 1.5);
            push_point_marker(out, sketch, *held, marker, preview, 1.5);
        }
        (None, []) => (),
    }
}

/// The places the preview draws through: the whole curve, or the half of it the
/// rise fell on.
fn run_of(drawn: EllipseDraft, mode: EllipseMode, rise: DVec2) -> Vec<DVec2> {
    match mode {
        EllipseMode::ByCentre => drawn.places(),
        EllipseMode::ByEnds => {
            let half = std::f64::consts::PI;
            let opens = match half_between(drawn, rise) {
                [1, _] => 0.0,
                _ => half,
            };
            places_of(drawn, opens, half)
        }
    }
}

#[cfg(test)]
mod tests;
