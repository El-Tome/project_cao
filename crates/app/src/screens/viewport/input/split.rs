//! What one click of the split tool does.

use cao_part::Operation;
use cao_sketch::Crossing;
use glam::DVec2;

use crate::screens::viewport::SketchContext;
use crate::wording::outcome;

/// One click of the split tool: drops a point where the traits nearest the
/// click cross, and cuts each of them in two there.
pub(crate) fn split(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> bool {
    context.editor.message = Some(context.lang.t("sketch.click_a_crossing"));

    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let (at, segments) = match sketch.crossing_at(cursor, snap) {
        Some(Crossing::Traits { at, segments }) => (at, segments),
        Some(Crossing::Curved) => {
            context.editor.message = Some(context.lang.t("sketch.split_needs_traits"));
            return false;
        }
        None => return false,
    };

    let applied = context.document.apply(Operation::Split {
        sketch: index,
        segments,
        at,
    });
    if let Some(message) = outcome::message(context.lang, applied) {
        context.editor.message = Some(message);
    }
    true
}
