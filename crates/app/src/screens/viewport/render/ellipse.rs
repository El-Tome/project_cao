//! What the ellipse tool shows before its curve is drawn: the width and the
//! angle of the first axis while its end is picked, and the width of the
//! second while it is.

use cao_sketch::{EllipseDraft, EllipseMode, Sketch, rise_of};
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
            let (axes, marks) = scaffolding(drawn, mode, aimed);
            for (from, to) in axes {
                push_preview_line(out, sketch, from, to, preview, true, scale);
            }
            for place in marks {
                push_point_marker(out, sketch, place, marker, preview, 1.5);
            }
        }
        // No curve yet — one place given, or a rise of nothing at the second.
        // Either way the first axis is what the clicks so far settle, and it
        // is what is shown: a line to the cursor from a place that is not on
        // that axis would be neither the axis nor anything else.
        (None, places) => {
            let Some((from, to)) = first_axis(mode, places, aimed) else {
                return;
            };
            push_preview_line(out, sketch, from, to, preview, true, scale);
            for place in [from, to] {
                push_point_marker(out, sketch, place, marker, preview, 1.5);
            }
        }
    }
}

/// The first axis the clicks so far settle, as the two places it runs between:
/// from the centre it reaches the same way either side, from the two ends it
/// runs between them. The cursor stands in for whichever end is not clicked yet.
fn first_axis(mode: EllipseMode, places: &[DVec2], aimed: DVec2) -> Option<(DVec2, DVec2)> {
    let (held, far) = match places {
        [held] => (*held, aimed),
        [held, far] => (*held, *far),
        _ => return None,
    };
    match mode {
        EllipseMode::ByCentre => Some((held - (far - held), far)),
        EllipseMode::ByEnds => Some((held, far)),
    }
}

/// The axes the preview shows, each as the two places it runs between, and the
/// places it marks.
///
/// Placed from its two ends, the second axis is shown **only on the side the
/// curve is drawn**, which is where the trait itself is laid: out from the
/// centre to the rise rather than across. A preview reaching past it would
/// read as the scaffolding of a whole ellipse rather than of the half.
fn scaffolding(
    drawn: EllipseDraft,
    mode: EllipseMode,
    rise: DVec2,
) -> (Vec<(DVec2, DVec2)>, Vec<DVec2>) {
    let (centre, along, across) = (drawn.centre, drawn.first, drawn.second_axis());
    let whole = (centre - along, centre + along);
    match mode {
        EllipseMode::ByCentre => (
            vec![whole, (centre - across, centre + across)],
            vec![centre + along, centre + across, centre],
        ),
        EllipseMode::ByEnds => {
            let bulge = rise_of(drawn, rise).reach(drawn);
            (
                vec![whole, (centre, centre + bulge)],
                vec![centre - along, centre + along, centre + bulge, centre],
            )
        }
    }
}

/// The places the preview draws through: the whole curve, or the half of it the
/// rise fell on.
fn run_of(drawn: EllipseDraft, mode: EllipseMode, rise: DVec2) -> Vec<DVec2> {
    match mode {
        EllipseMode::ByCentre => drawn.places(),
        EllipseMode::ByEnds => {
            let half = std::f64::consts::PI;
            let opens = match rise_of(drawn, rise).between() {
                [1, _] => 0.0,
                _ => half,
            };
            places_of(drawn, opens, half)
        }
    }
}

#[cfg(test)]
mod tests;
