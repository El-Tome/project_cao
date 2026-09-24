//! What a click right now would lay down, drawn as a promise: the line a chain
//! is about to add, the marks its ends would snap to, the right angle it would
//! square up against, and the annotation the dimension tool is offering.

use cao_prefs::theme::Theme;
use cao_sketch::{ChainAnchor, DimensionTarget, Sketch, Snap};
use glam::DVec2;

use super::curves::{push_circle_at, push_line};
use super::marks::{push_midpoint_mark, push_point_marker, push_square_mark};
use super::{arc, ellipse, emphasis, symmetric_line, tint_at};
use crate::screens::SketchContext;
use crate::screens::sketch::{DimensionMode, Tool};
use crate::screens::viewport::input::{
    annotation_position, circle_from, measure_preview, rectangle_corner, refine,
};
use crate::screens::viewport::{PICK_PIXELS, ViewScale};

/// The annotation the dimension tool is showing in advance: the one a click
/// would choose, or the one already chosen and looking for its place.
pub(crate) fn pending_annotation(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> Option<(DimensionTarget, DVec2)> {
    if let Some(target) = context.editor.placing() {
        // A second entity under the cursor turns the dimension into another
        // one; it is shown where it would land, not dragged to the cursor.
        if context.editor.dimension_mode == DimensionMode::Auto
            && let Some(refined) = refine(context, index, target, cursor, snap)
        {
            return Some((refined, DVec2::ZERO));
        }
        let target = context
            .document
            .sketches()
            .get(index)
            .map_or(target, |sketch| sketch.oriented(target, cursor));
        // The preview is nudged from where the annotation stands today, not
        // moved to an absolute offset: `push` adds a nudge on top of whatever
        // the dimension already carries.
        let nudge = annotation_position(context, index, target, pixel)
            .map(|placement| cursor - placement.text_at)
            .unwrap_or_default();
        return Some((target, nudge));
    }
    let target = measure_preview(context, index, cursor, snap)?;
    Some((target, DVec2::ZERO))
}

/// The shape about to be drawn, following the cursor: placing a point blind and
/// only then seeing where it went is needlessly uncomfortable.
pub(crate) fn push_preview(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    scale: ViewScale,
    context: &SketchContext<'_>,
) {
    let Some(cursor) = context.editor.cursor else {
        return;
    };
    let preview = emphasis::ghost(theme);

    if let Some(anchor) = context.editor.chain() {
        let from = match anchor {
            ChainAnchor::Pending(position) => position,
            ChainAnchor::Point(id) if id.0 < sketch.points().len() => sketch.point(id),
            ChainAnchor::Point(_) => cursor,
        };
        let to = context
            .editor
            .aimed
            .map(|aimed| aimed.position)
            .unwrap_or(cursor);
        push_preview_line(
            out,
            sketch,
            from,
            to,
            preview,
            context.editor.construction,
            scale,
        );

        // The little square of a right angle, drawn before it is committed to
        // so the constraint is never a surprise. Its two arms are the line
        // being drawn and the one it is squaring up against.
        if let Some(previous) = context.editor.aimed.and_then(|aimed| aimed.square_with)
            && previous.0 < sketch.segments().len()
        {
            let (start, end) = sketch.endpoints(previous);
            let arm = if start.distance(from) < end.distance(from) {
                end - start
            } else {
                start - end
            };
            push_square_mark(out, sketch, from, to - from, -arm, scale, preview);
        }
    }

    symmetric_line::push_preview(out, sketch, context, cursor, preview, scale);

    // The point tool has nothing pending, yet placing a point blind is exactly
    // as uncomfortable as the rest.
    if context.editor.tool == Tool::Point {
        push_point_marker(out, sketch, cursor, scale.world_size_of(4.0), preview, 1.5);
    }

    // The dimension a click would place, drawn faintly where it would land —
    // then, once it is chosen, the same annotation following the cursor to the spot it will sit on.
    if context.editor.tool == Tool::Dimension
        && let Some(index) = context.editor.active_sketch()
        && let Some((target, nudge)) = pending_annotation(
            context,
            index,
            cursor,
            scale.world_size_of(PICK_PIXELS),
            scale.units_per_pixel,
        )
    {
        let mut style = crate::screens::annotations::Style::driving(theme);
        style.color = preview;
        style.width *= 0.9;
        crate::screens::annotations::push(
            out,
            sketch,
            target,
            &style,
            scale.units_per_pixel,
            nudge,
        );
    }

    // A crossing borrows the mark a point wears rather than carrying a glyph of
    // its own, which nobody has asked for.
    match context.editor.snap {
        Some(Snap::Midpoint(at)) => {
            push_midpoint_mark(out, sketch, at, scale, tint_at(theme.highlight, 1.0))
        }
        Some(Snap::OnCurve(at) | Snap::Crossing(at)) => push_point_marker(
            out,
            sketch,
            at,
            scale.world_size_of(3.0),
            tint_at(theme.highlight, 1.0),
            2.0,
        ),
        _ => {}
    }

    if context.editor.tool == Tool::Circle
        && let Some(index) = context.editor.active_sketch()
        && let Some(found) = circle_from(context, index, cursor, scale.world_size_of(PICK_PIXELS))
    {
        push_circle_at(
            out,
            sketch,
            found.centre,
            found.radius,
            preview,
            1.5,
            context.editor.construction,
            scale,
        );
        push_point_marker(
            out,
            sketch,
            found.centre,
            scale.world_size_of(3.0),
            preview,
            1.5,
        );
    }

    if context.editor.tool == Tool::Arc {
        arc::push_preview(out, context, sketch, cursor, preview, scale);
    }
    if context.editor.tool == Tool::Ellipse {
        ellipse::push_preview(out, context, sketch, cursor, preview, scale);
    }

    let Some(start) = context.editor.pending_start() else {
        return;
    };
    if context.editor.tool == Tool::Rectangle {
        let far = rectangle_corner(context, cursor);
        let corners = [
            start,
            DVec2::new(far.x, start.y),
            far,
            DVec2::new(start.x, far.y),
        ];
        for index in 0..4 {
            push_preview_line(
                out,
                sketch,
                corners[index],
                corners[(index + 1) % 4],
                preview,
                context.editor.construction,
                scale,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_preview_line(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    from: DVec2,
    to: DVec2,
    color: [f32; 4],
    construction: bool,
    scale: ViewScale,
) {
    push_line(
        out,
        sketch.plane.to_world(from),
        sketch.plane.to_world(to),
        color,
        1.5,
        construction,
        scale,
    );
}

#[cfg(test)]
mod tests;
