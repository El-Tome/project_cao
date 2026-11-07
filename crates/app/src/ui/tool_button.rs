use egui::widget_style::{Classes, HasClasses, SELECTED_CLASS};
use egui::{
    Align2, Atom, Button, Color32, Id, Response, TextStyle, TextWrapMode, Ui, Vec2, WidgetInfo,
    WidgetText, WidgetType,
};

use crate::ui::icon::{Icon, IconSize};

/// A button that shows whether its tool is the one in hand, greyed out while
/// it cannot be pressed, its icon above its name and in the same colour.
///
/// Added straight to the row rather than through `add_enabled_ui`: a child ui
/// has no size before it is laid, so a wrapping row never moves it down.
pub fn tool_button(ui: &mut Ui, enabled: bool, on: bool, icon: Icon, label: String) -> Response {
    ui.add_enabled(enabled, move |ui: &mut Ui| {
        let name = WidgetText::from(label.as_str()).into_galley(
            ui,
            Some(TextWrapMode::Extend),
            f32::INFINITY,
            TextStyle::Button,
        );
        let side = IconSize::Large.points();
        let face_size = Vec2::new(
            name.size().x.max(side),
            side + ui.spacing().icon_spacing + name.size().y,
        );

        // `Button` lays its atoms out in a row, so it is handed one empty
        // atom the size of the face, and the face is painted where it lands.
        let face = Id::new("tool_button_face");
        let shown = Button::selectable(on, Atom::custom(face, face_size)).atom_ui(ui);
        let painted_at = shown.rect(face);
        let response = shown.response;
        if let Some(rect) = painted_at {
            let ink = ink(ui, &response, on);
            let icon_rect = Align2::CENTER_TOP.align_size_within_rect(Vec2::splat(side), rect);
            icon.image(IconSize::Large)
                .tint(ink)
                .paint_at(ui, icon_rect);
            let name_at = Align2::CENTER_BOTTOM.align_size_within_rect(name.size(), rect);
            ui.painter().galley(name_at.min, name, ink);
        }
        response
            .widget_info(|| WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), on, &label));
        response
    })
}

/// The same button with no name written: its icon alone, the name kept as
/// what it is called, for the tooltip and for whoever cannot see it.
pub fn icon_button(ui: &mut Ui, enabled: bool, on: bool, icon: Icon, name: &str) -> Response {
    ui.add_enabled(
        enabled,
        Button::selectable(on, icon.image(IconSize::Large).alt_text(name))
            .image_tint_follows_text_color(true),
    )
}

/// The same button as an entry of a menu: a row, its icon beside its name.
pub fn menu_entry(ui: &mut Ui, enabled: bool, on: bool, icon: Icon, label: String) -> Response {
    ui.add_enabled(
        enabled,
        Button::selectable(on, (icon.image(IconSize::Row), label))
            .image_tint_follows_text_color(true),
    )
}

/// The colour `egui::Button` writes its text in, in the state it is in.
fn ink(ui: &Ui, response: &Response, on: bool) -> Color32 {
    let mut classes = Classes::default();
    classes.add_class_if(SELECTED_CLASS, on);
    ui.style()
        .button_style(&classes, response.widget_state())
        .text_style
        .color
}

#[cfg(test)]
mod tests;
