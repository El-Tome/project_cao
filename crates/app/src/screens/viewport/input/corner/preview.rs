use cao_sketch::{Corner, Preview, Sketch, ToolState};
use glam::DVec2;

use crate::screens::sketch::{SketchEditor, Tool};

use super::{in_units, shape_of, typed};

/// What a corner would become, shown before the click that commits it.
///
/// The two sides are the ones already clicked, or the one clicked and the one
/// under the cursor. Nothing until the values are typed: a corner with no
/// radius has no curve to show. A value too big is shown as a cut that does
/// not fit, before Enter holds it back.
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
    let shape = shape_of(
        rounding,
        in_units(
            typed(editor.live.locked(), editor.chamfer_mode, rounding)?,
            scale,
        ),
    );
    let construction = editor.construction;
    sketch.preview(|trial| {
        let group = trial.fresh_cut_group();
        let mut laid = Vec::new();
        for corner in shown {
            let Some((first, second)) = trial.sides_of(corner) else {
                continue;
            };
            // A corner that cannot be cut is simply not shown: the gesture
            // lays the others, and the tool says how many it turned away.
            if let Ok(cut) = trial.cut_corner(first, second, shape, construction, group) {
                laid.push(cut);
            }
        }
        Some(laid)
    })
}

#[cfg(test)]
mod tests;
