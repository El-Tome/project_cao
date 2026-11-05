use cao_part::PartDocument;

use crate::lang::Catalogue;
use crate::screens::part_tree::state::{
    Body, Drawn, PartTree, Row, TreeAction, clickable, clicked, marked,
};
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;
use crate::ui::eye::eye;
use crate::ui::floating_panel::panel_row;
use crate::wording::part_tree as said;

/// What the tree reads to mark its rows, and the one thing asked of it.
struct Asked<'a> {
    document: &'a PartDocument,
    editor: &'a SketchEditor,
    viewport: &'a ViewportState,
    adding: bool,
    action: TreeAction,
}

/// The panel the tree is shown in, chrome and all.
pub fn panel(
    ui: &mut egui::Ui,
    document: &PartDocument,
    editor: &SketchEditor,
    viewport: &ViewportState,
    lang: &Catalogue,
) -> TreeAction {
    let mut asked = Asked {
        document,
        editor,
        viewport,
        adding: ui.input(|input| input.modifiers.command || input.modifiers.shift),
        action: TreeAction::None,
    };
    egui::Panel::left("part_tree_panel")
        .resizable(true)
        .default_size(240.0)
        .show(ui, |ui| show(ui, &mut asked, lang));
    asked.action
}

fn show(ui: &mut egui::Ui, asked: &mut Asked<'_>, lang: &Catalogue) {
    let tree = PartTree::of(asked.document, lang);

    ui.heading(lang.t("part_tree.title"));
    ui.add_space(4.0);

    if tree.is_empty() {
        ui.weak(lang.t("part_tree.empty"));
        return;
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.label(lang.t("part_tree.variables"));
        if tree.variables.is_empty() {
            ui.horizontal(|ui| {
                ui.add_space(18.0);
                ui.weak(lang.t("part_tree.no_variables"));
            });
        }
        for variable in &tree.variables {
            ui.horizontal(|ui| {
                ui.add_space(18.0);
                ui.label(variable);
            });
        }
        ui.add_space(8.0);

        if !tree.bodies.is_empty() {
            let mode = said::body(lang, asked.viewport.body_mode());
            if panel_row(ui, mode, false, false).clicked() {
                asked.action = TreeAction::NextBodyMode;
            }
            for body in &tree.bodies {
                made_of_matter(ui, body, asked, lang);
            }
            ui.add_space(8.0);
        }

        ui.label(lang.t("part_tree.sketches"));
        for drawing in &tree.sketches {
            // A single click folds; only a double-click reopens the sketch, so
            // that walking the tree cannot drop the user into drawing by
            // accident.
            let lit = asked.viewport.eye_lit(asked.document, drawing.sketch);
            ui.horizontal_top(|ui| {
                if eye(ui, lit, said::eye(lang, lit)) {
                    asked.action = TreeAction::ToggleEye(drawing.sketch);
                }
                if ui.vertical(|ui| drawn(ui, drawing, asked, lang)).inner {
                    asked.action = TreeAction::EditSketch(drawing.sketch);
                }
            });
        }
    });
}

fn made_of_matter(ui: &mut egui::Ui, body: &Body, asked: &mut Asked<'_>, lang: &Catalogue) {
    let name = &[("name", body.name.as_str())];
    let heading = match (body.declined, body.lost, &body.unstable) {
        (true, _, _) => lang.t_with("part_tree.declined", name),
        (false, true, _) => lang.t_with("part_tree.lost", name),
        (false, false, Some(_)) => lang.t_with("part_tree.unstable", name),
        (false, false, None) => body.name.clone(),
    };
    let header = egui::CollapsingHeader::new(heading)
        .id_salt(("body", body.step))
        .default_open(false)
        .show(ui, |ui| {
            ui.weak(&body.reads);
            for area in &body.areas {
                row(ui, area, asked);
            }
        });
    if let Some(said) = &body.unstable {
        header.header_response.on_hover_text(said);
    }
}

/// Draws one sketch, and says whether it was asked to be reopened.
fn drawn(ui: &mut egui::Ui, drawing: &Drawn, asked: &mut Asked<'_>, lang: &Catalogue) -> bool {
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
                asked,
                lang.t("part_tree.areas"),
                &drawing.areas,
                drawing.step,
                0,
            );
            kind(
                ui,
                asked,
                lang.t("part_tree.strokes"),
                &drawing.strokes,
                drawing.step,
                1,
            );
            kind(
                ui,
                asked,
                lang.t("part_tree.dimensions"),
                &drawing.dimensions,
                drawing.step,
                2,
            );
            kind(
                ui,
                asked,
                lang.t("part_tree.rules"),
                &drawing.rules,
                drawing.step,
                3,
            );
        });
    header.header_response.double_clicked()
}

fn kind(
    ui: &mut egui::Ui,
    asked: &mut Asked<'_>,
    heading: String,
    rows: &[Row],
    step: usize,
    rank: usize,
) {
    if rows.is_empty() {
        return;
    }
    egui::CollapsingHeader::new(heading)
        .id_salt(("kind", step, rank))
        .default_open(false)
        .show(ui, |ui| {
            for one in rows {
                row(ui, one, asked);
            }
        });
}

/// One row: a click where it asks something, marked while the open sketch
/// holds it or the canvas lights it; a plain line where it asks nothing.
fn row(ui: &mut egui::Ui, row: &Row, asked: &mut Asked<'_>) {
    let (document, editor) = (asked.document, asked.editor);
    if !clickable(document, editor, row.points) {
        ui.label(&row.name);
        return;
    }
    let lit = marked(document, editor, asked.viewport, row.points);
    if panel_row(ui, row.name.clone(), lit, false).clicked() {
        asked.action = clicked(document, editor, row.points, asked.adding);
    }
}
