use cao_sketch::{Chamfer, Corner, Laid, Preview, Sketch, ToolState};
use glam::DVec2;

use crate::screens::sketch::{SketchEditor, Tool};

use super::{in_units, typed};

/// What a corner would become, shown before the click that commits it.
///
/// The two sides are the ones already clicked, or the one clicked and the one
/// under the cursor. Nothing until the values are typed: a corner with no
/// radius has no curve to show.
pub(crate) fn previewed(
    sketch: &Sketch,
    editor: &SketchEditor,
    cursor: DVec2,
    snap: f64,
    scale: f64,
) -> Option<Preview> {
    if !matches!(editor.tool, Tool::Chamfer | Tool::Fillet) {
        return None;
    }
    let ToolState::Corner { taken, half } = &editor.tool_state else {
        return None;
    };
    // Every corner taken is shown, wherever the cursor is. The side under the
    // cursor only ever stands in for a second click that has not happened yet.
    let mut shown = taken.clone();
    if let Some(first) = half
        && let Some(under) = sketch
            .nearest_segment(cursor, snap)
            .filter(|under| under != first)
    {
        shown.push(Corner::Between(*first, under));
    }
    if shown.is_empty() {
        return None;
    }

    let rounding = editor.tool == Tool::Fillet;
    let asked = in_units(
        typed(editor.live.locked(), editor.chamfer_mode, rounding)?,
        scale,
    );
    sketch.preview(|trial| {
        let mut laid = Vec::new();
        for corner in shown {
            let Some((first, second)) = trial.sides_of(corner) else {
                continue;
            };
            // A corner the value does not fit is simply not shown: the gesture
            // lays the others, and the tool says how many it turned away.
            match (rounding, asked) {
                (true, Chamfer::Equal(radius)) => {
                    laid.extend(trial.fillet(first, second, radius)?.laid())
                }
                (true, _) => return None,
                (false, _) => laid.extend(trial.chamfer(first, second, asked)?.laid()),
            }
        }
        Some(laid)
    })
}

#[cfg(test)]
mod tests;
