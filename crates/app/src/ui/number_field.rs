//! A number field with its − and + buttons.

use egui::epaint::RectShape;
use egui::{
    Align, Button, FontId, Frame, Layout, Response, RichText, Sense, Shape, Stroke, StrokeKind,
    TextEdit, Ui, UiBuilder, Vec2, vec2,
};

use crate::ui::theme::tokens::{
    ACCENT_AMBER, BACKGROUND_FIELD, BORDER_CONTROL, RADIUS_CONTROL, TEXT_SECONDARY, TEXT_TITLE,
    TYPOGRAPHY_SIZES_BODY,
};

/// How tall the field stands: C14's 34 px.
const HEIGHT: f32 = 34.0;

/// Half the row `docs/redesign/source/part.html` gives a step's two fields,
/// side by side in the circular pattern's panel.
const WIDTH: f32 = 115.0;

/// A − or + button, as `part.html` draws it.
const BUTTON: Vec2 = vec2(28.0, 32.0);

/// What one of the two buttons does to the number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nudge {
    Minus,
    Plus,
}

/// What the field holds once nudged by one. Nothing when it holds anything
/// but a plain number — a formula is the user's, and stays as written — nor
/// when − would take it below `least`.
pub fn nudged(text: &str, nudge: Nudge, least: f64) -> Option<String> {
    let number = text
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|number| number.is_finite())?;
    let put = match nudge {
        Nudge::Minus => number - 1.0,
        Nudge::Plus => number + 1.0,
    };
    (nudge == Nudge::Plus || put >= least).then(|| put.to_string())
}

/// The field holding `text`, a − before it and a + after it, bordered amber
/// while it is the current step's or holds the keyboard. The response is the
/// field's, stretched over the whole control, and says it changed whether the
/// number was typed or nudged.
#[cfg_attr(not(test), expect(dead_code))]
pub fn number_field(ui: &mut Ui, text: &mut String, current: bool, least: f64) -> Response {
    let (rect, whole) = ui.allocate_exact_size(vec2(WIDTH, HEIGHT), Sense::hover());
    let frame = ui.painter().add(Shape::Noop);
    let mut row = ui.new_child(
        UiBuilder::new()
            .max_rect(rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    row.spacing_mut().item_spacing.x = 0.0;

    let minus = nudge_button(&mut row, "−", nudged(text, Nudge::Minus, least));
    let typed = row.add(
        TextEdit::singleline(text)
            .frame(Frame::NONE)
            .font(FontId::monospace(TYPOGRAPHY_SIZES_BODY))
            .text_color(TEXT_TITLE)
            .horizontal_align(Align::Center)
            .vertical_align(Align::Center)
            .min_size(vec2(0.0, BUTTON.y))
            .desired_width(WIDTH - 2.0 * BUTTON.x),
    );
    let plus = nudge_button(&mut row, "+", nudged(text, Nudge::Plus, least));

    let border = match current || typed.has_focus() {
        true => ACCENT_AMBER,
        false => BORDER_CONTROL,
    };
    ui.painter().set(
        frame,
        RectShape::new(
            rect,
            RADIUS_CONTROL,
            BACKGROUND_FIELD,
            Stroke::new(1.0, border),
            StrokeKind::Inside,
        ),
    );

    let mut response = typed.union(whole);
    if let Some(put) = minus.or(plus) {
        *text = put;
        response.mark_changed();
    }
    response
}

/// A − or + button: what it puts in the field when clicked, greyed out while
/// it would put nothing there.
fn nudge_button(ui: &mut Ui, label: &str, put: Option<String>) -> Option<String> {
    let button = Button::new(RichText::new(label).color(TEXT_SECONDARY))
        .frame(false)
        .min_size(BUTTON);
    let clicked = ui.add_enabled(put.is_some(), button).clicked();
    put.filter(|_| clicked)
}

#[cfg(test)]
mod tests;
