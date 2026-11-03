//! Closes #639.
//! - a chord fires only on the modifiers it names: Cmd+Shift+Z redoes, Cmd+Z
//!   undoes, Shift+E cuts, E extrudes —
//!   `every_default_chord_fires_its_own_command_and_no_other` and
//!   `a_modifier_the_chord_does_not_name_fires_nothing`
//! - that holds whatever order the bindings are listed in, including bindings
//!   the user rebound — `the_order_of_the_bindings_decides_nothing`
//! - a test presses every default chord and checks the one command it fires —
//!   `every_default_chord_fires_its_own_command_and_no_other`

#[cfg(test)]
use cao_prefs::{Chord, Key, Settings, Shortcuts};

use super::*;

fn as_a_mac_and_a_pc_send(chord: Chord) -> [egui::Modifiers; 2] {
    let shared = egui::Modifiers {
        shift: chord.shift,
        alt: chord.alt,
        command: chord.command,
        ..Default::default()
    };
    [
        egui::Modifiers {
            mac_cmd: chord.command,
            ..shared
        },
        egui::Modifiers {
            ctrl: chord.command,
            ..shared
        },
    ]
}

fn fired(shortcuts: &Shortcuts, key: Key, held: egui::Modifiers) -> Vec<Command> {
    let settings = Settings {
        shortcuts: shortcuts.clone(),
        ..Settings::default()
    };
    let input = egui::RawInput {
        events: vec![egui::Event::Key {
            key: to_egui_key(key).expect("every key a chord can name has an egui key"),
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: held,
        }],
        ..Default::default()
    };
    let mut commands = Vec::new();
    let mut output = egui::Context::default().run_ui(input, |ui| {
        commands = shortcuts_pressed(ui, &settings, |_| false);
    });
    output.textures_delta.clear();
    commands
}

fn each_chord_fires_its_own(shortcuts: &Shortcuts) {
    for (command, chord) in &shortcuts.bindings {
        for held in as_a_mac_and_a_pc_send(*chord) {
            assert_eq!(
                fired(shortcuts, chord.key, held),
                vec![*command],
                "{chord:?} pressed as {held:?}"
            );
        }
    }
}

#[test]
fn every_default_chord_fires_its_own_command_and_no_other() {
    each_chord_fires_its_own(&Shortcuts::default());
}

#[test]
fn the_order_of_the_bindings_decides_nothing() {
    let mut shortcuts = Shortcuts::default();
    shortcuts.bindings.reverse();
    each_chord_fires_its_own(&shortcuts);

    let mut rebound = Shortcuts::default();
    rebound.bind(Command::ToolLine, Chord::new(Key::S).alt());
    each_chord_fires_its_own(&rebound);
}

#[test]
fn a_modifier_the_chord_does_not_name_fires_nothing() {
    let shortcuts = Shortcuts::default();
    for chord in [Chord::new(Key::F).shift(), Chord::new(Key::N).cmd().alt()] {
        for held in as_a_mac_and_a_pc_send(chord) {
            assert_eq!(
                fired(&shortcuts, chord.key, held),
                Vec::new(),
                "{chord:?} pressed as {held:?}"
            );
        }
    }
}
