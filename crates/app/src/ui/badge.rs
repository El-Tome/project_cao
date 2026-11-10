//! A word that marks what stands beside it, in the colours of how grave it is:
//! C12 of `docs/redesign/components.md`.

use egui::{Frame, Margin, Response, RichText, Ui};

use crate::ui::theme::tokens::{
    RADIUS_PILL, SPACING_M, STATE_ERROR_BACKGROUND, STATE_ERROR_TEXT, STATE_WARNING_BACKGROUND,
    STATE_WARNING_TEXT,
};

/// The size C12 gives a badge's word, which the tokens do not carry.
const TEXT_SIZE: f32 = 10.0;

/// How grave what a badge says is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Warning,
    Error,
}

/// How wide `badge` draws `said`, patch and all.
pub fn badge_width(ui: &Ui, said: &str) -> f32 {
    let font = egui::FontId::proportional(TEXT_SIZE);
    let text = ui
        .painter()
        .layout_no_wrap(said.to_string(), font, STATE_WARNING_TEXT);
    text.size().x + 2.0 * SPACING_M
}

pub fn badge(ui: &mut Ui, said: &str, level: Level) -> Response {
    let (fill, ink) = match level {
        Level::Warning => (STATE_WARNING_BACKGROUND, STATE_WARNING_TEXT),
        Level::Error => (STATE_ERROR_BACKGROUND, STATE_ERROR_TEXT),
    };
    Frame::new()
        .fill(fill)
        .corner_radius(RADIUS_PILL)
        .inner_margin(Margin::symmetric(SPACING_M as i8, 1))
        .show(ui, |ui| {
            ui.label(RichText::new(said).size(TEXT_SIZE).color(ink))
        })
        .inner
}
