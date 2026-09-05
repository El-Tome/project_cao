use cao_core::PartDocument;

use crate::screens::sketch::{DimensionMode, SketchEditor, Tool};

/// A family of tools. Only sketching exists so far; extrusion and assembly are
/// listed so the shape of the menu is visible, and so adding them is a matter
/// of filling in their tools.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Category {
    #[default]
    Sketch,
    Extrusion,
    Assembly,
}

impl Category {
    pub const ALL: [Self; 3] = [Self::Sketch, Self::Extrusion, Self::Assembly];

    pub fn label(self) -> &'static str {
        match self {
            Self::Sketch => "Esquisse",
            Self::Extrusion => "Extrusion",
            Self::Assembly => "Assemblage",
        }
    }

    pub fn available(self) -> bool {
        matches!(self, Self::Sketch)
    }
}

/// What the user asked the toolbar to do. Kept as a value rather than acted on
/// in place: the toolbar only borrows the editor, while these need the whole
/// part.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RibbonAction {
    None,
    NewSketch,
    FinishSketch,
    Undo,
    Redo,
    RecenterOnSketch,
}

/// The toolbar: a row of categories, and under it the tools of the one picked.
///
/// It lives in a small window that is docked to the top of the screen by
/// default and can be pulled off and moved anywhere.
#[derive(Default)]
pub struct Ribbon {
    pub category: Category,
    pub docked: bool,
    pub history_open: bool,
}

impl Ribbon {
    pub fn new() -> Self {
        Self {
            category: Category::Sketch,
            docked: true,
            history_open: true,
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        document: &PartDocument,
        editor: &mut SketchEditor,
    ) -> RibbonAction {
        let mut action = RibbonAction::None;

        if self.docked {
            egui::Panel::top("ribbon_docked").show(ui, |ui| {
                action = self.contents(ui, document, editor);
            });
        } else {
            egui::Window::new("Outils")
                .default_pos(ui.max_rect().left_top() + egui::vec2(24.0, 80.0))
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    action = self.contents(ui, document, editor);
                });
        }

        action
    }

    fn contents(
        &mut self,
        ui: &mut egui::Ui,
        document: &PartDocument,
        editor: &mut SketchEditor,
    ) -> RibbonAction {
        let mut action = RibbonAction::None;

        ui.horizontal(|ui| {
            for category in Category::ALL {
                let enabled = category.available();
                let selected = self.category == category;
                let response = ui
                    .add_enabled_ui(enabled, |ui| {
                        ui.selectable_label(selected, category.label())
                    })
                    .inner;
                if response.clicked() {
                    self.category = category;
                }
                if !enabled {
                    response.on_hover_text("Pas encore disponible");
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let icon = if self.docked { "⏏" } else { "📌" };
                let hint = if self.docked {
                    "Détacher la barre d'outils"
                } else {
                    "Ancrer la barre en haut"
                };
                if ui.button(icon).on_hover_text(hint).clicked() {
                    self.docked = !self.docked;
                }
                ui.toggle_value(&mut self.history_open, "Historique");
            });
        });

        ui.separator();

        ui.horizontal_wrapped(|ui| {
            action = match self.category {
                Category::Sketch => self.sketch_tools(ui, document, editor),
                Category::Extrusion | Category::Assembly => {
                    ui.weak("Aucun outil pour cette catégorie pour l'instant.");
                    RibbonAction::None
                }
            };
        });

        action
    }

    fn sketch_tools(
        &self,
        ui: &mut egui::Ui,
        document: &PartDocument,
        editor: &mut SketchEditor,
    ) -> RibbonAction {
        let mut action = RibbonAction::None;

        if ui
            .button("Nouvelle esquisse")
            .on_hover_text("Choisir un plan pour commencer un dessin")
            .clicked()
        {
            action = RibbonAction::NewSketch;
        }

        ui.separator();

        let drawing = editor.active_sketch().is_some();
        ui.add_enabled_ui(drawing, |ui| {
            for tool in Tool::SKETCH_TOOLS {
                if ui
                    .selectable_label(editor.tool == tool, tool.label())
                    .on_hover_text(tool.hint())
                    .clicked()
                {
                    editor.tool = tool;
                    editor.reset_pending();
                }
            }
            if ui.button("Recadrer").clicked() {
                action = RibbonAction::RecenterOnSketch;
            }
            if ui.button("Terminer").clicked() {
                action = RibbonAction::FinishSketch;
            }
        });

        // The dimension tool's own row: what it is allowed to measure. Auto
        // covers most of the work; the others are there for when two things
        // sit under the same cursor and the wrong one keeps winning.
        if drawing && editor.tool == Tool::Dimension {
            ui.end_row();
            ui.separator();
            ui.label("Mesurer :");
            for mode in DimensionMode::ALL {
                if ui
                    .selectable_label(editor.dimension_mode == mode, mode.label())
                    .on_hover_text(mode.hint())
                    .clicked()
                {
                    editor.dimension_mode = mode;
                    editor.reset_pending();
                }
            }
        }

        ui.separator();

        ui.add_enabled_ui(document.history.can_undo(), |ui| {
            if ui.button("↶ Annuler").on_hover_text("Ctrl+Z").clicked() {
                action = RibbonAction::Undo;
            }
        });
        ui.add_enabled_ui(document.history.can_redo(), |ui| {
            if ui.button("↷ Rétablir").on_hover_text("Ctrl+Y").clicked() {
                action = RibbonAction::Redo;
            }
        });

        action
    }
}
