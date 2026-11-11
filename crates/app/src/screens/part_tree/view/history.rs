//! The history, folded at the bottom of the Part panel under the step
//! reader's card: its header counts the actions, and unfolded it lists them,
//! one line each, newest last.

use cao_part::PartDocument;

use super::reader::reader;
use crate::lang::Catalogue;
use crate::screens::part_tree::state::{History, TreeAction};
use crate::screens::viewport::ViewportState;
use crate::ui::history_line::{HistoryLine, history_line};
use crate::ui::question::{Question, ask};
use crate::ui::quiet_link::quiet_link;
use crate::ui::theme::tokens::SPACING_S;
use crate::ui::tree_row::{TreeRow, tree_row};

/// How tall the unfolded list grows before it scrolls.
const TALLEST: f32 = 230.0;

/// The step reader's card, then the history, in a panel of their own along
/// the bottom of the Part panel, as tall as what they hold.
///
/// A bottom panel is laid out at the height it had the frame before and only
/// ever grows, so it is given the height its content took last time, and asks
/// for one more frame whenever that changes.
pub(super) fn at_the_bottom(
    ui: &mut egui::Ui,
    document: &PartDocument,
    viewport: &ViewportState,
    unfolded: bool,
    lang: &Catalogue,
) -> TreeAction {
    let frame = egui::Frame::NONE.inner_margin(egui::Margin::symmetric(0, SPACING_S as i8));
    let margins = frame.total_margin().sum().y;
    let id = egui::Id::new("part_history_height");
    let height = ui.data(|data| data.get_temp::<f32>(id)).unwrap_or(margins);
    let shown = egui::Panel::bottom("part_history_panel")
        .resizable(false)
        .show_separator_line(false)
        .frame(frame)
        .exact_size(height)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                let read = reader(ui, document, viewport, lang);
                ui.separator();
                match history(ui, document, unfolded, lang) {
                    TreeAction::None => read,
                    asked => asked,
                }
            })
        });
    let wanted = shown.inner.response.rect.height() + margins;
    if (wanted - height).abs() > 0.5 {
        ui.data_mut(|data| data.insert_temp(id, wanted));
        ui.ctx().request_repaint();
    }
    shown.inner.inner
}

/// The header, and the lines under it while `unfolded`; what was asked of
/// them. Nothing at all while nothing was done to the part. Folded, the lines
/// are not even worked out.
fn history(
    ui: &mut egui::Ui,
    document: &PartDocument,
    unfolded: bool,
    lang: &Catalogue,
) -> TreeAction {
    let Some(actions) = History::actions_of(document, lang) else {
        return TreeAction::None;
    };
    let mut action = TreeAction::None;
    ui.spacing_mut().item_spacing.y = 0.0;
    let title = lang.t("part_tree.history.title");
    let header = TreeRow {
        name: &title,
        unfolded: Some(unfolded),
        value: Some(&actions),
        ..TreeRow::default()
    };
    if tree_row(ui, header).row.clicked() {
        action = TreeAction::ToggleHistory;
    }
    if !unfolded {
        return action;
    }
    let history = History::of(document, lang);
    egui::ScrollArea::vertical()
        .id_salt("part_history")
        .max_height(TALLEST)
        .min_scrolled_height(TALLEST)
        .show(ui, |ui| {
            for line in &history.lines {
                let shown = HistoryLine {
                    number: line.number,
                    name: &line.name,
                    detail: &line.detail,
                    current: line.current,
                    undone: line.undone,
                    warned: line.capped.is_some(),
                };
                let mut response = history_line(ui, shown);
                if let Some(said) = &line.capped {
                    response = response.on_hover_text(said);
                }
                if response.clicked() {
                    action = TreeAction::RewindTo(line.back_to);
                }
                if let Some(sketch) = line.sketch {
                    response.context_menu(|ui| {
                        if ui.button(lang.t("part_tree.history.edit_sketch")).clicked() {
                            action = TreeAction::EditSketch(sketch);
                            ui.close();
                        }
                    });
                }
            }
        });
    ui.add_space(SPACING_S);
    if quiet_link(ui, &lang.t("part_tree.history.compact")).clicked() {
        action = TreeAction::AskToCompact;
    }
    action
}

/// The question asked before compacting: `Some(true)` once confirmed,
/// `Some(false)` once backed out of, `None` while it is still asked.
pub fn compact_confirm(ui: &egui::Ui, lang: &Catalogue) -> Option<bool> {
    let question = Question {
        title: lang.t("part_tree.history.compact_confirm_title"),
        body: lang.t("part_tree.history.compact_confirm_body"),
        yes: lang.t("part_tree.history.compact_confirm_ok"),
        no: lang.t("part_tree.history.compact_confirm_cancel"),
    };
    ask(ui, "history_compact_confirm", &question)
}
