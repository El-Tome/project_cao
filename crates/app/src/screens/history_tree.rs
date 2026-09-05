use cao_core::PartDocument;
use cao_core::history::Operation;

/// The history panel: everything done to the part, newest last, with the steps
/// that have been undone shown greyed out below the current position.
///
/// Clicking a step puts the part back the way it was just after it. Returns the
/// number of operations to apply when the user asks to go somewhere.
pub fn show(ui: &mut egui::Ui, document: &PartDocument) -> Option<usize> {
    let mut rewind_to = None;
    let operations = document.history.operations();
    let applied = document.history.applied();

    ui.heading("Historique");
    ui.add_space(4.0);

    if operations.is_empty() {
        ui.weak("Rien encore. Commencez par une esquisse.");
        return None;
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        if ui
            .selectable_label(applied == 0, "Pièce vide")
            .on_hover_text("Revenir avant la première opération")
            .clicked()
        {
            rewind_to = Some(0);
        }

        // Steps are grouped under the feature that opened them, which is what
        // makes a long drawing readable: one line per sketch, unfolded on
        // demand.
        let mut index = 0;
        while index < operations.len() {
            let group_end = group_end(operations, index);
            if operations[index].starts_feature() {
                let header = egui::CollapsingHeader::new(operations[index].label())
                    .id_salt(index)
                    .default_open(true);
                header.show(ui, |ui| {
                    // Only overwrite on an actual click: a later group with
                    // nothing clicked must not erase an earlier one.
                    if let Some(step) = clicked_step(ui, operations, index, group_end, applied) {
                        rewind_to = Some(step);
                    }
                });
            } else if let Some(step) = clicked_step(ui, operations, index, group_end, applied) {
                rewind_to = Some(step);
            }
            index = group_end;
        }
    });

    rewind_to
}

/// Shows a run of history lines, returning where to rewind to if one was
/// clicked.
fn clicked_step(
    ui: &mut egui::Ui,
    operations: &[Operation],
    start: usize,
    end: usize,
    applied: usize,
) -> Option<usize> {
    let mut clicked = None;
    for (offset, operation) in operations[start..end].iter().enumerate() {
        let step = start + offset;
        if entry(ui, operation, step, applied) {
            clicked = Some(step + 1);
        }
    }
    clicked
}

/// Where the run of operations belonging to the feature at `start` ends.
fn group_end(operations: &[Operation], start: usize) -> usize {
    let mut end = start + 1;
    while end < operations.len() && !operations[end].starts_feature() {
        end += 1;
    }
    end
}

/// One line of the history. Returns true when it was clicked.
fn entry(ui: &mut egui::Ui, operation: &Operation, step: usize, applied: usize) -> bool {
    let is_current = step + 1 == applied;
    let undone = step >= applied;

    let mut text = egui::RichText::new(format!("{}. {}", step + 1, operation.label()));
    if undone {
        text = text.weak().italics();
    }

    let response = ui
        .selectable_label(is_current, text)
        .on_hover_text(operation.detail());
    response.clicked()
}
