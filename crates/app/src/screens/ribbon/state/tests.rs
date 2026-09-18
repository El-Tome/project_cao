//! What app · screens/ribbon/state.rs is held to.

use super::*;
use cao_sketch::WorkPlane;
use chrono::Utc;

fn a_part() -> PartDocument {
    PartDocument::new("part", Utc::now())
}

fn drawing_on_a_sketch() -> SketchEditor {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor
}

#[test]
fn a_drawing_tool_is_offered_only_while_a_sketch_is_open() {
    let part = a_part();
    let extrusion = ExtrusionState::default();

    assert!(!is_enabled(
        Command::ToolLine,
        &part,
        &SketchEditor::default(),
        &extrusion
    ));
    assert!(is_enabled(
        Command::ToolLine,
        &part,
        &drawing_on_a_sketch(),
        &extrusion
    ));
}

#[test]
fn a_fresh_part_has_nothing_to_undo_and_nothing_to_redo() {
    let part = a_part();
    let editor = SketchEditor::default();
    let extrusion = ExtrusionState::default();

    assert!(!is_enabled(Command::Undo, &part, &editor, &extrusion));
    assert!(!is_enabled(Command::Redo, &part, &editor, &extrusion));
    assert!(!is_enabled(
        Command::CompactHistory,
        &part,
        &editor,
        &extrusion
    ));
}

#[test]
fn an_extrusion_is_applied_only_once_it_holds_every_value_it_needs() {
    let part = a_part();
    let editor = SketchEditor::default();
    let ready = ExtrusionState::default();

    assert_eq!(
        is_enabled(Command::ExtrusionApply, &part, &editor, &ready),
        ready.is_ready(),
    );
}

#[test]
fn the_tool_in_hand_is_the_one_shown_as_pressed() {
    let part = a_part();
    let editor = drawing_on_a_sketch();
    let extrusion = ExtrusionState::default();
    let settings = Settings::default();
    let lang = Catalogue::french();
    let state = Context {
        settings: &settings,
        document: &part,
        editor: &editor,
        extrusion: &extrusion,
        lang: &lang,
    };

    assert!(active(Command::ToolLine, &state));
    assert!(!active(Command::ToolCircle, &state));
}
