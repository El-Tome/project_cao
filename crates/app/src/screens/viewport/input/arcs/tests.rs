//! What app · screens/viewport/input/arcs.rs is held to.
//!
//! Closes #478, for the arc.
//! - drawing an arc writes one step named "Arc", whatever it laid along the
//!   way — the values typed and the two construction radii of a swept angle —
//!   and one undo takes the whole of it back, one redo puts the whole of it
//!   back — `an_arc_its_values_and_the_radii_of_its_sweep_are_one_step_undone_whole`

use cao_part::{PartDocument, Variables};
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
    arcs: usize,
    rules: usize,
    values: usize,
}

fn held(document: &PartDocument) -> Held {
    let drawing = &document.sketches()[0];
    Held {
        points: drawing.live_points().count(),
        traits: drawing.live_segments().count(),
        arcs: drawing.live_arcs().count(),
        rules: drawing.constraints().len(),
        values: drawing.dimensions().len(),
    }
}

fn typed(context: &mut SketchContext<'_>, text: &str) {
    context.editor.live.field(0).text = text.to_string();
    context.editor.live.take(0, &Variables::default(), None);
}

#[test]
fn an_arc_its_values_and_the_radii_of_its_sweep_are_one_step_undone_whole() {
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

    draw_arc(&mut context, 0, DVec2::new(10.0, 10.0), SNAP, PIXEL);
    typed(&mut context, "20");
    draw_arc(&mut context, 0, DVec2::new(30.0, 10.0), SNAP, PIXEL);
    typed(&mut context, "90");
    draw_arc(&mut context, 0, DVec2::new(10.0, 30.0), SNAP, PIXEL);

    let drawn = held(&document);
    assert_eq!(
        (drawn.arcs, drawn.traits, drawn.values),
        (1, 2, 2),
        "an arc, the two radii of its sweep and the two values typed should be laid",
    );
    assert_eq!(
        document.history.operations().len(),
        written + 1,
        "one arc was drawn, and the history grew by {:?}",
        &document.history.operations()[written..],
    );
    let step = document.history.operations().last().expect("a step");
    assert_eq!(
        crate::wording::history::label(&lang, document.variables(), step),
        "Arc",
    );

    assert!(document.undo(), "there was an arc to take back");
    assert_eq!(
        held(&document),
        before,
        "one undo left a piece of the arc behind",
    );

    assert!(document.redo(), "there was an arc to put back");
    assert_eq!(
        held(&document),
        drawn,
        "one redo put back only a piece of the arc",
    );
}
