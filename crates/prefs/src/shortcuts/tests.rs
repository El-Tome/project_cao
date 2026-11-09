//! What prefs · shortcuts.rs is held to.

use super::*;

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
