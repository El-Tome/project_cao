//! What app · input/line.rs is held to.
//!
//! Closes #175.
//! - a formula typed in the fields at the cursor is what the dimension they
//!   leave keeps — `a_length_typed_as_a_formula_is_what_its_dimension_keeps`

use cao_part::history::PointRef;
use cao_part::{Formula, Operation, PartDocument, VariableChange};
use cao_sketch::{Aim, DimensionTarget, SegmentId, WorkPlane};
use chrono::Utc;
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

/// `width` at 120, and a trait 60 long.
fn a_width_and_a_trait() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document
        .change_variable(VariableChange::Added {
            name: "width".to_string(),
            formula: Formula::Number(120.0),
        })
        .expect("a variable");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::ZERO),
        end: PointRef::New(DVec2::new(60.0, 0.0)),
        construction: false,
    });
    document
}

#[test]
fn a_length_typed_as_a_formula_is_what_its_dimension_keeps() {
    let mut document = a_width_and_a_trait();
    let mut editor = SketchEditor::default();
    editor.live.open();
    let field = editor.live.field(0);
    field.text = "=width/2".to_string();
    field.take(document.variables());
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let aimed = Aim {
        position: DVec2::new(60.0, 0.0),
        square_with: None,
    };

    dimension_the_line(&mut context, 0, SegmentId(0), aimed, 0.05);

    assert_eq!(
        document.formula_of(0, DimensionTarget::Length(SegmentId(0))),
        Some("width / 2".to_string()),
    );
}
