//! What one click of the trim tool does.

use cao_part::Operation;
use glam::DVec2;

use crate::screens::viewport::SketchContext;
use crate::wording::outcome;

/// One click of the trim tool: takes out the stretch of trait the click fell
/// in, between the two points sitting on either side of it.
pub(crate) fn trim(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> bool {
    context.editor.message = Some(context.lang.t("sketch.click_a_stretch"));

    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let cut = sketch
        .nearest_segment(cursor, snap)
        .and_then(|segment| Some((segment, sketch.stretch_at(segment, cursor)?)));
    let Some((segment, (from, to))) = cut else {
        return false;
    };

    let applied = context.document.apply(Operation::Trim {
        sketch: index,
        segment,
        from,
        to,
    });
    if let Some(message) = outcome::message(context.lang, applied) {
        context.editor.message = Some(message);
    }
    true
}
