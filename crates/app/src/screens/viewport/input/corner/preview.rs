use cao_sketch::{Chamfer, Preview, Sketch, ToolState};
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
    let ToolState::Corner { sides } = &editor.tool_state else {
        return None;
    };
    let (first, second) = match sides.as_slice() {
        [first, second] => (*first, *second),
        [first] => (
            *first,
            sketch
                .nearest_segment(cursor, snap)
                .filter(|under| under != first)?,
        ),
        _ => return None,
    };

    let rounding = editor.tool == Tool::Fillet;
    let asked = in_units(
        typed(editor.live.locked(), editor.chamfer_mode, rounding)?,
        scale,
    );
    match (rounding, asked) {
        (true, Chamfer::Equal(radius)) => {
            sketch.preview(|trial| trial.fillet(first, second, radius))
        }
        (true, _) => None,
        (false, _) => sketch.preview(|trial| trial.chamfer(first, second, asked)),
    }
}

#[cfg(test)]
mod tests;
