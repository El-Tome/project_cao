//! Closes #314.
//! - a line drawn with a typed angle leaves a dashed construction segment
//!   along the horizontal, and the angle dimension reads between the two —
//!   `a_typed_angle_leaves_an_arm_along_the_horizontal_and_the_reading`
//! - a trait drawn at a right angle from a point on an origin axis is held on
//!   that axis, with no construction segment —
//!   `a_trait_square_to_the_axis_it_starts_on_is_simply_held_on_it`
//! - undo removes everything the gesture laid in one step —
//!   `a_line_drawn_at_a_typed_angle_is_one_step_of_the_history`

use cao_part::PartDocument;
use cao_part::history::PointRef;
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

/// A drawing holding one trait, drawn from `start` towards `degrees`.
fn a_part_with_a_trait(start: DVec2, degrees: f64) -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(start),
        end: PointRef::New(start + DVec2::from_angle(f64::to_radians(degrees)) * 10.0),
        construction: false,
    });
    document
}

/// What the gesture leaves once the arm has been laid beside the trait.
fn leaning(document: &mut PartDocument) {
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let drawn = SegmentId(0);
    let from = document.sketches()[0].segments()[0].start;
    let mut context = SketchContext {
        document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    lean_on_an_arm(&mut context, 0, drawn, from, 0.05);
}

#[test]
fn a_typed_angle_leaves_an_arm_along_the_horizontal_and_the_reading() {
    let mut document = a_part_with_a_trait(DVec2::new(2.0, 3.0), 150.0);

    leaning(&mut document);

    let sketch = &document.sketches()[0];
    let arm = SegmentId(1);
    assert!(
        sketch
            .segments()
            .get(arm.0)
            .is_some_and(|it| it.construction),
        "the arm is construction geometry, drawn dashed like the rest of it",
    );
    let (from, to) = sketch.endpoints(arm);
    assert!(
        (from.y - to.y).abs() < 1e-9 && to.x > from.x,
        "the arm runs from {from} to {to}, which is not east along the horizontal",
    );
    assert!(
        sketch.constraints().contains(&Constraint::AxisParallel {
            segment: arm,
            axis: SketchAxis::U,
        }),
        "nothing holds the arm horizontal, so the reading rests on nothing",
    );

    let reading = sketch
        .dimensions()
        .iter()
        .find(|value| matches!(value.target, DimensionTarget::Angle { .. }))
        .expect("the angle between the arm and the trait is written down");
    assert!(
        (reading.value - 150.0).abs() < 1e-6,
        "the reading says {} where 150 was typed",
        reading.value,
    );
}

#[test]
fn a_trait_square_to_the_axis_it_starts_on_is_simply_held_on_it() {
    let mut document = a_part_with_a_trait(DVec2::new(4.0, 0.0), 180.0);

    leaning(&mut document);

    let sketch = &document.sketches()[0];
    assert_eq!(
        sketch.live_segments().count(),
        1,
        "a trait already lying along an axis was given an arm to read it against",
    );
    assert!(
        sketch.constraints().contains(&Constraint::AxisCollinear {
            segment: SegmentId(0),
            axis: SketchAxis::U,
        }),
        "the rule that says it lies on the axis was not laid: {:?}",
        sketch.constraints(),
    );
}

#[test]
fn a_line_drawn_at_a_typed_angle_is_one_step_of_the_history() {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
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

    crate::screens::viewport::input::draw_line_point(&mut context, 0, DVec2::ZERO, 0.1, 0.05);
    context.editor.live.open_on(&[None, Some(150.0)]);
    crate::screens::viewport::input::draw_line_point(
        &mut context,
        0,
        DVec2::new(-8.0, 5.0),
        0.1,
        0.05,
    );

    assert_eq!(
        document.history.operations().len(),
        2,
        "the drawing was opened and one line was drawn, and the history says otherwise",
    );
    assert_eq!(
        document.sketches()[0].live_segments().count(),
        2,
        "the trait and the arm its angle is read against are not both there",
    );

    assert!(document.undo(), "there was a gesture to take back");
    assert_eq!(
        document.sketches()[0].live_segments().count(),
        0,
        "one undo left a piece of the gesture behind",
    );
}

#[test]
fn a_symmetric_line_drawn_at_a_typed_angle_leans_on_an_arm_from_its_middle() {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
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

    let draw = crate::screens::viewport::input::draw_symmetric_line_point;
    draw(&mut context, 0, DVec2::new(2.0, 3.0), 0.1, 0.05);
    context.editor.live.open_on(&[None, Some(150.0)]);
    draw(&mut context, 0, DVec2::new(-6.0, 8.0), 0.1, 0.05);

    let sketch = &document.sketches()[0];
    let arm = SegmentId(1);
    let (from, to) = sketch.endpoints(arm);
    assert!(
        sketch
            .segments()
            .get(arm.0)
            .is_some_and(|it| it.construction),
        "the symmetric line was left with no arm to read its angle against",
    );
    assert!(
        (from.y - to.y).abs() < 1e-9 && to.x > from.x,
        "the arm runs from {from} to {to}, which is not east along the horizontal",
    );
    assert!(
        sketch.constraints().contains(&Constraint::Midpoint {
            point: sketch.segments()[arm.0].start,
            segment: SegmentId(0),
        }),
        "the arm springs from something other than the middle of the trait",
    );
}
