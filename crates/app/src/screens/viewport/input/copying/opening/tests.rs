//! What app · screens/viewport/input/copying/opening.rs is held to.

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::{CircleId, DimensionTarget, WorkPlane};
use chrono::Utc;
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, SketchPhase, Tool};

/// A drawing holding one circle four units across, and nothing else.
fn a_circle_four_units_across() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(10.0, 10.0)),
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    document
}

/// The four fields as `Entrée` leaves them, that circle held.
fn fields_opened_by(tool: Tool, document: &mut PartDocument) -> Vec<Option<f64>> {
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool,
        tool_state: ToolState::Copying {
            held: vec![Element::Circle(CircleId(0))],
            naming_the_target: false,
        },
        ..Default::default()
    };
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    super::super::hold_is_done(&mut context);
    (0..4).map(|rank| context.editor.live.typed(rank)).collect()
}

#[test]
fn a_grid_opens_on_a_step_the_hold_fits_in_and_one_copy_each_way() {
    assert_eq!(
        fields_opened_by(Tool::RectangularPattern, &mut a_circle_four_units_across()),
        vec![Some(4.0), Some(2.0), Some(4.0), Some(2.0)]
    );
}

#[test]
fn a_circular_pattern_opens_on_a_quarter_turn_four_times_round() {
    assert_eq!(
        fields_opened_by(Tool::CircularPattern, &mut a_circle_four_units_across()),
        vec![Some(90.0), Some(4.0), None, None]
    );
}

#[test]
fn the_mirror_opens_no_fields_at_all() {
    assert_eq!(
        fields_opened_by(Tool::Mirror, &mut a_circle_four_units_across()),
        vec![None; 4]
    );
}

#[test]
fn a_step_opens_in_the_millimetres_its_field_is_labelled_with() {
    let mut document = a_circle_four_units_across();
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Diameter(CircleId(0)),
        value: 20.0.into(),
        placement: None,
    });

    let opened = fields_opened_by(Tool::RectangularPattern, &mut document);

    assert_eq!(
        opened[0],
        Some(20.0),
        "five millimetres to the unit, and the circle four units across"
    );
}
