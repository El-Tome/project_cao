//! One line of a history: its number in monospace, its name beside it, and
//! under the name a quieter line saying what it did. The line is one button,
//! read « 2. Rectangle »; the detail is a text of its own beneath.

use egui::{
    Align, Align2, FontId, Label, Layout, Rect, Response, RichText, Sense, Ui, UiBuilder,
    WidgetInfo, WidgetType, pos2, vec2,
};

use crate::ui::theme::tokens::{
    ACCENT_AMBER_BACKGROUND, ACCENT_AMBER_TEXT, BACKGROUND_HOVER, RADIUS_BUTTON, SPACING_M,
    SPACING_S, STATE_WARNING_TEXT, TEXT_DISABLED, TEXT_PRIMARY, TEXT_QUIET,
    TYPOGRAPHY_SIZES_BUTTON, TYPOGRAPHY_SIZES_DETAIL, TYPOGRAPHY_SIZES_MONO_VALUE,
};

/// The measures in points the tokens do not carry: a line's height, the room
/// its number is given, and how far below the name the detail starts.
const HEIGHT: f32 = 38.0;
const NUMBER_COLUMN: f32 = 28.0;
const DETAIL_BELOW: f32 = 16.0;

/// What a line shows. The current one stands selected; an undone line is
/// written quieter, all of it; a warned one has its name in the warning's
/// colour.
pub struct HistoryLine<'a> {
    pub number: usize,
    pub name: &'a str,
    pub detail: &'a str,
    pub current: bool,
    pub undone: bool,
    pub warned: bool,
}

pub fn history_line(ui: &mut Ui, line: HistoryLine<'_>) -> Response {
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), HEIGHT), Sense::click());
    let said = format!("{}. {}", line.number, line.name);
    response.widget_info(|| {
        WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), line.current, &said)
    });

    let painter = ui.painter_at(rect);
    let fill = match (line.current, response.hovered()) {
        (true, _) => Some(ACCENT_AMBER_BACKGROUND),
        (false, true) => Some(BACKGROUND_HOVER),
        (false, false) => None,
    };
    if let Some(fill) = fill {
        painter.rect_filled(rect, RADIUS_BUTTON, fill);
    }
    let (ink, quiet) = match (line.undone, line.warned, line.current) {
        (true, _, _) => (TEXT_QUIET, TEXT_DISABLED),
        (false, true, _) => (STATE_WARNING_TEXT, TEXT_QUIET),
        (false, false, true) => (ACCENT_AMBER_TEXT, TEXT_QUIET),
        (false, false, false) => (TEXT_PRIMARY, TEXT_QUIET),
    };
    let top = rect.top() + SPACING_S;
    painter.text(
        pos2(rect.left() + SPACING_M, top),
        Align2::LEFT_TOP,
        line.number.to_string(),
        FontId::monospace(TYPOGRAPHY_SIZES_MONO_VALUE),
        quiet,
    );
    let text_left = rect.left() + NUMBER_COLUMN;
    painter.text(
        pos2(text_left, top),
        Align2::LEFT_TOP,
        line.name,
        FontId::proportional(TYPOGRAPHY_SIZES_BUTTON),
        ink,
    );

    let under = Rect::from_min_max(
        pos2(text_left, top + DETAIL_BELOW),
        pos2(rect.right() - SPACING_M, rect.bottom()),
    );
    let mut inside = ui.new_child(
        UiBuilder::new()
            .max_rect(under)
            .layout(Layout::left_to_right(Align::Min)),
    );
    let detail = RichText::new(line.detail)
        .size(TYPOGRAPHY_SIZES_DETAIL)
        .color(quiet);
    inside.add(Label::new(detail).truncate().selectable(false));
    response
}
