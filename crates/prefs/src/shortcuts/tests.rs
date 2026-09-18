//! What prefs · shortcuts.rs is held to.

use super::*;

#[test]
fn a_profile_saved_before_a_command_existed_still_gets_its_chord() {
    let mut saved = Shortcuts {
        bindings: vec![(Command::Undo, Chord::new(Key::Z).cmd())],
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
