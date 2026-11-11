use egui::{Response, RichText, Ui};

use crate::ui::badge::Level;
use crate::ui::icon::{self, IconSize};
use crate::ui::secondary_button::secondary_button;
use crate::ui::theme::tokens::{
    TEXT_SECONDARY, TEXT_TITLE, TYPOGRAPHY_SIZES_BUTTON, TYPOGRAPHY_SIZES_DETAIL,
};

/// One alert of a list: the icon of its level on the left of its title, why
/// under it in a sentence, then the two things that can be done about it.
/// Hands back the two buttons, in the order they are named.
pub fn alert_line(
    ui: &mut Ui,
    level: Level,
    (title, sentence): (&str, &str),
    (first, second): (&str, &str),
) -> [Response; 2] {
    let (_, ink) = level.colours();
    let drawn = match level {
        Level::Warning => icon::WARNING,
        Level::Error => icon::ERROR,
    };
    ui.horizontal_top(|ui| {
        ui.add(drawn.image(IconSize::Row).tint(ink));
        ui.vertical(|ui| {
            let title = RichText::new(title).size(TYPOGRAPHY_SIZES_BUTTON);
            ui.label(title.color(TEXT_TITLE));
            let sentence = RichText::new(sentence).size(TYPOGRAPHY_SIZES_DETAIL);
            ui.label(sentence.color(TEXT_SECONDARY));
            ui.horizontal(|ui| {
                [
                    secondary_button(ui, true, first),
                    secondary_button(ui, true, second),
                ]
            })
            .inner
        })
        .inner
    })
    .inner
}
