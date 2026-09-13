//! What one click of the arc tool does, and what the canvas shows in between.

use cao_part::Operation;
use cao_sketch::{ArcDraft, ToolState, arc_from};
use glam::DVec2;

use super::point_ref_at;
use crate::screens::viewport::SketchContext;

/// One click of the arc tool: takes the place pointed at, and draws the arc as
/// soon as enough of it is known.
pub(crate) fn draw_arc(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> bool {
    let mode = context.editor.arc_mode;
    let mut places = places_so_far(context);
    let asks_for = crate::wording::arc::asks_for(context.lang, mode);

    if places.len() + 1 < mode.wants() {
        places.push(cursor);
        context.editor.tool_state = ToolState::Arc { places };
        context.editor.message = Some(asks_for);
        return false;
    }

    let Some(drawn) = arc_from(mode, &places, cursor) else {
        context.editor.message = Some(context.lang.t("sketch.no_arc_from_these"));
        return false;
    };

    // Each of the three reuses a point already drawn when one is under it, as
    // everywhere else, so an arc hangs off what is there instead of stacking
    // points on top of it.
    let center = point_ref_at(context, index, drawn.centre, snap);
    let start = point_ref_at(context, index, drawn.start, snap);
    let end = point_ref_at(context, index, drawn.end, snap);

    context.editor.tool_state = ToolState::None;
    context.document.apply(Operation::AddArc {
        sketch: index,
        center,
        start,
        end,
        construction: context.editor.construction,
    });
    context.editor.message = Some(asks_for);
    true
}

/// The arc a click right now would draw, for the canvas to show first.
pub(crate) fn arc_preview(context: &SketchContext<'_>, cursor: DVec2) -> Option<ArcDraft> {
    arc_from(context.editor.arc_mode, &places_so_far(context), cursor)
}

fn places_so_far(context: &SketchContext<'_>) -> Vec<DVec2> {
    match &context.editor.tool_state {
        ToolState::Arc { places } => places.clone(),
        _ => Vec::new(),
    }
}
