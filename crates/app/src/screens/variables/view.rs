//! The panel the variables are edited in: one row each — its name, its
//! formula, what it comes to — and a row at the foot that adds one.

use cao_part::{PartDocument, VariableId};

use crate::lang::Catalogue;
use crate::screens::variables::state::{Problem, Row, VariablesPanel};
use crate::screens::variables::{NAMING, offered, shown};
use crate::ui::dropping::grip;
use crate::ui::formula_field::{Offer, formula_field};
use crate::ui::text_edit::text_edit;
use crate::wording;

/// What the user asked of the panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VariablesAction {
    None,
    /// Ask the part for what the row being edited says.
    Commit,
    /// Drop what was typed into the row being edited.
    Cancel,
    /// Ask the part for the variable the foot of the table names.
    Add,
    Erase(VariableId),
}

/// The panel, chrome and all, beside the history.
pub fn panel(
    ui: &mut egui::Ui,
    panel: &mut VariablesPanel,
    document: &PartDocument,
    lang: &Catalogue,
) -> VariablesAction {
    egui::Panel::left("variables_panel")
        .resizable(true)
        .default_size(320.0)
        .show(ui, |ui| show(ui, panel, document, lang))
        .inner
}

fn show(
    ui: &mut egui::Ui,
    panel: &mut VariablesPanel,
    document: &PartDocument,
    lang: &Catalogue,
) -> VariablesAction {
    let mut action = VariablesAction::None;
    ui.heading(lang.t("variables.title"));
    ui.add_space(4.0);
    if let Some(problem) = panel.problem() {
        let said = match problem {
            Problem::Formula(wrong) => wording::formula::unusable(lang, wrong),
            Problem::Refused(refused) => wording::variables::refused(lang, document, refused),
        };
        ui.colored_label(ui.visuals().error_fg_color, said);
        ui.add_space(4.0);
    }

    let rows = panel.rows(document.variables());
    let offers = || offered(document.variables());
    let lit = match panel.rows_blinking(ui.input(|input| input.time)) {
        Some((named, lit)) => {
            ui.ctx().request_repaint();
            if lit { named.to_vec() } else { Vec::new() }
        }
        None => Vec::new(),
    };
    egui::Grid::new("variables_grid")
        .num_columns(5)
        .striped(true)
        .show(ui, |ui| {
            ui.label("");
            ui.strong(lang.t("variables.name"));
            ui.strong(lang.t("variables.formula"));
            ui.strong(lang.t("variables.value"));
            ui.end_row();
            for row in &rows {
                let lit = lit.contains(&row.variable);
                if let Some(asked) = one_row(ui, panel, row, (lit, &offers), lang) {
                    action = asked;
                }
                ui.end_row();
            }
            ui.label("");
            let name = text_edit(
                ui,
                &mut panel.adding.name,
                90.0,
                &lang.t("variables.new_name"),
            );
            let formula = formula_field(
                ui,
                egui::Id::new("variable_formula_added"),
                &mut panel.adding.formula,
                (140.0, &lang.t("variables.new_formula")),
                &offers,
                NAMING,
            )
            .response
            .response;
            ui.label("");
            if ui.small_button(lang.t("variables.add")).clicked()
                || entered(ui, &name)
                || entered(ui, &formula)
            {
                action = VariablesAction::Add;
            }
            ui.end_row();
        });
    if rows.is_empty() {
        ui.weak(lang.t("variables.empty"));
    }
    action
}

/// One variable: its two fields, what it comes to, and the button that erases
/// it. A row is asked of the part when the keyboard leaves it, the way a
/// spreadsheet takes a cell. A row a refusal named and `lit` at this instant
/// is written in the colour of what is wrong; its formula completes a name
/// from what `offers` hands back.
fn one_row(
    ui: &mut egui::Ui,
    panel: &mut VariablesPanel,
    row: &Row,
    (lit, offers): (bool, &dyn Fn() -> Vec<Offer>),
    lang: &Catalogue,
) -> Option<VariablesAction> {
    let written_as = ui.visuals().override_text_color;
    if lit {
        ui.visuals_mut().override_text_color = Some(ui.visuals().error_fg_color);
    }
    grip(
        ui,
        egui::Id::new(("variable_grip", row.variable.0)),
        &row.name,
        &lang.t("variables.drag_hint"),
    );
    let mut typed = panel.texts(row);
    let name = text_edit(ui, &mut typed.name, 90.0, "");
    let formula = formula_field(
        ui,
        egui::Id::new(("variable_formula", row.variable.0)),
        &mut typed.formula,
        (140.0, ""),
        offers,
        NAMING,
    )
    .response
    .response;
    if name.changed() || formula.changed() {
        panel.typed_into(row, typed.name, typed.formula);
    }
    match row.value {
        Some(value) => ui.label(shown(value)),
        None => ui.weak(lang.t("variables.no_value")),
    };
    let erase = ui.small_button(lang.t("variables.erase")).clicked();
    ui.visuals_mut().override_text_color = written_as;

    // Escape takes the keyboard away before any field is drawn, so a field it
    // was pressed in reads as one the keyboard has just left.
    let holding = name.has_focus() || formula.has_focus();
    let left = name.lost_focus() || formula.lost_focus();
    if (holding || left) && ui.input(|input| input.key_pressed(egui::Key::Escape)) {
        return Some(VariablesAction::Cancel);
    }
    match (erase, left && !holding && panel.is_editing(row.variable)) {
        (true, _) => Some(VariablesAction::Erase(row.variable)),
        (false, true) => Some(VariablesAction::Commit),
        (false, false) => None,
    }
}

/// Whether Enter was pressed in a field: it gives the keyboard back as it
/// goes, and is eaten so the shortcut bound to it does not fire too.
fn entered(ui: &mut egui::Ui, field: &egui::Response) -> bool {
    field.lost_focus()
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
}
