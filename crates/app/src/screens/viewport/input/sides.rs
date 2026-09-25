//! A side pressed and pulled: across it travels and the shape stretches,
//! along itself it turns the shape it belongs to. What it does is the
//! drawing's to say (`Sketch::side_drag`); what is here is the frame — the
//! preview shown while the hand moves, and the step written once on release.

use cao_part::Operation;
use cao_sketch::{LengthOutcome, SegmentId, SideDrag, Turn};

use crate::screens::SketchContext;

use super::dragging::Gesture;

/// One frame of a side drag.
pub(super) fn drag_side(
    context: &mut SketchContext<'_>,
    index: usize,
    side: SegmentId,
    response: &egui::Response,
    gesture: Gesture,
) -> bool {
    let Some(pressed) = context
        .editor
        .select_state()
        .and_then(|state| state.grabbed_at)
    else {
        return false;
    };
    let scale = context.document.scale();
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let drag = sketch.side_drag(side, pressed, gesture.raw_cursor, &gesture.magnets, scale);
    let mut settling = sketch.clone();
    let outcome = match &drag {
        SideDrag::Across { by } => settling.move_side(side, *by, scale),
        SideDrag::Along(turn) => settling.turn_shape(&turn.points, turn.about, turn.angle, scale),
    };
    // A side is pulled by the hand itself, not by the magnets: a mark saying
    // where they would pull the cursor would say where the side does not go.
    context.editor.snap = None;

    if !response.drag_stopped() {
        if let Some(state) = context.editor.select_state() {
            state.drag_position = Some(gesture.raw_cursor);
            state.drag_preview = Some(settling);
        }
        return false;
    }

    if let Some(state) = context.editor.select_state() {
        state.dragged_side = None;
        state.grabbed_at = None;
        state.drag_position = None;
        state.drag_preview = None;
    }
    // What the drawing refused writes nothing: a step that changes nothing is
    // what the next undo would take back.
    let written = (outcome == LengthOutcome::Exact)
        .then(|| side_operation(index, side, &drag))
        .flatten();
    match written {
        Some(operation) => {
            context.document.apply(operation);
            true
        }
        None => false,
    }
}

/// The step a side drag writes, or nothing when the hand came back to where
/// it pressed.
pub(super) fn side_operation(index: usize, side: SegmentId, drag: &SideDrag) -> Option<Operation> {
    match drag {
        SideDrag::Across { by } => (by.length() > 1e-9).then_some(Operation::MoveSegment {
            sketch: index,
            segment: side,
            by: *by,
        }),
        SideDrag::Along(turn) => turn_operation(index, turn),
    }
}

/// The step a turn writes, or nothing for a turn of nothing.
pub(super) fn turn_operation(index: usize, turn: &Turn) -> Option<Operation> {
    (turn.angle.abs() > 1e-12).then(|| Operation::TurnShape {
        sketch: index,
        points: turn.points.clone(),
        about: turn.about,
        angle: turn.angle,
    })
}

#[cfg(test)]
mod tests;
