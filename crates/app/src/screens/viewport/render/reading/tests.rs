//! What app · screens/viewport/render/reading.rs is held to.
//!
//! The vertices are read back onto the sketch's plane, which is what lets the
//! drawing of a measure be asserted with no window and no GPU: two vertices to
//! a straight step, and a step read back says where it was drawn. The numbers
//! are read the same way, by asking `egui` for one pass and looking at the
//! shapes it hands back.
//!
//! Closes #425.
//! - the area read is tinted while the measure is on screen —
//!   `the_area_a_measure_reads_is_tinted_while_it_is_on_screen`
//! - it says its surface and how far round it is —
//!   `an_area_says_its_surface_and_how_far_round_it_is`
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
//! - a run a hair off an axis keeps all three of its numbers —
//!   `a_run_a_hair_off_an_axis_still_says_all_three_of_its_numbers`
//! - and two that would land on each other step onto two lines —
//!   `two_numbers_that_would_land_on_each_other_step_onto_two_lines`
//! - a reach of nothing at all is not written —
//!   `a_reach_of_nothing_at_all_is_not_written`
//! - a number only steps aside for a real overlap, not for a graze —
//!   `a_number_stands_on_the_side_it_measures`, which fails the moment a
//!   corner touching by a tenth of a point sends one a whole line down
//! - a run that genuinely leans is drawn as three sides —
//!   `a_run_that_genuinely_leans_still_comes_apart_into_three`
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
            showing: showing.map(cao_sketch::Measured::Of),
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
    push_measure(
        &mut out,
        &mut Vec::new(),
        sketch,
        &context,
        &Theme::default(),
        a_view(),
    );
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

/// Where every pill a frame painted landed.
fn pills(shapes: &[egui::Shape]) -> Vec<egui::Rect> {
    shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect) => Some(rect.rect),
            _ => None,
        })
        .collect()
}

#[test]
fn a_run_a_hair_off_an_axis_still_says_all_three_of_its_numbers() {
    for to in [A_HAIR_OFF_FLAT, A_HAIR_OFF_UPRIGHT] {
        let said = words(&painted_of(to, Some(the_distance())));

        assert_eq!(
            said.len(),
            3,
            "the short reach is small but it is not nothing, and a measure that \
             quietly drops a true value is worse than one that has to stack two \
             lines: {said:?}",
        );
        assert!(
            said.iter().any(|(number, _)| number.contains("0.3")),
            "the short reach is one of them: {said:?}",
        );
    }
}

#[test]
fn two_numbers_that_would_land_on_each_other_step_onto_two_lines() {
    for to in [A_HAIR_OFF_FLAT, A_HAIR_OFF_UPRIGHT] {
        let written = pills(&painted_of(to, Some(the_distance())));

        assert_eq!(written.len(), 3, "one pill per number");
        for (rank, pill) in written.iter().enumerate() {
            for other in &written[rank + 1..] {
                assert!(
                    !pill.intersects(*other),
                    "two numbers written over each other for the run {to:?}: \
                     {pill:?} and {other:?}",
                );
            }
        }
    }
}

#[test]
fn a_reach_of_nothing_at_all_is_not_written() {
    let said = words(&painted_of(DVec2::new(40.0, 0.0), Some(the_distance())));

    assert_eq!(
        said.len(),
        2,
        "square on the axis, the run's length already is its reach that way, \
         and \"0 mm\" underneath says nothing: {said:?}",
    );
    assert!(
        !said.iter().any(|(number, _)| number.starts_with('0')),
        "and the one left out is the empty one: {said:?}",
    );
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

/// A drawing holding a rectangle forty across and twenty-five up, and an
/// editor measuring the area inside it.
fn measuring_the_area_inside() -> (PartDocument, SketchEditor) {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let corners: Vec<PointRef> = [(0.0, 0.0), (40.0, 0.0), (40.0, 25.0), (0.0, 25.0)]
        .into_iter()
        .map(|(x, y)| {
            document.apply(Operation::AddPoint {
                sketch: 0,
                position: DVec2::new(x, y),
                on: Vec::new(),
            });
            PointRef::Existing(cao_sketch::PointId(
                document.sketches()[0].points().len() - 1,
            ))
        })
        .collect();
    for side in 0..corners.len() {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: corners[side].clone(),
            end: corners[(side + 1) % corners.len()].clone(),
            construction: false,
        });
    }
    let editor = SketchEditor {
        phase: crate::screens::sketch::SketchPhase::Editing(0),
        plane: Some(WorkPlane::XY),
        tool: Tool::Measure,
        tool_state: ToolState::Measure {
            picks: cao_sketch::DimensionPicks::default(),
            showing: Some(cao_sketch::Measured::Inside(DVec2::new(20.0, 12.0))),
        },
        ..SketchEditor::default()
    };
    (document, editor)
}

/// The surfaces one frame tinted, read back onto the sketch's plane as
/// triangles.
fn tinted() -> Vec<[DVec2; 3]> {
    let (mut document, mut editor) = measuring_the_area_inside();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let sketch = &context.document.sketches()[0];
    let mut surfaces = Vec::new();
    push_measure(
        &mut Vec::new(),
        &mut surfaces,
        sketch,
        &context,
        &Theme::default(),
        a_view(),
    );
    surfaces
        .as_chunks::<3>()
        .0
        .iter()
        .map(|corners| {
            corners
                .map(|vertex| WorkPlane::XY.to_local(glam::Vec3::from(vertex.position).as_dvec3()))
        })
        .collect()
}

#[test]
fn the_area_a_measure_reads_is_tinted_while_it_is_on_screen() {
    let lit = tinted();

    assert!(
        !lit.is_empty(),
        "two shapes one inside the other make it genuinely ambiguous which was \
         read, and a number with no shape under it answers for nothing",
    );
    let surface: f64 = lit
        .iter()
        .map(|[a, b, c]| (*b - *a).perp_dot(*c - *a).abs() / 2.0)
        .sum();
    assert!(
        (surface - 1000.0).abs() < 1e-6,
        "what is lit is the surface the number reports, got {surface}",
    );
}

#[test]
fn an_area_says_its_surface_and_how_far_round_it_is() {
    let (mut document, mut editor) = measuring_the_area_inside();
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

    let said = words(&flat);
    assert_eq!(said.len(), 1, "one label, two lines in it: {said:?}");
    assert!(
        said[0].0.contains("1000") && said[0].0.contains("mm²"),
        "the surface, in the square of the unit: {said:?}",
    );
    assert!(
        said[0].0.contains("130"),
        "and how far round it is: {said:?}",
    );
}
