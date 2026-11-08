use egui::{
    Align, Label, Layout, Response, RichText, TextStyle, TextWrapMode, Ui, WidgetText, vec2,
};

use crate::ui::theme::tokens::{ACCENT_AMBER, TYPOGRAPHY_SIZES_BODY};

/// How far apart the logo's letters stand, in ems.
const LETTER_SPACING: f32 = 0.14;

/// The text logo at the head of the tab row: amber, its letters spaced out,
/// and not a text to select. It stands as tall as a button holding it, its
/// text in the middle, so that it reads on the line of the tabs beside it: a
/// row lays a shorter widget in the height it had when that widget came,
/// before the tabs made it taller.
pub fn logo(ui: &mut Ui, text: &str) -> Response {
    let text = RichText::new(text)
        .strong()
        .size(TYPOGRAPHY_SIZES_BODY)
        .color(ACCENT_AMBER)
        .extra_letter_spacing(LETTER_SPACING * TYPOGRAPHY_SIZES_BODY);
    let galley = WidgetText::from(text).into_galley(
        ui,
        Some(TextWrapMode::Extend),
        f32::INFINITY,
        TextStyle::Body,
    );
    let tall = galley.size().y + 2.0 * ui.spacing().button_padding.y;
    let centred = Layout::left_to_right(Align::Center);
    ui.allocate_ui_with_layout(vec2(0.0, tall), centred, |ui| {
        ui.add(Label::new(galley).selectable(false))
    })
    .inner
}
