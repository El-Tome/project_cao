use cao_part::PartDocument;
use cao_part::feature::Feature;
use cao_part::history::Operation;

use crate::lang::Catalogue;
use crate::wording;

/// The history panel: everything done to the part, newest last, with the steps
/// that have been undone shown greyed out below the current position.
///
/// Clicking a step puts the part back the way it was just after it; the pencil
/// beside a sketch reopens it so more can be drawn on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryAction {
    None,
    /// Put the part back to this many applied operations.
    RewindTo(usize),
    /// Reopen this sketch for drawing.
    EditSketch(usize),
}

/// The panel the history is shown in, chrome and all, so that how wide it
/// opens is settled here rather than by whoever puts it on screen.
pub fn panel(ui: &mut egui::Ui, document: &PartDocument, lang: &Catalogue) -> HistoryAction {
    egui::Panel::left("history_panel")
        .resizable(true)
        .default_size(220.0)
        .show(ui, |ui| show(ui, document, lang))
        .inner
}

fn show(ui: &mut egui::Ui, document: &PartDocument, lang: &Catalogue) -> HistoryAction {
    let mut action = HistoryAction::None;
    let mut rewind_to = None;
    let operations = document.history.operations();
    let applied = document.history.applied();

    ui.heading(lang.t("history_tree.title"));
    ui.add_space(4.0);

    if operations.is_empty() {
        ui.weak(lang.t("history_tree.empty"));
        return HistoryAction::None;
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        if ui
            .selectable_label(applied == 0, lang.t("history_tree.empty_part"))
            .on_hover_text(lang.t("history_tree.empty_part_hint"))
            .clicked()
        {
            rewind_to = Some(0);
        }

        // Steps are grouped under the feature that opened them, which is what
        // makes a long drawing readable: one line per sketch, unfolded on
        // demand.
        let features = Feature::all(operations);
        let first = features
            .first()
            .map_or(operations.len(), |feature| feature.start);
        if let Some(step) = clicked_step(ui, operations, 0, first, applied, lang) {
            rewind_to = Some(step);
        }

        for feature in &features {
            let header = egui::CollapsingHeader::new(wording::history::label(
                lang,
                &operations[feature.start],
            ))
            .id_salt(feature.start)
            .default_open(true);
            let response = header.show(ui, |ui| {
                // Only overwrite on an actual click: a later group with
                // nothing clicked must not erase an earlier one.
                if let Some(step) =
                    clicked_step(ui, operations, feature.start, feature.end, applied, lang)
                {
                    rewind_to = Some(step);
                }
            });

            let Some(sketch) = feature.sketch else {
                continue;
            };

            // Reopening a finished sketch is the common case of coming back
            // to a part, so it gets a button of its own rather than hiding
            // behind a right-click.
            response.header_response.context_menu(|ui| {
                if ui.button(lang.t("history_tree.edit_sketch")).clicked() {
                    action = HistoryAction::EditSketch(sketch);
                    ui.close();
                }
            });
            ui.horizontal(|ui| {
                ui.add_space(18.0);
                if ui
                    .small_button(lang.t("history_tree.edit"))
                    .on_hover_text(lang.t("history_tree.edit_hint"))
                    .clicked()
                {
                    action = HistoryAction::EditSketch(sketch);
                }
            });
        }
    });

    match (action, rewind_to) {
        (HistoryAction::None, Some(step)) => HistoryAction::RewindTo(step),
        (action, _) => action,
    }
}

/// Shows a run of history lines, returning where to rewind to if one was
/// clicked.
fn clicked_step(
    ui: &mut egui::Ui,
    operations: &[Operation],
    start: usize,
    end: usize,
    applied: usize,
    lang: &Catalogue,
) -> Option<usize> {
    let mut clicked = None;
    for (offset, operation) in operations[start..end].iter().enumerate() {
        let step = start + offset;
        if entry(ui, operation, step, applied, lang) {
            clicked = Some(step + 1);
        }
    }
    clicked
}

/// One line of the history. Returns true when it was clicked.
fn entry(
    ui: &mut egui::Ui,
    operation: &Operation,
    step: usize,
    applied: usize,
    lang: &Catalogue,
) -> bool {
    let is_current = step + 1 == applied;
    let undone = step >= applied;

    let mut text = egui::RichText::new(format!(
        "{}. {}",
        step + 1,
        wording::history::label(lang, operation)
    ));
    if undone {
        text = text.weak().italics();
    }

    let response = ui
        .selectable_label(is_current, text)
        .on_hover_text(wording::history::detail(lang, operation));
    response.clicked()
}
