//! What app · screens/variables/state.rs is held to.
//!
//! Closes #175.
//! - a row is added, edited or deleted in the panel, and one asked as it was
//!   records nothing — `every_variable_is_a_row_with_its_formula_and_what_it_comes_to`,
//!   `a_row_edited_and_committed_changes_the_variable_and_what_follows_it`,
//!   `a_row_committed_as_it_was_records_nothing`
//! - the dimensions a refused change would break are named for the drawing to
//!   show — `a_change_that_would_break_a_dimension_names_it_for_the_drawing_to_show`

use cao_part::history::{Operation, PointRef};
use cao_part::{NameProblem, PartDocument, Refused, Unreadable, Unusable, Use};
use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};
use chrono::Utc;
use glam::DVec2;

use super::*;

const HEIGHT: DimensionTarget = DimensionTarget::Length(SegmentId(1));

fn added(panel: &mut VariablesPanel, document: &mut PartDocument, name: &str, formula: &str) {
    panel.adding.name = name.to_string();
    panel.adding.formula = formula.to_string();
    assert!(
        panel.add(document),
        "{name} = {formula} is added, not refused: {:?}",
        panel.problem()
    );
}

/// `height` at 40, and a rectangle whose right side is as high as it.
fn a_plate(panel: &mut VariablesPanel) -> PartDocument {
    let mut document = PartDocument::new("plate", Utc::now());
    added(panel, &mut document, "height", "40");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(100.0, 30.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0.into(),
        placement: None,
    });
    let height = document.variables().read("height").expect("it reads");
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: HEIGHT,
        value: height,
        placement: None,
    });
    document
}

fn row(panel: &VariablesPanel, document: &PartDocument, name: &str) -> Row {
    panel
        .rows(document.variables())
        .into_iter()
        .find(|row| row.name == name)
        .unwrap_or_else(|| panic!("a row named {name}"))
}

#[test]
fn every_variable_is_a_row_with_its_formula_and_what_it_comes_to() {
    let mut panel = VariablesPanel::default();
    let mut document = PartDocument::new("plate", Utc::now());
    added(&mut panel, &mut document, "width", "120");
    added(&mut panel, &mut document, "middle", "=width/2+5");

    let middle = row(&panel, &document, "middle");

    assert_eq!(middle.formula, "width / 2 + 5");
    assert_eq!(middle.value, Some(65.0));
    assert!(panel.adding.name.is_empty() && panel.adding.formula.is_empty());
}

#[test]
fn a_formula_that_does_not_read_adds_nothing_and_says_what_is_wrong() {
    let mut panel = VariablesPanel::default();
    let mut document = PartDocument::new("plate", Utc::now());
    panel.adding.name = "width".to_string();
    panel.adding.formula = "12 +".to_string();

    assert!(!panel.add(&mut document));

    assert_eq!(
        panel.problem(),
        Some(&Problem::Formula(Unusable::Unreadable(
            Unreadable::MissingValue(None)
        )))
    );
    assert!(document.variables().live().next().is_none());
    assert_eq!(
        panel.adding.formula, "12 +",
        "what was typed stays to be fixed"
    );
}

#[test]
fn a_name_that_does_not_do_is_refused_by_the_part() {
    let mut panel = VariablesPanel::default();
    let mut document = PartDocument::new("plate", Utc::now());
    panel.adding.name = "2nd".to_string();
    panel.adding.formula = "1".to_string();

    assert!(!panel.add(&mut document));

    assert_eq!(
        panel.problem(),
        Some(&Problem::Refused(Refused::Name(NameProblem::OpensOnADigit)))
    );
}

#[test]
fn a_row_edited_and_committed_changes_the_variable_and_what_follows_it() {
    let mut panel = VariablesPanel::default();
    let mut document = a_plate(&mut panel);
    let height = row(&panel, &document, "height");

    panel.typed_into(&height, "height".to_string(), "60".to_string());
    assert!(panel.commit(&mut document));

    assert_eq!(document.measured(0, HEIGHT).map(f64::round), Some(60.0));
    assert!(!panel.is_editing(height.variable));
}

#[test]
fn a_row_committed_as_it_was_records_nothing() {
    let mut panel = VariablesPanel::default();
    let mut document = a_plate(&mut panel);
    let height = row(&panel, &document, "height");
    let before = document.history.operations().len();

    panel.typed_into(&height, "height".to_string(), "40".to_string());

    assert!(!panel.commit(&mut document));
    assert_eq!(document.history.operations().len(), before);
}

#[test]
fn a_variable_in_use_is_kept_and_the_panel_says_what_uses_it() {
    let mut panel = VariablesPanel::default();
    let mut document = a_plate(&mut panel);
    let height = row(&panel, &document, "height");

    assert!(!panel.erase(&mut document, height.variable));

    assert_eq!(
        panel.problem(),
        Some(&Problem::Refused(Refused::InUse(vec![Use::Dimension {
            sketch: 0,
            target: HEIGHT
        }])))
    );
}

#[test]
fn a_change_that_would_break_a_dimension_names_it_for_the_drawing_to_show() {
    let mut panel = VariablesPanel::default();
    let mut document = a_plate(&mut panel);
    let height = row(&panel, &document, "height");

    panel.typed_into(&height, "height".to_string(), "-5".to_string());

    assert!(!panel.commit(&mut document));
    assert_eq!(panel.breaking(), vec![(0, HEIGHT)]);
    assert!(
        panel.is_editing(height.variable),
        "what was typed stays in the row to be fixed"
    );
}
