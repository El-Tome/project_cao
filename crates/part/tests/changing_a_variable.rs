//! Changing the part's variables through the part: what is refused, and why,
//! and that nothing is applied when it is.
//!
//! Closes #175.
//! - a loop between variables is refused, naming the variables in it —
//!   `a_formula_that_would_lean_on_itself_is_refused_naming_the_loop`
//! - renaming a variable renames it in every formula that uses it —
//!   `renaming_a_variable_renames_it_in_what_was_written_from_it`
//! - a variable change that would break a size — a length at zero or below,
//!   a count no longer whole and positive, a dimension the drawing can no
//!   longer hold — is refused naming it —
//!   `a_change_that_would_leave_a_length_at_zero_or_below_is_refused_naming_it`,
//!   `a_change_that_would_leave_a_count_no_longer_whole_is_refused_naming_the_pattern`,
//!   `a_change_the_drawing_could_not_hold_is_refused_naming_the_value`,
//!   `a_change_that_would_leave_a_step_with_no_size_is_refused_naming_it`,
//!   `a_variable_that_would_come_to_no_number_is_refused`
//! - deleting a variable something uses is refused, with the list of what
//!   uses it — the dimensions, the features, the other variables —
//!   `a_variable_in_use_cannot_be_erased_and_says_what_uses_it`,
//!   `a_step_written_from_a_variable_is_named_as_using_it`,
//!   `a_value_laid_inside_a_gesture_is_named_as_the_value_and_not_as_the_gesture`
//! - changing a variable is one step of the history —
//!   `a_variable_added_through_the_part_is_one_step_undo_takes_back`
//! - compacting the history keeps the variables and the formulas that refer
//!   to them — `compacting_keeps_the_variables_and_what_is_written_from_them`,
//!   `compacting_leaves_an_erased_variable_out_and_the_others_still_answer`

use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_part::{
    Broken, Formula, NameProblem, PartDocument, Refused, Use, VariableChange, VariableId,
};
use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};
use glam::DVec2;

fn at_nine() -> chrono::DateTime<chrono::Utc> {
    "2026-01-02T09:00:00Z".parse().expect("a date")
}

fn written(document: &PartDocument, text: &str) -> Formula {
    document
        .variables()
        .read(text)
        .expect("a formula that reads")
}

fn added(name: &str, formula: Formula) -> VariableChange {
    VariableChange::Added {
        name: name.to_string(),
        formula,
    }
}

fn edited(variable: usize, name: &str, formula: Formula) -> VariableChange {
    VariableChange::Edited {
        variable: VariableId(variable),
        name: name.to_string(),
        formula,
    }
}

const HEIGHT: DimensionTarget = DimensionTarget::Length(SegmentId(1));

/// `height` at 40 and `thickness` at 4, a rectangle whose right side is as
/// high as `height`, raised by `thickness`.
fn plate() -> PartDocument {
    let mut document = PartDocument::new("Plate", at_nine());
    document
        .change_variable(added("height", Formula::Number(40.0)))
        .expect("a first variable");
    document
        .change_variable(added("thickness", Formula::Number(4.0)))
        .expect("a second one");
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
    let height = written(&document, "height");
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: HEIGHT,
        value: height,
        placement: None,
    });
    let areas = document.areas_at(0, &[DVec2::new(50.0, 15.0)]);
    let thickness = written(&document, "thickness");
    document.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: thickness,
        mode: ExtrusionMode::Add,
    });
    document
}

#[test]
fn a_variable_added_through_the_part_is_one_step_undo_takes_back() {
    let mut document = PartDocument::new("Plate", at_nine());

    document
        .change_variable(added("width", Formula::Number(120.0)))
        .expect("nothing to refuse");
    assert_eq!(document.variables().named("width"), Some(VariableId(0)));
    document.undo();

    assert_eq!(document.variables().named("width"), None);
}

#[test]
fn a_name_that_does_not_do_is_refused_and_nothing_is_applied() {
    let mut document = plate();
    let before = document.history.operations().len();

    let refused = document.change_variable(added("height", Formula::Number(1.0)));

    assert_eq!(refused, Err(Refused::Name(NameProblem::Taken)));
    assert_eq!(document.history.operations().len(), before);
}

#[test]
fn a_formula_that_would_lean_on_itself_is_refused_naming_the_loop() {
    let mut document = plate();
    let from_height = written(&document, "height / 10");
    document
        .change_variable(edited(1, "thickness", from_height))
        .expect("thickness may lean on height");
    let from_thickness = written(&document, "thickness * 10");

    let refused = document.change_variable(edited(0, "height", from_thickness));

    assert_eq!(
        refused,
        Err(Refused::Loop(vec![VariableId(0), VariableId(1)]))
    );
}

#[test]
fn a_variable_in_use_cannot_be_erased_and_says_what_uses_it() {
    let mut document = plate();
    let from_height = written(&document, "height * 2");
    document
        .change_variable(added("double", from_height))
        .expect("a variable written from another");

    let refused = document.change_variable(VariableChange::Erased {
        variable: VariableId(0),
    });

    let Err(Refused::InUse(uses)) = refused else {
        panic!("erasing a variable in use is refused, not {refused:?}");
    };
    assert!(
        uses.contains(&Use::Dimension {
            sketch: 0,
            target: HEIGHT
        }),
        "{uses:?}"
    );
    assert!(uses.contains(&Use::Variable(VariableId(2))), "{uses:?}");
    assert!(document.variables().named("height").is_some());
}

#[test]
fn a_step_written_from_a_variable_is_named_as_using_it() {
    let mut document = plate();

    let refused = document.change_variable(VariableChange::Erased {
        variable: VariableId(1),
    });

    let Err(Refused::InUse(uses)) = refused else {
        panic!("the extrusion uses thickness, so it stays: {refused:?}");
    };
    assert!(
        matches!(uses.as_slice(), [Use::Operation(_)]),
        "the extrusion, and nothing else: {uses:?}"
    );
}

#[test]
fn a_variable_nothing_uses_is_erased() {
    let mut document = plate();
    document
        .change_variable(added("margin", Formula::Number(5.0)))
        .expect("a variable nothing uses yet");

    document
        .change_variable(VariableChange::Erased {
            variable: VariableId(2),
        })
        .expect("nothing uses it");

    assert_eq!(document.variables().named("margin"), None);
}

#[test]
fn renaming_a_variable_renames_it_in_what_was_written_from_it() {
    let mut document = plate();

    document
        .change_variable(edited(0, "h", Formula::Number(40.0)))
        .expect("a new name for the same variable");

    assert_eq!(document.formula_of(0, HEIGHT), Some("h".to_string()));
}

#[test]
fn a_change_that_would_leave_a_length_at_zero_or_below_is_refused_naming_it() {
    let mut document = plate();

    let refused = document.change_variable(edited(0, "height", Formula::Number(-5.0)));

    assert_eq!(
        refused,
        Err(Refused::Breaks(vec![Broken::Dimension {
            sketch: 0,
            target: HEIGHT
        }]))
    );
    assert_eq!(
        document.measured(0, HEIGHT).map(f64::round),
        Some(40.0),
        "nothing moved"
    );
}

#[test]
fn a_change_that_would_leave_a_step_with_no_size_is_refused_naming_it() {
    let mut document = plate();

    let refused = document.change_variable(edited(1, "thickness", Formula::Number(0.0)));

    assert!(
        matches!(&refused, Err(Refused::Breaks(broken)) if matches!(broken.as_slice(), [Broken::Operation(_)])),
        "{refused:?}"
    );
}

#[test]
fn a_variable_that_would_come_to_no_number_is_refused() {
    let mut document = plate();
    let divided = written(&document, "10 / (height - 40)");

    let refused = document.change_variable(added("endless", divided));

    assert_eq!(
        refused,
        Err(Refused::Breaks(vec![Broken::Variable(VariableId(2))]))
    );
}

#[test]
fn a_value_laid_inside_a_gesture_is_named_as_the_value_and_not_as_the_gesture() {
    let mut document = plate();
    let from_height = written(&document, "height / 2");
    document.apply(Operation::Gesture(vec![
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(10.0, 50.0)),
            end: PointRef::New(DVec2::new(30.0, 50.0)),
            construction: false,
        },
        Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(4)),
            value: from_height,
            placement: None,
        },
    ]));

    let uses = document.uses_of(VariableId(0));

    assert!(
        !uses.iter().any(|used| matches!(used, Use::Operation(_))),
        "{uses:?}"
    );
    assert!(uses.contains(&Use::Dimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(4))
    }));
}

fn thickness(document: &PartDocument) -> f64 {
    let (min, max) = document.body().bounds().expect("some matter");
    max.z - min.z
}

#[test]
fn compacting_keeps_the_variables_and_what_is_written_from_them() {
    let mut document = plate();

    document.compact_history();
    document
        .change_variable(edited(0, "height", Formula::Number(60.0)))
        .expect("the variable is still there to change");
    document
        .change_variable(edited(1, "thickness", Formula::Number(10.0)))
        .expect("and so is the other");

    assert_eq!(document.formula_of(0, HEIGHT), Some("height".to_string()));
    assert_eq!(document.measured(0, HEIGHT).map(f64::round), Some(60.0));
    assert!((thickness(&document) - 10.0).abs() < 1e-6);
}

#[test]
fn compacting_leaves_an_erased_variable_out_and_the_others_still_answer() {
    let mut document = PartDocument::new("Plate", at_nine());
    document
        .change_variable(added("old", Formula::Number(1.0)))
        .expect("a variable soon erased");
    document
        .change_variable(VariableChange::Erased {
            variable: VariableId(0),
        })
        .expect("nothing uses it");
    document
        .change_variable(added("height", Formula::Number(40.0)))
        .expect("the one that stays");
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
    let height = written(&document, "height");
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: HEIGHT,
        value: height,
        placement: None,
    });

    document.compact_history();

    assert_eq!(document.variables().live().count(), 1);
    assert_eq!(document.formula_of(0, HEIGHT), Some("height".to_string()));
    let height = document.variables().named("height").expect("still there");
    document
        .change_variable(VariableChange::Edited {
            variable: height,
            name: "height".to_string(),
            formula: Formula::Number(55.0),
        })
        .expect("it still answers");
    assert_eq!(document.measured(0, HEIGHT).map(f64::round), Some(55.0));
}

#[test]
fn a_change_that_would_leave_a_count_no_longer_whole_is_refused_naming_the_pattern() {
    let mut document = plate();
    document
        .change_variable(added("holes", Formula::Number(8.0)))
        .expect("a count to write a pattern from");
    let count = written(&document, "holes / 2");
    document.apply(Operation::CircularPattern {
        sketch: 0,
        elements: vec![cao_sketch::Element::Segment(SegmentId(0))],
        centre: cao_sketch::Sketch::ORIGIN,
        degrees: 90.0.into(),
        count,
    });

    let refused = document.change_variable(edited(2, "holes", Formula::Number(5.0)));

    assert!(
        matches!(&refused, Err(Refused::Breaks(broken)) if matches!(broken.as_slice(), [Broken::Operation(_)])),
        "four copies would become two and a half: {refused:?}"
    );
}

#[test]
fn a_change_the_drawing_could_not_hold_is_refused_naming_the_value() {
    let mut document = PartDocument::new("Arc", at_nine());
    document
        .change_variable(added("radius", Formula::Number(50.0)))
        .expect("a radius");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(40.0, 30.0)),
        start: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
        end: PointRef::New(DVec2::new(80.0, 0.0)),
        construction: false,
    });
    let chord = DimensionTarget::Distance {
        from: cao_sketch::Sketch::ORIGIN,
        to: cao_sketch::PointId(2),
    };
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: chord,
        value: 80.0.into(),
        placement: None,
    });
    let radius = DimensionTarget::ArcRadius(cao_sketch::ArcId(0));
    let written_radius = written(&document, "radius");
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: radius,
        value: written_radius,
        placement: None,
    });

    // No arc through both ends of an 80 mm chord is smaller than a 40 mm
    // half circle.
    let refused = document.change_variable(edited(0, "radius", Formula::Number(5.0)));

    assert_eq!(
        refused,
        Err(Refused::Breaks(vec![Broken::Dimension {
            sketch: 0,
            target: radius
        }]))
    );
}
