use cao_prefs::{Command, Edge, Item};

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
/// A standard group is stored under a key and translated here; a group the
/// user made or renamed carries their own words and is handed back untouched.
pub fn group(lang: &Catalogue, name: &str) -> String {
    match name {
        "sketch" | "new_sketch" => lang.t("toolbar.group.sketch"),
        "drawing" => lang.t("toolbar.group.drawing"),
        "dimensions" => lang.t("toolbar.group.dimensions"),
        "modify" => lang.t("toolbar.group.modify"),
        "circles" => lang.t("toolbar.group.circles"),
        "arcs" => lang.t("toolbar.group.arcs"),
        "ellipses" => lang.t("toolbar.group.ellipses"),
        "chamfers" => lang.t("toolbar.group.chamfers"),
        "constraints" => lang.t("toolbar.group.constraints"),
        "edit" => lang.t("toolbar.group.edit"),
        "extrusion" => lang.t("toolbar.group.extrusion"),
        "matter" => lang.t("toolbar.group.matter"),
        "shape" => lang.t("toolbar.group.shape"),
        "export" => lang.t("toolbar.group.export"),
        "formats" => lang.t("toolbar.group.formats"),
        "stl" => lang.t("toolbar.group.stl"),
        theirs => theirs.to_string(),
    }
}

/// What a line of the STL menu says about its level, on the line's right.
pub fn stl_level_hint(lang: &Catalogue, level: Command) -> Option<String> {
    let key = match level {
        Command::ExportStlCoarse => "toolbar.stl_level.coarse",
        Command::ExportStlNormal => "toolbar.stl_level.normal",
        Command::ExportStlFine => "toolbar.stl_level.fine",
        _ => return None,
    };
    Some(lang.t(key))
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
