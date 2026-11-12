use egui::{Color32, Response, RichText, Ui};

use crate::ui::badge::Level;
use crate::ui::icon::{self, IconSize};
use crate::ui::secondary_button::secondary_button;
use crate::ui::theme::tokens::{
    TEXT_SECONDARY, TEXT_TITLE, TYPOGRAPHY_SIZES_BUTTON, TYPOGRAPHY_SIZES_DETAIL,
};

/// One alert of a list: the icon of its level on the left of its title, in
/// the level's ink, why under it in a sentence, then the two things that can
/// be done about it, when anything can. Hands back the two buttons, in the
/// order they are named.
pub fn alert_line(
    ui: &mut Ui,
    (level, ink): (Level, Color32),
    (title, sentence): (&str, &str),
    actions: Option<(&str, &str)>,
) -> Option<[Response; 2]> {
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
            let (first, second) = actions?;
            let buttons = ui.horizontal(|ui| {
                [
                    secondary_button(ui, true, first),
                    secondary_button(ui, true, second),
                ]
            });
            Some(buttons.inner)
        })
        .inner
    })
    .inner
}
