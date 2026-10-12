//! What app · screens/viewport/input/circles.rs is held to.
//!
//! Closes #478, for the circle.
//! - drawing a circle writes one step named "Cercle", whatever it laid along
//!   the way — the tangencies to the traits it was drawn against and the
//!   diameter typed — and one undo takes the whole of it back, one redo puts
//!   the whole of it back —
//!   `a_circle_its_tangencies_and_its_diameter_are_one_step_undone_whole`
//! - a shape drawn with nothing laid on it stays a plain operation rather than
//!   a gesture of one — `a_circle_with_nothing_laid_on_it_stays_a_plain_circle`

use cao_part::PartDocument;
use cao_part::history::PointRef;
use cao_sketch::WorkPlane;
use chrono::Utc;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

const SNAP: f64 = 0.5;
const PIXEL: f64 = 0.05;

/// A drawing holding the two traits of a corner, along the axes from the
/// origin.
fn a_part_with_a_corner() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for end in [DVec2::new(100.0, 0.0), DVec2::new(0.0, 100.0)] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(cao_sketch::PointId(0)),
            end: PointRef::New(end),
            construction: false,
        });
    }
    document
}

/// What the drawing holds, kind by kind: enough to tell a shape laid whole
/// from one laid in part.
#[derive(Debug, PartialEq)]
struct Held {
    points: usize,
    circles: usize,
    rules: usize,
    values: usize,
}

fn held(document: &PartDocument) -> Held {
    let drawing = &document.sketches()[0];
    Held {
        points: drawing.live_points().count(),
        circles: drawing.live_circles().count(),
        rules: drawing.constraints().len(),
        values: drawing.dimensions().len(),
    }
}

/// Clicks the circle tool at each place in turn, the fields standing on
/// `typed` before the last click.
fn clicked(
    document: &mut PartDocument,
    mode: CircleMode,
    places: &[DVec2],
    typed: Option<f64>,
) -> Catalogue {
    let mut editor = SketchEditor {
        circle_mode: mode,
        ..SketchEditor::default()
    };
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let (last, before) = places.split_last().expect("a click at least");
    for place in before {
        draw_circle(&mut context, 0, *place, SNAP, PIXEL);
    }
    context.editor.live.open_on(&[typed]);
    draw_circle(&mut context, 0, *last, SNAP, PIXEL);
    lang
}

#[test]
fn a_circle_its_tangencies_and_its_diameter_are_one_step_undone_whole() {
    let mut document = a_part_with_a_corner();
    let before = held(&document);
    let written = document.history.operations().len();

    let lang = clicked(
        &mut document,
        CircleMode::TwoTangents,
        &[
            DVec2::new(50.0, 0.0),
            DVec2::new(0.0, 50.0),
            DVec2::new(12.0, 12.0),
        ],
        Some(20.0),
    );

    let drawn = held(&document);
    assert_eq!(
        (drawn.circles, drawn.rules, drawn.values),
        (1, 2, 1),
        "a circle, its two tangencies and the diameter typed should be laid",
    );
    assert_eq!(
        document.history.operations().len(),
        written + 1,
        "one circle was drawn, and the history grew by {:?}",
        &document.history.operations()[written..],
    );
    let step = document.history.operations().last().expect("a step");
    assert_eq!(
        crate::wording::history::label(&lang, document.variables(), step),
        "Cercle",
    );

    assert!(document.undo(), "there was a circle to take back");
    assert_eq!(
        held(&document),
        before,
        "one undo left a piece of the circle behind",
    );

    assert!(document.redo(), "there was a circle to put back");
    assert_eq!(
        held(&document),
        drawn,
        "one redo put back only a piece of the circle",
    );
}

#[test]
fn a_circle_with_nothing_laid_on_it_stays_a_plain_circle() {
    let mut document = a_part_with_a_corner();

    clicked(
        &mut document,
        CircleMode::Center,
        &[DVec2::new(50.0, 50.0), DVec2::new(60.0, 50.0)],
        None,
    );

    assert!(
        matches!(
            document.history.operations().last(),
            Some(Operation::AddCircle { .. })
        ),
        "a circle with nothing on it should be written as itself, and is {:?}",
        document.history.operations().last(),
    );
}
