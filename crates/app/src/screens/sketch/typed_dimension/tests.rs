//! What app · screens/sketch/typed_dimension.rs is held to.
//!
//! Closes #175.
//! - a formula can be typed as the value of a dimension placed or edited —
//!   `a_formula_typed_into_a_dimension_is_what_the_dimension_follows`,
//!   `the_same_number_written_from_a_variable_is_still_a_step`
//! - one that does not read is refused there, saying why —
//!   `a_formula_that_does_not_read_is_refused_with_what_is_wrong_with_it`
//!
//! Closes #537.
//! - against an axis, 0° and 180° typed lay « Parallèle » to it and 90° lays
//!   « Perpendiculaire », the dimension gone; one undo brings the dimension
//!   back — `zero_ninety_and_a_half_turn_typed_against_an_axis_lay_its_rule`
//! - the same holds between two traits at a corner, in a T, crossing and
//!   apart, undo included — `zero_ninety_and_a_half_turn_typed_between_two_traits_lay_their_rule`
//! - 0.01° stays a dimension, and so does « pente » worth 0 against an axis —
//!   `a_hair_off_or_a_variable_worth_zero_stays_a_dimension`
//! - with the option off, the dimension field behaves as today —
//!   `an_angle_between_two_traits_typed_at_a_half_turn_is_refused`,
//!   `with_the_option_off_an_exact_angle_against_an_axis_is_a_dimension`

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
        placed_the_scale: false,
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
            placed_the_scale: false,
        }),
        sketching: dimensions_at_exact_angles(),
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
            placed_the_scale: false,
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

fn dimensions_at_exact_angles() -> cao_prefs::SketchingConfig {
    cao_prefs::SketchingConfig {
        exact_angles_lay_rules: false,
    }
}

/// An empty drawing, ready for traits.
fn a_sketch() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document
}

fn a_trait(document: &mut PartDocument, from: DVec2, to: DVec2, construction: bool) {
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(from),
        end: PointRef::New(to),
        construction,
    });
}

/// A trait leaning at 30° from (5, 5), its angle to the horizontal laid.
fn a_trait_at_thirty_to_the_axis() -> (PartDocument, DimensionTarget) {
    let mut document = a_sketch();
    let from = DVec2::new(5.0, 5.0);
    a_trait(
        &mut document,
        from,
        from + DVec2::from_angle(30.0_f64.to_radians()) * 20.0,
        false,
    );
    let target = DimensionTarget::AxisAngle {
        segment: cao_sketch::SegmentId(0),
        segment_toward: cao_sketch::Toward::End,
        axis: cao_sketch::SketchAxis::U,
        axis_toward: cao_sketch::AxisToward::Positive,
    };
    document.apply(Operation::SetDimension {
        sketch: 0,
        target,
        value: 30.0.into(),
        placement: None,
    });
    (document, target)
}

fn type_into(document: &mut PartDocument, target: DimensionTarget, text: &str) -> bool {
    let mut editor = typed_into(target, text);
    apply_dimension_value(document, &mut editor, 0, target, &Catalogue::french())
}

#[test]
fn zero_ninety_and_a_half_turn_typed_against_an_axis_lay_its_rule() {
    use cao_sketch::{Constraint, SegmentId, SketchAxis};
    for (typed, rule) in [
        (
            "0",
            Constraint::AxisParallel {
                segment: SegmentId(0),
                axis: SketchAxis::U,
            },
        ),
        (
            "180",
            Constraint::AxisParallel {
                segment: SegmentId(0),
                axis: SketchAxis::U,
            },
        ),
        (
            "90",
            Constraint::AxisPerpendicular {
                segment: SegmentId(0),
                axis: SketchAxis::U,
            },
        ),
    ] {
        let (mut document, target) = a_trait_at_thirty_to_the_axis();
        let steps = document.history.applied();

        assert!(type_into(&mut document, target, typed), "{typed} is laid");

        let sketch = &document.sketches()[0];
        assert_eq!(sketch.constraints(), &[rule], "{typed} typed");
        assert!(
            sketch.dimension_of(target).is_none(),
            "{typed} typed, no dimension reads it"
        );
        assert_eq!(document.history.applied(), steps + 1, "one step");

        document.undo();
        let sketch = &document.sketches()[0];
        assert!(sketch.constraints().is_empty());
        assert!(
            sketch
                .dimension_of(target)
                .is_some_and(|it| (it.value - 30.0).abs() < 1e-9),
            "one undo after {typed}, and the dimension is back as it was",
        );
    }
}

#[test]
fn zero_ninety_and_a_half_turn_typed_between_two_traits_lay_their_rule() {
    use cao_sketch::{Constraint, SegmentId, Toward};
    let first = SegmentId(0);
    let second = SegmentId(1);
    let corner = (
        [DVec2::ZERO, DVec2::new(20.0, 0.0)],
        [DVec2::ZERO, DVec2::new(10.0, 10.0)],
    );
    let crossing = (
        [DVec2::new(-10.0, 0.0), DVec2::new(10.0, 0.0)],
        [DVec2::new(-5.0, -5.0), DVec2::new(5.0, 5.0)],
    );
    let a_t = (
        [DVec2::new(-10.0, 0.0), DVec2::new(10.0, 0.0)],
        [DVec2::new(0.0, 0.0), DVec2::new(5.0, 5.0)],
    );
    let apart = (
        [DVec2::new(10.0, 3.0), DVec2::new(30.0, 10.0)],
        [DVec2::new(10.0, 20.0), DVec2::new(30.0, 30.0)],
    );
    let between = DimensionTarget::AngleBetween {
        first,
        first_toward: Toward::End,
        second,
        second_toward: Toward::End,
    };
    for (name, traits, target, start) in [
        (
            "a corner",
            corner,
            DimensionTarget::corner(first, second),
            45.0,
        ),
        ("a crossing", crossing, between, 45.0),
        ("a T", a_t, between, 45.0),
        ("traits apart", apart, between, 10.0),
    ] {
        for (typed, rule) in [
            ("0", Constraint::Parallel { first, second }),
            ("180", Constraint::Parallel { first, second }),
            ("90", Constraint::Perpendicular { first, second }),
        ] {
            let mut document = a_sketch();
            for [from, to] in [traits.0, traits.1] {
                a_trait(&mut document, from, to, false);
            }
            if name == "a corner" {
                document.apply(Operation::MergePoints {
                    sketch: 0,
                    kept: cao_sketch::PointId(1),
                    dropped: cao_sketch::PointId(3),
                });
            }
            if name == "a T" {
                document.apply(Operation::Constrain {
                    sketch: 0,
                    constraint: Constraint::OnSegment {
                        point: cao_sketch::PointId(3),
                        segment: first,
                        from: Default::default(),
                    },
                });
            }
            document.apply(Operation::SetDimension {
                sketch: 0,
                target,
                value: start.into(),
                placement: None,
            });
            assert!(
                document.sketches()[0].dimension_of(target).is_some(),
                "{name}: the angle is laid to begin with",
            );
            let steps = document.history.applied();

            assert!(type_into(&mut document, target, typed), "{name}, {typed}");

            let sketch = &document.sketches()[0];
            assert!(
                sketch.constraints().contains(&rule),
                "{name}, {typed} typed: {:?}",
                sketch.constraints(),
            );
            assert!(sketch.dimension_of(target).is_none(), "{name}, {typed}");
            assert_eq!(document.history.applied(), steps + 1, "{name}, {typed}");

            document.undo();
            let sketch = &document.sketches()[0];
            assert!(!sketch.constraints().contains(&rule), "{name}, {typed}");
            assert!(
                sketch
                    .dimension_of(target)
                    .is_some_and(|it| (it.value - start).abs() < 1e-6),
                "{name}, {typed}: one undo and the dimension is back",
            );
        }
    }
}

#[test]
fn a_hair_off_or_a_variable_worth_zero_stays_a_dimension() {
    let (mut document, target) = a_trait_at_thirty_to_the_axis();
    document
        .change_variable(cao_part::VariableChange::Added {
            name: "pente".to_string(),
            formula: cao_part::Formula::Number(0.0),
        })
        .expect("a variable");

    for typed in ["0.01", "pente", "0*2"] {
        assert!(type_into(&mut document, target, typed), "{typed} is laid");
        let sketch = &document.sketches()[0];
        assert!(sketch.constraints().is_empty(), "{typed}: no rule");
        assert!(
            sketch.dimension_of(target).is_some(),
            "{typed}: a dimension"
        );
    }
}

#[test]
fn with_the_option_off_an_exact_angle_against_an_axis_is_a_dimension() {
    let (mut document, target) = a_trait_at_thirty_to_the_axis();
    let mut editor = SketchEditor {
        sketching: dimensions_at_exact_angles(),
        ..typed_into(target, "0")
    };

    assert!(apply_dimension_value(
        &mut document,
        &mut editor,
        0,
        target,
        &Catalogue::french()
    ));

    let sketch = &document.sketches()[0];
    assert!(sketch.constraints().is_empty());
    assert!(
        sketch
            .dimension_of(target)
            .is_some_and(|it| it.value.abs() < 1e-9),
        "the dimension reads 0, as today",
    );
}
