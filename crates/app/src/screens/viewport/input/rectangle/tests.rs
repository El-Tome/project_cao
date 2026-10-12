//! What app · screens/viewport/input/rectangle.rs is held to.
//!
//! Closes #478, for the rectangle.
//! - drawing a rectangle writes one step named "Rectangle", whatever it laid
//!   along the way — its three right angles and the two sizes typed — and one
//!   undo takes the whole of it back, one redo puts the whole of it back —
//!   `a_rectangle_its_right_angles_and_its_sizes_are_one_step_undone_whole`
//! - the line, the symmetric line and the ellipse keep their one step — no
//!   test: `a_line_drawn_at_a_typed_angle_is_one_step_of_the_history` and the
//!   two ellipse tests the issue names hold it where they stand, untouched
//! - replaying the history still rebuilds the rectangle and its scale — no
//!   test: `replaying_the_history_rebuilds_the_rectangle_and_its_scale` holds
//!   it beside `values.rs`, and replays a rectangle that is now one gesture
//! - a typed value the drawing refuses still shows its message — no test: the
//!   outcome is read from each `apply` as it is laid, and the fold only
//!   rewrites the history afterwards. The circle and the arc showed none
//!   before this either: their own prompt overwrites it, which is not this
//!   issue's to change

use cao_part::PartDocument;
use cao_sketch::WorkPlane;
use chrono::Utc;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

const SNAP: f64 = 0.5;
const PIXEL: f64 = 0.05;

fn a_part_with_a_drawing() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document
}

/// What the drawing holds, kind by kind: enough to tell a shape laid whole
/// from one laid in part.
#[derive(Debug, PartialEq)]
struct Held {
    points: usize,
    traits: usize,
    rules: usize,
    values: usize,
}

fn held(document: &PartDocument) -> Held {
    let drawing = &document.sketches()[0];
    Held {
        points: drawing.live_points().count(),
        traits: drawing.live_segments().count(),
        rules: drawing.constraints().len(),
        values: drawing.dimensions().len(),
    }
}

#[test]
fn a_rectangle_its_right_angles_and_its_sizes_are_one_step_undone_whole() {
    let mut document = a_part_with_a_drawing();
    let before = held(&document);
    let written = document.history.operations().len();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    two_click_shape(&mut context, 0, DVec2::ZERO, SNAP, PIXEL);
    context.editor.live.open_on(&[Some(40.0), Some(20.0)]);
    let corner = rectangle_corner(&context, DVec2::new(30.0, 30.0));
    two_click_shape(&mut context, 0, corner, SNAP, PIXEL);

    let drawn = held(&document);
    assert_eq!(
        (drawn.traits, drawn.rules, drawn.values),
        (4, 3, 2),
        "four sides, three right angles and the two sizes typed should be laid",
    );
    assert_eq!(
        document.history.operations().len(),
        written + 1,
        "one rectangle was drawn, and the history grew by {:?}",
        &document.history.operations()[written..],
    );
    let step = document.history.operations().last().expect("a step");
    assert_eq!(
        crate::wording::history::label(&lang, document.variables(), step),
        "Rectangle",
    );

    assert!(document.undo(), "there was a rectangle to take back");
    assert_eq!(
        held(&document),
        before,
        "one undo left a piece of the rectangle behind",
    );

    assert!(document.redo(), "there was a rectangle to put back");
    assert_eq!(
        held(&document),
        drawn,
        "one redo put back only a piece of the rectangle",
    );
}
