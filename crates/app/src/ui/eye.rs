use egui::{Button, Vec2};

use crate::ui::icon::{self, IconSize};

/// C11's side, in points, which the tokens do not carry.
const SIDE: f32 = 22.0;

/// The eye beside a row: open while what the row names is shown, shut while
/// it is not, and called what a click on it does. Says whether it was clicked.
pub fn eye(ui: &mut egui::Ui, lit: bool, hover: String) -> bool {
    let drawn = if lit { icon::EYE } else { icon::EYE_OFF };
    let image = drawn.image(IconSize::Small).alt_text(hover.as_str());
    let button = Button::selectable(lit, image)
        .image_tint_follows_text_color(true)
        .min_size(Vec2::splat(SIDE));
    ui.scope(|ui| {
        let margin = (SIDE - IconSize::Small.points()) / 2.0;
        ui.spacing_mut().button_padding = Vec2::splat(margin);
        ui.add(button)
    })
    .inner
    .on_hover_text(hover)
    .clicked()
}
