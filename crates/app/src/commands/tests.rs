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
//!
//! Closes #193.
//! - « Vue en coupe » opens a section on XY through the middle of the part,
//!   one side opaque and the other hidden, and clicked again turns it off —
//!   `the_section_view_opens_a_section_through_the_middle_of_the_part_and_closes_it`
//! - the section view never touches the document: nothing enters the history,
//!   the `.caopart` is unchanged —
//!   `the_section_view_leaves_the_history_and_the_document_untouched`
//! - found in review: an undo that leaves the part with no matter turns the
//!   section off — `undoing_the_part_s_last_matter_away_turns_the_section_off`
//!
//! Closes #348 — the whole transcription is in
//! `crates/app/src/screens/viewport/render/tests.rs`; these bullets are the
//! ones a change to the history answers.
//! - found in review: the light goes out on undo and on redo —
//!   `an_undo_or_a_redo_puts_the_light_out`; a sketch drawn anew after an undo
//!   took one away is neither lit nor marked in the tree by its light —
//!   `a_sketch_drawn_anew_after_an_undo_takes_no_light_from_the_one_undone`
//! - found in review: the light goes out on compaction —
//!   `compacting_the_history_puts_the_light_out`; decided in review: compaction
//!   puts every eye back to its default —
//!   `compacting_the_history_puts_every_eye_back_to_its_default`
//!
//! Closes #728 — the whole transcription is in `crates/app/tests/view_bar.rs`;
//! this is the bullet a command answers. What « Recadrer » does to the camera
//! is held in `crates/app/src/screens/viewport/state/tests.rs`, the one module
//! that sees it.
//! - the ribbon's own « Matière devant » stays and keeps cycling —
//!   `the_matter_in_front_button_moves_the_open_sketch_to_the_next_state`,
//!   written for #319 and left as it is
//!
//! Closes #734 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests.rs`; this is the bullet a
//! change to the history answers.
//! - found in review: compaction puts every volume back to opaque, as it puts
//!   every eye back to its default, and changes no other —
//!   `compacting_the_history_puts_every_volume_back_to_opaque`

use cao_part::{ExtrusionMode, Operation, PartDocument, PointRef, VolumeState};
use cao_prefs::Command;
use cao_render::MatterInFront;
use cao_sketch::{DimensionTarget, Element, SegmentId, WorkPlane};
use chrono::Utc;
use glam::DVec2;

use super::{compact_history, run};
use crate::export::{ExportRequest, export_request};
use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::part_tree::state::{Points, marked};
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
        &mut crate::screens::blend::BlendState::default(),
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
            &mut crate::screens::blend::BlendState::default(),
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

/// Runs `command` on `document`, the way a click on its button does, and
/// says whether the part was left needing to be saved.
fn clicked(command: Command, document: &mut PartDocument, viewport: &mut ViewportState) -> bool {
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut part = SketchContext {
        document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    run(
        command,
        &mut part,
        &mut Ribbon::default(),
        viewport,
        &mut crate::screens::blend::BlendState::default(),
    )
}

#[test]
fn the_section_view_opens_a_section_through_the_middle_of_the_part_and_closes_it() {
    let mut document = a_raised_block(None);
    let mut viewport = ViewportState::default();

    clicked(Command::SectionView, &mut document, &mut viewport);
    let opened = viewport.section().map(|section| {
        (
            section.plane,
            section.offset,
            section.behind,
            section.in_front,
        )
    });
    clicked(Command::SectionView, &mut document, &mut viewport);

    let Some((plane, offset, behind, in_front)) = opened else {
        panic!("« Vue en coupe » turns no section on");
    };
    assert_eq!(
        plane,
        WorkPlane::XY,
        "the section opens on another plane than XY"
    );
    assert!(
        (offset - 2.0).abs() < 1e-9,
        "the section opens {offset} along Z, not through the middle of a block 4 high",
    );
    assert_eq!(
        (behind, in_front),
        (MatterInFront::Opaque, MatterInFront::Hidden),
        "the section opens with its sides set otherwise than one opaque, one hidden",
    );
    assert!(
        viewport.section().is_none(),
        "« Vue en coupe » clicked again leaves the section on",
    );
}

#[test]
fn the_section_view_leaves_the_history_and_the_document_untouched() {
    let mut document = a_raised_block(None);
    let before = document.clone();
    let mut viewport = ViewportState::default();

    let turned_on = clicked(Command::SectionView, &mut document, &mut viewport);
    let turned_off = clicked(Command::SectionView, &mut document, &mut viewport);

    assert!(
        !turned_on && !turned_off,
        "turning the section view on or off leaves the part needing to be saved",
    );
    assert_eq!(
        document.history, before.history,
        "the section view entered the history",
    );
    assert_eq!(
        document.body(),
        before.body(),
        "the section view changed the part itself",
    );
}

#[test]
fn undoing_the_part_s_last_matter_away_turns_the_section_off() {
    let mut document = a_raised_block(None);
    let mut viewport = ViewportState::default();
    clicked(Command::SectionView, &mut document, &mut viewport);
    assert!(
        viewport.section().is_some(),
        "« Vue en coupe » turns no section on"
    );

    clicked(Command::Undo, &mut document, &mut viewport);

    assert!(
        document.body().is_empty(),
        "the undo left matter in the part"
    );
    assert!(
        viewport.section().is_none(),
        "the part holds no matter any more, and a section is still on with nothing to cut",
    );
}

/// The block, and a rectangle drawn on a second sketch that nothing was
/// raised from.
fn a_raised_block_and_a_sketch_never_used() -> PartDocument {
    let mut document = a_raised_block(None);
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document
}

fn the_first_trait_of(sketch: usize) -> Points {
    Points::Element {
        sketch,
        element: Element::Segment(SegmentId(0)),
    }
}

#[test]
fn an_undo_or_a_redo_puts_the_light_out() {
    let mut document = a_raised_block_and_a_sketch_never_used();
    let mut viewport = ViewportState::default();

    viewport.light(the_first_trait_of(0));
    clicked(Command::Undo, &mut document, &mut viewport);
    assert_eq!(viewport.lit(), None, "an undo leaves a row of the tree lit");

    viewport.light(the_first_trait_of(0));
    assert!(
        clicked(Command::Redo, &mut document, &mut viewport),
        "the fixture has nothing to redo",
    );
    assert_eq!(viewport.lit(), None, "a redo leaves a row of the tree lit");
}

#[test]
fn a_sketch_drawn_anew_after_an_undo_takes_no_light_from_the_one_undone() {
    let mut document = a_raised_block_and_a_sketch_never_used();
    let mut viewport = ViewportState::default();
    viewport.light(the_first_trait_of(1));

    clicked(Command::Undo, &mut document, &mut viewport);
    clicked(Command::Undo, &mut document, &mut viewport);
    assert_eq!(
        document.sketches().len(),
        1,
        "two undos kept the second sketch"
    );
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::YZ,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(5.0, 5.0)),
        construction: false,
    });

    assert_eq!(
        viewport.lit(),
        None,
        "a sketch just drawn is lit by the light of the sketch undone",
    );
    assert!(
        !marked(
            &document,
            &SketchEditor::default(),
            &viewport,
            the_first_trait_of(1),
        ),
        "a trait of a sketch just drawn is marked in the tree by the light of the sketch undone",
    );
}

/// The history compacted, the way the panel of operations does it once its
/// warning is confirmed.
fn compacted(document: &mut PartDocument, viewport: &mut ViewportState) {
    compact_history(document, &mut SketchEditor::default(), viewport);
    assert_eq!(
        document.sketches().len(),
        2,
        "the compaction dropped a sketch the part still holds",
    );
}

#[test]
fn compacting_the_history_puts_the_light_out() {
    let mut document = a_raised_block_and_a_sketch_never_used();
    let mut viewport = ViewportState::default();
    viewport.light(the_first_trait_of(1));

    compacted(&mut document, &mut viewport);

    assert_eq!(
        viewport.lit(),
        None,
        "a compaction leaves a row of the tree lit"
    );
}

#[test]
fn compacting_the_history_puts_every_eye_back_to_its_default() {
    let mut document = a_raised_block_and_a_sketch_never_used();
    let mut viewport = ViewportState::default();
    viewport.toggle_eye(&document, 0);
    viewport.toggle_eye(&document, 1);
    assert!(
        viewport.eye_lit(&document, 0) && !viewport.eye_lit(&document, 1),
        "the fixture wants the used sketch's eye lit and the other's put out",
    );

    compacted(&mut document, &mut viewport);

    assert!(
        !viewport.eye_lit(&document, 0),
        "after a compaction, the sketch the block was raised from keeps the eye lit by hand",
    );
    assert!(
        viewport.eye_lit(&document, 1),
        "after a compaction, the sketch never used keeps the eye put out by hand",
    );
}

/// Where the volumes stand in the history: its extrusions.
fn volumes_of(document: &PartDocument) -> Vec<usize> {
    document
        .history
        .applied_operations()
        .iter()
        .enumerate()
        .filter(|(_, operation)| matches!(operation, Operation::Extrude { .. }))
        .map(|(step, _)| step)
        .collect()
}

#[test]
fn compacting_the_history_puts_every_volume_back_to_opaque() {
    let mut document = a_raised_block(None);
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(20.0, 0.0)),
        opposite: PointRef::New(DVec2::new(30.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(25.0, 10.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    let [first, second] = volumes_of(&document)[..] else {
        panic!("the fixture wants two blocks");
    };
    let mut viewport = ViewportState::default();
    viewport.next_volume_mode(&document, first);
    viewport.next_volume_mode(&document, first);
    assert_eq!(
        [first, second].map(|step| viewport.volume_mode(&document, step)),
        [VolumeState::Invisible, VolumeState::Opaque],
        "the fixture wants the first block hidden and the second left opaque",
    );

    compact_history(&mut document, &mut SketchEditor::default(), &mut viewport);

    let after: Vec<VolumeState> = volumes_of(&document)
        .into_iter()
        .map(|step| viewport.volume_mode(&document, step))
        .collect();
    assert_eq!(
        after,
        [VolumeState::Opaque, VolumeState::Opaque],
        "after a compaction, the hidden block stays hidden or the opaque one changes",
    );
}
