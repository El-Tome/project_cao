//! What app · input/measure.rs is held to.
//!
//! Closes #175.
//! - a dimension on the drawing shows its value; editing it shows its formula
//!   — `a_value_written_from_a_variable_is_opened_as_what_it_was_written_as`,
//!   `a_plain_value_is_opened_as_its_number`
//!
//! Closes #423.
//! - with the dimension tool, a value typed right after placing the part's
//!   first dimension sets the scale, and nothing moves —
//!   `a_value_typed_right_after_the_first_dimension_is_placed_sets_the_scale`
//! - a dimension placed and left as it reads is the first value itself: one
//!   unit is one millimetre, and a value typed into it later moves the drawing
//!   — `a_first_dimension_left_as_it_reads_is_the_first_value_itself`
//! - every later value is still a constraint —
//!   `a_value_typed_right_after_a_later_dimension_is_placed_moves_the_drawing`
//! - replaying the history rebuilds the same part, scale included —
//!   `replaying_the_history_rebuilds_the_first_dimension_and_its_scale`

use cao_part::history::PointRef;
use cao_part::{Formula, Operation, PartDocument, VariableChange};
use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};
use chrono::Utc;
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, apply_dimension_value};

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

/// A part with a trait 24 units long on it, and no value given yet.
fn a_trait_and_no_scale() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 10.0)),
        end: PointRef::New(DVec2::new(34.0, 10.0)),
        construction: false,
    });
    document
}

fn ends_of_the_trait(document: &PartDocument) -> [DVec2; 2] {
    let drawing = &document.sketches()[0];
    let side = drawing.segments()[0];
    [drawing.point(side.start), drawing.point(side.end)]
}

fn dimension_steps(document: &PartDocument) -> usize {
    document
        .history
        .applied_operations()
        .iter()
        .filter(|operation| matches!(operation, Operation::SetDimension { .. }))
        .count()
}

#[test]
fn a_value_typed_right_after_the_first_dimension_is_placed_sets_the_scale() {
    let mut document = a_trait_and_no_scale();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    place_dimension(&mut context, 0, SIDE, DVec2::new(22.0, 20.0), 0.05);
    let placed = context.document.sketches()[0]
        .dimension_of(SIDE)
        .and_then(|dimension| dimension.offset);

    if let Some(editing) = editor.editing.as_mut() {
        editing.input = "100".to_string();
    }
    assert!(apply_dimension_value(
        &mut document,
        &mut editor,
        0,
        SIDE,
        &lang
    ));

    assert!(
        (document.scale() - 100.0 / 24.0).abs() < 1e-9,
        "the trait measured 24 units when 100 was typed for it, and a unit is worth {} mm",
        document.scale(),
    );
    assert_eq!(
        ends_of_the_trait(&document),
        [DVec2::new(10.0, 10.0), DVec2::new(34.0, 10.0)],
        "nothing moves",
    );
    assert_eq!(
        dimension_steps(&document),
        1,
        "the value typed takes the place of the one the dimension was placed with",
    );
    assert_eq!(
        document.sketches()[0]
            .dimension_of(SIDE)
            .and_then(|dimension| dimension.offset),
        placed,
        "the dimension stays where it was put down",
    );
}

#[test]
fn a_first_dimension_left_as_it_reads_is_the_first_value_itself() {
    let mut document = a_trait_and_no_scale();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    place_dimension(&mut context, 0, SIDE, DVec2::new(22.0, 20.0), 0.05);
    context.editor.reset_pending();
    edit_dimension(&mut context, 0, SIDE);

    if let Some(editing) = editor.editing.as_mut() {
        editing.input = "100".to_string();
    }
    assert!(apply_dimension_value(
        &mut document,
        &mut editor,
        0,
        SIDE,
        &lang
    ));

    assert!(
        (document.scale() - 1.0).abs() < 1e-12,
        "the dimension left as it read said a unit is a millimetre, and a unit is worth {} mm",
        document.scale(),
    );
    let length = document.sketches()[0].segment_length(cao_sketch::SegmentId(0));
    assert!(
        (length - 100.0).abs() < 1e-6,
        "the trait is drawn to the 100 typed later, and runs {length}",
    );
}

#[test]
fn a_value_typed_right_after_a_later_dimension_is_placed_moves_the_drawing() {
    let mut document = a_trait_and_no_scale();
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: SIDE,
        value: 100.0.into(),
        placement: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 30.0)),
        end: PointRef::New(DVec2::new(22.0, 30.0)),
        construction: false,
    });
    let other = DimensionTarget::Length(cao_sketch::SegmentId(1));
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    place_dimension(&mut context, 0, other, DVec2::new(16.0, 40.0), 0.05);

    if let Some(editing) = editor.editing.as_mut() {
        editing.input = "100".to_string();
    }
    assert!(apply_dimension_value(
        &mut document,
        &mut editor,
        0,
        other,
        &lang
    ));

    assert!(
        (document.scale() - 100.0 / 24.0).abs() < 1e-9,
        "the scale the first value gave stands, and a unit is worth {} mm",
        document.scale(),
    );
    let length =
        document.to_millimeters(document.sketches()[0].segment_length(cao_sketch::SegmentId(1)));
    assert!(
        (length - 100.0).abs() < 1e-3,
        "the second trait gives way to the 100 mm typed for it, and measures {length} mm",
    );
    assert_eq!(
        dimension_steps(&document),
        3,
        "each value is a step of its own"
    );
}

#[test]
fn replaying_the_history_rebuilds_the_first_dimension_and_its_scale() {
    let mut document = a_trait_and_no_scale();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    place_dimension(&mut context, 0, SIDE, DVec2::new(22.0, 20.0), 0.05);
    if let Some(editing) = editor.editing.as_mut() {
        editing.input = "100".to_string();
    }
    apply_dimension_value(&mut document, &mut editor, 0, SIDE, &lang);

    let replayed = cao_part::PartState::rebuild(&document.history);

    assert_eq!(replayed.millimeters_per_unit, Some(document.scale()));
    assert_eq!(
        replayed.sketches[0].points(),
        document.sketches()[0].points()
    );
    let said = |state: Option<&cao_sketch::Dimension>| {
        state.map(|dimension| (dimension.value, dimension.offset))
    };
    assert_eq!(
        said(replayed.sketches[0].dimension_of(SIDE)),
        said(document.sketches()[0].dimension_of(SIDE)),
    );
}
