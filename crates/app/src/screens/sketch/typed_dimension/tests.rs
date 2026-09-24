//! What app · screens/sketch/typed_dimension.rs is held to.
//!
//! Closes #175.
//! - a formula can be typed as the value of a dimension placed or edited —
//!   `a_formula_typed_into_a_dimension_is_what_the_dimension_follows`,
//!   `the_same_number_written_from_a_variable_is_still_a_step`
//! - one that does not read is refused there, saying why —
//!   `a_formula_that_does_not_read_is_refused_with_what_is_wrong_with_it`

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
        value: 80.0.into(),
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

#[test]
fn an_angle_between_two_traits_typed_at_a_half_turn_is_refused() {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (from, to) in [
        (DVec2::new(10.0, 3.0), DVec2::new(30.0, 10.0)),
        (DVec2::new(10.0, 20.0), DVec2::new(30.0, 30.0)),
    ] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(from),
            end: PointRef::New(to),
            construction: false,
        });
    }
    let target = DimensionTarget::AngleBetween {
        first: cao_sketch::SegmentId(0),
        first_toward: cao_sketch::Toward::End,
        second: cao_sketch::SegmentId(1),
        second_toward: cao_sketch::Toward::End,
    };
    document.apply(Operation::SetDimension {
        sketch: 0,
        target,
        value: 10.0.into(),
        placement: None,
    });

    let mut editor = SketchEditor {
        editing: Some(DimensionEdit {
            target,
            input: "180".to_string(),
            focus: false,
        }),
        ..SketchEditor::default()
    };
    let lang = Catalogue::french();

    let applied = apply_dimension_value(&mut document, &mut editor, 0, target, &lang);

    assert!(!applied, "refused: the field stays open for another try");
    assert_eq!(
        editor.message.as_deref(),
        Some(lang.t("sketch.angle_would_lay_parallel").as_str()),
        "and the reason is given",
    );
    assert!(
        (document.sketches()[0]
            .dimension_of(target)
            .expect("still there")
            .value
            - 10.0)
            .abs()
            < 1e-9,
        "the angle already laid keeps the value it had",
    );
}

/// `width` at 60, and a trait 40 long with a plain length on it.
fn a_trait_and_a_width() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document
        .change_variable(cao_part::VariableChange::Added {
            name: "width".to_string(),
            formula: cao_part::Formula::Number(60.0),
        })
        .expect("a variable");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::ZERO),
        end: PointRef::New(DVec2::new(40.0, 0.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(cao_sketch::SegmentId(0)),
        value: 40.0.into(),
        placement: None,
    });
    document
}

fn typed_into(target: DimensionTarget, text: &str) -> SketchEditor {
    SketchEditor {
        editing: Some(DimensionEdit {
            target,
            input: text.to_string(),
            focus: false,
        }),
        ..SketchEditor::default()
    }
}

#[test]
fn a_formula_typed_into_a_dimension_is_what_the_dimension_follows() {
    let mut document = a_trait_and_a_width();
    let side = DimensionTarget::Length(cao_sketch::SegmentId(0));
    let mut editor = typed_into(side, "width / 2");

    let applied = apply_dimension_value(&mut document, &mut editor, 0, side, &Catalogue::french());

    assert!(applied);
    assert_eq!(document.formula_of(0, side), Some("width / 2".to_string()));
    assert_eq!(document.measured(0, side).map(f64::round), Some(30.0));
}

#[test]
fn a_formula_that_does_not_read_is_refused_with_what_is_wrong_with_it() {
    let mut document = a_trait_and_a_width();
    let side = DimensionTarget::Length(cao_sketch::SegmentId(0));
    let mut editor = typed_into(side, "width /");
    let before = document.history.operations().len();
    let lang = Catalogue::french();

    let applied = apply_dimension_value(&mut document, &mut editor, 0, side, &lang);

    assert!(!applied);
    assert_eq!(
        editor.message,
        Some(crate::wording::formula::unusable(
            &lang,
            &cao_part::Unusable::Unreadable(cao_part::Unreadable::MissingValue(None))
        )),
    );
    assert_eq!(
        document.history.operations().len(),
        before,
        "nothing is applied"
    );
}

#[test]
fn the_same_number_written_from_a_variable_is_still_a_step() {
    let mut document = a_trait_and_a_width();
    let side = DimensionTarget::Length(cao_sketch::SegmentId(0));
    let mut editor = typed_into(side, "width - 20");

    let applied = apply_dimension_value(&mut document, &mut editor, 0, side, &Catalogue::french());

    assert!(
        applied,
        "40 written from width follows width, where 40 did not"
    );
    assert_eq!(document.formula_of(0, side), Some("width - 20".to_string()));
}
