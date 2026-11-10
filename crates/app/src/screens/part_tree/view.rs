use cao_part::PartDocument;

use crate::lang::Catalogue;
use crate::screens::part_tree::state::{
    Body, Drawn, Kept, PartTree, Points, Row, TreeAction, Variable, clickable, clicked, marked,
};
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;
use crate::ui::badge::badge;
use crate::ui::eye::eye;
use crate::ui::floating_panel::panel_row;
use crate::ui::question::{Question, ask};
use crate::ui::theme::tokens;
use crate::wording::part_tree::{self as said, Problem, Rubric};

/// What the tree reads to mark its rows, and the one thing asked of it.
struct Asked<'a> {
    document: &'a PartDocument,
    editor: &'a SketchEditor,
    viewport: &'a ViewportState,
    adding: bool,
    action: TreeAction,
    hovering: Option<Points>,
}

/// The panel the tree is shown in, chrome and all: what was asked of it, and
/// the row the hand is on.
pub fn panel(
    ui: &mut egui::Ui,
    document: &PartDocument,
    editor: &SketchEditor,
    viewport: &ViewportState,
    lang: &Catalogue,
) -> (TreeAction, Option<Points>) {
    let mut asked = Asked {
        document,
        editor,
        viewport,
        adding: ui.input(|input| input.modifiers.command || input.modifiers.shift),
        action: TreeAction::None,
        hovering: None,
    };
    egui::Panel::left("part_tree_panel")
        .resizable(true)
        .default_size(240.0)
        .show(ui, |ui| show(ui, &mut asked, lang));
    (asked.action, asked.hovering)
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
        for (rank, rubric) in tree.rubrics().into_iter().enumerate() {
            if rank > 0 {
                ui.add_space(8.0);
            }
            ui.label(said::rubric(lang, rubric));
            match rubric {
                Rubric::Sketches => sketches(ui, &tree, asked, lang),
                Rubric::Volumes => {
                    for body in &tree.bodies {
                        made_of_matter(ui, body, asked, lang);
                    }
                }
                Rubric::Sections => {
                    for kept in &tree.sections {
                        section(ui, kept, asked, lang);
                    }
                }
                Rubric::WholeBody => {
                    let mode = said::body(lang, asked.viewport.body_mode());
                    if panel_row(ui, mode, false, false).clicked() {
                        asked.action = TreeAction::NextBodyMode;
                    }
                }
                Rubric::Variables => variables(ui, &tree.variables, lang),
            }
        }
    });
}

fn sketches(ui: &mut egui::Ui, tree: &PartTree, asked: &mut Asked<'_>, lang: &Catalogue) {
    for drawing in &tree.sketches {
        // A single click folds; only a double-click reopens the sketch, so
        // that walking the tree cannot drop the user into drawing by
        // accident.
        let lit = asked.viewport.eye_lit(asked.document, drawing.sketch);
        ui.horizontal_top(|ui| {
            if eye(ui, lit, said::eye(lang, lit)) {
                asked.action = TreeAction::ToggleEye(drawing.sketch);
            }
            let layout = egui::Layout::right_to_left(egui::Align::Min);
            ui.with_layout(layout, |ui| {
                ui.weak(&drawing.plane);
                problem(ui, drawing.badge, lang);
                if ui.vertical(|ui| drawn(ui, drawing, asked, lang)).inner {
                    asked.action = TreeAction::EditSketch(drawing.sketch);
                }
            });
        });
    }
}

/// The variables, read only: the name in the variable colour, the value on
/// the right.
fn variables(ui: &mut egui::Ui, variables: &[Variable], lang: &Catalogue) {
    if variables.is_empty() {
        ui.horizontal(|ui| {
            ui.add_space(18.0);
            ui.weak(lang.t("part_tree.no_variables"));
        });
    }
    for variable in variables {
        ui.horizontal(|ui| {
            ui.add_space(18.0);
            ui.colored_label(tokens::VARIABLE_TEXT, &variable.name);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.monospace(&variable.value);
            });
        });
    }
}

/// One kept section: a click switches it on or off, a double-click reopens it,
/// and its menu deletes it.
fn section(ui: &mut egui::Ui, kept: &Kept, asked: &mut Asked<'_>, lang: &Catalogue) {
    let on = asked.viewport.kept_sections.is_on(kept.id);
    let response = ui
        .horizontal(|ui| {
            let response = panel_row(ui, kept.name.clone(), on, false);
            problem(ui, kept.badge, lang);
            response
        })
        .inner;
    if response.double_clicked() {
        asked.action = TreeAction::EditSection(kept.id);
    } else if response.clicked() {
        asked.action = TreeAction::ToggleSection(kept.id);
    }
    response.context_menu(|ui| {
        if ui.button(lang.t("part_tree.delete")).clicked() {
            asked.action = TreeAction::DeleteSection(kept.id);
            ui.close();
        }
    });
}

fn problem(ui: &mut egui::Ui, found: Option<Problem>, lang: &Catalogue) {
    if let Some(found) = found {
        badge(ui, &said::badge(lang, found), found.level());
    }
}

/// The question asked before deleting a section a drawing stands on.
pub fn section_delete_confirm(ui: &egui::Ui, lang: &Catalogue) -> Option<bool> {
    let question = Question {
        title: lang.t("part_tree.delete_section_title"),
        body: lang.t("part_tree.delete_section_body"),
        yes: lang.t("part_tree.delete"),
        no: lang.t("part_tree.delete_section_cancel"),
    };
    ask(ui, "section_delete_confirm", &question)
}

fn made_of_matter(ui: &mut egui::Ui, body: &Body, asked: &mut Asked<'_>, lang: &Catalogue) {
    let layout = egui::Layout::right_to_left(egui::Align::Min);
    let header = ui
        .with_layout(layout, |ui| {
            problem(ui, body.badge, lang);
            ui.vertical(|ui| {
                egui::CollapsingHeader::new(&body.name)
                    .id_salt(("body", body.step))
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.weak(&body.reads);
                        for area in &body.areas {
                            row(ui, area, asked);
                        }
                    })
            })
            .inner
        })
        .inner;
    if let Some(said) = &body.unstable {
        header.header_response.on_hover_text(said);
    }
}

/// Draws one sketch, and says whether it was asked to be reopened.
fn drawn(ui: &mut egui::Ui, drawing: &Drawn, asked: &mut Asked<'_>, lang: &Catalogue) -> bool {
    let header = egui::CollapsingHeader::new(&drawing.name)
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
    let Some(value) = &row.value else {
        return named(ui, row, asked);
    };
    ui.horizontal(|ui| {
        named(ui, row, asked);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.monospace(value);
        });
    });
}

fn named(ui: &mut egui::Ui, row: &Row, asked: &mut Asked<'_>) {
    let (document, editor) = (asked.document, asked.editor);
    if !clickable(document, editor, row.points) {
        ui.label(&row.name);
        return;
    }
    let lit = marked(document, editor, asked.viewport, row.points);
    let response = panel_row(ui, row.name.clone(), lit, false);
    if response.hovered() {
        asked.hovering = Some(row.points);
    }
    if response.clicked() {
        asked.action = clicked(document, editor, row.points, asked.adding);
    }
}
