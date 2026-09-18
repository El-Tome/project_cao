use cao_sketch::{Preview, Repeats, Sketch, ToolState};
use glam::DVec2;

use crate::screens::sketch::{SketchEditor, Tool};

use super::{axis_at, filled, turned};

/// What a tool that lays copies would leave, shown before the click that names
/// where the copies go.
///
/// Nothing until the selection is closed and the values are typed: a pattern
/// with no step has no copies to show.
pub(crate) fn previewed(
    sketch: &Sketch,
    editor: &SketchEditor,
    cursor: DVec2,
    snap: f64,
    scale: f64,
) -> Option<Preview> {
    let ToolState::Copying {
        held,
        naming_the_target: true,
    } = &editor.tool_state
    else {
        return None;
    };
    if held.is_empty() {
        return None;
    }

    match editor.tool {
        Tool::Mirror => {
            let axis = axis_at(sketch, cursor, snap)?;
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
