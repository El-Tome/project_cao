use cao_prefs::{Chord, Command, Key, Profiles};

use crate::lang::Catalogue;

use super::SettingsEditor;

pub(super) fn section(
    ui: &mut egui::Ui,
    profiles: &mut Profiles,
    editor: &mut SettingsEditor,
    lang: &Catalogue,
) -> bool {
    let mut touched = false;
    ui.label(lang.t("settings.shortcuts.intro"));
    ui.add_space(6.0);

    // A key pressed while recording is read here, before any widget sees it.
    let pressed = editor.recording.and_then(|_| read_chord(ui));
    if let (Some(command), Some(chord)) = (editor.recording, pressed) {
        profiles.active_mut().shortcuts.bind(command, chord);
        editor.recording = None;
        touched = true;
    }

    let mut family = None;
    let shortcuts = profiles.active().shortcuts.clone();
    for command in Command::ALL {
        if family != Some(command.family()) {
            family = Some(command.family());
            ui.add_space(8.0);
            ui.heading(crate::wording::command::family_heading(
                lang,
                command.family(),
            ));
        }
        ui.horizontal(|ui| {
            ui.label(crate::wording::command::label(lang, command));
            let recording = editor.recording == Some(command);
            let label = if recording {
                lang.t("settings.shortcuts.recording")
            } else {
                shortcuts
                    .chord_for(command)
                    .map(|chord| crate::wording::shortcuts::chord(lang, chord))
                    .unwrap_or_else(|| "—".to_string())
            };
            if ui.selectable_label(recording, label).clicked() {
                editor.recording = if recording { None } else { Some(command) };
            }
            if ui.button(lang.t("settings.shortcuts.unbind")).clicked() {
                profiles.active_mut().shortcuts.unbind(command);
                touched = true;
            }
        });
    }

    ui.add_space(10.0);
    if ui.button(lang.t("settings.shortcuts.reset")).clicked() {
        profiles.active_mut().shortcuts = Default::default();
        touched = true;
    }
    touched
}

/// The first key pressed this frame, with the modifiers held with it.
fn read_chord(ui: &egui::Ui) -> Option<Chord> {
    ui.input(|input| {
        let modifiers = input.modifiers;
        input.events.iter().find_map(|event| match event {
            egui::Event::Key {
                key,
                pressed: true,
                repeat: false,
                ..
            } => from_egui_key(*key).map(|key| Chord {
                key,
                command: modifiers.command,
                shift: modifiers.shift,
                alt: modifiers.alt,
            }),
            _ => None,
        })
    })
}

fn from_egui_key(key: egui::Key) -> Option<Key> {
    use egui::Key as E;
    Some(match key {
        E::A => Key::A,
        E::B => Key::B,
        E::C => Key::C,
        E::D => Key::D,
        E::E => Key::E,
        E::F => Key::F,
        E::G => Key::G,
        E::H => Key::H,
        E::I => Key::I,
        E::J => Key::J,
        E::K => Key::K,
        E::L => Key::L,
        E::M => Key::M,
        E::N => Key::N,
        E::O => Key::O,
        E::P => Key::P,
        E::Q => Key::Q,
        E::R => Key::R,
        E::S => Key::S,
        E::T => Key::T,
        E::U => Key::U,
        E::V => Key::V,
        E::W => Key::W,
        E::X => Key::X,
        E::Y => Key::Y,
        E::Z => Key::Z,
        E::Num0 => Key::Num0,
        E::Num1 => Key::Num1,
        E::Num2 => Key::Num2,
        E::Num3 => Key::Num3,
        E::Num4 => Key::Num4,
        E::Num5 => Key::Num5,
        E::Num6 => Key::Num6,
        E::Num7 => Key::Num7,
        E::Num8 => Key::Num8,
        E::Num9 => Key::Num9,
        E::F1 => Key::F1,
        E::F2 => Key::F2,
        E::F3 => Key::F3,
        E::F4 => Key::F4,
        E::F5 => Key::F5,
        E::F6 => Key::F6,
        E::F7 => Key::F7,
        E::F8 => Key::F8,
        E::F9 => Key::F9,
        E::F10 => Key::F10,
        E::F11 => Key::F11,
        E::F12 => Key::F12,
        E::Escape => Key::Escape,
        E::Tab => Key::Tab,
        E::Space => Key::Space,
        E::Enter => Key::Enter,
        E::Backspace => Key::Backspace,
        E::Delete => Key::Delete,
        E::Home => Key::Home,
        E::End => Key::End,
        E::PageUp => Key::PageUp,
        E::PageDown => Key::PageDown,
        E::ArrowLeft => Key::Left,
        E::ArrowRight => Key::Right,
        E::ArrowUp => Key::Up,
        E::ArrowDown => Key::Down,
        E::Plus => Key::Plus,
        E::Minus => Key::Minus,
        E::Comma => Key::Comma,
        E::Period => Key::Period,
        _ => return None,
    })
}
