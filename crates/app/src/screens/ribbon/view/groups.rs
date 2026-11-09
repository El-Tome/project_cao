use cao_prefs::{Command, GroupSize, Item};
use egui::{
    Align, FontSelection, Id, Label, Layout, RichText, Sense, Stroke, TextWrapMode, Ui, Vec2,
    WidgetText, vec2,
};

use crate::ui::theme::tokens::{BORDER_GROUP, TEXT_QUIET};
use crate::ui::tool_button::{Rows, small_column, small_grid, small_separator};
use crate::wording::toolbar::group;

use super::super::state::{Ribbon, Run, SPREAD_OUT, runs};
use super::split::split_button;
use super::{Context, Shown, button, menu};

const NAME_SIZE: f32 = 10.0;
/// 0.08 em of `NAME_SIZE`.
const NAME_LETTER_SPACING: f32 = 0.8;
const SEPARATOR_MARGIN: f32 = 10.0;
const SEPARATOR_HEIGHT: f32 = 64.0;

/// What a large group reads as down a side, where every button is small.
const SMALLEST: Rows = Rows::Three;

/// The depth of a group's own entries: a group inside it is a menu.
const INSIDE_A_GROUP: usize = SPREAD_OUT + 1;

/// What every entry of the bar needs in order to be drawn and to answer.
pub(super) struct Bar<'a> {
    pub(super) state: &'a Context<'a>,
    pub(super) asked: &'a mut Vec<Command>,
    pub(super) ribbon: &'a mut Ribbon,
}

/// The entries of the open tab: stacked down a side, otherwise along the bar,
/// each group one block that goes whole to the next line when it does not fit
/// on this one. Answers how wide it all stands on one line.
pub(super) fn open_tab(
    ui: &mut Ui,
    items: &[Item],
    state: &Context<'_>,
    asked: &mut Vec<Command>,
    ribbon: &mut Ribbon,
    side: bool,
    bar_width: f32,
) -> f32 {
    let mut bar = Bar {
        state,
        asked,
        ribbon,
    };
    if side {
        let runs = runs(items, SPREAD_OUT);
        small_column(ui, runs.len(), |ui, rank| {
            let after_a_group =
                rank > 0 && matches!(runs[rank - 1], Run::Plain(Item::Group { .. }));
            run_down_a_side(ui, &runs[rank], after_a_group, &mut bar);
        });
        return 0.0;
    }
    ui.with_layout(wrapped_row(), |ui| {
        let spacing = ui.spacing().item_spacing.x;
        let runs = runs(items, SPREAD_OUT);
        let mut held = 0.0;
        for (rank, run) in runs.iter().enumerate() {
            let after_a_group =
                rank > 0 && matches!(runs[rank - 1], Run::Plain(Item::Group { .. }));
            let width = match run {
                Run::Split(split) => one_block(ui, rank, bar_width, |ui| {
                    split_button(ui, split, Shown::Large, &mut bar);
                }),
                Run::Plain(Item::Command(command)) => one_block(ui, rank, bar_width, |ui| {
                    button(ui, *command, bar.state, bar.asked, Shown::Large);
                }),
                Run::Plain(Item::Separator) => rule_before(ui, rank + 1, rank + 1 < runs.len()),
                Run::Plain(Item::Group { name, size, items }) => {
                    let rule = rule_before(ui, rank, after_a_group);
                    rule + one_block(ui, rank, bar_width, |ui| {
                        named_group(ui, name, *size, items, false, &mut bar);
                    })
                }
            };
            // The row stands each thing on rounded points, which the sum of
            // their widths does not: short of it, the last block would wrap.
            held += width.ceil() + spacing;
        }
        held - spacing
    })
    .inner
}

/// One entry of a tab down a side, where every button is small.
fn run_down_a_side(ui: &mut Ui, run: &Run<'_>, after_a_group: bool, bar: &mut Bar<'_>) {
    let shown = Shown::Small(SMALLEST);
    match run {
        Run::Plain(Item::Group { name, size, items }) => {
            if after_a_group {
                ui.separator();
            }
            named_group(ui, name, *size, items, true, bar);
        }
        run => entry(ui, run, shown, 0, 0.0, bar),
    }
}

fn wrapped_row() -> Layout {
    Layout::left_to_right(Align::TOP).with_main_wrap(true)
}

/// Where the size a block came out at last frame is kept, by its rank, so that
/// a rule drawn or not before it leaves it the same.
fn block_id(ui: &Ui, rank: usize) -> Id {
    ui.id().with(("ribbon_block", rank))
}

fn last_frame_size(ui: &Ui, rank: usize) -> Vec2 {
    let id = block_id(ui, rank);
    ui.data(|data| data.get_temp(id)).unwrap_or_default()
}

/// Asks the row for the size this block came out at last frame, which is what
/// sends it whole to the next line, while letting it lay out over the whole
/// bar: laid out in what is left of a line, it would wrap inside itself, and
/// remember the narrower size it came out at. Answers its width.
fn one_block(ui: &mut Ui, rank: usize, bar_width: f32, add: impl FnOnce(&mut Ui)) -> f32 {
    let last_frame = last_frame_size(ui, rank);
    let block = ui.allocate_ui_with_layout(last_frame, Layout::top_down(Align::LEFT), |ui| {
        ui.set_max_width(bar_width);
        add(ui);
    });
    let size = block.response.rect.size();
    let id = block_id(ui, rank);
    ui.data_mut(|data| data.insert_temp(id, size));
    size.x
}

/// The rule before the block at `next`, when one is wanted and that block will
/// stay on this line. Answers the width it takes on one line either way.
fn rule_before(ui: &mut Ui, next: usize, wanted: bool) -> f32 {
    if !wanted {
        return 0.0;
    }
    let room = separator_room(ui);
    let spacing = ui.spacing().item_spacing.x;
    if ui.available_size_before_wrap().x >= room + spacing + last_frame_size(ui, next).x {
        separator(ui);
    }
    room + spacing
}

fn named_group(
    ui: &mut Ui,
    name: &str,
    size: GroupSize,
    items: &[Item],
    side: bool,
    bar: &mut Bar<'_>,
) {
    let runs = runs(items, INSIDE_A_GROUP);
    let buttons = ui
        .with_layout(wrapped_row(), |ui| match (rows_of(size), side) {
            (rows, true) => {
                let shown = Shown::Small(rows.unwrap_or(SMALLEST));
                small_column(ui, runs.len(), |ui, rank| {
                    entry(ui, &runs[rank], shown, rank, 0.0, bar);
                });
            }
            (None, false) => {
                let bar_width = ui.max_rect().width();
                for (rank, run) in runs.iter().enumerate() {
                    entry(ui, run, Shown::Large, rank, bar_width, bar);
                }
            }
            (Some(rows), false) => small_grid(ui, rows, runs.len(), |ui, rank| {
                entry(ui, &runs[rank], Shown::Small(rows), rank, 0.0, bar);
            }),
        })
        .response
        .rect;

    let written = RichText::new(group(bar.state.lang, name).to_uppercase())
        .size(NAME_SIZE)
        .color(TEXT_QUIET)
        .extra_letter_spacing(NAME_LETTER_SPACING);
    let galley = WidgetText::from(written).into_galley(
        ui,
        Some(TextWrapMode::Extend),
        f32::INFINITY,
        FontSelection::Default,
    );
    ui.allocate_ui_with_layout(
        vec2(buttons.width().max(galley.size().x), 0.0),
        Layout::top_down(Align::Center),
        |ui| ui.add(Label::new(galley)),
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
fn entry(ui: &mut Ui, run: &Run<'_>, shown: Shown, rank: usize, bar_width: f32, bar: &mut Bar<'_>) {
    match run {
        Run::Split(split) => match shown {
            Shown::Small(_) => split_button(ui, split, shown, bar),
            _ => {
                one_block(ui, rank, bar_width, |ui| {
                    split_button(ui, split, shown, bar);
                });
            }
        },
        Run::Plain(Item::Command(command)) => {
            button(ui, *command, bar.state, bar.asked, shown);
        }
        Run::Plain(Item::Separator) => match shown {
            Shown::Small(_) => small_separator(ui),
            _ => {
                ui.separator();
            }
        },
        Run::Plain(Item::Group { name, items, .. }) => {
            menu(ui, name, items, bar.state, bar.asked);
        }
    }
}

/// The rule between two groups. The row already spaces what stands beside it,
/// so it takes only what that spacing leaves of the margin.
fn separator_room(ui: &Ui) -> f32 {
    2.0 * (SEPARATOR_MARGIN - ui.spacing().item_spacing.x) + 1.0
}

fn separator(ui: &mut Ui) {
    let (rect, _) =
        ui.allocate_exact_size(vec2(separator_room(ui), SEPARATOR_HEIGHT), Sense::hover());
    ui.painter().vline(
        rect.center().x,
        rect.y_range(),
        Stroke::new(1.0, BORDER_GROUP),
    );
}
