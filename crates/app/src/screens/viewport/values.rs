//! The values a shape earns as it is drawn, laid down written as what was
//! typed for them at the cursor — so that a size typed from the part's
//! variables leaves a dimension that goes on following them.

use cao_part::{Formula, Operation};
use cao_sketch::DimensionTarget;

use super::input::annotation_position;
use crate::screens::SketchContext;
use crate::wording::outcome;

/// A value a shape earned, written as the formula typed for it when it comes
/// to the same number.
///
/// Most shapes read their values back off the drawing rather than repeat what
/// was typed, and a number read back is the typed one only when the two
/// agree. A plain number is left as the drawing reads it, as it always was.
pub(crate) fn as_typed(typed: Option<(Formula, f64)>, laid: f64) -> Formula {
    match typed {
        Some((written, value))
            if written.as_number().is_none()
                && (value - laid).abs() <= 1e-6 * value.abs().max(1.0) =>
        {
            written
        }
        _ => Formula::Number(laid),
    }
}

/// Lays each value down, pinned where it is drawn, and written as what `typed`
/// says was typed for its target.
pub(crate) fn lay_values(
    context: &mut SketchContext<'_>,
    index: usize,
    wanted: Vec<(DimensionTarget, f64)>,
    typed: impl Fn(DimensionTarget) -> Option<(Formula, f64)>,
    pixel: f64,
) {
    for (target, value) in wanted {
        // Pinned down where it is drawn, in sketch units: left to stand off by
        // a distance in pixels, an annotation slides back over the drawing as
        // soon as one zooms out.
        let applied = context.document.apply(Operation::SetDimension {
            sketch: index,
            target,
            value: as_typed(typed(target), value),
            placement: annotation_position(context, index, target, pixel)
                .map(|placement| placement.offset),
        });
        if let Some(message) = outcome::message(context.lang, applied) {
            context.editor.message = Some(message);
        }
    }
}

/// Whether a click, or Enter, is refused because a field at the cursor holds
/// something that cannot be used: what was typed would be dropped without a
/// word, and the shape laid at the cursor instead. The field says why.
pub(crate) fn refused_for_what_is_typed(context: &mut SketchContext<'_>) -> bool {
    let Some(wrong) = context.editor.live.wrong() else {
        return false;
    };
    context.editor.message = Some(crate::wording::formula::unusable(context.lang, wrong));
    true
}

#[cfg(test)]
mod tests;
