//! What app · input/line.rs is held to.
//!
//! Closes #175.
//! - a formula typed in the fields at the cursor is what the dimension they
//!   leave keeps — `a_length_typed_as_a_formula_is_what_its_dimension_keeps`
//!
//! Closes #537.
//! - a trait drawn within 4° of square to the one before carries
//!   « Perpendiculaire » to it, not an angle of 90° —
//!   `a_trait_drawn_nearly_square_to_the_one_before_carries_the_rule`
//! - with the option off, the line tool behaves as today —
//!   `with_the_option_off_a_trait_drawn_nearly_square_gets_an_angle_of_ninety`

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
    let aimed = Aim::at(DVec2::new(60.0, 0.0));

    dimension_the_line(&mut context, 0, SegmentId(0), aimed, 0.05);

    assert_eq!(
        document.formula_of(0, DimensionTarget::Length(SegmentId(0))),
        Some("width / 2".to_string()),
    );
}

/// Two traits chained from (2, 3), the second clicked 2° off square to the
/// first.
fn nearly_square(sketching: cao_prefs::SketchingConfig) -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let mut editor = SketchEditor {
        sketching,
        ..SketchEditor::default()
    };
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    for click in [
        DVec2::new(2.0, 3.0),
        DVec2::new(12.0, 3.0),
        DVec2::new(12.3, 11.0),
    ] {
        super::super::draw_line_point(&mut context, 0, click, 0.1, 0.05);
    }
    document
}

#[test]
fn a_trait_drawn_nearly_square_to_the_one_before_carries_the_rule() {
    let document = nearly_square(cao_prefs::SketchingConfig::default());

    let sketch = &document.sketches()[0];
    assert_eq!(
        sketch.constraints(),
        &[cao_sketch::Constraint::Perpendicular {
            first: SegmentId(0),
            second: SegmentId(1),
        }],
    );
    assert!(sketch.dimensions().is_empty(), "{:?}", sketch.dimensions());
    assert_eq!(document.history.operations().len(), 3, "one step a trait");
}

#[test]
fn with_the_option_off_a_trait_drawn_nearly_square_gets_an_angle_of_ninety() {
    let document = nearly_square(cao_prefs::SketchingConfig {
        exact_angles_lay_rules: false,
    });

    let sketch = &document.sketches()[0];
    assert!(
        sketch.constraints().is_empty(),
        "{:?}",
        sketch.constraints()
    );
    let corner = sketch
        .dimension_of(DimensionTarget::corner(SegmentId(0), SegmentId(1)))
        .expect("the right angle is a dimension");
    assert!((corner.value - 90.0).abs() < 1e-6, "{}", corner.value);
}
