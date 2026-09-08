use cao_prefs::{Edge, Item};

use crate::wording::command;

/// The only place an `Edge` is turned into a name.
///
/// The settings screen offers these five as the placements of the toolbar.
pub fn edge(edge: Edge) -> &'static str {
    match edge {
        Edge::Top => "En haut",
        Edge::Bottom => "En bas",
        Edge::Left => "À gauche",
        Edge::Right => "À droite",
        Edge::Floating => "Flottante",
    }
}

/// One entry of the toolbar tree, as the settings screen lists it.
///
/// A borrow rather than a `String`: the settings screen walks the whole tree
/// again on every frame it is open.
pub fn item(item: &Item) -> &str {
    match item {
        Item::Command(chosen) => command::label(*chosen),
        Item::Group { name, .. } => name,
        Item::Separator => "— séparateur —",
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use cao_prefs::Command;

    use super::*;

    #[test]
    fn no_two_edges_offered_by_the_settings_screen_read_the_same() {
        let mut seen: BTreeMap<&str, Edge> = BTreeMap::new();

        for offered in Edge::ALL {
            assert!(
                !edge(offered).is_empty(),
                "{offered:?} has no name, so the settings screen offers a blank",
            );
            if let Some(taken) = seen.insert(edge(offered), offered) {
                panic!(
                    "{taken:?} and {offered:?} both read {:?}: the settings screen \
                     offers two placements a user cannot tell apart",
                    edge(offered),
                );
            }
        }
    }

    #[test]
    fn an_entry_of_the_tree_reads_as_its_command_its_group_or_a_separator() {
        assert_eq!(
            item(&Item::Command(Command::Undo)),
            command::label(Command::Undo),
            "a command entry says what the command says",
        );
        assert_eq!(item(&Item::group("Dessin", Vec::new())), "Dessin");
        assert_eq!(item(&Item::Separator), "— séparateur —");
    }
}
