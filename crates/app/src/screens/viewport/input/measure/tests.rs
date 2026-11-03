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
//!
//! Closes #460.
//! - with the smart dimension tool, a trait carrying its length then a second
//!   trait gives the angle between them, waiting to be placed —
//!   `a_trait_carrying_its_length_then_a_second_trait_gives_the_angle`
//! - the same click still opens the length for retyping, and a value typed
//!   then retypes it — `a_trait_carrying_its_length_still_opens_it_for_retyping`
//!
//! Decided on the way: a value typed ends the gesture, so the next trait
//! clicked gets its own length —
//! `a_trait_clicked_after_a_length_was_retyped_gets_its_own_length`.
//!
//! Closes #484.
//! - a trait dimensioned against an axis measures the quarter the click puts
//!   it down in, around the middle of the trait —
//!   `a_trait_then_an_axis_then_a_place_lays_the_angle_of_that_quarter`
//!
//! Closes #485.
//! - a hand wavering on the line between two quarters keeps the one shown, and
//!   the click lays it — `a_hand_wavering_on_the_line_keeps_the_quarter_it_shows`
//!
//! Closes #567.
//! - An X carrying a driving 60°: put down in the 120° quarter, a read-only
//!   120° is laid beside it, and the 60° stays the only value driving the
//!   traits — `an_x_put_down_in_another_quarter_lays_a_read_only_angle_beside_the_driving_one`
//! - Put down again in the 60° quarter, the 60° opens, as today —
//!   `an_x_put_down_again_in_the_driving_quarter_opens_the_driving_angle`
//! - The same at a corner, a T, two traits apart, and a trait and an axis —
//!   `a_corner_put_down_in_another_quarter_lays_a_read_only_angle`,
//!   `a_corner_put_down_in_the_driving_quarter_opens_the_driving_angle`,
//!   `a_t_put_down_in_another_quarter_lays_a_read_only_angle`,
//!   `a_t_put_down_in_the_driving_quarter_opens_the_driving_angle`,
//!   `two_traits_apart_put_down_in_another_quarter_lay_a_read_only_angle`,
//!   `two_traits_apart_put_down_in_the_driving_quarter_open_the_driving_angle`,
//!   `a_trait_and_an_axis_put_down_in_another_quarter_lay_a_read_only_angle`,
//!   `a_trait_and_an_axis_put_down_in_the_driving_quarter_open_the_driving_angle`
//! - A read-only angle reads its quarter again after the traits move —
//!   `a_read_only_angle_reads_its_quarter_again_after_the_traits_move`
//!
//! Also asked by the issue, outside its Done when:
//! - the preview before the click shows which of the two the click does — the
//!   quarter the presenter holds as the cursor moves is the angle the preview
//!   draws: `the_quarter_shown_before_the_click_is_the_read_only_one_it_would_lay`,
//!   and what the preview hands the painter says whether the click lays a
//!   readout or opens the driving angle —
//!   `the_preview_says_whether_the_click_lays_a_readout_or_opens_the_driving_angle`,
//!   both in `render/preview/tests.rs`; the colour that flag is painted in
//!   gets no test: it is chosen inside `paint_dimension_labels`, which needs
//!   an `egui::Ui`
//! - a read-only angle goes when one of its traits goes —
//!   `a_read_only_angle_goes_with_a_trait_it_measures`
//! - another quarter is any of the three: the one opposite the driving angle
//!   reads the same value and is laid read-only too —
//!   `an_x_put_down_in_the_opposite_quarter_lays_a_read_only_angle_of_the_same_value`;
//!   and the preview and the click agree on which is read-only, held in
//!   `render/preview/tests.rs` by
//!   `what_the_preview_calls_read_only_is_what_the_click_records`
//!
//! What the issue says must not break:
//! - Clicking the pair again in the quarter its angle measures opens it —
//!   `an_x_put_down_again_in_the_driving_quarter_opens_the_driving_angle`,
//!   `a_corner_put_down_in_the_driving_quarter_opens_the_driving_angle`,
//!   `a_t_put_down_in_the_driving_quarter_opens_the_driving_angle`,
//!   `two_traits_apart_put_down_in_the_driving_quarter_open_the_driving_angle`,
//!   `a_trait_and_an_axis_put_down_in_the_driving_quarter_open_the_driving_angle`
//! - A value typed on the driving angle drives the traits on its side; the
//!   read-only ones follow —
//!   `a_read_only_angle_reads_its_quarter_again_after_the_traits_move`
//! - The quarter held through a margin, with a driving angle already laid: a
//!   hair past its edge, it stays and opens; past the margin, the next quarter
//!   is laid read-only —
//!   `a_hand_a_hair_past_the_driving_quarter_keeps_it_and_opens_it`
//! - Trimming, compaction and the corner tools carry every angle with its
//!   quarter — no test: held in cao_part, where the trim, the compaction and
//!   the fillet run, by the corner-angle tests that file transcribes for this
//!   issue

use cao_part::history::PointRef;
use cao_part::{Formula, Operation, PartDocument, VariableChange};
use cao_sketch::{DimensionMode, DimensionTarget, SegmentId, ToolState, WorkPlane};
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

/// Two traits meeting at (10, 10), one along to (34, 10) and carrying its
/// length, the other up to (10, 40) carrying none.
fn a_corner_with_one_side_measured() -> PartDocument {
    let mut document = a_trait_and_no_scale();
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(cao_sketch::PointId(1)),
        end: PointRef::New(DVec2::new(10.0, 40.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: SIDE,
        value: 24.0.into(),
        placement: None,
    });
    document
}

const ON_THE_MEASURED_SIDE: DVec2 = DVec2::new(22.0, 10.0);
const ON_THE_OTHER_SIDE: DVec2 = DVec2::new(10.0, 25.0);
const OTHER_SIDE: DimensionTarget = DimensionTarget::Length(SegmentId(1));

fn click(context: &mut SketchContext<'_>, at: DVec2) {
    measure(context, 0, at, 1.0, 0.05);
}

fn placing(editor: &SketchEditor) -> Option<DimensionTarget> {
    match editor.tool_state {
        ToolState::Dimension { placing, .. } => placing,
        _ => None,
    }
}

#[test]
fn a_trait_carrying_its_length_then_a_second_trait_gives_the_angle() {
    let mut document = a_corner_with_one_side_measured();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    assert_eq!(context.editor.dimension_mode, DimensionMode::Auto);

    click(&mut context, ON_THE_MEASURED_SIDE);
    click(&mut context, ON_THE_OTHER_SIDE);

    assert_eq!(
        placing(&editor),
        Some(DimensionTarget::corner(SegmentId(0), SegmentId(1))),
        "the second trait turned the first into an angle"
    );
    assert!(editor.editing.is_none(), "the length's field stayed open");
}

#[test]
fn a_trait_carrying_its_length_still_opens_it_for_retyping() {
    let mut document = a_corner_with_one_side_measured();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    click(&mut context, ON_THE_MEASURED_SIDE);
    let opened = editor.editing.as_ref().map(|editing| editing.target);
    if let Some(editing) = editor.editing.as_mut() {
        editing.input = "30".to_string();
    }
    assert!(apply_dimension_value(
        &mut document,
        &mut editor,
        0,
        SIDE,
        &lang
    ));

    assert_eq!(opened, Some(SIDE), "the click did not open the length");
    let retyped = document.sketches()[0].dimension_of(SIDE).map(|d| d.value);
    assert_eq!(retyped, Some(30.0));
}

#[test]
fn a_trait_clicked_after_a_length_was_retyped_gets_its_own_length() {
    let mut document = a_corner_with_one_side_measured();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    click(&mut context, ON_THE_MEASURED_SIDE);
    if let Some(editing) = editor.editing.as_mut() {
        editing.input = "30".to_string();
    }
    apply_dimension_value(&mut document, &mut editor, 0, SIDE, &lang);
    editor.editing = None;

    let other_side = document.sketches()[0].segments()[1];
    let halfway = (document.sketches()[0].point(other_side.start)
        + document.sketches()[0].point(other_side.end))
        / 2.0;
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    click(&mut context, halfway);

    assert_eq!(placing(&editor), Some(OTHER_SIDE));
}

#[test]
fn a_trait_then_an_axis_then_a_place_lays_the_angle_of_that_quarter() {
    for (place, expected) in [
        (DVec2::new(40.0, 80.0), 101.309_932_474_020_2),
        (DVec2::new(100.0, 80.0), 78.690_067_525_979_8),
    ] {
        let mut document = PartDocument::new("part", Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(20.0, 30.0)),
            end: PointRef::New(DVec2::new(120.0, 50.0)),
            construction: false,
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

        click(&mut context, DVec2::new(70.0, 40.0));
        click(&mut context, DVec2::new(0.0, 150.0));
        click(&mut context, place);

        let laid: Vec<_> = document.sketches()[0]
            .dimensions()
            .iter()
            .map(|dimension| (dimension.target, dimension.value))
            .collect();
        let [(DimensionTarget::AxisAngle { .. }, value)] = laid[..] else {
            panic!("one angle to the axis laid at {place:?}, got {laid:?}");
        };
        assert!(
            (value - expected).abs() < 1e-6,
            "put down at {place:?} it reads {value}°, not {expected}°",
        );
    }
}

#[test]
fn a_hand_wavering_on_the_line_keeps_the_quarter_it_shows() {
    let crossing = DVec2::new(50.0, 0.0);
    let thirty = DVec2::from_angle(30.0_f64.to_radians());
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (start, end) in [
        (DVec2::ZERO, DVec2::new(100.0, 0.0)),
        (crossing - thirty * 40.0, crossing + thirty * 40.0),
    ] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(start),
            end: PointRef::New(end),
            construction: false,
        });
    }
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let pixel = 0.05;
    let follow = super::super::following::follow_the_cursor;

    click(&mut context, DVec2::new(20.0, 0.0));
    click(&mut context, crossing + thirty * 30.0);
    follow(&mut context, 0, crossing + DVec2::new(30.0, 0.3), pixel);
    let a_hair_under = crossing + DVec2::new(30.0, -0.1);
    follow(&mut context, 0, a_hair_under, pixel);
    follow(&mut context, 0, crossing + DVec2::new(30.0, 0.1), pixel);
    follow(&mut context, 0, a_hair_under, pixel);
    click(&mut context, a_hair_under);

    let sketch = &document.sketches()[0];
    let laid: Vec<_> = sketch
        .dimensions()
        .iter()
        .map(|dimension| sketch.opening(dimension.target))
        .collect();
    let [Some(opening)] = laid[..] else {
        panic!("one angle laid, got {laid:?}");
    };
    assert!(
        (opening - 30.0).abs() < 1e-6,
        "a hair under the line, the narrow quarter shown above it stays: {opening}°",
    );
}

/// A corner at (10, 10) rounded to 5: one side right to (60, 10), one up to
/// (10, 50). The fillet meets the first at (15, 10).
fn a_rounded_corner() -> PartDocument {
    let mut document = a_trait_and_no_scale();
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(cao_sketch::PointId(1)),
        end: PointRef::New(DVec2::new(10.0, 50.0)),
        construction: false,
    });
    document.apply(Operation::Fillet {
        sketch: 0,
        corners: vec![cao_sketch::Corner::Between(SegmentId(0), SegmentId(1))],
        radius: 5.0.into(),
        construction: false,
    });
    document
}

#[test]
fn a_dimension_from_a_cut_s_point_is_one_step_with_the_point_it_hangs() {
    for mode in [DimensionMode::Auto, DimensionMode::PointToPoint] {
        let mut document = a_rounded_corner();
        let steps = document.history.applied_operations().len();
        let points = document.sketches()[0].points().len();
        let mut editor = SketchEditor {
            dimension_mode: mode,
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

        measure(&mut context, 0, DVec2::new(15.0, 10.0), 1.0, 0.05);
        measure(&mut context, 0, DVec2::new(34.0, 10.0), 1.0, 0.05);
        measure(&mut context, 0, DVec2::new(25.0, 0.0), 1.0, 0.05);

        let hung = cao_sketch::PointId(points);
        let sketch = &document.sketches()[0];
        assert_eq!(
            sketch.holds_on(hung),
            vec![cao_sketch::Support::CutPoint(
                cao_sketch::CornerCutId(0),
                cao_sketch::CutPoint::First
            )],
            "{mode:?}",
        );
        let visible = DimensionTarget::Distance {
            from: hung,
            to: cao_sketch::PointId(2),
        };
        let laid = sketch
            .dimension_of(visible)
            .expect("the visible part is dimensioned");
        assert!(
            (laid.value - 19.0).abs() < 1e-6,
            "{mode:?} reads {}",
            laid.value
        );
        assert_eq!(
            document.history.applied_operations().len(),
            steps + 1,
            "{mode:?}: one step"
        );

        assert!(document.undo());
        assert_eq!(
            document.sketches()[0].points().len(),
            points,
            "{mode:?}: one undo takes both"
        );
    }
}

#[test]
fn a_dimension_placed_on_a_cut_s_point_hangs_nothing_there() {
    let mut document = a_rounded_corner();
    let points = document.sketches()[0].points().len();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    measure(&mut context, 0, DVec2::new(10.0, 40.0), 1.0, 0.05);
    measure(&mut context, 0, DVec2::new(10.0, 15.0), 1.0, 0.05);

    assert_eq!(document.sketches()[0].points().len(), points);
    assert!(
        document.sketches()[0]
            .dimension_of(DimensionTarget::Length(SegmentId(1)))
            .is_some(),
        "the click placed the length of the side it was waiting with",
    );
}

/// Two traits crossing at (50, 0) at 60°: one along the U axis from the origin
/// to (100, 0), the other rising through the crossing.
fn an_x_at_sixty_degrees() -> PartDocument {
    let crossing = DVec2::new(50.0, 0.0);
    let sixty = DVec2::from_angle(60.0_f64.to_radians());
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (start, end) in [
        (DVec2::ZERO, DVec2::new(100.0, 0.0)),
        (crossing - sixty * 40.0, crossing + sixty * 40.0),
    ] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(start),
            end: PointRef::New(end),
            construction: false,
        });
    }
    document
}

const ON_THE_FLAT_TRAIT: DVec2 = DVec2::new(20.0, 0.0);

fn on_the_rising_trait() -> DVec2 {
    DVec2::new(50.0, 0.0) + DVec2::from_angle(60.0_f64.to_radians()) * 30.0
}

fn in_the_sixty_degree_quarter() -> DVec2 {
    DVec2::new(50.0, 0.0) + DVec2::from_angle(30.0_f64.to_radians()) * 20.0
}

fn in_the_hundred_and_twenty_degree_quarter() -> DVec2 {
    DVec2::new(50.0, 0.0) + DVec2::from_angle(120.0_f64.to_radians()) * 20.0
}

/// Clicks the two traits of the X, then puts the angle down at `place`, as a
/// user does with the smart dimension tool; the field it opens is let go.
fn dimension_the_x(context: &mut SketchContext<'_>, place: DVec2) {
    click(context, ON_THE_FLAT_TRAIT);
    click(context, on_the_rising_trait());
    click(context, place);
    context.editor.editing = None;
}

#[test]
fn an_x_put_down_in_another_quarter_lays_a_read_only_angle_beside_the_driving_one() {
    let mut document = an_x_at_sixty_degrees();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    dimension_the_x(&mut context, in_the_sixty_degree_quarter());
    dimension_the_x(&mut context, in_the_hundred_and_twenty_degree_quarter());

    let sketch = &document.sketches()[0];
    let laid: Vec<_> = sketch
        .dimensions()
        .iter()
        .map(|dimension| (dimension.target, dimension.value, dimension.driven))
        .collect();
    let [driving, read_only] = laid[..] else {
        panic!("the driving 60° and a read-only 120° beside it, got {laid:?}");
    };
    assert!(
        matches!(driving.0, DimensionTarget::AngleBetween { .. })
            && (driving.1 - 60.0).abs() < 1e-6
            && !driving.2,
        "the first angle laid drives at 60°, got {driving:?}",
    );
    assert!(
        matches!(read_only.0, DimensionTarget::AngleBetween { .. })
            && (read_only.1 - 120.0).abs() < 1e-6
            && read_only.2,
        "put down in the 120° quarter, a read-only 120° is laid, got {read_only:?}",
    );
    let opening = sketch.opening(read_only.0);
    assert!(
        opening.is_some_and(|opening| (opening - 120.0).abs() < 1e-6),
        "the read-only angle names the 120° quarter itself, which reads {opening:?}",
    );
    let driving_values: Vec<_> = sketch
        .dimensions()
        .iter()
        .filter(|dimension| !dimension.driven)
        .map(|dimension| dimension.value)
        .collect();
    assert_eq!(
        driving_values.len(),
        1,
        "the 60° stays the only value driving the traits, got {driving_values:?}",
    );
}

/// Two traits making an angle, where to click them, and two places to put
/// their angle down: one in the quarter the driving angle measures, one in
/// another, each with what its quarter reads.
struct APair {
    document: PartDocument,
    picks: [DVec2; 2],
    driving: (DVec2, f64),
    other: (DVec2, f64),
}

fn degrees(angle: f64) -> DVec2 {
    DVec2::from_angle(angle.to_radians())
}

fn a_part_with_traits(traits: &[(DVec2, DVec2)]) -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (start, end) in traits {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(*start),
            end: PointRef::New(*end),
            construction: false,
        });
    }
    document
}

fn the_x() -> APair {
    let crossing = DVec2::new(50.0, 0.0);
    APair {
        document: an_x_at_sixty_degrees(),
        picks: [ON_THE_FLAT_TRAIT, on_the_rising_trait()],
        driving: (crossing + degrees(30.0) * 20.0, 60.0),
        other: (crossing + degrees(120.0) * 20.0, 120.0),
    }
}

/// A corner at (10, 10) of 60°: one side along to (40, 10), carrying its
/// length, the other rising from the corner.
fn a_corner_whose_first_side_carries_its_length() -> APair {
    let corner = DVec2::new(10.0, 10.0);
    let mut document = a_part_with_traits(&[(corner, DVec2::new(40.0, 10.0))]);
    let pivot = document.sketches()[0].segments()[0].start;
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(pivot),
        end: PointRef::New(corner + degrees(60.0) * 30.0),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: SIDE,
        value: 30.0.into(),
        placement: None,
    });
    APair {
        document,
        picks: [DVec2::new(30.0, 10.0), corner + degrees(60.0) * 20.0],
        driving: (corner + degrees(30.0) * 12.0, 60.0),
        other: (corner + degrees(120.0) * 12.0, 120.0),
    }
}

/// A T: a bar from (10, 0) to (90, 0), carrying its length, and a stem
/// rising at 60° from its middle.
fn a_t_whose_bar_carries_its_length() -> APair {
    let foot = DVec2::new(50.0, 0.0);
    let mut document = a_part_with_traits(&[
        (DVec2::new(10.0, 0.0), DVec2::new(90.0, 0.0)),
        (foot, foot + degrees(60.0) * 30.0),
    ]);
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: SIDE,
        value: 80.0.into(),
        placement: None,
    });
    APair {
        document,
        picks: [DVec2::new(20.0, 0.0), foot + degrees(60.0) * 20.0],
        driving: (foot + degrees(30.0) * 12.0, 60.0),
        other: (foot + degrees(120.0) * 12.0, 120.0),
    }
}

/// Two traits lying apart: one from the origin to (40, 0), the other at 60°
/// from (60, 10), its line meeting the first's further along.
fn two_traits_lying_apart() -> APair {
    let start = DVec2::new(60.0, 10.0);
    let crossing = start - degrees(60.0) * (10.0 / 60.0_f64.to_radians().sin());
    APair {
        document: a_part_with_traits(&[
            (DVec2::ZERO, DVec2::new(40.0, 0.0)),
            (start, start + degrees(60.0) * 30.0),
        ]),
        picks: [DVec2::new(20.0, 0.0), start + degrees(60.0) * 15.0],
        driving: (crossing + degrees(30.0) * 12.0, 60.0),
        other: (crossing + degrees(120.0) * 12.0, 120.0),
    }
}

/// A trait at 60° from (20, 30), against the vertical axis: 30° between them
/// up the axis, 150° down it, about the trait's middle.
fn a_trait_and_the_vertical_axis() -> APair {
    let start = DVec2::new(20.0, 30.0);
    let middle = start + degrees(60.0) * 20.0;
    APair {
        document: a_part_with_traits(&[(start, start + degrees(60.0) * 40.0)]),
        picks: [start + degrees(60.0) * 30.0, DVec2::new(0.0, 150.0)],
        driving: (middle + degrees(75.0) * 12.0, 30.0),
        other: (middle + degrees(-15.0) * 12.0, 150.0),
    }
}

/// Clicks the two halves of the pair and puts their angle down at `place`,
/// the field the last gesture opened let go first.
fn put_down(context: &mut SketchContext<'_>, picks: [DVec2; 2], place: DVec2) {
    context.editor.editing = None;
    for pick in picks {
        click(context, pick);
    }
    click(context, place);
}

/// The angles on the drawing, in the order they were laid: what each names,
/// the value it shows, and whether it only reads it.
fn angles_on(document: &PartDocument) -> Vec<(DimensionTarget, f64, bool)> {
    document.sketches()[0]
        .dimensions()
        .iter()
        .filter(|dimension| dimension.is_angle())
        .map(|dimension| {
            let shown = match dimension.driven {
                true => document.measured(0, dimension.target).unwrap_or(f64::NAN),
                false => dimension.value,
            };
            (dimension.target, shown, dimension.driven)
        })
        .collect()
}

fn lays_a_read_only_angle_beside_the_driving_one(pair: APair) {
    let APair {
        mut document,
        picks,
        driving,
        other,
    } = pair;
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    put_down(&mut context, picks, driving.0);
    put_down(&mut context, picks, other.0);

    let laid = angles_on(&document);
    let [first, second] = laid[..] else {
        panic!(
            "the driving {}° and a read-only {}° beside it, got {laid:?}",
            driving.1, other.1,
        );
    };
    assert!(
        !first.2 && (first.1 - driving.1).abs() < 1e-6,
        "the first angle laid drives at {}°, got {first:?}",
        driving.1,
    );
    assert!(
        second.2 && (second.1 - other.1).abs() < 1e-6,
        "put down in the other quarter, a read-only {}° is laid, got {second:?}",
        other.1,
    );
}

fn opens_the_driving_angle_in_its_own_quarter(pair: APair) {
    let APair {
        mut document,
        picks,
        driving,
        ..
    } = pair;
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    put_down(&mut context, picks, driving.0);
    let laid = angles_on(context.document);
    put_down(&mut context, picks, driving.0);

    let [(target, ..)] = laid[..] else {
        panic!("one driving angle laid at first, got {laid:?}");
    };
    assert_eq!(
        angles_on(&document),
        laid,
        "put down again in its own quarter, nothing is laid beside the driving angle",
    );
    assert_eq!(
        editor.editing.as_ref().map(|editing| editing.target),
        Some(target),
        "put down again in its own quarter, the driving {}° opens",
        driving.1,
    );
}

#[test]
fn an_x_put_down_again_in_the_driving_quarter_opens_the_driving_angle() {
    opens_the_driving_angle_in_its_own_quarter(the_x());
}

#[test]
fn a_corner_put_down_in_another_quarter_lays_a_read_only_angle() {
    lays_a_read_only_angle_beside_the_driving_one(a_corner_whose_first_side_carries_its_length());
}

#[test]
fn a_corner_put_down_in_the_driving_quarter_opens_the_driving_angle() {
    opens_the_driving_angle_in_its_own_quarter(a_corner_whose_first_side_carries_its_length());
}

#[test]
fn a_t_put_down_in_another_quarter_lays_a_read_only_angle() {
    lays_a_read_only_angle_beside_the_driving_one(a_t_whose_bar_carries_its_length());
}

#[test]
fn a_t_put_down_in_the_driving_quarter_opens_the_driving_angle() {
    opens_the_driving_angle_in_its_own_quarter(a_t_whose_bar_carries_its_length());
}

#[test]
fn two_traits_apart_put_down_in_another_quarter_lay_a_read_only_angle() {
    lays_a_read_only_angle_beside_the_driving_one(two_traits_lying_apart());
}

#[test]
fn two_traits_apart_put_down_in_the_driving_quarter_open_the_driving_angle() {
    opens_the_driving_angle_in_its_own_quarter(two_traits_lying_apart());
}

#[test]
fn a_trait_and_an_axis_put_down_in_another_quarter_lay_a_read_only_angle() {
    lays_a_read_only_angle_beside_the_driving_one(a_trait_and_the_vertical_axis());
}

#[test]
fn a_trait_and_an_axis_put_down_in_the_driving_quarter_open_the_driving_angle() {
    opens_the_driving_angle_in_its_own_quarter(a_trait_and_the_vertical_axis());
}

#[test]
fn a_read_only_angle_reads_its_quarter_again_after_the_traits_move() {
    let APair {
        mut document,
        picks,
        driving,
        other,
    } = the_x();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    put_down(&mut context, picks, driving.0);
    put_down(&mut context, picks, other.0);
    put_down(&mut context, picks, driving.0);
    let opened = context
        .editor
        .editing
        .as_ref()
        .map(|editing| editing.target);
    let Some(sixty) = opened else {
        panic!("the driving 60° opens for retyping");
    };
    if let Some(editing) = context.editor.editing.as_mut() {
        editing.input = "50".to_string();
    }
    assert!(apply_dimension_value(
        context.document,
        context.editor,
        0,
        sixty,
        &lang
    ));

    let laid = angles_on(&document);
    let [
        (driving_target, ..),
        (read_only_target, read_only_shows, true),
    ] = laid[..]
    else {
        panic!("the driving angle and a read-only one beside it, got {laid:?}");
    };
    let turned = document.sketches()[0].opening(driving_target);
    assert!(
        turned.is_some_and(|turned| (turned - 50.0).abs() < 1e-4),
        "the traits turned to the 50° typed, read {turned:?}",
    );
    assert!(
        (read_only_shows - 130.0).abs() < 1e-4,
        "the read-only angle reads what its quarter measures once the traits turned: \
         130°, shows {read_only_shows}° on {read_only_target:?}",
    );
}

#[test]
fn a_read_only_angle_goes_with_a_trait_it_measures() {
    let APair {
        mut document,
        picks,
        driving,
        other,
    } = the_x();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    put_down(&mut context, picks, driving.0);
    put_down(&mut context, picks, other.0);
    let before = angles_on(&document);
    assert!(
        before.iter().any(|(_, _, read_only)| *read_only),
        "a read-only angle stands on the X before its trait goes, got {before:?}",
    );

    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: vec![cao_sketch::Element::Segment(SegmentId(1))],
        dimensions: vec![],
        constraints: vec![],
    });

    assert_eq!(
        angles_on(&document),
        vec![],
        "with the rising trait gone, no angle is left reading it",
    );
}

#[test]
fn an_x_put_down_in_the_opposite_quarter_lays_a_read_only_angle_of_the_same_value() {
    let crossing = DVec2::new(50.0, 0.0);
    lays_a_read_only_angle_beside_the_driving_one(APair {
        other: (crossing + degrees(210.0) * 20.0, 60.0),
        ..the_x()
    });
}

#[test]
fn a_hand_a_hair_past_the_driving_quarter_keeps_it_and_opens_it() {
    let crossing = DVec2::new(50.0, 0.0);
    let pixel = 0.05;
    let follow = super::super::following::follow_the_cursor;
    for (drifted_to, opens, said) in [
        (
            crossing + DVec2::new(30.0, -0.1),
            true,
            "a hair under the flat trait",
        ),
        (crossing + DVec2::new(30.0, -5.0), false, "well under it"),
    ] {
        let APair {
            mut document,
            picks,
            driving,
            ..
        } = the_x();
        let mut editor = SketchEditor::default();
        let mut extrusion = ExtrusionState::default();
        let lang = Catalogue::french();
        let mut context = SketchContext {
            document: &mut document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };
        put_down(&mut context, picks, driving.0);
        let [(sixty, ..)] = angles_on(context.document)[..] else {
            panic!("{said}: the driving 60° laid first");
        };

        context.editor.editing = None;
        click(&mut context, picks[0]);
        click(&mut context, picks[1]);
        follow(&mut context, 0, crossing + DVec2::new(30.0, 0.3), pixel);
        follow(&mut context, 0, drifted_to, pixel);
        let shown = placing(context.editor);
        click(&mut context, drifted_to);

        let laid = angles_on(&document);
        if opens {
            assert_eq!(
                shown,
                Some(sixty),
                "{said}: the driving 60° is still the one shown"
            );
            assert_eq!(
                laid.len(),
                1,
                "{said}: nothing laid beside it, got {laid:?}"
            );
            assert_eq!(
                editor.editing.as_ref().map(|editing| editing.target),
                Some(sixty),
                "{said}: the click opens the driving 60°",
            );
        } else {
            assert_ne!(
                shown,
                Some(sixty),
                "{said}: past the margin, the next quarter shows"
            );
            let [_, (_, value, true)] = laid[..] else {
                panic!("{said}: a read-only angle laid beside the 60°, got {laid:?}");
            };
            assert!(
                (value - 120.0).abs() < 1e-6,
                "{said}: the quarter under the flat trait reads 120°, got {value}°",
            );
        }
    }
}
