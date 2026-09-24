//! Drawing a circle, an arc or an ellipse to another size by dragging the
//! curve itself.
//!
//! The thing that moves is the thing pointed at: pressing on the outline, away
//! from any point of the drawing, and pulling draws the curve to where the
//! cursor is, about the centre it already has. A click is still a click — only
//! a drag resizes — and the centre still carries the whole shape.

use cao_part::Operation;
use cao_sketch::Curved;
use glam::DVec2;

use crate::screens::SketchContext;

/// The curve a press takes hold of, when it takes hold of no point, no
/// selection and no annotation.
pub(super) fn grabbed_curve(
    context: &SketchContext<'_>,
    index: usize,
    pressed: DVec2,
    snap: f64,
) -> Option<Curved> {
    context
        .document
        .sketches()
        .get(index)?
        .curve_at(pressed, snap)
}

/// One frame of that drag: shown as the curve the release would draw, written
/// once on release, as a single entry in the history.
pub(super) fn drag_curve(
    context: &mut SketchContext<'_>,
    index: usize,
    curve: Curved,
    cursor: DVec2,
    response: &egui::Response,
) -> bool {
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let reach = sketch.reach_through(curve, cursor);
    let scale = context.document.scale();

    if !response.drag_stopped() {
        let mut settling = sketch.clone();
        settling.resize(curve, reach, scale);
        if let Some(state) = context.editor.select_state() {
            state.drag_position = Some(cursor);
            state.drag_preview = Some(settling);
        }
        return false;
    }

    if let Some(state) = context.editor.select_state() {
        state.dragged_curve = None;
        state.drag_position = None;
        state.drag_preview = None;
    }
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
