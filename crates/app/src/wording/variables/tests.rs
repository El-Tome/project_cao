//! What app · wording/variables.rs is held to.
//!
//! Closes #175.
//! - a refusal names the variables of a loop, what still uses a variable, and
//!   what a change would break —
//!   `a_loop_is_said_by_the_names_of_the_variables_in_it`,
//!   `what_still_uses_a_variable_is_listed_by_the_names_the_panels_use`,
//!   `what_a_change_would_break_is_said_by_the_value_on_the_drawing`,
//!   `a_change_that_would_renumber_a_step_or_set_a_sketch_adrift_says_so`

use cao_part::VariableChange;
use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_sketch::{SegmentId, WorkPlane};
use chrono::Utc;
use glam::DVec2;

use super::*;

/// `height` at 40, `double` at twice that, a rectangle whose right side is
/// as high as `height`, raised by `height / 10`.
fn a_plate() -> PartDocument {
    let mut document = PartDocument::new("plate", Utc::now());
    for (name, text) in [("height", "40"), ("double", "height * 2")] {
        let formula = document.variables().read(text).expect("it reads");
        document
            .change_variable(VariableChange::Added {
                name: name.to_string(),
                formula,
            })
            .expect("nothing to refuse");
    }
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
        target: DimensionTarget::Length(SegmentId(1)),
        value: height,
        placement: None,
    });
    let areas = document.areas_at(0, &[DVec2::new(50.0, 15.0)]);
    let thickness = document.variables().read("height / 10").expect("it reads");
    document.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: thickness,
        mode: ExtrusionMode::Add,
    });
    document
}

fn said(document: &PartDocument, refusal: &Refused) -> String {
    refused(&Catalogue::french(), document, refusal)
}

#[test]
fn what_still_uses_a_variable_is_listed_by_the_names_the_panels_use() {
    let document = a_plate();
    let uses = document.uses_of(VariableId(0));

    let sentence = said(&document, &Refused::InUse(uses));

    assert!(sentence.contains("double"), "{sentence}");
    assert!(sentence.contains("esquisse 1"), "{sentence}");
    assert!(
        sentence.contains("40"),
        "the value is named by what it measures: {sentence}"
    );
    let extrusion = document
        .history
        .operations()
        .iter()
        .position(|operation| matches!(operation, Operation::Extrude { .. }))
        .expect("an extrusion");
    assert!(
        sentence.contains(&(extrusion + 1).to_string()),
        "the step is found where the history panel shows it: {sentence}"
    );
}

#[test]
fn a_loop_is_said_by_the_names_of_the_variables_in_it() {
    let document = a_plate();

    let sentence = said(
        &document,
        &Refused::Loop(vec![VariableId(0), VariableId(1)]),
    );

    assert!(
        sentence.contains("height") && sentence.contains("double"),
        "{sentence}"
    );
}

#[test]
fn every_way_a_name_does_not_do_has_a_sentence_of_its_own() {
    let lang = Catalogue::french();

    let not_allowed = name(&lang, NameProblem::NotAllowed('-'));

    assert!(not_allowed.contains('-'), "{not_allowed}");
    for problem in [
        NameProblem::Empty,
        NameProblem::OpensOnADigit,
        NameProblem::Taken,
    ] {
        assert!(
            !name(&lang, problem).starts_with("variables."),
            "{problem:?}"
        );
    }
}

#[test]
fn what_a_change_would_break_is_said_by_the_value_on_the_drawing() {
    let document = a_plate();

    let sentence = said(
        &document,
        &Refused::Breaks(vec![Broken::Dimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(1)),
        }]),
    );

    assert!(sentence.contains("casserait"), "{sentence}");
    assert!(
        sentence.contains("esquisse 1") && sentence.contains("40"),
        "{sentence}"
    );
}

#[test]
fn a_change_that_would_renumber_a_step_or_set_a_sketch_adrift_says_so() {
    let document = a_plate();
    // Nothing was undone, so the numbers run from one in the order things
    // were done: the rectangle, fourth, is number 4, and the extrusion,
    // seventh, number 7.
    let renumbered = said(
        &document,
        &Refused::Breaks(vec![Broken::Renumbered {
            changed: 4,
            followed_by: 7,
        }]),
    );
    let adrift = said(&document, &Refused::Breaks(vec![Broken::Adrift(0)]));

    assert!(
        renumbered.contains("autre nombre d'éléments")
            && renumbered.contains("étape 4")
            && renumbered.contains("étape 7"),
        "{renumbered}"
    );
    assert!(
        adrift.contains("esquisse 1") && adrift.contains("face"),
        "{adrift}"
    );
}
