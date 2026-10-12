//! What the tool in hand works out from the cursor once a frame and shows as
//! its preview: the end a line is aimed at, and the quarter an angle being put
//! down faces. Worked out as the cursor moves rather than at the click, so that
//! what is drawn on screen is exactly what a click would record.

use cao_sketch::{Aim, ChainAnchor, SegmentId, ToolState};
use glam::DVec2;

use crate::screens::SketchContext;
use crate::screens::viewport::HOLD_PIXELS;
use crate::screens::viewport::values::shape_scale;

pub(super) fn follow_the_cursor(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    pixel: f64,
) {
    context.editor.aimed = match context.editor.tool_state {
        ToolState::Line { anchor, previous } => Some(aim(context, index, anchor, previous, cursor)),
        _ => None,
    };
    hold_the_quarter(context, index, cursor, pixel);
}

/// Keeps the quarter an angle being put down faces, through a margin of a few
/// pixels: the one shown stays until the cursor is clearly past the line to
/// the next.
fn hold_the_quarter(context: &mut SketchContext<'_>, index: usize, cursor: DVec2, pixel: f64) {
    let ToolState::Dimension {
        placing: Some(placing),
        ..
    } = context.editor.tool_state
    else {
        return;
    };
    let Some(sketch) = context.document.sketches().get(index) else {
        return;
    };
    if !placing.is_angle() {
        return;
    }
    let held = sketch.oriented_holding(placing, cursor, pixel * HOLD_PIXELS);
    if let ToolState::Dimension { placing, .. } = &mut context.editor.tool_state {
        *placing = Some(held);
    }
}

/// Applies to the cursor everything the user has already decided. The rules are
/// the drawing's; what this adds is the state the tool is holding.
fn aim(
    context: &SketchContext<'_>,
    index: usize,
    anchor: ChainAnchor,
    previous: Option<SegmentId>,
    cursor: DVec2,
) -> Aim {
    let Some(sketch) = context.document.sketches().get(index) else {
        return Aim::at(cursor);
    };
    sketch.aim(
        anchor,
        previous,
        cursor,
        context.editor.live.locked(),
        shape_scale(context),
    )
}
