//! A value typed into a dimension already on the drawing, and what the part
//! makes of it.

use cao_part::{DimensionOutcome, Outcome, PartDocument, history::Operation};
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

#[cfg(test)]
mod tests {
    use cao_part::{Operation, PointRef};
    use cao_sketch::{ArcId, PointId, Sketch, WorkPlane};
    use chrono::Utc;
    use glam::DVec2;

    use super::*;
    use crate::lang::Catalogue;
    use crate::screens::sketch::DimensionEdit;

    #[test]
    fn a_radius_a_fixed_chord_already_rules_out_warns_instead_of_bending_the_arc_silently() {
        let mut document = PartDocument::new("part", Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        document.apply(Operation::AddArc {
            sketch: 0,
            center: PointRef::New(DVec2::new(40.0, 30.0)),
            start: PointRef::Existing(Sketch::ORIGIN),
            end: PointRef::New(DVec2::new(80.0, 0.0)),
            construction: false,
        });
        document.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Distance {
                from: Sketch::ORIGIN,
                to: PointId(2),
            },
            value: 80.0,
            placement: None,
        });

        let mut editor = SketchEditor::default();
        let target = DimensionTarget::ArcRadius(ArcId(0));
        editor.editing = Some(DimensionEdit {
            target,
            input: "5".to_string(),
            focus: false,
        });
        let lang = Catalogue::french();

        let applied = apply_dimension_value(&mut document, &mut editor, 0, target, &lang);

        assert!(!applied, "refused: the field stays open for another try");
        assert_eq!(
            editor.message.as_deref(),
            Some(crate::wording::dimension::conflict_warning(&lang).as_str()),
        );
        assert!(
            document.sketches()[0].dimension_of(target).is_none(),
            "the refused value is not kept"
        );
    }
}
