use cao_prefs::Command;

/// The commands whose shortcut was pressed this frame.
///
/// Nothing is read while a text field has the keyboard: typing "50" into a
/// dimension must not also fire whatever those keys are bound to.
pub fn shortcuts_pressed(ui: &egui::Ui, settings: &cao_prefs::Settings) -> Vec<Command> {
    if ui.ctx().egui_wants_keyboard_input() {
        return Vec::new();
    }
    ui.input_mut(|input| {
        settings
            .shortcuts
            .bindings
            .iter()
            .filter(|(_, chord)| {
                to_egui_key(chord.key).is_some_and(|key| {
                    input.consume_shortcut(&egui::KeyboardShortcut::new(modifiers(*chord), key))
                })
            })
            .map(|(command, _)| *command)
            .collect()
    })
}

fn modifiers(chord: cao_prefs::Chord) -> egui::Modifiers {
    let mut modifiers = egui::Modifiers::NONE;
    if chord.command {
        modifiers = modifiers.plus(egui::Modifiers::COMMAND);
    }
    if chord.shift {
        modifiers = modifiers.plus(egui::Modifiers::SHIFT);
    }
    if chord.alt {
        modifiers = modifiers.plus(egui::Modifiers::ALT);
    }
    modifiers
}

fn to_egui_key(key: cao_prefs::Key) -> Option<egui::Key> {
    use cao_prefs::Key as K;
    use egui::Key as E;
    Some(match key {
        K::A => E::A,
        K::B => E::B,
        K::C => E::C,
        K::D => E::D,
        K::E => E::E,
        K::F => E::F,
        K::G => E::G,
        K::H => E::H,
        K::I => E::I,
        K::J => E::J,
        K::K => E::K,
        K::L => E::L,
        K::M => E::M,
        K::N => E::N,
        K::O => E::O,
        K::P => E::P,
        K::Q => E::Q,
        K::R => E::R,
        K::S => E::S,
        K::T => E::T,
        K::U => E::U,
        K::V => E::V,
        K::W => E::W,
        K::X => E::X,
        K::Y => E::Y,
        K::Z => E::Z,
        K::Num0 => E::Num0,
        K::Num1 => E::Num1,
        K::Num2 => E::Num2,
        K::Num3 => E::Num3,
        K::Num4 => E::Num4,
        K::Num5 => E::Num5,
        K::Num6 => E::Num6,
        K::Num7 => E::Num7,
        K::Num8 => E::Num8,
        K::Num9 => E::Num9,
        K::F1 => E::F1,
        K::F2 => E::F2,
        K::F3 => E::F3,
        K::F4 => E::F4,
        K::F5 => E::F5,
        K::F6 => E::F6,
        K::F7 => E::F7,
        K::F8 => E::F8,
        K::F9 => E::F9,
        K::F10 => E::F10,
        K::F11 => E::F11,
        K::F12 => E::F12,
        K::Escape => E::Escape,
        K::Tab => E::Tab,
        K::Space => E::Space,
        K::Enter => E::Enter,
        K::Backspace => E::Backspace,
        K::Delete => E::Delete,
        K::Home => E::Home,
        K::End => E::End,
        K::PageUp => E::PageUp,
        K::PageDown => E::PageDown,
        K::Left => E::ArrowLeft,
        K::Right => E::ArrowRight,
        K::Up => E::ArrowUp,
        K::Down => E::ArrowDown,
        K::Plus => E::Plus,
        K::Minus => E::Minus,
        K::Comma => E::Comma,
        K::Period => E::Period,
    })
}
