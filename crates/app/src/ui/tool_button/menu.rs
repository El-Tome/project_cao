use egui::{Align2, CornerRadius, FontId, Frame, Margin, Popup, Response, Stroke, Ui, Vec2, vec2};

use super::{ink, key_ink, pressable, undo_the_fade};
use crate::ui::icon::{Icon, IconSize};
use crate::ui::theme::tokens::{
    BORDER_FIELD, BORDER_SEPARATOR, SHADOW_MENU, SPACING_M, TYPOGRAPHY_SIZES_BUTTON,
    TYPOGRAPHY_SIZES_MONO_VALUE,
};

const MENU_WIDTH: f32 = 230.0;
const MENU_RADIUS: u8 = 8;
const MENU_BORDER: f32 = 1.0;
const ROW_HEIGHT: f32 = 30.0;
const ROW_INSET: f32 = 10.0;
const ICON_GAP: f32 = 8.0;

/// The dropdown (C8) a chevron opens. The rows are the caller's to add.
pub fn ribbon_menu(opened_by: &Response, rows: impl FnOnce(&mut Ui)) {
    let margin = SPACING_M;
    let frame = Frame::new()
        .fill(BORDER_SEPARATOR)
        .stroke(Stroke::new(MENU_BORDER, BORDER_FIELD))
        .corner_radius(CornerRadius::same(MENU_RADIUS))
        .shadow(SHADOW_MENU)
        .inner_margin(Margin::same(margin as i8));
    Popup::menu(opened_by)
        .frame(frame)
        .width(MENU_WIDTH)
        .show(|ui| {
            ui.set_width(MENU_WIDTH - 2.0 * (margin + MENU_BORDER));
            ui.spacing_mut().item_spacing.y = 0.0;
            rows(ui);
        });
}

/// A line of the menu: its icon and its name, and on its right what `hint`
/// says, the key or a word about the entry. `key` is only what it is called.
pub fn menu_row(
    ui: &mut Ui,
    enabled: bool,
    on: bool,
    icon: Icon,
    name: &str,
    hint: Option<&str>,
    key: Option<&str>,
) -> Response {
    ui.add_enabled(enabled, move |ui: &mut Ui| {
        undo_the_fade(ui);
        let ink = ink(ui.is_enabled(), on);
        let font = FontId::proportional(TYPOGRAPHY_SIZES_BUTTON);
        let written_name = ui.painter().layout_no_wrap(name.to_owned(), font, ink);
        let hint_ink = key_ink(ui.is_enabled());
        let written_hint = hint.map(|hint| {
            let font = FontId::monospace(TYPOGRAPHY_SIZES_MONO_VALUE);
            ui.painter().layout_no_wrap(hint.to_owned(), font, hint_ink)
        });

        let size = vec2(ui.available_width(), ROW_HEIGHT);
        let (rect, response) = pressable(ui, size, on, name, key);

        let inside = rect.shrink2(vec2(ROW_INSET, 0.0));
        let side = IconSize::Row.points();
        let icon_rect = Align2::LEFT_CENTER.align_size_within_rect(Vec2::splat(side), inside);
        icon.image(IconSize::Row).tint(ink).paint_at(ui, icon_rect);
        let name_at = Align2::LEFT_CENTER.align_size_within_rect(
            written_name.size(),
            inside.with_min_x(icon_rect.right() + ICON_GAP),
        );
        ui.painter().galley(name_at.min, written_name, ink);
        if let Some(hint) = written_hint {
            let at = Align2::RIGHT_CENTER.align_size_within_rect(hint.size(), inside);
            ui.painter().galley(at.min, hint, hint_ink);
        }

        response
    })
}
