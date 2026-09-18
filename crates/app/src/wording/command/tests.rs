//! What app · wording/command.rs is held to.

use std::collections::BTreeMap;

use super::*;

#[test]
fn no_two_commands_of_the_palette_read_the_same() {
    let lang = Catalogue::french();
    let mut seen: BTreeMap<String, Command> = BTreeMap::new();

    for command in Command::ALL {
        assert!(
            !label(&lang, command).is_empty(),
            "{command:?} has no name, so its button is a blank",
        );
        if let Some(taken) = seen.insert(label(&lang, command), command) {
            panic!(
                "{taken:?} and {command:?} both read {:?}: the palette offers two \
                 entries a user cannot tell apart",
                label(&lang, command),
            );
        }
    }
}

#[test]
fn resting_on_a_command_says_more_than_its_button_already_shows() {
    let lang = Catalogue::french();

    for command in Command::ALL {
        assert!(
            !hint(&lang, command).is_empty(),
            "{command:?} has nothing to say on hover",
        );
        assert_ne!(
            hint(&lang, command),
            label(&lang, command),
            "{command:?} repeats its own name on hover instead of helping",
        );
    }
}

#[test]
fn no_two_families_of_the_palette_are_headed_the_same() {
    let lang = Catalogue::french();
    let mut seen: BTreeMap<String, CommandFamily> = BTreeMap::new();

    for command in Command::ALL {
        let family = command.family();
        assert!(
            !family_heading(&lang, family).is_empty(),
            "{family:?} heads its run of the palette with a blank",
        );
        if let Some(taken) = seen.insert(family_heading(&lang, family), family) {
            assert_eq!(
                taken,
                family,
                "{taken:?} and {family:?} both head {:?}: the palette reads as \
                 one family cut in two",
                family_heading(&lang, family),
            );
        }
    }
}
