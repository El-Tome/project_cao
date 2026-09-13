//! What one click of the arc tool does, and what the canvas shows in between.

use cao_part::Operation;
use cao_sketch::{ArcDraft, PointId, Sketch, ToolState, arc_aimed, arc_from};
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
    let cursor = aimed(context, &places, cursor);

    if places.len() + 1 < mode.wants() {
        places.push(cursor);
        context.editor.tool_state = ToolState::Arc { places };
        context.editor.live.open();
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
    context.editor.live.clear();
    context.editor.message = Some(asks_for);
    true
}

/// The arc a click right now would draw, for the canvas to show first.
pub(crate) fn arc_preview(context: &SketchContext<'_>, cursor: DVec2) -> Option<ArcDraft> {
    let places = places_so_far(context);
    let cursor = aimed(context, &places, cursor);
    arc_from(context.editor.arc_mode, &places, cursor)
}

fn places_so_far(context: &SketchContext<'_>) -> Vec<DVec2> {
    match &context.editor.tool_state {
        ToolState::Arc { places } => places.clone(),
        _ => Vec::new(),
    }
}

/// The cursor, once a value typed into the live field has had its say. What
/// that value means at each stage is `cao_sketch`'s to decide; this only
/// hands over what has been picked so far and what was typed.
pub(crate) fn aimed(context: &SketchContext<'_>, places: &[DVec2], cursor: DVec2) -> DVec2 {
    arc_aimed(
        context.editor.arc_mode,
        places,
        cursor,
        context.editor.live.first.locked,
        context.document.scale(),
    )
}

/// The points a drag on `point` would carry along, when it is the centre of
/// an arc: the centre and its two ends move as one, the way a circle's rim
/// follows a dragged centre. `None` for a point that is nobody's centre, so a
/// plain drag of it stays a plain drag.
pub(crate) fn arc_centre_group(sketch: &Sketch, point: PointId) -> Option<Vec<PointId>> {
    let ends = sketch.arc_ends_around(point);
    (!ends.is_empty()).then(|| {
        let mut group = vec![point];
        group.extend(ends);
        group
    })
}
