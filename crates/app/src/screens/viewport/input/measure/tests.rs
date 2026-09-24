//! What app · input/measure.rs is held to.
//!
//! Closes #175.
//! - a dimension on the drawing shows its value; editing it shows its formula
//!   — `a_value_written_from_a_variable_is_opened_as_what_it_was_written_as`,
//!   `a_plain_value_is_opened_as_its_number`

use cao_part::history::PointRef;
use cao_part::{Formula, Operation, PartDocument, VariableChange};
use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};
use chrono::Utc;
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

const SIDE: DimensionTarget = DimensionTarget::Length(SegmentId(0));

/// A trait whose length is written from `width`.
fn a_trait_as_long_as_a_width() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document
        .change_variable(VariableChange::Added {
            name: "width".to_string(),
            formula: Formula::Number(60.0),
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
    let width = document.variables().read("width").expect("it reads");
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: SIDE,
        value: width,
        placement: None,
    });
    document
}

#[test]
fn a_value_written_from_a_variable_is_opened_as_what_it_was_written_as() {
    let mut document = a_trait_as_long_as_a_width();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    edit_dimension(&mut context, 0, SIDE);

    assert_eq!(
        editor
            .editing
            .as_ref()
            .map(|editing| editing.input.as_str()),
        Some("width"),
    );
}

#[test]
fn a_plain_value_is_opened_as_its_number() {
    let mut document = a_trait_as_long_as_a_width();
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: SIDE,
        value: 60.0.into(),
        placement: None,
    });
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    edit_dimension(&mut context, 0, SIDE);

    assert_eq!(
        editor
            .editing
            .as_ref()
            .map(|editing| editing.input.as_str()),
        Some("60"),
    );
}
