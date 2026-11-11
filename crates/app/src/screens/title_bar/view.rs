use crate::lang::Catalogue;
use crate::ui::alerts_button::alerts_button;
use crate::ui::icon;
use crate::ui::panel_toggle::panel_button;
use crate::ui::pill::{MESSAGE_HEIGHT, Tone, message_pill, pill};

use super::state::{Said, TitleBar};

/// Draws the title bar along the top of the screen, and says whether the way
/// home was clicked. The message stands in the middle of the window rather
/// than of what the breadcrumb leaves, so it does not move as the part's name
/// grows; the count of alerts at its right end, which the message never covers.
pub fn show(ui: &mut egui::Ui, bar: &TitleBar<'_>, lang: &Catalogue) -> bool {
    let middle = ui.ctx().content_rect().center().x;
    egui::Panel::top("part_title_bar")
        .show(ui, |ui| {
            let row = egui::vec2(ui.available_width(), MESSAGE_HEIGHT);
            let centred = egui::Layout::left_to_right(egui::Align::Center);
            ui.allocate_ui_with_layout(row, centred, |ui| {
                let home = panel_button(ui, false, icon::HOME, lang.t("app.home")).clicked();
                ui.weak("/");
                ui.strong(bar.name)
                    .on_hover_text(bar.path.display().to_string());
                ui.weak("/");
                pill(ui, &bar.view);
                let from_the_right = egui::Layout::right_to_left(egui::Align::Center);
                ui.with_layout(from_the_right, |ui| {
                    if let Some(count) = &bar.alerts_button {
                        alerts_button(ui, &count.text, count.level);
                    }
                    ui.with_layout(centred, |ui| {
                        if let Some(said) = bar.said {
                            let (text, tone) = match said {
                                Said::Message(message) => (message, Tone::Said),
                                Said::Error(error) => (error, Tone::Failed),
                            };
                            message_pill(ui, middle, text, tone);
                        }
                    });
                });
                home
            })
            .inner
        })
        .inner
}
