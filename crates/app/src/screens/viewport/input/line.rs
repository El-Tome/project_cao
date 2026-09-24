//! What a freshly-drawn trait earns on its own: whichever length the user typed
//! while dragging it, and the right angle they aimed at.

use cao_part::Operation;
use cao_sketch::{Aim, SegmentId};

use super::annotation_position;
use crate::screens::SketchContext;
use crate::wording::outcome;

/// Places on the line just drawn whatever the user typed, and the right angle
/// they aimed at.
///
/// A value that would say nothing is left out: the drawing already holds it,
/// and a second copy could only be redundant.
pub(super) fn dimension_the_line(
    context: &mut SketchContext<'_>,
    index: usize,
    segment: SegmentId,
    aimed: Aim,
    pixel: f64,
) {
    let locked = context.editor.live.locked();
    let scale = context.document.scale();
    let wanted = cao_sketch::line_dimensions(
        &context.document.sketches()[index],
        segment,
        locked,
        aimed.square_with,
        scale,
    );

    for (target, value) in wanted {
        // Pinned down where it is drawn, in sketch units: left to stand off by
        // a distance in pixels, an annotation slides back over the drawing as
        // soon as one zooms out.
        let applied = context.document.apply(Operation::SetDimension {
            sketch: index,
            target,
            value,
            placement: annotation_position(context, index, target, pixel)
                .map(|placement| placement.offset),
        });
        if let Some(message) = outcome::message(context.lang, applied) {
            context.editor.message = Some(message);
        }
    }
}
