//! What app · screens/viewport/render/reading.rs is held to.
//!
//! The vertices are read back onto the sketch's plane, which is what lets the
//! drawing of a measure be asserted with no window and no GPU: two vertices to
//! a straight step, and a step read back says where it was drawn. The numbers
//! are read the same way, by asking `egui` for one pass and looking at the
//! shapes it hands back.
//!
//! Closes #176.
//! - a straight run is drawn as a right triangle standing on it —
//!   `a_run_is_drawn_as_a_right_triangle_standing_on_it`
//! - each reach is drawn in the colour of the axis it runs along —
//!   `each_reach_is_drawn_in_the_colour_of_the_axis_it_runs_along`
//! - the sides are broken into dashes, so a measure reads as a measure and not
//!   as geometry — `a_measure_is_drawn_broken_into_dashes`
//! - each side carries its own number —
//!   `each_side_of_the_triangle_carries_its_own_number`
//! - and carries it on the side it measures —
//!   `a_number_stands_on_the_side_it_measures`
//! - the numbers are in front of the drawing rather than merely painted after
//!   it — `every_number_sits_on_a_pill_so_the_drawing_cannot_swallow_it`
//! - a run square to an axis is drawn as itself rather than as a sliver —
//!   `a_run_square_to_an_axis_is_drawn_as_itself_and_not_as_a_sliver`
//! - and says one number rather than two on the same spot —
//!   `a_run_square_to_an_axis_says_one_number_rather_than_two_on_the_same_line`
//! - a run that genuinely leans still comes apart —
//!   `a_run_that_genuinely_leans_still_comes_apart_into_three`
//! - the sides and the numbers never disagree about which of the two it is —
//!   `the_lines_and_the_numbers_never_disagree_about_whether_there_is_a_triangle`
//! - nothing is drawn while no measure is being shown —
//!   `nothing_is_drawn_while_no_measure_is_being_shown`
//! - a measure is drawn apart from a real dimension, so the two are never
//!   confused — `a_measure_is_not_drawn_in_the_colour_a_dimension_drives_with`,
//!   which reads the colour off the vertices rather than comparing two entries
//!   of the theme, since it is the drawing that has to differ

use cao_part::PartDocument;
use cao_part::history::{Operation, PointRef};
use cao_prefs::config::ViewportConfig;
use cao_prefs::theme::{Rgba, Theme};
use cao_render::OrbitCamera;
use cao_sketch::{DimensionTarget, PointId, ToolState, WorkPlane};
use chrono::Utc;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, Tool};

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

/// A trait running thirty across and forty up, so its three sides are fifty,
/// thirty and forty and no two of them can be taken for each other.
const ACROSS: f64 = 30.0;
const UP: f64 = 40.0;

/// A drawing holding that trait, and an editor showing the distance between
/// its two ends.
fn measuring_a_run(to: DVec2, showing: Option<DimensionTarget>) -> (PartDocument, SketchEditor) {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::ZERO),
        end: PointRef::New(to),
        construction: false,
    });
    let editor = SketchEditor {
        phase: crate::screens::sketch::SketchPhase::Editing(0),
        plane: Some(WorkPlane::XY),
        tool: Tool::Measure,
        tool_state: ToolState::Measure {
            picks: cao_sketch::DimensionPicks::default(),
            showing,
        },
        ..SketchEditor::default()
    };
    (document, editor)
}

/// The vertices the measure is drawn as, two to a straight step.
fn drawn(showing: Option<DimensionTarget>) -> Vec<cao_render::Vertex> {
    drawn_of(DVec2::new(ACROSS, UP), showing)
}

fn drawn_of(to: DVec2, showing: Option<DimensionTarget>) -> Vec<cao_render::Vertex> {
    let (mut document, mut editor) = measuring_a_run(to, showing);
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let sketch = &context.document.sketches()[0];
    let mut out = Vec::new();
    push_measure(&mut out, sketch, &context, &Theme::default(), a_view());
    out
}

/// The same, read back onto the sketch's plane as the steps they draw, each
/// with the colour it was drawn in.
fn steps(showing: Option<DimensionTarget>) -> Vec<(DVec2, DVec2, [f32; 4])> {
    let onto_the_plane = |vertex: &cao_render::Vertex| {
        WorkPlane::XY.to_local(glam::Vec3::from(vertex.position).as_dvec3())
    };
    drawn(showing)
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            (
                onto_the_plane(&pair[0]),
                onto_the_plane(&pair[1]),
                pair[0].color,
            )
        })
        .collect()
}

fn a_view() -> ViewScale {
    ViewScale::of(
        &OrbitCamera::default(),
        SCREEN,
        1.0,
        &ViewportConfig::default(),
        1.0,
    )
}

fn the_distance() -> DimensionTarget {
    DimensionTarget::Distance {
        from: PointId(1),
        to: PointId(2),
    }
}

fn shade(colour: Rgba) -> [f32; 4] {
    cao_render::srgb(colour.r, colour.g, colour.b, colour.a)
}

/// How far the steps drawn in one colour reach, end to end — the length of
/// that side of the triangle, whatever the dashes broke it into.
///
/// A dash is what it measures to within one dash: the walk stops at the end of
/// the side, so the last dash is cut short and the drawn extent falls a little
/// under the true length. [`A_DASH`] is the slack that leaves.
const A_DASH: f64 = 2.0;

fn side_in(colour: Rgba) -> f64 {
    let wanted = shade(colour);
    let places: Vec<DVec2> = steps(Some(the_distance()))
        .into_iter()
        .filter(|(.., drawn)| *drawn == wanted)
        .flat_map(|(from, to, _)| [from, to])
        .collect();
    let mut reach = 0.0_f64;
    for from in &places {
        for to in &places {
            reach = reach.max(from.distance(*to));
        }
    }
    reach
}

#[test]
fn nothing_is_drawn_while_no_measure_is_being_shown() {
    assert!(
        steps(None).is_empty(),
        "a tool in hand with nothing read yet draws nothing",
    );
}

#[test]
fn a_run_is_drawn_as_a_right_triangle_standing_on_it() {
    let theme = Theme::default();

    let span = side_in(theme.measure);
    let across = side_in(theme.axis_x);
    let up = side_in(theme.axis_y);

    assert!(
        (span - 50.0).abs() < A_DASH,
        "the hypotenuse is the run that was read, got {span}",
    );
    assert!(
        (across - ACROSS).abs() < A_DASH && (up - UP).abs() < A_DASH,
        "the two legs are the reaches themselves, got {across} across and {up} up",
    );
    assert!(
        (across * across + up * up - span * span).abs() < 200.0,
        "the three sides make a right triangle, which is the whole reason the \
         two reaches can be read off the shape instead of off a block of text",
    );
}

#[test]
fn each_reach_is_drawn_in_the_colour_of_the_axis_it_runs_along() {
    let theme = Theme::default();

    for (from, to, colour) in steps(Some(the_distance())) {
        if colour == shade(theme.axis_x) {
            assert!(
                (from.y - to.y).abs() < 1e-9,
                "the side wearing the horizontal axis's colour has to run that \
                 way, or the colour tells the user the wrong thing",
            );
        }
        if colour == shade(theme.axis_y) {
            assert!(
                (from.x - to.x).abs() < 1e-9,
                "and the one wearing the vertical axis's colour, that way",
            );
        }
    }
}

#[test]
fn a_measure_is_drawn_broken_into_dashes() {
    let drawn = steps(Some(the_distance()));

    assert!(!drawn.is_empty(), "the measure is drawn");
    let longest = drawn
        .iter()
        .map(|(from, to, _)| from.distance(*to))
        .fold(0.0_f64, f64::max);
    assert!(
        longest < ACROSS,
        "the shortest side is thirty, so a step longer than that is a solid \
         line rather than a dash: {longest}",
    );
}

#[test]
fn a_measure_is_not_drawn_in_the_colour_a_dimension_drives_with() {
    let theme = Theme::default();
    let driving = shade(theme.dimension);

    let colours: Vec<[f32; 4]> = drawn(Some(the_distance()))
        .iter()
        .map(|vertex| vertex.color)
        .collect();

    assert!(!colours.is_empty(), "the measure is drawn");
    assert!(
        colours.iter().all(|colour| *colour != driving),
        "a dimension is a promise the drawing is held to and a measure is a \
         glance; drawn in the same colour the two are the same thing to look at",
    );
}

/// Everything one frame painted, with no window and no GPU: egui is asked to
/// run a single pass and hand back the shapes it would have sent out.
fn painted(showing: Option<DimensionTarget>) -> Vec<egui::Shape> {
    painted_of(DVec2::new(ACROSS, UP), showing)
}

fn painted_of(to: DVec2, showing: Option<DimensionTarget>) -> Vec<egui::Shape> {
    let (mut document, mut editor) = measuring_a_run(to, showing);
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let state = ViewportState::default();
    let egui = egui::Context::default();
    let mut output = egui::FullOutput::default();
    // The first pass has no size for the area yet and lays nothing down; the
    // second paints against what the first worked out.
    for _ in 0..2 {
        output.textures_delta.clear();
        egui.begin_pass(egui::RawInput {
            screen_rect: Some(SCREEN),
            ..Default::default()
        });
        egui::Area::new(egui::Id::new("the canvas"))
            .fixed_pos(SCREEN.min)
            .show(&egui, |ui| paint_measure(ui, &state, SCREEN, &context));
        output = egui.end_pass();
    }
    output.textures_delta.clear();
    let mut flat = Vec::new();
    for clipped in output.shapes {
        flatten(clipped.shape, &mut flat);
    }
    flat
}

fn flatten(shape: egui::Shape, out: &mut Vec<egui::Shape>) {
    match shape {
        egui::Shape::Vec(shapes) => {
            for shape in shapes {
                flatten(shape, out);
            }
        }
        shape => out.push(shape),
    }
}

/// What each piece of text a frame painted said, and where its middle landed.
///
/// egui reports a text shape by its top-left corner, and a number's middle is
/// half its size below that — the same order as the distances measured here.
fn words(shapes: &[egui::Shape]) -> Vec<(String, egui::Pos2)> {
    shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(text) => Some((
                text.galley.text().to_string(),
                text.pos + text.galley.size() / 2.0,
            )),
            _ => None,
        })
        .collect()
}

#[test]
fn each_side_of_the_triangle_carries_its_own_number() {
    let said = words(&painted(Some(the_distance())));

    assert_eq!(
        said.len(),
        3,
        "one number per side, and no block of text left over: {said:?}",
    );
    let numbers: Vec<&str> = said.iter().map(|(text, _)| text.as_str()).collect();
    assert!(
        numbers.iter().any(|said| said.contains("50")),
        "the hypotenuse says the distance: {numbers:?}",
    );
    assert!(
        numbers.iter().any(|said| said.contains("30"))
            && numbers.iter().any(|said| said.contains("40")),
        "and each leg its own reach: {numbers:?}",
    );
}

#[test]
fn a_number_stands_on_the_side_it_measures() {
    let state = ViewportState::default();
    let view_projection = state
        .camera
        .view_projection(SCREEN.width() / SCREEN.height());
    let onto_the_screen = |place: DVec2| {
        to_screen(WorkPlane::XY.to_world(place), view_projection, SCREEN)
            .expect("the drawing is on screen")
    };
    let middles = [
        DVec2::new(ACROSS, UP) / 2.0,
        DVec2::new(ACROSS / 2.0, 0.0),
        DVec2::new(ACROSS, UP / 2.0),
    ]
    .map(onto_the_screen);

    for (number, at) in words(&painted(Some(the_distance()))) {
        let nearest = middles
            .iter()
            .map(|middle| middle.distance(at))
            .fold(f32::INFINITY, f32::min);
        assert!(
            nearest < 25.0,
            "{number:?} landed {nearest} points from the middle of any side; a \
             number that is not on what it measures is the block of text this \
             was meant to replace",
        );
    }
}

#[test]
fn every_number_sits_on_a_pill_so_the_drawing_cannot_swallow_it() {
    let shapes = painted(Some(the_distance()));
    let pills: Vec<&egui::epaint::RectShape> = shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect) => Some(rect),
            _ => None,
        })
        .collect();

    assert_eq!(
        pills.len(),
        3,
        "one behind each number: painting the text last puts it after the \
         drawing, not in front of it — over an ellipse or a filled area the \
         lines still run through the digits",
    );
    for (number, at) in words(&shapes) {
        assert!(
            pills.iter().any(|pill| pill.rect.contains(at)),
            "{number:?} has no pill under it",
        );
    }
}

/// A trait a hair off the horizontal: forty across and a third of a millimetre
/// up, which is about half a degree.
const A_HAIR_OFF_FLAT: DVec2 = DVec2::new(40.0, 0.35);
const A_HAIR_OFF_UPRIGHT: DVec2 = DVec2::new(0.35, 40.0);

/// The colours the sides of a run were drawn in, each once.
fn colours_of(to: DVec2) -> Vec<[f32; 4]> {
    let mut seen: Vec<[f32; 4]> = Vec::new();
    for vertex in drawn_of(to, Some(the_distance())) {
        if !seen.contains(&vertex.color) {
            seen.push(vertex.color);
        }
    }
    seen
}

#[test]
fn a_run_square_to_an_axis_is_drawn_as_itself_and_not_as_a_sliver() {
    for to in [A_HAIR_OFF_FLAT, A_HAIR_OFF_UPRIGHT] {
        let colours = colours_of(to);

        assert_eq!(
            colours,
            vec![shade(Theme::default().measure)],
            "the run {to:?} is square to an axis to within half a degree: its \
             long reach lies along it and the short one is nothing, so a \
             triangle is one line drawn three times over",
        );
    }
}

#[test]
fn a_run_square_to_an_axis_says_one_number_rather_than_two_on_the_same_line() {
    for to in [A_HAIR_OFF_FLAT, A_HAIR_OFF_UPRIGHT] {
        let said = words(&painted_of(to, Some(the_distance())));

        assert_eq!(
            said.len(),
            1,
            "its length is its reach along that axis, and two names for one \
             measurement land on top of each other: {said:?}",
        );
        assert!(
            said[0].0.contains("40"),
            "and the one it says is the length: {said:?}",
        );
    }
}

/// Right on the threshold: a reach of exactly the eighteen pixels a triangle
/// needs, worked out from the view the tests draw through.
fn a_run_on_the_very_edge() -> DVec2 {
    DVec2::new(40.0, a_view().units_per_pixel * 18.0)
}

#[test]
fn the_lines_and_the_numbers_never_disagree_about_whether_there_is_a_triangle() {
    // A run either side of the threshold, and one sitting on it: the shape is
    // decided twice — once pushing vertices, once painting text — and the two
    // readings drifting apart is what leaves a number with no side under it.
    let edge = a_run_on_the_very_edge();
    for to in [
        edge,
        DVec2::new(edge.x, edge.y * 0.9),
        DVec2::new(edge.x, edge.y * 1.1),
    ] {
        assert_eq!(
            colours_of(to).len(),
            words(&painted_of(to, Some(the_distance()))).len(),
            "a side drawn with no number on it, or a number with no side, for \
             the run {to:?}",
        );
    }
}

#[test]
fn a_run_that_genuinely_leans_still_comes_apart_into_three() {
    let colours = colours_of(DVec2::new(ACROSS, UP));

    assert_eq!(
        colours.len(),
        3,
        "a run leaning well off both axes has two reaches worth reading, and \
         dropping them would cost the whole point of the triangle",
    );
}
