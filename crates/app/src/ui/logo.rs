use egui::{Label, Response, RichText, Ui};

use crate::ui::theme::tokens::{ACCENT_AMBER, TYPOGRAPHY_SIZES_BODY};

/// How far apart the logo's letters stand, in ems.
const LETTER_SPACING: f32 = 0.14;

/// The text logo at the head of the tab row: amber, its letters spaced out,
/// and not a text to select.
pub fn logo(ui: &mut Ui, text: &str) -> Response {
    let text = RichText::new(text)
        .strong()
        .size(TYPOGRAPHY_SIZES_BODY)
        .color(ACCENT_AMBER)
        .extra_letter_spacing(LETTER_SPACING * TYPOGRAPHY_SIZES_BODY);
    ui.add(Label::new(text).selectable(false))
}
