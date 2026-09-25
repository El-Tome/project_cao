//! Drawing a circle, an arc or an ellipse to another size by dragging the
//! curve itself, and turning an arc or an ellipse by sliding along it.
//!
//! The thing that moves is the thing pointed at: pressing on the outline, away
//! from any point of the drawing, and pulling draws the curve to where the
//! cursor is, about the centre it already has. A click is still a click — only
//! a drag resizes — and the centre still carries the whole shape.

use cao_part::Operation;
use cao_sketch::{CurveDrag, Curved};
use glam::DVec2;

use crate::screens::SketchContext;

use super::dragging::Gesture;
use super::sides::turn_operation;

/// One frame of that drag: shown as the curve the release would draw, written
/// once on release, as a single entry in the history.
///
/// Towards or away from the centre it is drawn to another size, read at the
/// cursor the magnets left as it always was; slid round the centre an arc or
/// an ellipse turns, read from the hand's own places.
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

    if !response.drag_stopped() {
        let mut settling = sketch.clone();
        match &drag {
            CurveDrag::Resize { reach } => settling.resize(curve, *reach, scale),
            CurveDrag::Along(turn) => {
                settling.turn_shape(&turn.points, turn.about, turn.angle, scale)
            }
        };
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
    let reach = match drag {
        CurveDrag::Resize { reach } => reach,
        CurveDrag::Along(turn) => {
            return turn_operation(index, &turn)
                .map(|operation| context.document.apply(operation))
                .is_some();
        }
    };
    context.document.apply(match curve {
        Curved::Circle(circle) => Operation::ResizeCircle {
            sketch: index,
            circle,
            reach,
        },
        Curved::Arc(arc) => Operation::ResizeArc {
            sketch: index,
            arc,
            reach,
        },
        Curved::Ellipse(ellipse) => Operation::ResizeEllipse {
            sketch: index,
            ellipse,
            reach,
        },
    });
    true
}
