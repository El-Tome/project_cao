use cao_part::PartDocument;
use cao_prefs::{Command, Edge, Item, Settings, ToolbarLayout};

use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::extrusion_row::extrusion_row;
use crate::screens::sketch::SketchEditor;
use crate::wording::{command as wording, shortcuts, toolbar::group};

use super::state::{Context, Ribbon, active, enabled};

/// How wide a toolbar starts when it is down one side.
const SIDE_WIDTH: f32 = 210.0;

impl Ribbon {
    /// Draws the bar and returns every command the user asked for this frame.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        settings: &Settings,
        document: &PartDocument,
        editor: &SketchEditor,
        extrusion: &mut ExtrusionState,
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
                        asked = self.contents(ui, settings, document, editor, extrusion, lang);
                    });
            }
            Edge::Top => {
                egui::Panel::top("ribbon").show(ui, |ui| {
                    asked = self.contents(ui, settings, document, editor, extrusion, lang);
                });
            }
            Edge::Bottom => {
                egui::Panel::bottom("ribbon").show(ui, |ui| {
                    asked = self.contents(ui, settings, document, editor, extrusion, lang);
                });
            }
            // A side bar has to be told how wide to start: left to itself it takes the room its
            // widest button asks for, which on a wide window is the whole window.
            Edge::Left => {
                egui::Panel::left("ribbon")
                    .resizable(true)
                    .default_size(SIDE_WIDTH)
                    .show(ui, |ui| {
                        asked = self.contents(ui, settings, document, editor, extrusion, lang);
                    });
            }
            Edge::Right => {
                egui::Panel::right("ribbon")
                    .resizable(true)
                    .default_size(SIDE_WIDTH)
                    .show(ui, |ui| {
                        asked = self.contents(ui, settings, document, editor, extrusion, lang);
                    });
            }
        }

        asked
    }

    fn contents(
        &mut self,
        ui: &mut egui::Ui,
        settings: &Settings,
        document: &PartDocument,
        editor: &SketchEditor,
        extrusion: &mut ExtrusionState,
        lang: &Catalogue,
    ) -> Vec<Command> {
        let layout = &settings.toolbar;
        let mut asked = Vec::new();
        let vertical = layout.edge.is_vertical() || layout.edge == Edge::Floating;

        self.header(ui, layout, &mut asked, vertical, lang);
        ui.separator();

        let state = Context {
            settings,
            document,
            editor,
            extrusion: &*extrusion,
            lang,
        };

        // The open tab, then whatever sits at the top level outside any group.
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

        extrusion_row(ui, document, extrusion, &mut asked, lang);
        asked
    }

    /// The logo, the tabs and the window toggles.
    fn header(
        &mut self,
        ui: &mut egui::Ui,
        layout: &ToolbarLayout,
        asked: &mut Vec<Command>,
        vertical: bool,
        lang: &Catalogue,
    ) {
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
                ui.toggle_value(&mut self.history_open, lang.t("ribbon.history"));
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
                ui.toggle_value(&mut self.history_open, lang.t("ribbon.history"));
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
                            ui.horizontal(inner);
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
