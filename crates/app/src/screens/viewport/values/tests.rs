//! What app · viewport/values.rs is held to.
//!
//! Closes #175.
//! - a formula typed at the cursor that does not read is refused where it is
//!   typed, with a message saying what is wrong, and nothing is laid —
//!   `a_field_at_the_cursor_that_does_not_read_refuses_the_click_and_says_why`,
//!   `fields_holding_what_reads_let_the_click_through`

use cao_part::{VariableChange, Variables};

use super::*;

fn half_of_a_width() -> Formula {
    let mut variables = Variables::default();
    variables.change(&VariableChange::Added {
        name: "largeur".to_string(),
        formula: Formula::Number(120.0),
    });
    variables.read("largeur / 2").expect("it reads")
}

#[test]
fn a_value_read_back_as_the_formula_typed_for_it_keeps_the_formula() {
    let half = half_of_a_width();

    assert_eq!(as_typed(Some((half.clone(), 60.0)), 60.000_000_001), half);
}

#[test]
fn a_value_the_drawing_reads_otherwise_is_the_number_it_reads() {
    let half = half_of_a_width();

    assert_eq!(as_typed(Some((half, 60.0)), 120.0), Formula::Number(120.0));
    assert_eq!(as_typed(None, 42.0), Formula::Number(42.0));
}

#[test]
fn a_plain_number_typed_is_left_as_the_drawing_reads_it() {
    assert_eq!(
        as_typed(Some((Formula::Number(40.0), 40.0)), 40.000_000_001),
        Formula::Number(40.000_000_001),
    );
}

#[test]
fn a_field_at_the_cursor_that_does_not_read_refuses_the_click_and_says_why() {
    let mut document = cao_part::PartDocument::new("part", chrono::Utc::now());
    let mut editor = crate::screens::sketch::SketchEditor::default();
    editor.live.open();
    let field = editor.live.field(0);
    field.text = "=depth".to_string();
    field.take(document.variables());
    let mut extrusion = crate::screens::extrusion::ExtrusionState::default();
    let lang = crate::lang::Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    assert!(refused_for_what_is_typed(&mut context));
    assert_eq!(
        editor.message,
        Some(crate::wording::formula::unusable(
            &lang,
            &cao_part::Unusable::Unreadable(cao_part::Unreadable::UnknownName("depth".to_string()))
        )),
    );
}

#[test]
fn fields_holding_what_reads_let_the_click_through() {
    let mut document = cao_part::PartDocument::new("part", chrono::Utc::now());
    let mut editor = crate::screens::sketch::SketchEditor::default();
    editor.live.open();
    let field = editor.live.field(0);
    field.text = "40".to_string();
    field.take(document.variables());
    let mut extrusion = crate::screens::extrusion::ExtrusionState::default();
    let lang = crate::lang::Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    assert!(!refused_for_what_is_typed(&mut context));
}
