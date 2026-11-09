use cao_prefs::{Command, GroupSize, Item};
use egui::{Align, Label, Layout, RichText, Sense, Stroke, Ui, Vec2, vec2};

use crate::ui::theme::tokens::{BORDER_GROUP, TEXT_QUIET};
use crate::ui::tool_button::{Rows, small_grid};
use crate::wording::toolbar::group;

use super::super::state::{Ribbon, takes_a_menu};
use super::split::{Split, split_button};
use super::{Context, SPREAD_OUT, Shown, button, menu};

const NAME_SIZE: f32 = 10.0;
/// 0.08 em of `NAME_SIZE`.
const NAME_LETTER_SPACING: f32 = 0.8;
const SEPARATOR_MARGIN: f32 = 10.0;
const SEPARATOR_HEIGHT: f32 = 64.0;

/// The depth of a group's own entries: a group inside it is a menu.
const INSIDE_A_GROUP: usize = SPREAD_OUT + 1;

/// What every entry of the bar needs in order to be drawn and to answer.
pub(super) struct Bar<'a> {
    pub(super) state: &'a Context<'a>,
    pub(super) asked: &'a mut Vec<Command>,
    pub(super) ribbon: &'a mut Ribbon,
}

/// An entry of the tree as the bar draws it: a command and the menu right
/// after it are one split button.
enum Run<'a> {
    Plain(&'a Item),
    Split(Split<'a>),
}

fn runs(items: &[Item], depth: usize) -> Vec<Run<'_>> {
    let mut runs = Vec::new();
    let mut rest = items.iter().peekable();
    while let Some(item) = rest.next() {
        if let Some(split) = Split::of(item, None, depth) {
            runs.push(Run::Split(split));
            continue;
        }
        let before = match item {
            Item::Command(command) if takes_a_menu(*command) => Some(*command),
            _ => None,
        };
        let paired = before.and_then(|_| Split::of(rest.peek()?, before, depth));
        match paired {
            Some(split) => {
                rest.next();
                runs.push(Run::Split(split));
            }
            None => runs.push(Run::Plain(item)),
        }
    }
    runs
}

/// The entries of the open tab along the bar, each group one block that goes
/// whole to the next line when it does not fit on this one.
pub(super) fn open_tab(
    ui: &mut Ui,
    items: &[Item],
    state: &Context<'_>,
    asked: &mut Vec<Command>,
    ribbon: &mut Ribbon,
) {
    let mut bar = Bar {
        state,
        asked,
        ribbon,
    };
    ui.with_layout(wrapped_row(), |ui| {
        let bar_width = ui.max_rect().width();
        let mut after_a_group = false;
        for run in runs(items, SPREAD_OUT) {
            match &run {
                Run::Split(split) => {
                    one_block(ui, bar_width, |ui| {
                        split_button(ui, split, Shown::Large, &mut bar);
                    });
                }
                Run::Plain(Item::Command(command)) => {
                    button(ui, *command, bar.state, bar.asked, Shown::Large);
                }
                Run::Plain(Item::Separator) => separator(ui),
                Run::Plain(Item::Group { name, size, items }) => {
                    if after_a_group {
                        separator(ui);
                    }
                    one_block(ui, bar_width, |ui| {
                        named_group(ui, name, *size, items, &mut bar);
                    });
                }
            }
            after_a_group = matches!(run, Run::Plain(Item::Group { .. }));
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
fn one_block(ui: &mut Ui, bar_width: f32, add: impl FnOnce(&mut Ui)) {
    let id = ui.next_auto_id().with("ribbon_group_size");
    let last_frame: Vec2 = ui.data(|data| data.get_temp(id)).unwrap_or_default();
    let block = ui.allocate_ui_with_layout(last_frame, Layout::top_down(Align::LEFT), |ui| {
        ui.set_max_width(bar_width);
        add(ui);
    });
    let size = block.response.rect.size();
    ui.data_mut(|data| data.insert_temp(id, size));
}

fn named_group(ui: &mut Ui, name: &str, size: GroupSize, items: &[Item], bar: &mut Bar<'_>) {
    let runs = runs(items, INSIDE_A_GROUP);
    let buttons = ui
        .with_layout(wrapped_row(), |ui| match rows_of(size) {
            None => {
                let bar_width = ui.max_rect().width();
                for run in &runs {
                    entry(ui, run, Shown::Large, bar_width, bar);
                }
            }
            Some(rows) => small_grid(ui, rows, runs.len(), |ui, rank| {
                entry(ui, &runs[rank], Shown::Small(rows), 0.0, bar);
            }),
        })
        .response
        .rect;

    let written = RichText::new(group(bar.state.lang, name).to_uppercase())
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
        GroupSize::Large | GroupSize::Menu => None,
        GroupSize::SmallTwoRows => Some(Rows::Two),
        GroupSize::SmallThreeRows => Some(Rows::Three),
    }
}

/// A split in a small column stands in the column's own `ui`, so that its key
/// ends with the others; a large one is one block to its row.
fn entry(ui: &mut Ui, run: &Run<'_>, shown: Shown, bar_width: f32, bar: &mut Bar<'_>) {
    match run {
        Run::Split(split) => match shown {
            Shown::Small(_) => split_button(ui, split, shown, bar),
            _ => one_block(ui, bar_width, |ui| split_button(ui, split, shown, bar)),
        },
        Run::Plain(Item::Command(command)) => {
            button(ui, *command, bar.state, bar.asked, shown);
        }
        Run::Plain(Item::Separator) => {
            ui.separator();
        }
        Run::Plain(Item::Group { name, items, .. }) => {
            menu(ui, name, items, INSIDE_A_GROUP, bar.state, bar.asked);
        }
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
