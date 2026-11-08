use egui::{Frame, Margin, Response, RichText, Ui};

use crate::ui::theme::tokens::{
    BACKGROUND_HOVER, RADIUS_CONTROL, SPACING_L, SPACING_S, TEXT_BUTTON, TYPOGRAPHY_SIZES_DETAIL,
};

/// A short run of text on a rounded patch of its own, set apart from what
/// stands beside it on a bar.
pub fn pill(ui: &mut Ui, text: &str) -> Response {
    Frame::new()
        .fill(BACKGROUND_HOVER)
        .corner_radius(RADIUS_CONTROL)
        .inner_margin(Margin::symmetric(SPACING_L as i8, SPACING_S as i8))
        .show(ui, |ui| {
            ui.label(
                RichText::new(text)
                    .color(TEXT_BUTTON)
                    .size(TYPOGRAPHY_SIZES_DETAIL),
            )
        })
        .inner
}
