//! What the toolbar's labels say today, witnessed before they move up into
//! `cao_app`. Nothing here is a specification: it is what the settings screen
//! shows, so that the move can be read as a move.

use std::collections::BTreeMap;

use cao_prefs::{Command, Edge, Item};

#[test]
fn no_two_edges_offered_by_the_settings_screen_read_the_same() {
    let mut seen: BTreeMap<&str, Edge> = BTreeMap::new();

    for offered in Edge::ALL {
        assert!(
            !offered.label().is_empty(),
            "{offered:?} has no name, so the settings screen offers a blank",
        );
        if let Some(taken) = seen.insert(offered.label(), offered) {
            panic!(
                "{taken:?} and {offered:?} both read {:?}: the settings screen \
                 offers two placements a user cannot tell apart",
                offered.label(),
            );
        }
    }
}

#[test]
fn an_entry_of_the_tree_reads_as_its_command_its_group_or_a_separator() {
    assert_eq!(
        Item::Command(Command::Undo).label(),
        Command::Undo.label(),
        "a command entry says what the command says",
    );
    assert_eq!(Item::group("Dessin", Vec::new()).label(), "Dessin");
    assert_eq!(Item::Separator.label(), "— séparateur —");
}
