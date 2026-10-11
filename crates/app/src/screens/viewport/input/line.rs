//! What a freshly-drawn trait earns on its own: whichever length the user typed
//! while dragging it, and the right angle they aimed at — the perpendicular
//! rule, or with the option off a dimension of 90°.

use cao_part::Operation;
use cao_sketch::{Aim, Constraint, DimensionTarget, SegmentId};

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
    let square_rule = aimed
        .square_with
        .filter(|_| context.editor.sketching.exact_angles_lay_rules);
    let wanted = cao_sketch::line_dimensions(
        &context.document.sketches()[index],
        segment,
        locked,
        aimed.square_with.filter(|_| square_rule.is_none()),
        scale,
    );

    let length = context.editor.live.typed_as_written(0);
    let typed = |target| match target {
        DimensionTarget::Length(drawn) if drawn == segment => length.clone(),
        _ => None,
    };
    lay_values(context, index, wanted, typed, pixel);
    if let Some(first) = square_rule {
        context.document.apply(Operation::Constrain {
            sketch: index,
            constraint: Constraint::Perpendicular {
                first,
                second: segment,
            },
        });
    }
}

#[cfg(test)]
mod tests;
