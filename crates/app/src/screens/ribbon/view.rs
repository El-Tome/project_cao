use cao_part::PartDocument;
use cao_prefs::{Command, Edge, Item, Settings, ToolbarLayout};

use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::extrusion_row::extrusion_row;
use crate::screens::sketch::SketchEditor;
use crate::wording::{command as wording, shortcuts, toolbar::group};

use super::state::{Context, Ribbon, active, enabled};

/// What the bar needs to know about the part being drawn.
///
/// Gathered rather than passed one by one so that the call that draws the bar
/// does not grow a parameter every time a panel or a tool is added — which is
/// what #210 is about, one function along.
pub struct Drawn<'a> {
    pub document: &'a PartDocument,
    pub editor: &'a SketchEditor,
    pub extrusion: &'a mut ExtrusionState,
    /// Whether the files panel is showing, so its toggle can report it.
    pub explorer_open: bool,
}

/// How wide a toolbar starts when it is down one side.
const SIDE_WIDTH: f32 = 210.0;

impl Ribbon {
    /// Draws the bar and returns every command the user asked for this frame.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        settings: &Settings,
        drawn: &mut Drawn<'_>,
        lang: &Catalogue,
    ) -> Vec<Command> {
        let layout = &settings.toolbar;
        let mut asked = Vec::new();

        match layout.edge {
            Edge::Floating => {
                egui::Window::new(lang.t("ribbon.tools"))
                    .default_pos(ui.max_rect().left_top() + egui::vec2(24.0, 80.0))
                    .resizable(false)
                    .show(ui.ctx(), |ui| {
                        asked = self.contents(ui, settings, drawn, lang);
                    });
            }
            // egui remembers a panel's size under its id, and a bar along the top remembers a
            // height where one down a side reads a width: a placement each, or they share one.
            Edge::Top => {
                egui::Panel::top("ribbon_top").show(ui, |ui| {
                    asked = self.contents(ui, settings, drawn, lang);
                });
            }
            Edge::Bottom => {
                egui::Panel::bottom("ribbon_bottom").show(ui, |ui| {
                    asked = self.contents(ui, settings, drawn, lang);
                });
            }
            Edge::Left => {
                egui::Panel::left("ribbon_left")
                    .resizable(true)
                    .default_size(SIDE_WIDTH)
                    .show(ui, |ui| {
                        asked = self.contents(ui, settings, drawn, lang);
                    });
            }
            Edge::Right => {
                egui::Panel::right("ribbon_right")
                    .resizable(true)
                    .default_size(SIDE_WIDTH)
                    .show(ui, |ui| {
                        asked = self.contents(ui, settings, drawn, lang);
                    });
            }
        }

        asked
    }

    fn contents(
        &mut self,
        ui: &mut egui::Ui,
        settings: &Settings,
        drawn: &mut Drawn<'_>,
        lang: &Catalogue,
    ) -> Vec<Command> {
        let layout = &settings.toolbar;
        let mut asked = Vec::new();
        let vertical = layout.edge.is_vertical() || layout.edge == Edge::Floating;

        self.header(ui, layout, &mut asked, vertical, drawn.explorer_open, lang);
        ui.separator();

        let state = Context {
            settings,
            document: drawn.document,
            editor: drawn.editor,
            extrusion: drawn.extrusion,
            lang,
        };

        // The open tab, then whatever sits at the top level outside any group. A
        // button's own label must never wrap: a row that no longer fits the widest
        // one is what pushes it to the next row (see `lay_out`); a label left free
        // to shrink to fit the leftover space instead reports itself as always
        // fitting, and egui's own fallback for text with nowhere to go is to break
        // it one letter per line.
        ui.scope(|ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

            let open = layout.items.get(self.tab);
            if let Some(Item::Group { items, .. }) = open {
                lay_out(ui, items, 1, &state, &mut asked, vertical);
            }
            for item in &layout.items {
                if !matches!(item, Item::Group { .. }) {
                    lay_out(
                        ui,
                        std::slice::from_ref(item),
                        1,
                        &state,
                        &mut asked,
                        vertical,
                    );
                }
            }
        });

        extrusion_row(ui, drawn.document, drawn.extrusion, &mut asked, lang);
        asked
    }

    /// The logo, the tabs and the window toggles.
    fn header(
        &mut self,
        ui: &mut egui::Ui,
        layout: &ToolbarLayout,
        asked: &mut Vec<Command>,
        vertical: bool,
        explorer_open: bool,
        lang: &Catalogue,
    ) {
        // The panel's own state lives in the panel, so the toggle reports
        // rather than holds: what it is shown is handed back as a command.
        let files = |ui: &mut egui::Ui, asked: &mut Vec<Command>| {
            let mut shown = explorer_open;
            if ui
                .toggle_value(&mut shown, lang.t("ribbon.files"))
                .changed()
            {
                asked.push(Command::ToggleExplorer);
            }
        };

        let mut row = |ui: &mut egui::Ui| {
            if layout.show_logo {
                ui.strong(&layout.logo_text);
                ui.separator();
            }
            for (rank, item) in layout.items.iter().enumerate() {
                let Item::Group { name, .. } = item else {
                    continue;
                };
                if ui
                    .selectable_label(self.tab == rank, group(lang, name))
                    .clicked()
                {
                    self.tab = rank;
                }
            }
        };

        if vertical {
            ui.vertical(|ui| row(ui));
            ui.horizontal_wrapped(|ui| {
                files(ui, asked);
                ui.toggle_value(&mut self.part_tree_open, lang.t("ribbon.part_tree"));
                ui.toggle_value(&mut self.history_open, lang.t("ribbon.history"));
                ui.toggle_value(&mut self.variables_open, lang.t("ribbon.variables"));
                if ui
                    .button("⚙")
                    .on_hover_text(lang.t("ribbon.settings"))
                    .clicked()
                {
                    asked.push(Command::OpenSettings);
                }
            });
            return;
        }

        ui.horizontal(|ui| {
            row(ui);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .button("⚙")
                    .on_hover_text(lang.t("ribbon.settings"))
                    .clicked()
                {
                    asked.push(Command::OpenSettings);
                }
                ui.toggle_value(&mut self.variables_open, lang.t("ribbon.variables"));
                ui.toggle_value(&mut self.history_open, lang.t("ribbon.history"));
                ui.toggle_value(&mut self.part_tree_open, lang.t("ribbon.part_tree"));
                files(ui, asked);
            });
        });
    }
}

/// Draws a run of entries, at the depth they sit in the tree.
///
/// A group one level in is spread out where it is, with its name beside it; any
/// deeper and it becomes a menu that opens on click. Laying a third level out
/// in place would push everything else off the bar, and there is no bottom to
/// how deep the user may go.
fn lay_out(
    ui: &mut egui::Ui,
    items: &[Item],
    depth: usize,
    state: &Context<'_>,
    asked: &mut Vec<Command>,
    vertical: bool,
) {
    let run = |ui: &mut egui::Ui| {
        for item in items {
            match item {
                Item::Command(command) => button(ui, *command, state, asked),
                Item::Separator => {
                    ui.separator();
                }
                Item::Group { name, items } if depth <= 1 => {
                    ui.group(|ui| {
                        let inner = |ui: &mut egui::Ui| {
                            ui.weak(group(state.lang, name));
                            lay_out(ui, items, depth + 1, state, asked, vertical);
                        };
                        if vertical {
                            ui.vertical(inner);
                        } else {
                            ui.horizontal_wrapped(inner);
                        }
                    });
                }
                Item::Group { name, items } => {
                    ui.menu_button(group(state.lang, name), |ui| {
                        lay_out(ui, items, depth + 1, state, asked, true);
                    });
                }
            }
        }
    };

    if vertical {
        ui.vertical(run);
    } else {
        ui.horizontal_wrapped(run);
    }
}

fn button(ui: &mut egui::Ui, command: Command, state: &Context<'_>, asked: &mut Vec<Command>) {
    let name = wording::label(state.lang, command);
    let label = if state.settings.toolbar.show_labels {
        match state.settings.shortcuts.chord_for(command) {
            Some(chord) => format!("{name} ({})", shortcuts::chord(state.lang, chord)),
            None => name,
        }
    } else {
        name.chars().take(2).collect()
    };

    let response = ui
        .add_enabled_ui(enabled(command, state), |ui| {
            ui.selectable_label(active(command, state), label)
        })
        .inner
        .on_hover_text(wording::hint(state.lang, command));
    if response.clicked() {
        asked.push(command);
    }
}

#[cfg(test)]
mod tests;
