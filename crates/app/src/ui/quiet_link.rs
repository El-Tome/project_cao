use egui::{Button, Response, RichText, Ui};

use crate::ui::theme::tokens::{TEXT_QUIET, TYPOGRAPHY_SIZES_DETAIL};

/// An action set aside at the foot of a list: quiet text with no frame, which
/// asks to be found rather than pressed by accident.
pub fn quiet_link(ui: &mut Ui, label: &str) -> Response {
    let text = RichText::new(label)
        .size(TYPOGRAPHY_SIZES_DETAIL)
        .color(TEXT_QUIET);
    ui.add(Button::new(text).frame(false))
}
