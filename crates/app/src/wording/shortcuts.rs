use cao_prefs::{Chord, Key};

use crate::lang::Catalogue;

/// The only place a `Key` is turned into a name.
///
/// A letter, a digit and an arrow are said by the language file like the rest,
/// even where the entry repeats the key: a keyboard whose legends differ has
/// one file to change rather than a rule about which keys were left out.
/// Private on purpose — the screens show chords, never a key on its own.
fn key(lang: &Catalogue, key: Key) -> String {
    lang.t(match key {
        Key::A => "shortcuts.key.a",
        Key::B => "shortcuts.key.b",
        Key::C => "shortcuts.key.c",
        Key::D => "shortcuts.key.d",
        Key::E => "shortcuts.key.e",
        Key::F => "shortcuts.key.f",
        Key::G => "shortcuts.key.g",
        Key::H => "shortcuts.key.h",
        Key::I => "shortcuts.key.i",
        Key::J => "shortcuts.key.j",
        Key::K => "shortcuts.key.k",
        Key::L => "shortcuts.key.l",
        Key::M => "shortcuts.key.m",
        Key::N => "shortcuts.key.n",
        Key::O => "shortcuts.key.o",
        Key::P => "shortcuts.key.p",
        Key::Q => "shortcuts.key.q",
        Key::R => "shortcuts.key.r",
        Key::S => "shortcuts.key.s",
        Key::T => "shortcuts.key.t",
        Key::U => "shortcuts.key.u",
        Key::V => "shortcuts.key.v",
        Key::W => "shortcuts.key.w",
        Key::X => "shortcuts.key.x",
        Key::Y => "shortcuts.key.y",
        Key::Z => "shortcuts.key.z",
        Key::Num0 => "shortcuts.key.num0",
        Key::Num1 => "shortcuts.key.num1",
        Key::Num2 => "shortcuts.key.num2",
        Key::Num3 => "shortcuts.key.num3",
        Key::Num4 => "shortcuts.key.num4",
        Key::Num5 => "shortcuts.key.num5",
        Key::Num6 => "shortcuts.key.num6",
        Key::Num7 => "shortcuts.key.num7",
        Key::Num8 => "shortcuts.key.num8",
        Key::Num9 => "shortcuts.key.num9",
        Key::F1 => "shortcuts.key.f1",
        Key::F2 => "shortcuts.key.f2",
        Key::F3 => "shortcuts.key.f3",
        Key::F4 => "shortcuts.key.f4",
        Key::F5 => "shortcuts.key.f5",
        Key::F6 => "shortcuts.key.f6",
        Key::F7 => "shortcuts.key.f7",
        Key::F8 => "shortcuts.key.f8",
        Key::F9 => "shortcuts.key.f9",
        Key::F10 => "shortcuts.key.f10",
        Key::F11 => "shortcuts.key.f11",
        Key::F12 => "shortcuts.key.f12",
        Key::Escape => "shortcuts.key.escape",
        Key::Tab => "shortcuts.key.tab",
        Key::Space => "shortcuts.key.space",
        Key::Enter => "shortcuts.key.enter",
        Key::Backspace => "shortcuts.key.backspace",
        Key::Delete => "shortcuts.key.delete",
        Key::Home => "shortcuts.key.home",
        Key::End => "shortcuts.key.end",
        Key::PageUp => "shortcuts.key.page_up",
        Key::PageDown => "shortcuts.key.page_down",
        Key::Left => "shortcuts.key.left",
        Key::Right => "shortcuts.key.right",
        Key::Up => "shortcuts.key.up",
        Key::Down => "shortcuts.key.down",
        Key::Plus => "shortcuts.key.plus",
        Key::Minus => "shortcuts.key.minus",
        Key::Comma => "shortcuts.key.comma",
        Key::Period => "shortcuts.key.period",
    })
}

/// A chord as the settings screen and the toolbar show it.
pub fn chord(lang: &Catalogue, chord: Chord) -> String {
    let mut text = String::new();
    for (held, modifier) in [
        (chord.command, "shortcuts.modifier.command"),
        (chord.shift, "shortcuts.modifier.shift"),
        (chord.alt, "shortcuts.modifier.alt"),
    ] {
        if held {
            text.push_str(&lang.t(modifier));
            text.push('+');
        }
    }
    text.push_str(&key(lang, chord.key));
    text
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn no_two_keys_offered_when_recording_read_the_same() {
        let lang = Catalogue::french();
        let mut seen: BTreeMap<String, Key> = BTreeMap::new();

        for offered in Key::ALL {
            assert!(
                !key(&lang, offered).is_empty(),
                "{offered:?} has no name, so a shortcut using it reads as blank",
            );
            if let Some(taken) = seen.insert(key(&lang, offered), offered) {
                panic!(
                    "{taken:?} and {offered:?} both read {:?}: the settings screen \
                     shows two shortcuts a user cannot tell apart",
                    key(&lang, offered),
                );
            }
        }
    }

    #[test]
    fn a_chord_names_its_modifiers_before_its_key_and_always_in_the_same_order() {
        let lang = Catalogue::french();

        assert_eq!(chord(&lang, Chord::new(Key::Z)), "Z");
        assert_eq!(chord(&lang, Chord::new(Key::Z).cmd()), "Cmd+Z");
        assert_eq!(chord(&lang, Chord::new(Key::Z).shift()), "Maj+Z");
        assert_eq!(chord(&lang, Chord::new(Key::Z).alt()), "Alt+Z");
        assert_eq!(
            chord(&lang, Chord::new(Key::Z).alt().shift().cmd()),
            "Cmd+Maj+Alt+Z",
        );
    }
}
