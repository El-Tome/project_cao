//! What the arc tool shows before its curve is drawn: a radius or a distance
//! typed while the second place is picked, and while the third is, the angle a
//! `ByCenter` arc sweeps or the radius a `ByEnds` one is bent to.

use cao_sketch::{ArcMode, Sketch, arc_angle_reference, sweep_of};
use glam::DVec2;

use super::curves::push_arc_at;
use super::marks::push_point_marker;
use super::preview::push_preview_line;
use crate::screens::viewport::input::{arc_aimed, arc_preview};
use crate::screens::viewport::{SketchContext, ViewScale};

pub(crate) fn live_fields(
    context: &SketchContext<'_>,
    cursor: DVec2,
) -> Option<([&'static str; 2], [f64; 2])> {
    let places = context.editor.arc_places();
    match (context.editor.arc_mode, places.len()) {
        (ArcMode::ByCenter, 1) | (ArcMode::ByEnds, 1) => {
            let aimed = arc_aimed(context, places, cursor);
            let distance = places[0].distance(aimed) * context.document.scale();
            Some((["mm", ""], [distance, 0.0]))
        }
        (ArcMode::ByCenter, 2) => {
            let drawn = arc_preview(context, cursor)?;
            Some((["°", ""], [sweep_of(drawn).to_degrees(), 0.0]))
        }
        (ArcMode::ByEnds, 2) => {
            let drawn = arc_preview(context, cursor)?;
            let radius = drawn.centre.distance(drawn.start) * context.document.scale();
            Some((["mm", ""], [radius, 0.0]))
        }
        _ => None,
    }
}

/// The curve a click right now would draw, the places it stands on, and — while
/// a by-centre arc is being swept — the leg its angle opens from.
pub(crate) fn push_preview(
    out: &mut Vec<cao_render::Vertex>,
    context: &SketchContext<'_>,
    sketch: &Sketch,
    cursor: DVec2,
    preview: [f32; 4],
    scale: ViewScale,
) {
    let marker = scale.world_size_of(3.0);
    match arc_preview(context, cursor) {
        Some(drawn) => {
            push_arc_at(
                out,
                sketch,
                drawn,
                preview,
                1.5,
                context.editor.construction,
                scale,
            );
            for place in [drawn.centre, drawn.start, drawn.end] {
                push_point_marker(out, sketch, place, marker, preview, 1.5);
            }
        }
        // A single place given so far is not a curve yet, whichever mode: a
        // centre alone has no radius, and one end alone has no distance to
        // the other. What follows the cursor — aimed, so a typed value bends
        // it exactly as far as the field says rather than wherever the mouse
        // happens to sit — is the reach the next click is about to fix.
        // Past two places, `None` means the places chosen do not bend into an
        // arc at all, which the blocking message already says; nothing here
        // should look like one.
        None => {
            if let [only] = context.editor.arc_places() {
                let reach = arc_aimed(context, context.editor.arc_places(), cursor);
                push_preview_line(out, sketch, *only, reach, preview, false, scale);
                push_point_marker(out, sketch, *only, marker, preview, 1.5);
            }
        }
    }
    if let Some((centre, towards)) =
        arc_angle_reference(context.editor.arc_mode, context.editor.arc_places())
    {
        push_preview_line(out, sketch, centre, towards, preview, true, scale);
    }
}

#[cfg(test)]
mod tests;
