//! What app · screens/viewport/input/arcs.rs is held to.
//!
//! Closes #478, for the arc.
//! - drawing an arc writes one step named "Arc", whatever it laid along the
//!   way — the values typed and the two construction radii of a swept angle —
//!   and one undo takes the whole of it back, one redo puts the whole of it
//!   back — `an_arc_its_values_and_the_radii_of_its_sweep_are_one_step_undone_whole`
//!
//! Closes #483, for what the tool lays.
//! - a clockwise arc gets its angle dimension, and its two construction radii,
//!   just as a counter-clockwise one does —
//!   `an_arc_swept_clockwise_with_a_typed_angle_keeps_that_angle_and_the_radii_of_its_sector`
//! - gone down, typed `270`: the arc the tool records runs three quarters
//!   clockwise — `an_arc_swept_clockwise_with_a_typed_angle_keeps_that_angle_and_the_radii_of_its_sector`
//! - the tool follows the hand between the second click and the third: gone
//!   down a quarter, it lays the short quarter clockwise, recorded
//!   counter-clockwise with its ends swapped —
//!   `the_arc_tool_lays_the_quarter_the_hand_went_down_through`
//! - what the canvas shows before the third click is the arc that click lays —
//!   `the_preview_shows_the_quarter_the_hand_went_down_through`
//! - the three places reusing a point already under them —
//!   `an_arc_swept_clockwise_hangs_on_the_three_points_already_under_it`

use cao_part::{PartDocument, Variables};
use cao_sketch::WorkPlane;
use chrono::Utc;

use super::super::following::follow_the_cursor;
use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, Tool};

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

const CENTRE: DVec2 = DVec2::new(10.0, 10.0);
const START: DVec2 = DVec2::new(30.0, 10.0);

fn on_the_circle_at(degrees: f64) -> DVec2 {
    CENTRE + DVec2::from_angle(degrees.to_radians()) * 20.0
}

fn hand_going_down_to(context: &mut SketchContext<'_>, degrees: f64) {
    let mut at = 0.0;
    while at > degrees {
        at = f64::max(at - 5.0, degrees);
        follow_the_cursor(context, 0, on_the_circle_at(at), PIXEL);
    }
}

fn centre_and_start_clicked(context: &mut SketchContext<'_>) {
    context.editor.tool = Tool::Arc;
    follow_the_cursor(context, 0, CENTRE, PIXEL);
    draw_arc(context, 0, CENTRE, SNAP, PIXEL);
    follow_the_cursor(context, 0, START, PIXEL);
    draw_arc(context, 0, START, SNAP, PIXEL);
}

fn assert_runs(drawn: ArcDraft, from: DVec2, to: DVec2, sweep_degrees: f64) {
    assert!(
        drawn.start.distance(from) < 1e-6
            && drawn.end.distance(to) < 1e-6
            && (cao_sketch::sweep_of(drawn).to_degrees() - sweep_degrees).abs() < 1e-6,
        "expected {sweep_degrees} degrees counter-clockwise from {from:?} to {to:?}, \
         got {} degrees from {:?} to {:?}",
        cao_sketch::sweep_of(drawn).to_degrees(),
        drawn.start,
        drawn.end,
    );
}

#[test]
fn the_arc_tool_lays_the_quarter_the_hand_went_down_through() {
    let mut document = a_part_with_a_drawing();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    centre_and_start_clicked(&mut context);
    hand_going_down_to(&mut context, -90.0);
    let laid = draw_arc(&mut context, 0, on_the_circle_at(-90.0), SNAP, PIXEL);

    assert!(laid, "the third click should lay the arc");
    let drawing = &document.sketches()[0];
    let (arc, _) = drawing.live_arcs().last().expect("an arc was laid");
    assert_runs(drawing.arc_draft(arc), on_the_circle_at(-90.0), START, 90.0);
}

#[test]
fn the_preview_shows_the_quarter_the_hand_went_down_through() {
    let mut document = a_part_with_a_drawing();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    centre_and_start_clicked(&mut context);
    hand_going_down_to(&mut context, -90.0);
    let shown = arc_preview(&context, on_the_circle_at(-90.0)).expect("an arc is being shown");

    assert_runs(shown, on_the_circle_at(-90.0), START, 90.0);
}

#[test]
fn an_arc_swept_clockwise_with_a_typed_angle_keeps_that_angle_and_the_radii_of_its_sector() {
    let mut document = a_part_with_a_drawing();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    centre_and_start_clicked(&mut context);
    hand_going_down_to(&mut context, -30.0);
    typed(&mut context, "270");
    draw_arc(&mut context, 0, on_the_circle_at(-30.0), SNAP, PIXEL);

    let drawing = &document.sketches()[0];
    let (arc, _) = drawing.live_arcs().last().expect("an arc was laid");
    assert_runs(drawing.arc_draft(arc), on_the_circle_at(90.0), START, 270.0);
    let sweep = drawing
        .dimensions()
        .iter()
        .find(|dimension| dimension.target == DimensionTarget::ArcSweep(arc))
        .expect("the typed angle should be left on the arc as its dimension");
    assert!(
        (sweep.value - 270.0).abs() < 1e-6,
        "the angle typed was 270, and the arc is dimensioned {}",
        sweep.value,
    );
    let radii: Vec<_> = drawing
        .live_segments()
        .filter(|(_, segment)| segment.construction)
        .map(|(_, segment)| (drawing.point(segment.start), drawing.point(segment.end)))
        .collect();
    let rim = drawing.arc_draft(arc);
    for end in [rim.start, rim.end] {
        assert!(
            radii
                .iter()
                .any(|&(from, to)| from.distance(CENTRE) < 1e-6 && to.distance(end) < 1e-6),
            "a construction radius from the centre to {end:?} should be left, among {radii:?}",
        );
    }
}

#[test]
fn an_arc_swept_clockwise_hangs_on_the_three_points_already_under_it() {
    let mut document = a_part_with_a_drawing();
    for position in [CENTRE, START, on_the_circle_at(-90.0)] {
        document.apply(Operation::AddPoint {
            sketch: 0,
            position,
            on: Vec::new(),
        });
    }
    let already_there: Vec<_> = document.sketches()[0].live_points().collect();
    let point_at = |place: DVec2| {
        already_there
            .iter()
            .find(|(_, at)| at.distance(place) < 1e-9)
            .map(|(id, _)| *id)
            .expect("a point was put there beforehand")
    };
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    centre_and_start_clicked(&mut context);
    hand_going_down_to(&mut context, -90.0);
    draw_arc(&mut context, 0, on_the_circle_at(-90.0), SNAP, PIXEL);

    let drawing = &document.sketches()[0];
    assert_eq!(
        drawing.live_points().count(),
        already_there.len(),
        "the arc stacked a point of its own on one already there",
    );
    let (_, arc) = drawing.live_arcs().last().expect("an arc was laid");
    assert_eq!(
        (arc.center, arc.start, arc.end),
        (
            point_at(CENTRE),
            point_at(on_the_circle_at(-90.0)),
            point_at(START),
        ),
        "a clockwise arc hangs on the centre, the point the hand went down to, and the start",
    );
}
