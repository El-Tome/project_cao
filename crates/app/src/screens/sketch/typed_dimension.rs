//! A value typed into a dimension already on the drawing, and what the part
//! makes of it.

use cao_part::{DimensionOutcome, Outcome, PartDocument, history::Operation};
use cao_sketch::{DimensionTarget, LengthOutcome};

use super::SketchEditor;

/// `placement: None` leaves the annotation where it was put down, not reset;
/// a value taking the place of the one a dimension was placed with says where
/// that was, since the step that said it is taken back.
pub(crate) fn apply_dimension_value(
    document: &mut PartDocument,
    editor: &mut SketchEditor,
    index: usize,
    target: DimensionTarget,
    lang: &crate::lang::Catalogue,
) -> bool {
    let Some(typed) = editor.editing.as_ref().map(|editing| editing.input.clone()) else {
        return false;
    };
    let (written, value) = match document.variables().size_of(&typed) {
        Ok(read) => read,
        Err(wrong) => {
            editor.message = Some(crate::wording::formula::unusable(lang, &wrong));
            return false;
        }
    };
    if !target.takes(value) {
        editor.message = Some(lang.t("sketch.angle_would_lay_parallel"));
        return false;
    }

    // The same value twice must not repeat an identical step in the history —
    // though the same number written from a variable, or no longer from one,
    // is a step: it changes what the value follows.
    if document.sketches()[index]
        .dimension_of(target)
        .is_some_and(|dimension| {
            (dimension.value - value).abs() < 1e-4 && dimension.written == written.note()
        })
    {
        editor.message = None;
        return false;
    }

    let placement = take_back_the_first_placing(document, editor, index, target, value);
    match document.apply(Operation::SetDimension {
        sketch: index,
        target,
        value: written,
        placement,
    }) {
        Some(Outcome::Dimension(DimensionOutcome::ScaleDefined {
            millimeters_per_unit: mm,
        })) => {
            editor.message = Some(lang.t_with("sketch.scale_set", &[("mm", &format!("{mm:.4}"))]));
            true
        }
        Some(Outcome::Dimension(DimensionOutcome::Geometry(LengthOutcome::Exact))) => {
            editor.message = None;
            true
        }
        Some(Outcome::Dimension(DimensionOutcome::Geometry(LengthOutcome::BestEffort))) => {
            editor.message = Some(crate::wording::dimension::conflict_warning(lang));
            false
        }
        Some(Outcome::Dimension(DimensionOutcome::Reference)) => {
            editor.message = Some(crate::wording::dimension::redundant_warning(lang));
            true
        }
        _ => {
            editor.message = Some(lang.t("sketch.no_dimension_here"));
            false
        }
    }
}

/// Placing the part's first dimension gave it a scale of one unit to the
/// millimetre, at what the dimension read. A length typed straight into it is
/// the value it is placed with instead: the step that placed it is taken back,
/// so that this one is what says what a unit is worth, and nothing moves. Says
/// where the dimension was put down, for it to stay there.
fn take_back_the_first_placing(
    document: &mut PartDocument,
    editor: &SketchEditor,
    index: usize,
    target: DimensionTarget,
    value: f64,
) -> Option<glam::DVec2> {
    let placed = editor
        .editing
        .as_ref()
        .is_some_and(|editing| editing.placed_the_scale);
    let last = document.history.applied_operations().last();
    let placing = matches!(last, Some(Operation::SetDimension { sketch, target: set, .. })
        if *sketch == index && *set == target);
    if !placed || !placing || value <= 0.0 {
        return None;
    }
    let offset = document.sketches()[index]
        .dimension_of(target)
        .and_then(|dimension| dimension.offset);
    document.undo();
    offset
}

#[cfg(test)]
mod tests;
