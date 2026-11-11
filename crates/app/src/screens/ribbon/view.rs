use cao_part::PartDocument;
use cao_prefs::{Command, Edge, Item, Settings, ToolbarLayout};

use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::extrusion_row::extrusion_row;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::chamfer_to_swap;
use crate::ui::icon;
use crate::ui::logo::logo;
use crate::ui::panel_toggle::{panel_button, panel_toggle};
use crate::ui::theme::tokens::SPACING_L;
use crate::ui::tool_button::{
    Rows, icon_button, menu_entry, name_and_key, small_button, small_icon_button, sub_menu,
    tool_button, tooltip,
};
use crate::wording::{command as wording, shortcuts, toolbar::group};

use super::icons;
use super::state::{Context, Ribbon, active, enabled, icon, label};

mod groups;
mod split;

/// What the bar needs to know about the part being drawn.
///
/// Gathered rather than passed one by one so that the call that draws the bar
/// does not grow a parameter every time a panel or a tool is added — which is
/// what #210 is about, one function along.
pub struct Drawn<'a> {
    pub document: &'a PartDocument,
    pub editor: &'a SketchEditor,
    pub extrusion: &'a mut ExtrusionState,
}

impl Drawn<'_> {
    /// What a button needs to be drawn in the right state, out of the part.
    fn context<'a>(
        &'a self,
        settings: &'a Settings,
        lang: &'a Catalogue,
        section_on: bool,
    ) -> Context<'a> {
        Context {
            settings,
            document: self.document,
            editor: self.editor,
            extrusion: self.extrusion,
            lang,
            section_on,
        }
    }
}

/// How wide a toolbar starts when it is down one side.
const SIDE_WIDTH: f32 = 210.0;

/// How far a floating toolbar opens from the left of the window.
const FLOATING_MARGIN: f32 = 24.0;

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
        self.sketch_tab = layout
            .items
            .iter()
            .position(|item| matches!(item, Item::Group { name, .. } if name == "sketch"));
        let mut asked = Vec::new();

        match layout.edge {
            Edge::Floating => {
                // Auto-sized, a window measures a wrapped row against its own
                // narrow width: it gets what it held last frame, at most the
                // width of the bar along the top.
                let bar_width = ui.max_rect().width() - 2.0 * FLOATING_MARGIN;
                let width = if self.floating_width > 0.0 {
                    self.floating_width.min(bar_width)
                } else {
                    bar_width
                };
                egui::Window::new(lang.t("ribbon.tools"))
                    .default_pos(ui.max_rect().left_top() + egui::vec2(FLOATING_MARGIN, 80.0))
                    .min_width(width)
                    .max_width(width)
                    .resizable(false)
                    .show(ui.ctx(), |ui| {
                        asked = self.contents(ui, settings, drawn, lang, bar_width);
                    });
            }
            // egui remembers a panel's size under its id, and a bar along the top remembers a
            // height where one down a side reads a width: a placement each, or they share one.
            Edge::Top => {
                egui::Panel::top("ribbon_top").show(ui, |ui| {
                    let bar_width = ui.max_rect().width();
                    asked = self.contents(ui, settings, drawn, lang, bar_width);
                });
            }
            Edge::Bottom => {
                egui::Panel::bottom("ribbon_bottom").show(ui, |ui| {
                    let bar_width = ui.max_rect().width();
                    asked = self.contents(ui, settings, drawn, lang, bar_width);
                });
            }
            Edge::Left => {
                egui::Panel::left("ribbon_left")
                    .resizable(true)
                    .default_size(SIDE_WIDTH)
                    .show(ui, |ui| {
                        asked = self.scrolled_contents(ui, settings, drawn, lang);
                    });
            }
            Edge::Right => {
                egui::Panel::right("ribbon_right")
                    .resizable(true)
                    .default_size(SIDE_WIDTH)
                    .show(ui, |ui| {
                        asked = self.scrolled_contents(ui, settings, drawn, lang);
                    });
            }
        }

        asked
    }

    /// A column taller than the window scrolls rather than running off its
    /// bottom, where nothing could be reached.
    fn scrolled_contents(
        &mut self,
        ui: &mut egui::Ui,
        settings: &Settings,
        drawn: &mut Drawn<'_>,
        lang: &Catalogue,
    ) -> Vec<Command> {
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                let bar_width = ui.max_rect().width();
                self.contents(ui, settings, drawn, lang, bar_width)
            })
            .inner
    }

    fn contents(
        &mut self,
        ui: &mut egui::Ui,
        settings: &Settings,
        drawn: &mut Drawn<'_>,
        lang: &Catalogue,
        bar_width: f32,
    ) -> Vec<Command> {
        let layout = &settings.toolbar;
        let mut asked = Vec::new();
        let side = layout.edge.is_vertical();
        let shown = if side {
            Shown::Small(Rows::Three)
        } else {
            Shown::Large
        };

        let header_width = self.header(ui, layout, &mut asked, side, lang);
        ui.separator();

        let state = drawn.context(settings, lang, self.section_on);

        // The open tab, then whatever sits at the top level outside any group. A
        // button's own label must never wrap: a row that no longer fits the widest
        // one is what pushes it to the next row; a label left free
        // to shrink to fit the leftover space instead reports itself as always
        // fitting, and egui's own fallback for text with nowhere to go is to break
        // it one letter per line.
        ui.scope(|ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

            let mut held = 0.0;
            if let Some(Item::Group { items, .. }) = layout.items.get(self.tab) {
                held = groups::open_tab(ui, items, &state, &mut asked, self, side, bar_width);
            }
            self.floating_width = held.max(header_width);
            for item in &layout.items {
                match item {
                    Item::Command(command) => button(ui, *command, &state, &mut asked, shown),
                    Item::Separator => {
                        ui.separator();
                    }
                    Item::Group { .. } => {}
                }
            }
        });

        extrusion_row(ui, drawn.document, drawn.extrusion, &mut asked, lang);
        let state = drawn.context(settings, lang, self.section_on);
        chamfer_row(ui, &state, &mut asked, shown);
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
    ) -> f32 {
        let preferences = |ui: &mut egui::Ui, asked: &mut Vec<Command>| {
            let worn = icons::of(Command::OpenSettings);
            if panel_button(ui, false, worn, lang.t("ribbon.settings")).clicked() {
                asked.push(Command::OpenSettings);
            }
        };

        let mut row = |ui: &mut egui::Ui| {
            if layout.show_logo {
                logo(ui, &layout.logo_text);
                // Along the top the logo is set apart from the tabs by room
                // alone, as the redesign draws it; down a side, by a rule.
                if vertical {
                    ui.separator();
                } else {
                    ui.add_space(SPACING_L);
                }
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
                let part = lang.t("ribbon.part_tree");
                panel_toggle(ui, &mut self.part_tree_open, icon::PART, part);
                let variables = lang.t("ribbon.variables");
                panel_toggle(ui, &mut self.variables_open, icon::VARIABLES, variables);
                ui.separator();
                preferences(ui, asked);
            });
            return 0.0;
        }

        ui.horizontal(|ui| {
            row(ui);
            let tabs = ui.min_rect().width() + ui.spacing().item_spacing.x;
            let toggles = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                preferences(ui, asked);
                ui.separator();
                let variables = lang.t("ribbon.variables");
                panel_toggle(ui, &mut self.variables_open, icon::VARIABLES, variables);
                let part = lang.t("ribbon.part_tree");
                panel_toggle(ui, &mut self.part_tree_open, icon::PART, part);
            });
            tabs + toggles.response.rect.width()
        })
        .inner
    }
}

/// A group at menu depth: its commands open on click, a level further down.
fn menu(
    ui: &mut egui::Ui,
    name: &str,
    items: &[Item],
    state: &Context<'_>,
    asked: &mut Vec<Command>,
) {
    sub_menu(ui, &group(state.lang, name), |ui| {
        for item in items {
            match item {
                Item::Command(command) => button(ui, *command, state, asked, Shown::MenuEntry),
                Item::Separator => {
                    ui.separator();
                }
                Item::Group { name, items, .. } => menu(ui, name, items, state, asked),
            }
        }
    });
}

/// How a command is drawn, from where it stands in the tree.
#[derive(Clone, Copy)]
enum Shown {
    Large,
    Small(Rows),
    MenuEntry,
}

fn button(
    ui: &mut egui::Ui,
    command: Command,
    state: &Context<'_>,
    asked: &mut Vec<Command>,
    shown: Shown,
) {
    let name = label(command, state);
    let key = key_of(command, state);
    named_button(ui, command, &name, key.as_deref(), state, asked, shown);
}

fn key_of(command: Command, state: &Context<'_>) -> Option<String> {
    state
        .settings
        .shortcuts
        .chord_for(command)
        .map(|chord| shortcuts::chord(state.lang, chord))
}

/// A button that runs `command` under a name that need not be the command's.
fn named_button(
    ui: &mut egui::Ui,
    command: Command,
    name: &str,
    key: Option<&str>,
    state: &Context<'_>,
    asked: &mut Vec<Command>,
    shown: Shown,
) -> egui::Response {
    let hint = wording::hint(state.lang, command);
    let pressable = enabled(command, state);
    let on = active(command, state);
    let worn = icon(command, state);

    let response = match (shown, state.settings.toolbar.show_labels) {
        (Shown::MenuEntry, _) => {
            let called = name_and_key(name, key);
            menu_entry(ui, pressable, on, worn, called).on_hover_text(hint)
        }
        (shown, labels) => {
            let drawn = match (shown, labels) {
                (Shown::Small(rows), true) => {
                    small_button(ui, pressable, on, worn, name, key, rows)
                }
                (Shown::Small(rows), false) => {
                    small_icon_button(ui, pressable, on, worn, name, rows)
                }
                (_, true) => tool_button(ui, pressable, on, worn, name, key),
                (_, false) => icon_button(ui, pressable, on, worn, name),
            };
            tooltip(drawn, name, key, Some(&hint))
        }
    };
    if response.clicked() {
        asked.push(command);
    }
    response
}

#[cfg(test)]
mod tests;

/// What a chamfer held offers besides its values, while one is held: its two
/// sides swapped. Here until a panel of the feature's own takes it.
fn chamfer_row(ui: &mut egui::Ui, state: &Context<'_>, asked: &mut Vec<Command>, shown: Shown) {
    if chamfer_to_swap(state.document, state.editor).is_none() {
        return;
    }
    ui.horizontal_wrapped(|ui| button(ui, Command::SwapChamferSides, state, asked, shown));
}
