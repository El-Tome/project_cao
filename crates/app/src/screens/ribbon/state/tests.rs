//! What app · screens/ribbon/state.rs is held to.
//!
//! Closes #319.
//! - the three-state button is shown only while a sketch is open, next to the
//!   re-align button — read, with the user, as a ribbon button beside
//!   « Recadrer » that is disabled outside a sketch rather than hidden, as
//!   « Recadrer » is —
//!   `the_matter_in_front_button_is_offered_only_while_a_sketch_is_open`
//! - its icon shows the current one — the ribbon draws labels, not icons, so it
//!   is the label that names the state —
//!   `the_matter_in_front_button_names_the_state_it_is_in` — and the two-letter
//!   face it falls back to with labels off, which must still tell the three
//!   apart — `with_labels_off_the_three_states_still_read_apart`,
//!   `with_labels_off_every_other_button_keeps_its_two_letters`
//!
//! Closes #580.
//! - the three levels are offered while the part holds matter and not before
//!   — `the_three_stl_levels_are_offered_only_while_the_part_holds_matter`
//! - they are one menu, « Exporter en STL… », under a tab « Export » of its
//!   own rather than inside « Extrusion » —
//!   `the_three_stl_levels_are_one_menu_under_an_export_tab_of_their_own`
//! - to try it: the file opens in a slicer, PrusaSlicer or Cura, at the right
//!   size — no test: a human opens the file in a slicer, which nothing in this
//!   workspace runs

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
        ready.is_ready(part.variables()),
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

#[test]
fn the_matter_in_front_button_is_offered_only_while_a_sketch_is_open() {
    let part = a_part();
    let extrusion = ExtrusionState::default();
    let mut closed = drawing_on_a_sketch();
    closed.close();

    assert!(!is_enabled(
        Command::NextMatterInFront,
        &part,
        &SketchEditor::default(),
        &extrusion
    ));
    assert!(is_enabled(
        Command::NextMatterInFront,
        &part,
        &drawing_on_a_sketch(),
        &extrusion
    ));
    assert!(
        !is_enabled(Command::NextMatterInFront, &part, &closed, &extrusion),
        "a closed sketch has no matter in front of it to show",
    );
}

#[test]
fn the_matter_in_front_button_names_the_state_it_is_in() {
    let part = a_part();
    let extrusion = ExtrusionState::default();
    let settings = Settings::default();
    let lang = Catalogue::french();
    let mut editor = drawing_on_a_sketch();

    let mut faces = Vec::new();
    for _ in 0..4 {
        let state = Context {
            settings: &settings,
            document: &part,
            editor: &editor,
            extrusion: &extrusion,
            lang: &lang,
        };
        faces.push(label(Command::NextMatterInFront, &state));
        editor.next_matter_in_front();
    }

    assert!(
        faces.iter().all(|face| face.contains("Matière devant")),
        "the button is not the one a hand looks for: {faces:?}",
    );
    assert!(
        faces[0] != faces[1] && faces[1] != faces[2] && faces[0] != faces[2],
        "two of the three states read alike: {faces:?}",
    );
    assert_eq!(
        faces[3], faces[0],
        "a third click is back on Hidden, and the button says something else",
    );
}

fn faces_of_the_three_states(face: fn(Command, &Context<'_>) -> String) -> Vec<String> {
    let part = a_part();
    let extrusion = ExtrusionState::default();
    let mut settings = Settings::default();
    settings.toolbar.show_labels = false;
    let lang = Catalogue::french();
    let mut editor = drawing_on_a_sketch();

    let mut faces = Vec::new();
    for _ in 0..3 {
        let state = Context {
            settings: &settings,
            document: &part,
            editor: &editor,
            extrusion: &extrusion,
            lang: &lang,
        };
        faces.push(face(Command::NextMatterInFront, &state));
        editor.next_matter_in_front();
    }
    faces
}

#[test]
fn with_labels_off_the_three_states_still_read_apart() {
    let faces = faces_of_the_three_states(short_label);

    assert!(
        faces[0] != faces[1] && faces[1] != faces[2] && faces[0] != faces[2],
        "with labels off, two of the three states wear the same face: {faces:?}",
    );
}

#[test]
fn with_labels_off_every_other_button_keeps_its_two_letters() {
    let part = a_part();
    let extrusion = ExtrusionState::default();
    let settings = Settings::default();
    let lang = Catalogue::french();
    let editor = drawing_on_a_sketch();
    let state = Context {
        settings: &settings,
        document: &part,
        editor: &editor,
        extrusion: &extrusion,
        lang: &lang,
    };

    for command in Command::ALL
        .into_iter()
        .filter(|command| *command != Command::NextMatterInFront)
    {
        let two_letters: String = label(command, &state).chars().take(2).collect();
        assert_eq!(
            short_label(command, &state),
            two_letters,
            "{command:?} changed face with labels off",
        );
    }
}

const STL_LEVELS: [Command; 3] = [
    Command::ExportStlCoarse,
    Command::ExportStlNormal,
    Command::ExportStlFine,
];

fn reached(items: &[cao_prefs::Item]) -> Vec<Command> {
    items
        .iter()
        .flat_map(|item| match item {
            cao_prefs::Item::Command(command) => vec![*command],
            cao_prefs::Item::Group { items, .. } => reached(items),
            cao_prefs::Item::Separator => Vec::new(),
        })
        .collect()
}

fn holds_the_levels_together(items: &[cao_prefs::Item]) -> bool {
    items.iter().any(|item| match item {
        cao_prefs::Item::Group { items, .. } => {
            STL_LEVELS
                .iter()
                .all(|level| items.contains(&cao_prefs::Item::Command(*level)))
                || holds_the_levels_together(items)
        }
        _ => false,
    })
}

fn a_raised_block() -> PartDocument {
    use cao_part::{ExtrusionMode, Operation, PointRef};
    use glam::DVec2;

    let mut document = a_part();
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
fn the_three_stl_levels_are_offered_only_while_the_part_holds_matter() {
    let editor = SketchEditor::default();
    let extrusion = ExtrusionState::default();
    let (empty, raised) = (a_part(), a_raised_block());

    for level in STL_LEVELS {
        assert!(
            !is_enabled(level, &empty, &editor, &extrusion),
            "{level:?} is offered on a part with no matter"
        );
        assert!(
            is_enabled(level, &raised, &editor, &extrusion),
            "{level:?} is not offered on a part holding a block"
        );
        assert!(
            is_enabled(level, &raised, &drawing_on_a_sketch(), &extrusion),
            "{level:?} is not offered while a sketch is open on a part holding a block"
        );
    }
}

fn tab<'a>(items: &'a [cao_prefs::Item], name: &str) -> Option<&'a [cao_prefs::Item]> {
    items.iter().find_map(|item| match item {
        cao_prefs::Item::Group { name: named, items } if named == name => Some(items.as_slice()),
        _ => None,
    })
}

#[test]
fn the_three_stl_levels_are_one_menu_under_an_export_tab_of_their_own() {
    let toolbar = Settings::default().toolbar;

    let export = tab(&toolbar.items, "export")
        .unwrap_or_else(|| panic!("the ribbon has no tab « export »: {:?}", toolbar.items));
    assert!(
        holds_the_levels_together(export),
        "the tab « export » holds the three levels as one menu: {export:?}",
    );
    let extrusion = tab(&toolbar.items, "extrusion").expect("the ribbon has its tab « extrusion »");
    let strays: Vec<Command> = reached(extrusion)
        .into_iter()
        .filter(|command| STL_LEVELS.contains(command))
        .collect();
    assert!(
        strays.is_empty(),
        "the tab « extrusion » still holds {strays:?}"
    );
}
