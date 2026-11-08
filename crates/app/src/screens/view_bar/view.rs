//! The drawing of the view bar, through the `ui/` primitives.

use crate::screens::ribbon::icons;
use crate::ui::floating_bar::{bar_button, bar_menu, bar_reading, bar_separator, floating_bar};

use super::state::{Offered, Picked, ViewBar};

/// Draws the bar against the bottom right corner of `view`, and says what was
/// picked on it.
pub fn show(ctx: &egui::Context, view: egui::Rect, bar: &ViewBar) -> Option<Picked> {
    floating_bar(ctx, egui::Id::new("view_bar"), view, |ui| {
        let mut picked = None;
        for offered in [&bar.reframe, &bar.section_view] {
            if button(ui, offered).clicked() {
                picked = Some(Picked::Command(offered.command));
            }
        }
        let menu = &bar.matter_in_front;
        let entries: Vec<(&str, bool)> = menu
            .choices
            .iter()
            .map(|choice| (choice.label.as_str(), choice.current))
            .collect();
        let worn = icons::matter_in_front(menu.state);
        if let Some(rank) = bar_menu(ui, menu.enabled, worn, &menu.label, &entries) {
            picked = Some(Picked::MatterInFront(menu.choices[rank].state));
        }
        bar_separator(ui);
        bar_reading(ui, &bar.coordinates, &bar.widest_coordinates);
        picked
    })
}

fn button(ui: &mut egui::Ui, offered: &Offered) -> egui::Response {
    let worn = icons::of(offered.command);
    let key = offered.key.as_deref();
    bar_button(ui, offered.enabled, offered.on, worn, &offered.name, key)
        .on_hover_text(&offered.hint)
}
