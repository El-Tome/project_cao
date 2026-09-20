//! What prefs · shortcuts.rs is held to.

use super::*;

#[test]
fn a_profile_saved_before_a_command_existed_still_gets_its_chord() {
    let mut saved = Shortcuts {
        bindings: vec![(Command::Undo, Chord::new(Key::Z).cmd())],
        ..Shortcuts::default()
    };

    saved.adopt_new_bindings(&Shortcuts::default());

    assert_eq!(
        saved.chord_for(Command::ToggleHistory),
        Some(Chord::new(Key::H)),
    );
    assert_eq!(
        saved.chord_for(Command::Undo),
        Some(Chord::new(Key::Z).cmd()),
        "what the user set stays put",
    );
}

#[test]
fn a_chord_the_user_gave_to_something_else_is_not_taken_back() {
    let mut saved = Shortcuts {
        bindings: vec![(Command::ToolLine, Chord::new(Key::H))],
        ..Shortcuts::default()
    };

    saved.adopt_new_bindings(&Shortcuts::default());

    assert_eq!(
        saved.command_for(Chord::new(Key::H)),
        Some(Command::ToolLine)
    );
    assert_eq!(
        saved.chord_for(Command::ToggleHistory),
        None,
        "a shortcut the user uses is broken for one they never asked for",
    );
}

#[test]
fn no_two_commands_answer_to_the_same_chord_on_a_fresh_installation() {
    let mut taken: Vec<(Chord, Command)> = Vec::new();

    for (command, chord) in Shortcuts::default().bindings {
        if let Some((_, first)) = taken.iter().find(|(bound, _)| *bound == chord) {
            panic!(
                "{chord:?} answers for both {first:?} and {command:?}: the second is unreachable and says nothing about why"
            );
        }
        taken.push((chord, command));
    }
}

#[test]
fn the_key_that_pulls_a_point_off_is_not_one_the_view_is_turned_with() {
    use crate::config::{NavigationPreset, PointerButton};

    let let_go = Shortcuts::default().let_go;

    for preset in [
        NavigationPreset::Fusion360,
        NavigationPreset::SolidWorks,
        NavigationPreset::Blender,
    ] {
        for binding in preset.orbit().iter().chain(preset.pan()) {
            let held = match let_go {
                Modifier::Command => binding.ctrl,
                Modifier::Shift => binding.shift,
                Modifier::Alt => binding.alt,
            };
            assert!(
                !(binding.button == PointerButton::Primary && held),
                "{binding:?} drags the view with the very key a point is pulled off with",
            );
        }
    }
}
