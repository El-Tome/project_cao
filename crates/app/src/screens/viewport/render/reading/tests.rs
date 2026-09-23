//! What app · screens/viewport/render/reading.rs is held to.
//!
//! The vertices are read back onto the sketch's plane, which is what lets the
//! drawing of a measure be asserted with no window and no GPU: two vertices to
//! a straight step, and a step read back says where it was drawn.
//!
//! Closes #176.
//! - a measure is drawn as a dashed line between what was read —
//!   `a_measure_is_drawn_broken_into_dashes`
//! - the dashes run between the two places measured —
//!   `the_dashes_stand_between_the_two_places_measured`
//! - nothing is drawn while no measure is being shown —
//!   `nothing_is_drawn_while_no_measure_is_being_shown`
//! - a measure is drawn apart from a real dimension, so the two are never
//!   confused — `a_measure_is_not_drawn_in_the_colour_a_dimension_drives_with`,
//!   which reads the colour off the vertices rather than comparing two entries
//!   of the theme, since it is the drawing that has to differ
//! - the label says what was read —
//!   `the_label_says_the_distance_and_the_reach_along_each_axis`
//! - and it is beside the dashes, not floating clear of them —
//!   `the_label_stands_off_the_trait_as_far_as_the_dashes_do`

use cao_part::PartDocument;
use cao_part::history::{Operation, PointRef};
use cao_prefs::config::ViewportConfig;
use cao_prefs::theme::Theme;
use cao_render::OrbitCamera;
use cao_sketch::{DimensionTarget, PointId, ToolState, WorkPlane};
use chrono::Utc;

use super::*;

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, Tool};

/// Two points forty apart, and the editor showing the distance between them.
fn measuring(showing: Option<DimensionTarget>) -> (PartDocument, SketchEditor) {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 0.0)),
        end: PointRef::New(DVec2::new(40.0, 0.0)),
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
    drawn_at(showing, 1.0)
}

fn drawn_at(showing: Option<DimensionTarget>, pixels_per_point: f32) -> Vec<cao_render::Vertex> {
    let (mut document, mut editor) = measuring(showing);
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
        sketch,
        &context,
        &Theme::default(),
        a_view_at(pixels_per_point),
    );
    out
}

/// The same, read back onto the sketch's plane as the steps they draw.
fn steps(showing: Option<DimensionTarget>) -> Vec<(DVec2, DVec2)> {
    steps_at(showing, 1.0)
}

fn steps_at(showing: Option<DimensionTarget>, pixels_per_point: f32) -> Vec<(DVec2, DVec2)> {
    let onto_the_plane = |vertex: &cao_render::Vertex| {
        cao_sketch::WorkPlane::XY.to_local(glam::Vec3::from(vertex.position).as_dvec3())
    };
    drawn_at(showing, pixels_per_point)
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| (onto_the_plane(&pair[0]), onto_the_plane(&pair[1])))
        .collect()
}

fn a_view_at(pixels_per_point: f32) -> ViewScale {
    ViewScale::of(
        &OrbitCamera::default(),
        SCREEN,
        pixels_per_point,
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

#[test]
fn nothing_is_drawn_while_no_measure_is_being_shown() {
    assert!(
        steps(None).is_empty(),
        "a tool in hand with nothing read yet draws nothing",
    );
}

#[test]
fn a_measure_is_drawn_broken_into_dashes() {
    let drawn = steps(Some(the_distance()));

    assert!(!drawn.is_empty(), "the measure is drawn");
    let longest = drawn
        .iter()
        .map(|(from, to)| from.distance(*to))
        .fold(0.0_f64, f64::max);
    assert!(
        longest < 40.0,
        "a run of forty drawn as one step would be a solid line, not dashes: {longest}",
    );
}

#[test]
fn the_dashes_stand_between_the_two_places_measured() {
    let drawn = steps(Some(the_distance()));

    let along = drawn
        .iter()
        .flat_map(|(from, to)| [from.x, to.x])
        .fold(f64::NEG_INFINITY, f64::max);
    assert!(
        (0.0..=40.0).contains(&along),
        "the measure is drawn across the run it read, got as far as {along}",
    );
}

#[test]
fn a_measure_is_not_drawn_in_the_colour_a_dimension_drives_with() {
    let theme = Theme::default();
    let driving = cao_render::srgb(
        theme.dimension.r,
        theme.dimension.g,
        theme.dimension.b,
        theme.dimension.a,
    );

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
fn painted(showing: Option<DimensionTarget>, pixels_per_point: f32) -> Vec<egui::Shape> {
    let (mut document, mut editor) = measuring(showing);
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
    egui.set_pixels_per_point(pixels_per_point);
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
/// egui reports a text shape by its top-left corner whatever it was aligned
/// by, and a label three lines tall has its middle some twenty points below
/// that — which is the same order as the gap being measured here.
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
fn the_label_says_the_distance_and_the_reach_along_each_axis() {
    let said = words(&painted(Some(the_distance()), 1.0));

    assert_eq!(said.len(), 1, "one label, whatever it has to say: {said:?}");
    let lines: Vec<&str> = said[0].0.lines().collect();
    assert_eq!(
        lines.len(),
        3,
        "the distance and the reach along each axis: {lines:?}",
    );
    assert!(
        lines[0].contains("40"),
        "the trait is forty long, and that is the first thing said: {lines:?}",
    );
}

/// How far a place stands from a straight run, square onto it.
fn off_the_run(place: egui::Pos2, from: egui::Pos2, to: egui::Pos2) -> f32 {
    let span = to - from;
    let reach = span.length_sq();
    if reach == 0.0 {
        return place.distance(from);
    }
    let along = ((place - from).dot(span) / reach).clamp(0.0, 1.0);
    place.distance(from + span * along)
}

#[test]
fn the_label_stands_off_the_trait_as_far_as_the_dashes_do() {
    let state = ViewportState::default();
    let view_projection = state
        .camera
        .view_projection(SCREEN.width() / SCREEN.height());
    let plane = cao_sketch::WorkPlane::XY;
    let trait_at = |x: f64| {
        to_screen(plane.to_world(DVec2::new(x, 0.0)), view_projection, SCREEN)
            .expect("the trait is on screen")
    };
    let (start, end) = (trait_at(0.0), trait_at(40.0));

    // Both are placed by the same arithmetic — `offset_pixels` times a pixel
    // size — so they stand off the trait by the same amount or the two were
    // handed different pixel sizes. Checked at two screen scales because that
    // ratio is `pixels_per_point`: the mistake is nearly invisible at 1× and
    // gapes on the retina screen this is actually used on.
    for pixels_per_point in [1.0_f32, 2.0] {
        let dashes = steps_at(Some(the_distance()), pixels_per_point);
        let dash_standoff = dashes
            .iter()
            .flat_map(|(from, to)| [*from, *to])
            .filter_map(|place| to_screen(plane.to_world(place), view_projection, SCREEN))
            .map(|place| off_the_run(place, start, end))
            .fold(0.0_f32, f32::max);
        let said = words(&painted(Some(the_distance()), pixels_per_point));
        let (_, middle) = said.first().expect("a label was painted");
        let label_standoff = off_the_run(*middle, start, end);

        assert!(
            (label_standoff - dash_standoff).abs() < 3.0,
            "at {pixels_per_point}× the dashes stand {dash_standoff} points off \
             the trait and their label {label_standoff}; a number that far from \
             the run it belongs to is reading as nobody's",
        );
    }
}
