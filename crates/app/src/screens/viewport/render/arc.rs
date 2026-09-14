//! What the arc tool shows before its curve is drawn: a radius or a distance
//! typed while the second place is picked, and while the third is, the angle a
//! `ByCenter` arc sweeps or the radius a `ByEnds` one is bent to.

use cao_sketch::{ArcMode, Sketch, arc_angle_reference, sweep_of};
use glam::DVec2;

use super::curves::push_arc_at;
use super::{push_point_marker, push_preview_line};
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
        // An arc given only its centre is not a curve yet, so what follows the
        // cursor is the reach it is about to be drawn at: clicking into an empty
        // canvas should never be clicking into the dark.
        None => {
            if let Some(centre) = context.editor.arc_places().first().copied() {
                push_preview_line(out, sketch, centre, cursor, preview, false, scale);
                push_point_marker(out, sketch, centre, marker, preview, 1.5);
            }
        }
    }
    if let Some((centre, towards)) =
        arc_angle_reference(context.editor.arc_mode, context.editor.arc_places())
    {
        push_preview_line(out, sketch, centre, towards, preview, true, scale);
    }
}
