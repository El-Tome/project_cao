use cao_prefs::{Chord, Key};

/// The only place a `Key` is turned into a name.
///
/// The names are French and the arrows are the glyphs themselves. Private on
/// purpose: `&'static str` is what keeps this file clear of the interface, and
/// the day a translation system wants a `Cow` back there is one caller to
/// follow rather than a screen that reached in.
fn key(key: Key) -> &'static str {
    match key {
        Key::A => "A",
        Key::B => "B",
        Key::C => "C",
        Key::D => "D",
        Key::E => "E",
        Key::F => "F",
        Key::G => "G",
        Key::H => "H",
        Key::I => "I",
        Key::J => "J",
        Key::K => "K",
        Key::L => "L",
        Key::M => "M",
        Key::N => "N",
        Key::O => "O",
        Key::P => "P",
        Key::Q => "Q",
        Key::R => "R",
        Key::S => "S",
        Key::T => "T",
        Key::U => "U",
        Key::V => "V",
        Key::W => "W",
        Key::X => "X",
        Key::Y => "Y",
        Key::Z => "Z",
        Key::Num0 => "0",
        Key::Num1 => "1",
        Key::Num2 => "2",
        Key::Num3 => "3",
        Key::Num4 => "4",
        Key::Num5 => "5",
        Key::Num6 => "6",
        Key::Num7 => "7",
        Key::Num8 => "8",
        Key::Num9 => "9",
        Key::F1 => "F1",
        Key::F2 => "F2",
        Key::F3 => "F3",
        Key::F4 => "F4",
        Key::F5 => "F5",
        Key::F6 => "F6",
        Key::F7 => "F7",
        Key::F8 => "F8",
        Key::F9 => "F9",
        Key::F10 => "F10",
        Key::F11 => "F11",
        Key::F12 => "F12",
        Key::Escape => "Échap",
        Key::Tab => "Tab",
        Key::Space => "Espace",
        Key::Enter => "Entrée",
        Key::Backspace => "Retour arrière",
        Key::Delete => "Suppr",
        Key::Home => "Début",
        Key::End => "Fin",
        Key::PageUp => "Page préc.",
        Key::PageDown => "Page suiv.",
        Key::Left => "←",
        Key::Right => "→",
        Key::Up => "↑",
        Key::Down => "↓",
        Key::Plus => "+",
        Key::Minus => "-",
        Key::Comma => ",",
        Key::Period => ".",
    }
}

/// A chord as the settings screen and the toolbar show it.
pub fn chord(chord: Chord) -> String {
    let mut text = String::new();
    if chord.command {
        text.push_str("Cmd+");
    }
    if chord.shift {
        text.push_str("Maj+");
    }
    if chord.alt {
        text.push_str("Alt+");
    }
    text.push_str(key(chord.key));
    text
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn no_two_keys_offered_when_recording_read_the_same() {
        let mut seen: BTreeMap<&str, Key> = BTreeMap::new();

        for offered in Key::ALL {
            assert!(
                !key(offered).is_empty(),
                "{offered:?} has no name, so a shortcut using it reads as blank",
            );
            if let Some(taken) = seen.insert(key(offered), offered) {
                panic!(
                    "{taken:?} and {offered:?} both read {:?}: the settings screen \
                     shows two shortcuts a user cannot tell apart",
                    key(offered),
                );
            }
        }
    }

    #[test]
    fn a_chord_names_its_modifiers_before_its_key_and_always_in_the_same_order() {
        assert_eq!(chord(Chord::new(Key::Z)), "Z");
        assert_eq!(chord(Chord::new(Key::Z).cmd()), "Cmd+Z");
        assert_eq!(chord(Chord::new(Key::Z).shift()), "Maj+Z");
        assert_eq!(chord(Chord::new(Key::Z).alt()), "Alt+Z");
        assert_eq!(
            chord(Chord::new(Key::Z).alt().shift().cmd()),
            "Cmd+Maj+Alt+Z",
        );
    }
}
