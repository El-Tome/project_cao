//! What app · wording/shortcuts.rs is held to.

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
