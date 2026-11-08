use crate::lang::Catalogue;
use crate::ui::icon;
use crate::ui::panel_toggle::panel_button;
use crate::ui::pill::pill;
use crate::ui::theme::tokens;

use super::state::TitleBar;

/// Draws the title bar along the top of the screen, and says whether the way
/// home was clicked.
pub fn show(ui: &mut egui::Ui, bar: &TitleBar<'_>, lang: &Catalogue) -> bool {
    egui::Panel::top("part_title_bar")
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let home = panel_button(ui, false, icon::HOME, lang.t("app.home")).clicked();
                ui.weak("/");
                ui.strong(bar.name)
                    .on_hover_text(bar.path.display().to_string());
                ui.weak("/");
                pill(ui, &bar.view);
                if let Some(message) = bar.message {
                    ui.separator();
                    ui.colored_label(tokens::ACCENT_AMBER_TEXT, message);
                }
                home
            })
            .inner
        })
        .inner
}
