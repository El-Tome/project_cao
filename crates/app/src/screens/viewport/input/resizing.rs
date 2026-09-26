//! Drawing a circle, an arc or an ellipse to another size by dragging the
//! curve itself, and turning an arc or an ellipse by sliding along it.
//!
//! The thing that moves is the thing pointed at: pressing on the outline, away
//! from any point of the drawing, and pulling draws the curve to where the
//! cursor is, about the centre it already has. A click is still a click — only
//! a drag resizes — and the centre still carries the whole shape.

use cao_sketch::{CurveDrag, Curved};
use glam::DVec2;

use crate::screens::SketchContext;

use super::dragging::Gesture;
use super::sides::{curve_turn_operation, resize_operation};

/// One frame of that drag: shown as the curve the release would draw, written
/// once on release, as a single entry in the history.
///
/// Towards or away from the centre it is drawn to another size, read at the
/// cursor the magnets left as it always was; slid round the centre an arc or
/// an ellipse turns, read from the hand's own places, and is drawn to the size
/// that keeps the place grabbed under the hand.
pub(super) fn drag_curve(
    context: &mut SketchContext<'_>,
    index: usize,
    curve: Curved,
    cursor: DVec2,
    response: &egui::Response,
    gesture: Gesture,
) -> bool {
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let pressed = context
        .editor
        .select_state()
        .and_then(|state| state.grabbed_at)
        .unwrap_or(gesture.raw_pressed);
    let scale = context.document.scale();
    let drag = sketch.curve_drag(
        curve,
        pressed,
        gesture.raw_cursor,
        cursor,
        &gesture.magnets,
        scale,
    );

    let mut settling = sketch.clone();
    let written = match &drag {
        CurveDrag::Resize { reach } => {
            settling.resize(curve, *reach, scale);
            Some(resize_operation(index, curve, *reach))
        }
        CurveDrag::Along { turn, reach } => {
            // A curve slid round turns with the hand itself, not with the
            // magnets, whose mark would say where it does not go.
            context.editor.snap = None;
            let turned = settling.turn_shape(&turn.points, turn.about, turn.angle, scale);
            // It is drawn to the hand only about the centre it turned on: a
            // size a rule holds is left as it was.
            let resized = reach.map(|reach| (reach, settling.resize_in_place(curve, reach, scale)));
            curve_turn_operation(index, curve, turn, turned, resized)
        }
    };

    if !response.drag_stopped() {
        if let Some(state) = context.editor.select_state() {
            state.drag_position = Some(cursor);
            state.drag_preview = Some(settling);
        }
        return false;
    }

    if let Some(state) = context.editor.select_state() {
        state.dragged_curve = None;
        state.grabbed_at = None;
        state.drag_position = None;
        state.drag_preview = None;
    }
    match written {
        Some(operation) => {
            context.document.apply(operation);
            true
        }
        None => false,
    }
}
