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
//!   confused — `a_measure_is_not_drawn_in_the_colour_a_dimension_drives_with`
//! - the label says what was read — no test: it is `egui` text, asserted
//!   through the wording in `wording/measure/tests.rs`

use cao_part::PartDocument;
use cao_part::history::{Operation, PointRef};
use cao_prefs::config::ViewportConfig;
use cao_prefs::theme::Theme;
use cao_render::OrbitCamera;
use cao_sketch::{DimensionTarget, PointId, ToolState, WorkPlane};
use chrono::Utc;

use super::*;
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
        tool: Tool::Measure,
        tool_state: ToolState::Measure {
            picks: cao_sketch::DimensionPicks::default(),
            showing,
        },
        ..SketchEditor::default()
    };
    (document, editor)
}

/// The steps the measure is drawn as, read back onto the sketch's plane.
fn steps(showing: Option<DimensionTarget>) -> Vec<(DVec2, DVec2)> {
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
    push_measure(&mut out, sketch, &context, &Theme::default(), a_view());
    let onto_the_plane = |vertex: &cao_render::Vertex| {
        sketch
            .plane
            .to_local(glam::Vec3::from(vertex.position).as_dvec3())
    };
    out.as_chunks::<2>()
        .0
        .iter()
        .map(|pair| (onto_the_plane(&pair[0]), onto_the_plane(&pair[1])))
        .collect()
}

fn a_view() -> ViewScale {
    ViewScale::of(
        &OrbitCamera::default(),
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0)),
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

    assert_ne!(
        theme.dimension_driven, theme.dimension,
        "a measure borrows the quieter colour, and is the wrong thing to draw \
         if the two are ever made the same",
    );
}
