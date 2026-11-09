use egui::{CornerRadius, Frame, Label, Margin, Response, RichText, Stroke, Tooltip};

use super::key_frame;
use crate::ui::theme::tokens::{
    BORDER_GROUP, BORDER_TOOLTIP, RADIUS_CONTROL, SPACING_L, TEXT_SUBDUED,
};

const TOOLTIP_WIDTH: f32 = 230.0;
const TOOLTIP_BORDER: f32 = 1.0;

/// What a button shows when hovered (C9): its name, its key in a frame if it
/// has one, then a sentence of help if it has one. A greyed-out button shows
/// nothing.
pub fn tooltip(response: Response, name: &str, key: Option<&str>, help: Option<&str>) -> Response {
    let mut shown = Tooltip::for_enabled(&response);
    shown.popup = shown.popup.frame(
        Frame::new()
            .fill(BORDER_GROUP)
            .stroke(Stroke::new(TOOLTIP_BORDER, BORDER_TOOLTIP))
            .corner_radius(CornerRadius::same(RADIUS_CONTROL))
            .inner_margin(Margin::same(SPACING_L as i8)),
    );
    shown.show(|ui| {
        ui.set_width(TOOLTIP_WIDTH - 2.0 * (SPACING_L + TOOLTIP_BORDER));
        ui.horizontal(|ui| {
            ui.label(name);
            if let Some(key) = key {
                key_frame(ui, key);
            }
        });
        if let Some(help) = help {
            ui.add(Label::new(RichText::new(help).small().color(TEXT_SUBDUED)).wrap());
        }
    });
    response
}
