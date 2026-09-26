//! Moving an annotation out of the way, and finding the one under the cursor.

use cao_part::Operation;
use cao_sketch::DimensionTarget;
use glam::DVec2;

use crate::screens::SketchContext;

/// Moving an annotation out of the way. Same rule as a point: shown following
/// the cursor, written once on release.
pub(super) fn drag_annotation(
    context: &mut SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    cursor: DVec2,
    response: &egui::Response,
    pixel: f64,
) -> bool {
    let Some(origin) = context
        .editor
        .select_state()
        .and_then(|state| state.drag_origin)
    else {
        return false;
    };
    let travelled = cursor - origin;

    if !response.drag_stopped() {
        if let Some(state) = context.editor.select_state() {
            state.drag_position = Some(cursor);
        }
        return false;
    }

    // Where the annotation sits right now, whether that was recorded before or
    // is still the standing-off distance it was drawn with.
    let previous = annotation_position(context, index, target, pixel)
        .map(|placement| placement.offset)
        .unwrap_or_default();

    if let Some(state) = context.editor.select_state() {
        state.dragged_dimension = None;
        state.drag_origin = None;
        state.drag_position = None;
    }
    context.document.apply(Operation::MoveDimension {
        sketch: index,
        target,
        offset: previous + travelled,
    });
    true
}

/// Which annotation sits under the cursor. Asked straight of the sketch: it
/// is the one that knows where each of its dimensions is drawn.
pub(super) fn nearest_annotation(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    tolerance: f64,
    pixel: f64,
) -> Option<DimensionTarget> {
    context.document.sketches().get(index)?.nearest_dimension(
        cursor,
        tolerance,
        crate::screens::annotations::metrics(pixel, DVec2::ZERO),
    )
}

/// Where an annotation's value sits right now, and the offset that would
/// record it there.
///
/// Asked straight of `sketch.place`: no colour needed just to find a
/// position, so no theme has to be made up to get one.
pub(crate) fn annotation_position(
    context: &SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    pixel: f64,
) -> Option<cao_sketch::Placement> {
    context.document.sketches().get(index)?.place(
        target,
        crate::screens::annotations::metrics(pixel, DVec2::ZERO),
    )
}
