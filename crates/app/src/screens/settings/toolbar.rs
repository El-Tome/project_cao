use cao_prefs::{Command, Edge, Item, Path, Profiles, ToolbarLayout};

use crate::lang::Catalogue;
use crate::ui::text_edit::text_edit;

use super::SettingsEditor;

pub(super) fn section(
    ui: &mut egui::Ui,
    profiles: &mut Profiles,
    editor: &mut SettingsEditor,
    lang: &Catalogue,
) -> bool {
    let before = profiles.active().toolbar.clone();
    let layout = &mut profiles.active_mut().toolbar;

    ui.horizontal_wrapped(|ui| {
        ui.label(lang.t("settings.toolbar.edge"));
        for edge in Edge::ALL {
            if ui
                .selectable_label(
                    layout.edge == edge,
                    crate::wording::toolbar::edge(lang, edge),
                )
                .clicked()
            {
                layout.edge = edge;
            }
        }
    });
    ui.horizontal(|ui| {
        ui.checkbox(&mut layout.show_logo, lang.t("settings.toolbar.logo"));
        text_edit(
            ui,
            &mut layout.logo_text,
            140.0,
            &lang.t("settings.toolbar.logo_text"),
        );
        ui.checkbox(&mut layout.show_labels, lang.t("settings.toolbar.labels"));
    });

    ui.add_space(8.0);
    ui.label(lang.t("settings.toolbar.intro"));
    ui.add_space(6.0);

    let selected = editor.selected.clone();
    ui.horizontal(|ui| {
        ui.add_enabled_ui(!selected.is_empty(), |ui| {
            if ui.button("↑").clicked()
                && let Some(moved) = layout.shift(&selected, false)
            {
                editor.selected = moved;
            }
            if ui.button("↓").clicked()
                && let Some(moved) = layout.shift(&selected, true)
            {
                editor.selected = moved;
            }
            if ui
                .button(lang.t("settings.toolbar.nest"))
                .on_hover_text(lang.t("settings.toolbar.nest_hint"))
                .clicked()
                && let Some(moved) = layout.nest(&selected)
            {
                editor.selected = moved;
            }
            if ui.button(lang.t("settings.toolbar.unnest")).clicked()
                && let Some(moved) = layout.unnest(&selected)
            {
                editor.selected = moved;
            }
            if ui.button(lang.t("settings.toolbar.remove")).clicked() {
                layout.remove(&selected);
                editor.selected.clear();
            }
        });
    });

    ui.horizontal(|ui| {
        text_edit(
            ui,
            &mut editor.new_group_name,
            140.0,
            &lang.t("settings.toolbar.group_name"),
        );
        if ui.button(lang.t("settings.toolbar.add_group")).clicked() {
            let name = if editor.new_group_name.trim().is_empty() {
                lang.t("settings.toolbar.new_group")
            } else {
                editor.new_group_name.trim().to_string()
            };
            layout.items.push(Item::group(&name, Vec::new()));
            editor.new_group_name.clear();
        }
        if ui
            .button(lang.t("settings.toolbar.add_separator"))
            .clicked()
        {
            layout.items.push(Item::Separator);
        }
        if ui.button(lang.t("settings.toolbar.rename")).clicked()
            && !editor.new_group_name.trim().is_empty()
        {
            layout.rename(&selected, editor.new_group_name.trim().to_string());
            editor.new_group_name.clear();
        }
        if ui.button(lang.t("settings.toolbar.reset")).clicked() {
            *layout = Default::default();
            editor.selected.clear();
        }
    });

    ui.separator();
    egui::ScrollArea::vertical()
        .max_height(220.0)
        .id_salt("toolbar_tree")
        .show(ui, |ui| {
            let items = layout.items.clone();
            tree(ui, &items, &mut Vec::new(), &mut editor.selected, lang);
        });

    ui.separator();
    ui.label(lang.t("settings.toolbar.add_command"));
    egui::ScrollArea::vertical()
        .max_height(160.0)
        .id_salt("command_palette")
        .show(ui, |ui| {
            let mut family = None;
            for command in Command::ALL {
                if family != Some(command.family()) {
                    family = Some(command.family());
                    ui.add_space(6.0);
                    ui.weak(crate::wording::command::family_heading(
                        lang,
                        command.family(),
                    ));
                }
                if ui
                    .button(crate::wording::command::label(lang, command))
                    .clicked()
                {
                    let into = group_path(layout, &editor.selected);
                    layout.push_into(&into, Item::Command(command));
                }
            }
        });

    before != profiles.active().toolbar
}

/// Where a new entry goes: into the selection when it is a group, otherwise
/// into the group holding it, otherwise at the root.
fn group_path(layout: &ToolbarLayout, selected: &[usize]) -> Path {
    if matches!(layout.at(selected), Some(Item::Group { .. })) {
        return selected.to_vec();
    }
    let mut parent = selected.to_vec();
    parent.pop();
    parent
}

fn tree(ui: &mut egui::Ui, items: &[Item], path: &mut Path, selected: &mut Path, lang: &Catalogue) {
    for (rank, item) in items.iter().enumerate() {
        path.push(rank);
        ui.horizontal(|ui| {
            ui.add_space(12.0 * (path.len() - 1) as f32);
            let name = crate::wording::toolbar::item(lang, item);
            if ui.selectable_label(*path == *selected, name).clicked() {
                *selected = path.clone();
            }
        });
        if let Item::Group { items, .. } = item {
            tree(ui, items, path, selected, lang);
        }
        path.pop();
    }
}
