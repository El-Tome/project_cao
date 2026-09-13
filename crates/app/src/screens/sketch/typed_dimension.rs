//! A value typed into a dimension already on the drawing, and what the part
//! makes of it.

use cao_part::{DimensionOutcome, PartDocument, history::Operation};
use cao_sketch::{DimensionTarget, LengthOutcome};

use super::SketchEditor;

/// `placement: None` leaves the annotation where it was put down, not reset.
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
    let Ok(value) = typed.trim().replace(',', ".").parse::<f64>() else {
        editor.message = Some(lang.t("sketch.invalid_value"));
        return false;
    };

    // The same value twice must not repeat an identical step in the history.
    if document.sketches()[index]
        .dimension_of(target)
        .is_some_and(|dimension| (dimension.value - value).abs() < 1e-4)
    {
        editor.message = None;
        return false;
    }

    match document.apply(Operation::SetDimension {
        sketch: index,
        target,
        value,
        placement: None,
    }) {
        Some(DimensionOutcome::ScaleDefined {
            millimeters_per_unit: mm,
        }) => {
            editor.message = Some(lang.t_with("sketch.scale_set", &[("mm", &format!("{mm:.4}"))]));
            true
        }
        Some(DimensionOutcome::Geometry(LengthOutcome::Exact)) => {
            editor.message = None;
            true
        }
        Some(DimensionOutcome::Geometry(LengthOutcome::BestEffort)) => {
            editor.message = Some(lang.t("sketch.closed_outline"));
            true
        }
        Some(DimensionOutcome::Reference) => {
            editor.message = Some(crate::wording::dimension::redundant_warning(lang));
            true
        }
        _ => {
            editor.message = Some(lang.t("sketch.no_dimension_here"));
            false
        }
    }
}
