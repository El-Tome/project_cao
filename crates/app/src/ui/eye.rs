use crate::ui::icon::{self, IconSize};

/// The eye beside a row: open while what the row names is shown, shut while
/// it is not, and called what a click on it does. Says whether it was clicked.
pub fn eye(ui: &mut egui::Ui, lit: bool, hover: String) -> bool {
    let drawn = if lit { icon::EYE } else { icon::EYE_OFF };
    let image = drawn.image(IconSize::Small).alt_text(hover.as_str());
    ui.add(egui::Button::selectable(lit, image).image_tint_follows_text_color(true))
        .on_hover_text(hover)
        .clicked()
}
