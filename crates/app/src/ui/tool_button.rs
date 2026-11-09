use std::sync::Arc;

use egui::text::LayoutJob;
use egui::{
    Align, Align2, Button, Color32, FontId, Galley, Rect, Response, Sense, Stroke, StrokeKind, Ui,
    Vec2, Widget, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::ui::icon::{Icon, IconSize};
use crate::ui::theme::tokens::{
    ACCENT_AMBER_BACKGROUND, ACCENT_AMBER_BORDER, ACCENT_AMBER_LIGHT, BACKGROUND_HOVER_STRONG,
    RADIUS_CONTROL, TEXT_BUTTON, TEXT_DISABLED, TEXT_SHORTCUT, TYPOGRAPHY_SIZES_RIBBON_BUTTON,
    TYPOGRAPHY_SIZES_SHORTCUT,
};

mod chevron;
mod menu;
mod small;

pub use chevron::{CHEVRON_WIDTH, chevron_button};
pub use menu::{menu_row, ribbon_menu};
pub use small::{Rows, small_button, small_grid};

const LARGE_BUTTON_AT_LEAST: Vec2 = vec2(50.0, 64.0);
const NAME_WRAPS_AT: f32 = 70.0;
const NAME_WRAPS_AT_MOST_AT: f32 = 210.0;
const NAME_WIDENS_BY: f32 = 4.0;
const NAME_ROWS: usize = 2;
const KEY_INSET: f32 = 4.0;

/// What a button is called for whoever cannot see it, and in its tooltip.
pub fn name_and_key(name: &str, key: Option<&str>) -> String {
    match key {
        Some(key) => format!("{name} ({key})"),
        None => name.to_owned(),
    }
}

/// A large button of the ribbon: its icon above its name, its key in the top
/// right corner, greyed out while it cannot be pressed.
///
/// Added straight to the row rather than through `add_enabled_ui`: a child ui
/// has no size before it is laid, so a wrapping row never moves it down.
pub fn tool_button(
    ui: &mut Ui,
    enabled: bool,
    on: bool,
    icon: Icon,
    name: &str,
    key: Option<&str>,
) -> Response {
    ui.add_enabled(enabled, move |ui: &mut Ui| {
        undo_the_fade(ui);
        let ink = ink(ui.is_enabled(), on);

        let written_name = written_name(ui, name, ink);
        let written_key = written_key(ui, key);

        let side = IconSize::Large.points();
        let padding = ui.spacing().button_padding;
        let gap = ui.spacing().icon_spacing;
        let key_width = written_key
            .as_ref()
            .map_or(0.0, |key| key.size().x + 2.0 * KEY_INSET);
        let size = LARGE_BUTTON_AT_LEAST.max(vec2(
            (written_name.size().x + 2.0 * padding.x).max(key_width),
            2.0 * padding.y + side + gap + written_name.size().y,
        ));

        let (rect, response) = pressable(ui, size, on, name, key);

        let icon_rect =
            Align2::CENTER_TOP.align_size_within_rect(Vec2::splat(side), rect.shrink2(padding));
        icon.image(IconSize::Large)
            .tint(ink)
            .paint_at(ui, icon_rect);
        ui.painter().galley(
            pos2(rect.center().x, icon_rect.bottom() + gap),
            written_name,
            ink,
        );
        if let Some(key) = written_key {
            let at = Align2::RIGHT_TOP.align_size_within_rect(key.size(), rect.shrink(KEY_INSET));
            ui.painter().galley(at.min, key, key_ink(ui.is_enabled()));
        }

        response
    })
}

/// The same button with no name written: its icon alone, the name kept as
/// what it is called, for the tooltip and for whoever cannot see it.
pub fn icon_button(ui: &mut Ui, enabled: bool, on: bool, icon: Icon, name: &str) -> Response {
    ui.add_enabled(enabled, move |ui: &mut Ui| {
        undo_the_fade(ui);
        let ink = ink(ui.is_enabled(), on);
        Button::selectable(on, icon.image(IconSize::Large).tint(ink).alt_text(name)).ui(ui)
    })
}

/// The same button as an entry of a menu: a row, its icon beside its name.
pub fn menu_entry(ui: &mut Ui, enabled: bool, on: bool, icon: Icon, label: String) -> Response {
    ui.add_enabled(
        enabled,
        Button::selectable(on, (icon.image(IconSize::Row), label))
            .image_tint_follows_text_color(true),
    )
}

/// A name wrapped at the width C1 asks for, or wider when it would take more
/// than two rows there: « Matière devant : cachée » ends on the word that says
/// the state, which cutting it short would lose.
fn written_name(ui: &Ui, name: &str, ink: Color32) -> Arc<Galley> {
    let wrapped_at = |width: f32| {
        let font = FontId::proportional(TYPOGRAPHY_SIZES_RIBBON_BUTTON);
        let mut job = LayoutJob::simple(name.to_owned(), font, ink, width);
        job.halign = Align::Center;
        ui.painter().layout_job(job)
    };
    let mut width = NAME_WRAPS_AT;
    let mut written = wrapped_at(width);
    while written.rows.len() > NAME_ROWS && width < NAME_WRAPS_AT_MOST_AT {
        width += NAME_WIDENS_BY;
        written = wrapped_at(width);
    }
    written
}

fn written_key(ui: &Ui, key: Option<&str>) -> Option<Arc<Galley>> {
    key.map(|key| {
        let font = FontId::monospace(TYPOGRAPHY_SIZES_SHORTCUT);
        ui.painter()
            .layout_no_wrap(key.to_owned(), font, key_ink(ui.is_enabled()))
    })
}

/// Takes a button's room, paints its state behind what it will hold, and says
/// what it is called.
fn pressable(ui: &mut Ui, size: Vec2, on: bool, name: &str, key: Option<&str>) -> (Rect, Response) {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    paint_the_state(ui, &response, rect, on);
    let called = name_and_key(name, key);
    response.widget_info(|| WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), on, &called));
    (rect, response)
}

/// The redesign names the colour of a greyed-out button outright, so the
/// opacity `add_enabled` lays over the whole widget would fade it a second time.
fn undo_the_fade(ui: &mut Ui) {
    if !ui.is_enabled() {
        ui.set_opacity(ui.opacity() / ui.visuals().disabled_alpha());
    }
}

fn ink(enabled: bool, on: bool) -> Color32 {
    match (enabled, on) {
        (false, _) => TEXT_DISABLED,
        (true, true) => ACCENT_AMBER_LIGHT,
        (true, false) => TEXT_BUTTON,
    }
}

fn key_ink(enabled: bool) -> Color32 {
    if enabled {
        TEXT_SHORTCUT
    } else {
        TEXT_DISABLED
    }
}

fn paint_the_state(ui: &Ui, response: &Response, rect: Rect, on: bool) {
    if !response.enabled() {
        return;
    }
    let (fill, stroke) = if on {
        (
            ACCENT_AMBER_BACKGROUND,
            Stroke::new(1.0, ACCENT_AMBER_BORDER),
        )
    } else if response.hovered() || response.has_focus() {
        (BACKGROUND_HOVER_STRONG, Stroke::NONE)
    } else {
        return;
    };
    ui.painter()
        .rect(rect, RADIUS_CONTROL, fill, stroke, StrokeKind::Inside);
}

#[cfg(test)]
mod tests;
