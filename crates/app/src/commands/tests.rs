//! What app · commands.rs is held to.
//!
//! Closes #210.
//! - `run` takes the part being worked on as one `SketchContext` instead of
//!   four parameters — no test: the signature is what the compiler holds, and
//!   the harness below could not be written against the old one without
//!   threading each of the four by hand
//! - `SketchContext` lives in `screens/`, not in `screens/viewport/` — no test:
//!   the path this file imports it from is the check, and it fails to compile
//!   if the type goes back
//! - a command clears the message the last one left in the title bar —
//!   `a_command_clears_the_message_the_last_one_left`
//! - `app.rs` stays under its line budget — no test: `LINE_BUDGET` in
//!   `architecture.rs` already weighs it, and it is not on the list of files
//!   over it
//!
//! Closes #319.
//! - each click on the three-state button moves to the next state —
//!   `the_matter_in_front_button_moves_the_open_sketch_to_the_next_state`
//!
//! Closes #580.
//! - the three levels give the gaps they promise: « Grossière » 0.1 mm,
//!   « Normale » 0.02 mm, « Fine » 0.005 mm, each picked asking the shell for
//!   an export at its gap —
//!   `each_stl_level_picked_asks_for_an_export_at_the_gap_it_promises`
//! - a drawing in units other than millimetres is written in millimetres: the
//!   export asked carries the part's millimetres per unit —
//!   `an_stl_export_asked_from_a_part_drawn_at_a_scale_carries_its_millimetres_per_unit`

use cao_part::{ExtrusionMode, Operation, PartDocument, PointRef};
use cao_prefs::Command;
use cao_render::MatterInFront;
use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};
use chrono::Utc;
use glam::DVec2;

use super::{ExportRequest, export_request, run};
use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;

#[test]
fn a_command_clears_the_message_the_last_one_left() {
    let mut document = PartDocument::new("part", Utc::now());
    let mut editor = SketchEditor {
        message: Some("left by the last command".into()),
        ..SketchEditor::default()
    };
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut part = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    run(
        Command::Undo,
        &mut part,
        &mut Ribbon::default(),
        &mut ViewportState::default(),
    );

    assert_eq!(
        editor.message, None,
        "an undo that had nothing to undo still leaves no old message standing"
    );
}

#[test]
fn the_matter_in_front_button_moves_the_open_sketch_to_the_next_state() {
    let mut document = PartDocument::new("part", Utc::now());
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut ribbon = Ribbon::default();
    let mut viewport = ViewportState::default();

    let mut states = Vec::new();
    for _ in 0..3 {
        let mut part = SketchContext {
            document: &mut document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };
        run(
            Command::NextMatterInFront,
            &mut part,
            &mut ribbon,
            &mut viewport,
        );
        states.push(editor.matter_in_front);
    }

    assert_eq!(
        states,
        [
            MatterInFront::SeeThrough,
            MatterInFront::Opaque,
            MatterInFront::Hidden
        ],
        "a click on the button does not move the sketch on from Hidden",
    );
}

/// A block 10 by 20 units raised 4, its first side typed `first_side` mm
/// before anything is raised when given: the value that sets the scale.
fn a_raised_block(first_side: Option<f64>) -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 20.0)),
        construction: false,
    });
    if let Some(millimetres) = first_side {
        document.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(0)),
            value: millimetres.into(),
            placement: None,
        });
    }
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 10.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    assert!(!document.body().is_empty(), "the block is raised");
    document
}

#[test]
fn each_stl_level_picked_asks_for_an_export_at_the_gap_it_promises() {
    let document = a_raised_block(None);

    for (command, promised) in [
        (Command::ExportStlCoarse, 0.1),
        (Command::ExportStlNormal, 0.02),
        (Command::ExportStlFine, 0.005),
    ] {
        let Some(ExportRequest { gap, .. }) = export_request(command, &document) else {
            panic!("{command:?} asks for no export");
        };
        assert!(
            (gap - promised).abs() < 1e-12,
            "{command:?} asks for {gap} mm, it promises {promised} mm",
        );
    }
    assert!(
        export_request(Command::Undo, &document).is_none(),
        "a command that is no export asks for none",
    );
}

#[test]
fn an_stl_export_asked_from_a_part_drawn_at_a_scale_carries_its_millimetres_per_unit() {
    let scaled = a_raised_block(Some(25.0));
    let unscaled = a_raised_block(None);
    assert!((scaled.scale() - 2.5).abs() < 1e-9, "{}", scaled.scale());

    for (document, expected) in [(&scaled, 2.5), (&unscaled, 1.0)] {
        let request =
            export_request(Command::ExportStlNormal, document).expect("an export is asked");
        assert!(
            (request.millimetres_per_unit - expected).abs() < 1e-9,
            "the export carries {} mm a unit, the part {expected}",
            request.millimetres_per_unit,
        );
    }
}
