//! What one click of the trim tool does.

use cao_part::Operation;
use cao_sketch::{Going, Sketch};
use glam::DVec2;

use crate::screens::sketch::{SketchEditor, Tool};
use crate::screens::viewport::SketchContext;
use crate::wording::outcome;

/// One click of the trim tool: takes out the stretch of trait, of curve, of
/// round or of ellipse the click fell in, between the two points sitting on
/// either side of it.
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
/// The straight trait first, then the curve, then the round — the order
/// `Sketch::pick` and the constraint tool already read a click in. Where a
/// trait runs into a curve both are within reach of the same click, and a tool
/// that answered with whichever came out of the drawing first would cut a
/// different element depending on the order they were drawn in.
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
    if let Some(arc) = sketch.nearest_arc(cursor, snap)
        && let Some((from, to)) = sketch.arc_stretch_at(arc, cursor)
    {
        return Some(Operation::TrimArc {
            sketch: index,
            arc,
            from,
            to,
        });
    }
    if let Some(circle) = sketch.nearest_circle(cursor, snap) {
        return Some(Operation::TrimCircle {
            sketch: index,
            circle,
            between: sketch.circle_stretch_at(circle, cursor),
        });
    }
    let ellipse = sketch.nearest_ellipse(cursor, snap)?;
    Some(Operation::TrimEllipse {
        sketch: index,
        ellipse,
        between: sketch.ellipse_stretch_at(ellipse, cursor),
    })
}

/// What a click right now would take out of the drawing, read off the very
/// call that click commits.
///
/// Nothing when the trim is not the tool in hand, and nothing where a click
/// would do nothing: a preview of what would be refused is a preview that
/// lies.
pub(crate) fn previewed(
    sketch: &Sketch,
    editor: &SketchEditor,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> Option<Going> {
    if editor.tool != Tool::Trim {
        return None;
    }
    match cut_under(sketch, index, cursor, snap)? {
        Operation::Trim {
            segment, from, to, ..
        } => sketch.trim_takes(segment, from, to),
        Operation::TrimArc { arc, from, to, .. } => sketch.arc_trim_takes(arc, from, to),
        Operation::TrimCircle {
            circle, between, ..
        } => sketch.circle_trim_takes(circle, between),
        Operation::TrimEllipse {
            ellipse, between, ..
        } => sketch.ellipse_trim_takes(ellipse, between),
        // `cut_under` lays no other kind of step, and a wildcard here is what
        // let the round slip through when trimming one landed.
        other => unreachable!("the trim tool asked for {other:?}"),
    }
}

#[cfg(test)]
mod tests;
