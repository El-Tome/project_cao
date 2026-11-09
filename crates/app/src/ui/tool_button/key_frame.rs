use egui::{CornerRadius, Frame, Margin, Response, RichText, Stroke, Ui};

use crate::ui::theme::tokens::{BORDER_KEY, SPACING_S};

const KEY_SIZE: f32 = 10.5;
const KEY_RADIUS: u8 = 3;

/// A key written in its frame (C14).
pub fn key_frame(ui: &mut Ui, key: &str) -> Response {
    Frame::new()
        .stroke(Stroke::new(1.0, BORDER_KEY))
        .corner_radius(CornerRadius::same(KEY_RADIUS))
        .inner_margin(Margin::symmetric(SPACING_S as i8, 1))
        .show(ui, |ui| {
            ui.label(RichText::new(key).monospace().size(KEY_SIZE));
        })
        .response
}
