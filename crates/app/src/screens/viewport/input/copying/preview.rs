use cao_sketch::{Preview, Repeats, Sketch};

use crate::screens::sketch::{CopyTarget, SketchEditor, Tool};

use super::{filled, turned};

/// What a tool that lays copies would leave, shown before it is laid.
///
/// Nothing until the panel holds what to copy and where, and the values are
/// typed: a pattern with no step has no copies to show.
pub(crate) fn previewed(sketch: &Sketch, editor: &SketchEditor, scale: f64) -> Option<Preview> {
    let panel = editor.copy_panel()?;
    let held = panel.elements();
    if held.is_empty() {
        return None;
    }

    match (editor.tool, panel.target()?) {
        (Tool::Mirror, CopyTarget::Axis(axis)) => sketch.preview(|trial| trial.mirror(held, axis)),
        (Tool::CircularPattern, CopyTarget::Centre(centre)) => {
            let (degrees, count) = turned(editor.live.locked())?;
            sketch.preview(|trial| trial.pattern_around(held, centre, degrees, count))
        }
        (Tool::RectangularPattern, CopyTarget::Axis(direction)) => {
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
