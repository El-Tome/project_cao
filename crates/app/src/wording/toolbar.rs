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
        "arcs" => lang.t("toolbar.group.arcs"),
        "chamfers" => lang.t("toolbar.group.chamfers"),
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
mod tests;
