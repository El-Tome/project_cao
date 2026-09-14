//! What one click of the trim tool does.

use cao_part::Operation;
use cao_sketch::Sketch;
use glam::DVec2;

use crate::screens::viewport::SketchContext;
use crate::wording::outcome;

/// One click of the trim tool: takes out the stretch of trait or of curve the
/// click fell in, between the two points sitting on either side of it.
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
    let Some(cut) = cut_under(sketch, index, cursor, snap) else {
        return false;
    };

    let applied = context.document.apply(cut);
    if let Some(message) = outcome::message(context.lang, applied) {
        context.editor.message = Some(message);
    }
    true
}

/// Which cut the click is asking for.
///
/// The straight trait first, since a trait running into a curve puts the two
/// within reach of one another and a click meant for the corner would otherwise
/// pick whichever came out of the drawing first.
fn cut_under(sketch: &Sketch, index: usize, cursor: DVec2, snap: f64) -> Option<Operation> {
    if let Some(segment) = sketch.nearest_segment(cursor, snap)
        && let Some((from, to)) = sketch.stretch_at(segment, cursor)
    {
        return Some(Operation::Trim {
            sketch: index,
            segment,
            from,
            to,
        });
    }
    let arc = sketch.nearest_arc(cursor, snap)?;
    let (from, to) = sketch.arc_stretch_at(arc, cursor)?;
    Some(Operation::TrimArc {
        sketch: index,
        arc,
        from,
        to,
    })
}
