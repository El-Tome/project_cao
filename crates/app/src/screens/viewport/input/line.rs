//! What a freshly-drawn trait earns on its own: whichever length the user typed
//! while dragging it, and the right angle they aimed at.

use cao_sketch::{Aim, DimensionTarget, SegmentId};

use super::super::values::{lay_values, shape_scale};
use crate::screens::SketchContext;

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
    let scale = shape_scale(context);
    let wanted = cao_sketch::line_dimensions(
        &context.document.sketches()[index],
        segment,
        locked,
        aimed.square_with,
        scale,
    );

    let length = context.editor.live.typed_as_written(0);
    let typed = |target| match target {
        DimensionTarget::Length(drawn) if drawn == segment => length.clone(),
        _ => None,
    };
    lay_values(context, index, wanted, typed, pixel);
}

#[cfg(test)]
mod tests;
