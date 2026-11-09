use cao_prefs::{Command, GroupSize, Item};
use egui::{Align, Label, Layout, RichText, Sense, Stroke, Ui, Vec2, vec2};

use crate::ui::theme::tokens::{BORDER_GROUP, TEXT_QUIET};
use crate::ui::tool_button::{Rows, small_grid};
use crate::wording::toolbar::group;

use super::{Context, Shown, button, menu};

const NAME_SIZE: f32 = 10.0;
/// 0.08 em of `NAME_SIZE`.
const NAME_LETTER_SPACING: f32 = 0.8;
const SEPARATOR_MARGIN: f32 = 10.0;
const SEPARATOR_HEIGHT: f32 = 64.0;

/// The depth of a group's own entries: a group inside it is a menu.
const INSIDE_A_GROUP: usize = 2;

/// The entries of the open tab along the bar, each group one block that goes
/// whole to the next line when it does not fit on this one.
pub(super) fn open_tab(ui: &mut Ui, items: &[Item], state: &Context<'_>, asked: &mut Vec<Command>) {
    ui.with_layout(wrapped_row(), |ui| {
        let bar = ui.max_rect().width();
        let mut after_a_group = false;
        for item in items {
            match item {
                Item::Command(command) => button(ui, *command, state, asked, Shown::Large),
                Item::Separator => separator(ui),
                Item::Group { name, size, items } => {
                    if after_a_group {
                        separator(ui);
                    }
                    one_block(ui, bar, |ui| {
                        named_group(ui, name, *size, items, state, asked);
                    });
                }
            }
            after_a_group = matches!(item, Item::Group { .. });
        }
    });
}

fn wrapped_row() -> Layout {
    Layout::left_to_right(Align::TOP).with_main_wrap(true)
}

/// Asks the row for the size this block came out at last frame, which is what
/// sends it whole to the next line, while letting it lay out over the whole
/// bar: laid out in what is left of a line, it would wrap inside itself, and
/// remember the narrower size it came out at.
fn one_block(ui: &mut Ui, bar: f32, add: impl FnOnce(&mut Ui)) {
    let id = ui.next_auto_id().with("ribbon_group_size");
    let last_frame: Vec2 = ui.data(|data| data.get_temp(id)).unwrap_or_default();
    let block = ui.allocate_ui_with_layout(last_frame, Layout::top_down(Align::LEFT), |ui| {
        ui.set_max_width(bar);
        add(ui);
    });
    let size = block.response.rect.size();
    ui.data_mut(|data| data.insert_temp(id, size));
}

fn named_group(
    ui: &mut Ui,
    name: &str,
    size: GroupSize,
    items: &[Item],
    state: &Context<'_>,
    asked: &mut Vec<Command>,
) {
    let buttons = ui
        .with_layout(wrapped_row(), |ui| match rows_of(size) {
            None => {
                for item in items {
                    entry(ui, item, Shown::Large, state, asked);
                }
            }
            Some(rows) => small_grid(ui, rows, items.len(), |ui, rank| {
                entry(ui, &items[rank], Shown::Small(rows), state, asked);
            }),
        })
        .response
        .rect;

    let written = RichText::new(group(state.lang, name).to_uppercase())
        .size(NAME_SIZE)
        .color(TEXT_QUIET)
        .extra_letter_spacing(NAME_LETTER_SPACING);
    ui.allocate_ui_with_layout(
        vec2(buttons.width(), 0.0),
        Layout::top_down(Align::Center),
        |ui| ui.add(Label::new(written).truncate()),
    );
}

fn rows_of(size: GroupSize) -> Option<Rows> {
    match size {
        GroupSize::Large => None,
        GroupSize::SmallTwoRows => Some(Rows::Two),
        GroupSize::SmallThreeRows => Some(Rows::Three),
    }
}

fn entry(ui: &mut Ui, item: &Item, shown: Shown, state: &Context<'_>, asked: &mut Vec<Command>) {
    match item {
        Item::Command(command) => button(ui, *command, state, asked, shown),
        Item::Separator => {
            ui.separator();
        }
        Item::Group { name, items, .. } => menu(ui, name, items, INSIDE_A_GROUP, state, asked),
    }
}

/// The rule between two groups. The row already spaces what stands beside it,
/// so it takes only what that spacing leaves of the margin.
fn separator(ui: &mut Ui) {
    let room = 2.0 * (SEPARATOR_MARGIN - ui.spacing().item_spacing.x) + 1.0;
    let (rect, _) = ui.allocate_exact_size(vec2(room, SEPARATOR_HEIGHT), Sense::hover());
    ui.painter().vline(
        rect.center().x,
        rect.y_range(),
        Stroke::new(1.0, BORDER_GROUP),
    );
}
