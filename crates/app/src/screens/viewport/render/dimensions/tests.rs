//! What app · screens/viewport/render/dimensions.rs is held to.
//!
//! Closes #724.
//! - found in review: every number in JetBrains Mono, as "To try it" promises —
//!   the value a dimension carries is written in the monospace family —
//!   `a_dimension_s_value_is_written_in_the_monospace_family`

use cao_part::history::PointRef;
use cao_part::{Operation, PartDocument};
use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

/// A plate forty by twenty whose bottom side is held at forty.
fn a_dimensioned_plate() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 40.0.into(),
        placement: None,
    });
    document
}

/// The text one pass of `egui` paints for the plate's dimensions, its sketch
/// open, each with the family it is written in.
fn the_plate_s_dimensions_written() -> Vec<(String, egui::FontFamily)> {
    let mut document = a_dimensioned_plate();
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
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
            .fade_in(false)
            .show(&egui, |ui| {
                paint_dimension_labels(ui, &state, SCREEN, &context, None);
            });
        output = egui.end_pass();
    }
    output.textures_delta.clear();
    let mut written = Vec::new();
    let mut pending: Vec<egui::Shape> = output
        .shapes
        .into_iter()
        .map(|clipped| clipped.shape)
        .collect();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(shapes) => pending.extend(shapes),
            egui::Shape::Text(text) => {
                for section in &text.galley.job.sections {
                    written.push((
                        text.galley.text().to_string(),
                        section.format.font_id.family.clone(),
                    ));
                }
            }
            _ => {}
        }
    }
    written
}

#[test]
fn a_dimension_s_value_is_written_in_the_monospace_family() {
    let written = the_plate_s_dimensions_written();

    assert!(
        written.iter().any(|(text, _)| holds_a_figure(text))
            && written
                .iter()
                .filter(|(text, _)| holds_a_figure(text))
                .all(|(_, family)| *family == egui::FontFamily::Monospace),
        "a dimension's value is written {written:?}: the figures go in the monospace family",
    );
}

/// Whether a piece of text holds a figure: a label may set a word beside it,
/// « Distance » or « R », and the word may stay in the proportional family.
fn holds_a_figure(text: &str) -> bool {
    text.chars().any(|character| character.is_ascii_digit())
}
