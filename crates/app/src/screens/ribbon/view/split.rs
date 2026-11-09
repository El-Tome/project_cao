use cao_prefs::{Command, Item};
use egui::{Rect, Response, Ui, UiBuilder, vec2};

use crate::ui::tool_button::{CHEVRON_WIDTH, chevron_button, menu_row, ribbon_menu, tooltip};
use crate::wording::{command as wording, toolbar};

use super::super::state::{Split, active, any_entry_enabled, enabled, icon, label};
use super::groups::Bar;
use super::{Shown, key_of, menu, named_button};

/// The body runs what the group's menu stands for; the chevron beside it opens
/// that menu. In a small column the chevron stands past the body, which keeps
/// the column's width for its key to end with the others.
pub(super) fn split_button(ui: &mut Ui, split: &Split<'_>, shown: Shown, bar: &mut Bar<'_>) {
    let entries = split.entries();
    let Some(primary) = bar.ribbon.primary_of(split.group, split.before, &entries) else {
        menu(ui, split.group, split.items, bar.state, bar.asked);
        return;
    };

    let group_name = toolbar::group(bar.state.lang, split.group);
    let (name, key) = match split.before {
        Some(command) => (label(command, bar.state), key_of(command, bar.state)),
        None => (group_name.clone(), None),
    };
    let chevron_name = if group_name == name {
        format!("{group_name} ⌄")
    } else {
        group_name.clone()
    };
    let openable = any_entry_enabled(&entries, bar.state);

    let mut body = |ui: &mut Ui| {
        named_button(
            ui,
            primary,
            &name,
            key.as_deref(),
            bar.state,
            bar.asked,
            shown,
        )
    };
    let chevron = match shown {
        Shown::Small(_) => {
            let body = body(ui);
            let at = Rect::from_min_size(
                body.rect.right_top(),
                vec2(CHEVRON_WIDTH, body.rect.height()),
            );
            let mut beside = ui.new_child(UiBuilder::new().max_rect(at));
            let chevron = chevron_button(&mut beside, openable, at.height(), &chevron_name);
            ui.expand_to_include_rect(chevron.rect);
            chevron
        }
        _ => {
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                let body = body(ui);
                chevron_button(ui, openable, body.rect.height(), &chevron_name)
            })
            .inner
        }
    };

    let chevron = tooltip(chevron, &group_name, None, None);
    show_menu(&chevron, split, bar);
}

fn show_menu(chevron: &Response, split: &Split<'_>, bar: &mut Bar<'_>) {
    ribbon_menu(chevron, |ui| {
        for item in split.items {
            match item {
                Item::Command(command) => menu_entry_row(ui, *command, split.group, bar),
                Item::Separator => {
                    ui.separator();
                }
                Item::Group { name, items, .. } => {
                    menu(ui, name, items, bar.state, bar.asked);
                }
            }
        }
    });
}

fn menu_entry_row(ui: &mut Ui, command: Command, group: &str, bar: &mut Bar<'_>) {
    let state = bar.state;
    let key = key_of(command, state);
    let hint = toolbar::stl_level_hint(state.lang, command).or_else(|| key.clone());
    let response = menu_row(
        ui,
        enabled(command, state),
        active(command, state),
        icon(command, state),
        &label(command, state),
        hint.as_deref(),
        key.as_deref(),
    )
    .on_hover_text(wording::hint(state.lang, command));
    if response.clicked() {
        bar.asked.push(command);
        bar.ribbon.used_from_menu(group, command);
    }
}
