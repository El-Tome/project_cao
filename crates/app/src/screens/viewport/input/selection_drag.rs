//! Moving a whole selection at once, the way a desktop moves a group of
//! icons: a drag that starts on something already picked carries all of it.

use cao_part::Operation;
use cao_sketch::PointId;
use glam::DVec2;

use crate::screens::SketchContext;

use super::pick;

/// The points a drag would carry along, when it starts on something the
/// selection tool is already holding.
///
/// Empty when the press lands anywhere else: a drag beside a selection is a
/// new box, not a move of the old one.
pub(super) fn grabbed_group(
    context: &SketchContext<'_>,
    index: usize,
    pressed: DVec2,
    snap: f64,
    pixel: f64,
) -> Vec<PointId> {
    let Some(what) = pick(context, index, pressed, snap, pixel) else {
        return Vec::new();
    };
    if !context.editor.is_selected(what) {
        return Vec::new();
    }
    let Some(sketch) = context.document.sketches().get(index) else {
        return Vec::new();
    };
    sketch.points_of(context.editor.selection())
}

/// Moving a whole selection at once. Same rule as a point: shown following the
/// cursor, written once on release, as a single entry in the history.
pub(super) fn drag_group(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    response: &egui::Response,
) -> bool {
    let Some(origin) = context
        .editor
        .select_state()
        .and_then(|state| state.drag_origin)
    else {
        return false;
    };
    let travelled = cursor - origin;
    let points = context
        .editor
        .select_state()
        .map(|state| state.dragged_group.clone())
        .unwrap_or_default();

    if !response.drag_stopped() {
        let mut settling = context.document.sketches()[index].clone();
        let dropped: Vec<(PointId, DVec2)> = points
            .iter()
            .filter_map(|point| {
                settling
                    .points()
                    .get(point.0)
                    .map(|place| (*point, *place + travelled))
            })
            .collect();
        settling.settle_around_all(&dropped, context.document.scale());
        if let Some(state) = context.editor.select_state() {
            state.drag_position = Some(cursor);
            state.drag_preview = Some(settling);
        }
        return false;
    }

    if let Some(state) = context.editor.select_state() {
        state.dragged_group.clear();
        state.drag_origin = None;
        state.drag_position = None;
        state.drag_preview = None;
    }
    if travelled.length() < 1e-9 {
        return false;
    }
    context.document.apply(Operation::MoveMany {
        sketch: index,
        points,
        by: travelled,
    });
    true
}
