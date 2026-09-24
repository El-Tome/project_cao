//! What app · viewport/values.rs is held to.
//!
//! Closes #175.
//! - a formula typed at the cursor that does not read is refused where it is
//!   typed, with a message saying what is wrong, and nothing is laid —
//!   `a_field_at_the_cursor_that_does_not_read_refuses_the_click_and_says_why`,
//!   `fields_holding_what_reads_let_the_click_through`
//!
//! Closes #423.
//! - a length typed while a shape is drawn, when it is the first value the
//!   part is given, sets the scale, and the shape is laid where it is drawn —
//!   `a_first_length_typed_on_a_line_sets_the_scale_where_the_line_is_drawn`,
//!   `a_rectangle_takes_the_scale_from_its_width_and_draws_its_height_to_it`,
//!   `a_first_diameter_typed_on_a_circle_sets_the_scale_where_it_is_drawn`,
//!   `a_first_radius_typed_on_an_arc_sets_the_scale_where_it_is_drawn`,
//!   `an_ellipse_takes_the_scale_from_its_first_axis_and_draws_its_second_to_it`,
//!   `a_first_length_typed_on_a_symmetric_line_sets_the_scale_where_it_is_drawn`
//! - a second size typed on the same shape is drawn at once at the scale the
//!   first one gives, and at the click nothing moves —
//!   `a_rectangle_takes_the_scale_from_its_width_and_draws_its_height_to_it`,
//!   `an_ellipse_takes_the_scale_from_its_first_axis_and_draws_its_second_to_it`

use cao_part::{PartDocument, VariableChange, Variables};
use cao_sketch::{Sketch, WorkPlane};
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::input::{
    draw_arc, draw_circle, draw_ellipse, draw_line_point, draw_symmetric_line_point,
    rectangle_corner, two_click_shape,
};

/// How far a click reaches for a point already there, in the drawing's units:
/// short enough that nothing below is joined by accident.
const SNAP: f64 = 0.5;
const PIXEL: f64 = 0.05;

/// A part with a drawing on the ground plane, and no value given yet.
fn a_part_with_no_scale() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document
}

/// What the field of that rank holds once `text` is typed into it, over a
/// shape that measured `over` units along it on screen.
fn type_over(context: &mut SketchContext<'_>, rank: usize, text: &str, over: f64) {
    context.editor.live.field(rank).text = text.to_string();
    context
        .editor
        .live
        .take(rank, context.document.variables(), Some(over));
}

fn assert_near(found: DVec2, expected: DVec2, what: &str) {
    assert!(
        found.distance(expected) < 1e-6,
        "{what} should stand at {expected}, and stands at {found}",
    );
}

fn assert_scale(document: &PartDocument, expected: f64) {
    assert!(document.has_scale(), "the part was given no scale");
    assert!(
        (document.scale() - expected).abs() < 1e-9,
        "a unit should be worth {expected} mm, and is worth {}",
        document.scale(),
    );
}

fn drawing(document: &PartDocument) -> &Sketch {
    &document.sketches()[0]
}

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

#[test]
fn a_first_length_typed_on_a_line_sets_the_scale_where_the_line_is_drawn() {
    let mut document = a_part_with_no_scale();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let start = DVec2::new(10.0, 10.0);

    draw_line_point(&mut context, 0, start, SNAP, PIXEL);
    type_over(&mut context, 0, "100", 24.0);
    draw_line_point(&mut context, 0, DVec2::new(40.0, 10.0), SNAP, PIXEL);

    assert_scale(&document, 100.0 / 24.0);
    let line = drawing(&document).segments()[0];
    assert_near(drawing(&document).point(line.start), start, "the start");
    assert_near(
        drawing(&document).point(line.end),
        DVec2::new(34.0, 10.0),
        "the end, 24 units on towards the cursor",
    );
}

#[test]
fn a_rectangle_takes_the_scale_from_its_width_and_draws_its_height_to_it() {
    let mut document = a_part_with_no_scale();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let start = DVec2::new(10.0, 10.0);

    two_click_shape(&mut context, 0, start, SNAP, PIXEL);
    type_over(&mut context, 0, "100", 24.0);
    type_over(&mut context, 1, "50", 30.0);
    let corner = rectangle_corner(&context, DVec2::new(40.0, 40.0));
    assert_near(
        corner,
        DVec2::new(34.0, 22.0),
        "the corner, 24 wide as drawn and half as high as typed",
    );
    two_click_shape(&mut context, 0, corner, SNAP, PIXEL);

    assert_scale(&document, 100.0 / 24.0);
    let corners: Vec<DVec2> = drawing(&document).points()[1..].to_vec();
    for expected in [
        start,
        DVec2::new(34.0, 10.0),
        corner,
        DVec2::new(10.0, 22.0),
    ] {
        assert!(
            corners.iter().any(|found| found.distance(expected) < 1e-6),
            "a corner should stand at {expected}, and the corners are {corners:?}",
        );
    }
}

#[test]
fn a_first_diameter_typed_on_a_circle_sets_the_scale_where_it_is_drawn() {
    let mut document = a_part_with_no_scale();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let centre = DVec2::new(10.0, 10.0);

    draw_circle(&mut context, 0, centre, SNAP, PIXEL);
    type_over(&mut context, 0, "100", 20.0);
    draw_circle(&mut context, 0, DVec2::new(25.0, 10.0), SNAP, PIXEL);

    assert_scale(&document, 5.0);
    let circle = drawing(&document).circles()[0];
    assert_near(
        drawing(&document).point(circle.center),
        centre,
        "the centre",
    );
    assert!(
        (circle.radius - 10.0).abs() < 1e-6,
        "the circle should keep the 20 units across it was typed over, and is {} across",
        circle.radius * 2.0,
    );
}

#[test]
fn a_first_radius_typed_on_an_arc_sets_the_scale_where_it_is_drawn() {
    let mut document = a_part_with_no_scale();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let centre = DVec2::new(10.0, 10.0);

    draw_arc(&mut context, 0, centre, SNAP, PIXEL);
    type_over(&mut context, 0, "100", 20.0);
    draw_arc(&mut context, 0, DVec2::new(40.0, 10.0), SNAP, PIXEL);
    draw_arc(&mut context, 0, DVec2::new(10.0, 40.0), SNAP, PIXEL);

    assert_scale(&document, 5.0);
    let arc = drawing(&document).arcs()[0];
    assert_near(drawing(&document).point(arc.center), centre, "the centre");
    assert_near(
        drawing(&document).point(arc.start),
        DVec2::new(30.0, 10.0),
        "the start, 20 units out as it was typed over",
    );
}

#[test]
fn an_ellipse_takes_the_scale_from_its_first_axis_and_draws_its_second_to_it() {
    let mut document = a_part_with_no_scale();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let centre = DVec2::new(10.0, 10.0);

    draw_ellipse(&mut context, 0, centre, SNAP, PIXEL);
    type_over(&mut context, 0, "100", 40.0);
    draw_ellipse(&mut context, 0, DVec2::new(35.0, 10.0), SNAP, PIXEL);
    type_over(&mut context, 0, "50", 60.0);
    draw_ellipse(&mut context, 0, DVec2::new(10.0, 40.0), SNAP, PIXEL);

    assert_scale(&document, 2.5);
    let oval = drawing(&document).ellipses()[0];
    for (axis, units, which) in [(oval.first, 40.0, "first"), (oval.second, 20.0, "second")] {
        let length = drawing(&document).segment_length(axis);
        assert!(
            (length - units).abs() < 1e-6,
            "the {which} axis should run {units} units, and runs {length}",
        );
    }
}

#[test]
fn a_first_length_typed_on_a_symmetric_line_sets_the_scale_where_it_is_drawn() {
    let mut document = a_part_with_no_scale();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    draw_symmetric_line_point(&mut context, 0, DVec2::new(10.0, 10.0), SNAP, PIXEL);
    type_over(&mut context, 0, "50", 12.0);
    draw_symmetric_line_point(&mut context, 0, DVec2::new(40.0, 10.0), SNAP, PIXEL);

    assert_scale(&document, 50.0 / 12.0);
    let line = drawing(&document).segments()[0];
    let ends = [line.start, line.end].map(|end| drawing(&document).point(end));
    for expected in [DVec2::new(-2.0, 10.0), DVec2::new(22.0, 10.0)] {
        assert!(
            ends.iter().any(|end| end.distance(expected) < 1e-6),
            "an end should stand at {expected}, 12 units from the middle, and the ends are {ends:?}",
        );
    }
}
