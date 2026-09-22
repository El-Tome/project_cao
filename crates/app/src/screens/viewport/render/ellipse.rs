//! What the ellipse tool shows before its curve is drawn: the width and the
//! angle of the first axis while its end is picked, and the width of the
//! second while it is.

use cao_sketch::Sketch;
use glam::DVec2;

use super::curves::push_ellipse_at;
use super::marks::push_point_marker;
use super::preview::push_preview_line;
use crate::screens::viewport::input::{ellipse_aimed_at, ellipse_preview};
use crate::screens::viewport::{SketchContext, ViewScale};

pub(crate) fn live_fields(
    context: &SketchContext<'_>,
    cursor: DVec2,
) -> Option<([&'static str; 2], [f64; 2])> {
    let scale = context.document.scale();
    match context.editor.ellipse_places() {
        [centre] => {
            let span = ellipse_aimed_at(context, cursor) - *centre;
            Some((
                ["mm", "°"],
                [span.length() * 2.0 * scale, span.to_angle().to_degrees()],
            ))
        }
        // Shown whether or not the places make an ellipse just now: a field
        // that disappears while being typed into cannot be typed into.
        [_, _] => {
            let across = ellipse_preview(context, cursor).map_or(0.0, |drawn| drawn.second);
            Some((["mm", ""], [across * 2.0 * scale, 0.0]))
        }
        _ => None,
    }
}

/// The curve a click right now would draw with its two axes, or — while only
/// the centre is given — the first axis across the whole curve, which is what
/// the next click fixes.
pub(crate) fn push_preview(
    out: &mut Vec<cao_render::Vertex>,
    context: &SketchContext<'_>,
    sketch: &Sketch,
    cursor: DVec2,
    preview: [f32; 4],
    scale: ViewScale,
) {
    let marker = scale.world_size_of(3.0);
    let axes = match (
        ellipse_preview(context, cursor),
        context.editor.ellipse_places(),
    ) {
        (Some(drawn), _) => {
            push_ellipse_at(
                out,
                sketch,
                drawn.places(),
                preview,
                1.5,
                context.editor.construction,
                scale,
            );
            vec![drawn.first, drawn.second_axis()]
        }
        (None, [centre, ..]) => vec![ellipse_aimed_at(context, cursor) - *centre],
        (None, []) => return,
    };
    let Some(centre) = context.editor.ellipse_places().first().copied() else {
        return;
    };
    for reach in axes {
        push_preview_line(
            out,
            sketch,
            centre - reach,
            centre + reach,
            preview,
            true,
            scale,
        );
        push_point_marker(out, sketch, centre + reach, marker, preview, 1.5);
    }
    push_point_marker(out, sketch, centre, marker, preview, 1.5);
}

#[cfg(test)]
mod tests;
