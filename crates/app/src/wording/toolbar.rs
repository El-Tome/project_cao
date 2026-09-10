use cao_prefs::{Edge, Item};

use crate::lang::Catalogue;
use crate::wording::command;

/// The only place an `Edge` is turned into a name.
///
/// The settings screen offers these five as the placements of the toolbar.
pub fn edge(lang: &Catalogue, edge: Edge) -> String {
    lang.t(match edge {
        Edge::Top => "toolbar.edge.top",
        Edge::Bottom => "toolbar.edge.bottom",
        Edge::Left => "toolbar.edge.left",
        Edge::Right => "toolbar.edge.right",
        Edge::Floating => "toolbar.edge.floating",
    })
}

/// The only place a group of the standard toolbar is turned into a name.
///
/// The standard groups are keyed, because a new command finds its place by
/// matching the key. A group the user made or renamed carries their own words
/// and is handed back untouched.
pub fn group(lang: &Catalogue, name: &str) -> String {
    match name {
        "sketch" => lang.t("toolbar.group.sketch"),
        "drawing" => lang.t("toolbar.group.drawing"),
        "circles" => lang.t("toolbar.group.circles"),
        "constraints" => lang.t("toolbar.group.constraints"),
        "edit" => lang.t("toolbar.group.edit"),
        "extrusion" => lang.t("toolbar.group.extrusion"),
        theirs => theirs.to_string(),
    }
}

/// One entry of the toolbar tree, as the settings screen lists it.
pub fn item(lang: &Catalogue, item: &Item) -> String {
    match item {
        Item::Command(chosen) => command::label(lang, *chosen),
        Item::Group { name, .. } => group(lang, name),
        Item::Separator => lang.t("toolbar.separator"),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use cao_prefs::{Command, ToolbarLayout};

    use super::*;

    #[test]
    fn no_two_edges_offered_by_the_settings_screen_read_the_same() {
        let lang = Catalogue::french();
        let mut seen: BTreeMap<String, Edge> = BTreeMap::new();

        for offered in Edge::ALL {
            assert!(
                !edge(&lang, offered).is_empty(),
                "{offered:?} has no name, so the settings screen offers a blank",
            );
            if let Some(taken) = seen.insert(edge(&lang, offered), offered) {
                panic!(
                    "{taken:?} and {offered:?} both read {:?}: the settings screen \
                     offers two placements a user cannot tell apart",
                    edge(&lang, offered),
                );
            }
        }
    }

    #[test]
    fn an_entry_of_the_tree_reads_as_its_command_its_group_or_a_separator() {
        let lang = Catalogue::french();

        assert_eq!(
            item(&lang, &Item::Command(Command::Undo)),
            command::label(&lang, Command::Undo),
            "a command entry says what the command says",
        );
        assert_eq!(item(&lang, &Item::group("drawing", Vec::new())), "Dessin");
        assert_eq!(item(&lang, &Item::Separator), "— séparateur —");
    }

    #[test]
    fn no_group_of_the_standard_toolbar_is_left_reading_as_its_key() {
        fn walk(items: &[Item], out: &mut Vec<String>) {
            for entry in items {
                if let Item::Group { name, items } = entry {
                    out.push(name.clone());
                    walk(items, out);
                }
            }
        }

        let lang = Catalogue::french();
        let mut keys = Vec::new();
        walk(&ToolbarLayout::default().items, &mut keys);

        assert!(!keys.is_empty(), "the standard toolbar holds groups");
        for key in keys {
            assert_ne!(
                group(&lang, &key),
                key,
                "the toolbar shows {key:?} as it is written in the profile, \
                 which is a key and not a name",
            );
        }
    }

    /// A group the user made, or renamed, carries their own words: the
    /// interface has nothing to say about it.
    #[test]
    fn a_group_the_user_named_reads_as_they_named_it() {
        assert_eq!(group(&Catalogue::french(), "Mes outils"), "Mes outils");
    }
}
