//! What the symmetric line tool shows before its segment is drawn: the
//! preview growing from its middle, and the length and angle typed into it.

use cao_sketch::{Sketch, ToolState};
use glam::DVec2;

use crate::screens::SketchContext;
use crate::screens::viewport::ViewScale;
use crate::screens::viewport::values::shape_scale;

pub(crate) fn push_preview(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    context: &SketchContext<'_>,
    cursor: DVec2,
    color: [f32; 4],
    scale: ViewScale,
) {
    let ToolState::SymmetricLine { middle } = context.editor.tool_state else {
        return;
    };
    let Some(middle) = sketch.anchor_position(middle) else {
        return;
    };
    let locked = context.editor.live.locked();
    let (start, end) = sketch.symmetric_ends(middle, cursor, locked, shape_scale(context));
    super::preview::push_preview_line(
        out,
        sketch,
        start,
        end,
        color,
        context.editor.construction,
        scale,
    );
    super::marks::push_point_marker(out, sketch, middle, scale.world_size_of(3.0), color, 1.5);
}

pub(crate) fn live_fields(
    context: &SketchContext<'_>,
    sketch: &Sketch,
    raw_cursor: DVec2,
) -> Option<([&'static str; 2], [f64; 2])> {
    let ToolState::SymmetricLine { middle } = context.editor.tool_state else {
        return None;
    };
    let middle = sketch.anchor_position(middle)?;
    let locked = context.editor.live.locked();
    let scale = shape_scale(context);
    let (_, end) = sketch.symmetric_ends(middle, raw_cursor, locked, scale);
    let span = end - middle;
    Some((
        ["mm", "°"],
        [span.length() * scale, span.y.atan2(span.x).to_degrees()],
    ))
}
