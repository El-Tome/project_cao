use cao_part::PartDocument;

use crate::lang::Catalogue;
use crate::screens::part_tree::state::{Body, Drawn, PartTree, Row};

/// What the user asked of the tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeAction {
    None,
    /// Reopen this sketch for drawing.
    EditSketch(usize),
}

/// The panel the tree is shown in, chrome and all.
pub fn panel(ui: &mut egui::Ui, document: &PartDocument, lang: &Catalogue) -> TreeAction {
    egui::Panel::left("part_tree_panel")
        .resizable(true)
        .default_size(240.0)
        .show(ui, |ui| show(ui, document, lang))
        .inner
}

fn show(ui: &mut egui::Ui, document: &PartDocument, lang: &Catalogue) -> TreeAction {
    let mut action = TreeAction::None;
    let tree = PartTree::of(document, lang);

    ui.heading(lang.t("part_tree.title"));
    ui.add_space(4.0);

    if tree.is_empty() {
        ui.weak(lang.t("part_tree.empty"));
        return action;
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.label(lang.t("part_tree.variables"));
        ui.horizontal(|ui| {
            ui.add_space(18.0);
            ui.weak(lang.t("part_tree.no_variables"));
        });
        ui.add_space(8.0);

        if !tree.bodies.is_empty() {
            ui.label(lang.t("part_tree.bodies"));
            for body in &tree.bodies {
                made_of_matter(ui, body, lang);
            }
            ui.add_space(8.0);
        }

        ui.label(lang.t("part_tree.sketches"));
        for drawing in &tree.sketches {
            // A single click folds; only a double-click reopens the sketch, so
            // that walking the tree cannot drop the user into drawing by
            // accident.
            if drawn(ui, drawing, lang) {
                action = TreeAction::EditSketch(drawing.sketch);
            }
        }
    });

    action
}

fn made_of_matter(ui: &mut egui::Ui, body: &Body, lang: &Catalogue) {
    let heading = match body.lost {
        true => lang.t_with("part_tree.lost", &[("name", &body.name)]),
        false => body.name.clone(),
    };
    egui::CollapsingHeader::new(heading)
        .id_salt(("body", body.step))
        .default_open(false)
        .show(ui, |ui| {
            ui.weak(&body.reads);
            for area in &body.areas {
                row(ui, area);
            }
        });
}

/// Draws one sketch, and says whether it was asked to be reopened.
fn drawn(ui: &mut egui::Ui, drawing: &Drawn, lang: &Catalogue) -> bool {
    let heading = match drawing.adrift {
        true => lang.t_with("part_tree.adrift", &[("name", &drawing.name)]),
        false => drawing.name.clone(),
    };
    let header = egui::CollapsingHeader::new(heading)
        .id_salt(("sketch", drawing.step))
        .default_open(false)
        .show(ui, |ui| {
            kind(
                ui,
                lang.t("part_tree.areas"),
                &drawing.areas,
                drawing.step,
                0,
            );
            kind(
                ui,
                lang.t("part_tree.strokes"),
                &drawing.strokes,
                drawing.step,
                1,
            );
            kind(
                ui,
                lang.t("part_tree.dimensions"),
                &drawing.dimensions,
                drawing.step,
                2,
            );
            kind(
                ui,
                lang.t("part_tree.rules"),
                &drawing.rules,
                drawing.step,
                3,
            );
        });
    header.header_response.double_clicked()
}

fn kind(ui: &mut egui::Ui, heading: String, rows: &[Row], step: usize, rank: usize) {
    if rows.is_empty() {
        return;
    }
    egui::CollapsingHeader::new(heading)
        .id_salt(("kind", step, rank))
        .default_open(false)
        .show(ui, |ui| {
            for one in rows {
                row(ui, one);
            }
        });
}

fn row(ui: &mut egui::Ui, row: &Row) {
    ui.label(&row.name);
}
