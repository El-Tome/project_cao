use cao_sketch::{Preview, Repeats, Sketch, ToolState};
use glam::DVec2;

use crate::screens::sketch::{CopyPanel, SketchEditor, Tool};

use super::{axis_at, filled, turned};

/// What a tool that lays copies would leave, shown before the click that names
/// where the copies go.
///
/// Nothing until the selection is closed or the axis is held, and the values
/// are typed: a pattern with no step has no copies to show.
pub(crate) fn previewed(
    sketch: &Sketch,
    editor: &SketchEditor,
    cursor: DVec2,
    snap: f64,
    scale: f64,
) -> Option<Preview> {
    let held = editor.copied();
    let axis = editor.copy_panel().and_then(CopyPanel::axis);
    let naming = matches!(
        editor.tool_state,
        ToolState::Copying {
            naming_the_target: true,
            ..
        }
    );
    if held.is_empty() || (axis.is_none() && !naming) {
        return None;
    }

    match editor.tool {
        Tool::Mirror => {
            let axis = axis.or_else(|| axis_at(sketch, cursor, snap))?;
            sketch.preview(|trial| trial.mirror(held, axis))
        }
        Tool::CircularPattern => {
            let centre = sketch.nearest_point(cursor, snap)?;
            let (degrees, count) = turned(editor.live.locked())?;
            sketch.preview(|trial| trial.pattern_around(held, centre, degrees, count))
        }
        Tool::RectangularPattern => {
            let direction = axis_at(sketch, cursor, snap)?;
            let live = &editor.live;
            let (along, across) =
                filled([live.typed(0), live.typed(1), live.typed(2), live.typed(3)])?;
            let in_units = |run: Repeats| Repeats {
                step: run.step / scale,
                count: run.count,
            };
            sketch.preview(|trial| {
                trial.pattern_along(held, direction, in_units(along), in_units(across))
            })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests;
