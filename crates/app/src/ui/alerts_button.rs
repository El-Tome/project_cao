use egui::{Button, Response, RichText, Stroke, Ui};

use crate::ui::badge::Level;
use crate::ui::icon::{self, IconSize};
use crate::ui::theme::tokens::RADIUS_PILL;

/// How many things are wrong, behind the warning triangle, on a patch with
/// round ends in the colours of `level`, as a badge of that level wears them.
pub fn alerts_button(ui: &mut Ui, text: &str, level: Level) -> Response {
    let (fill, ink) = level.colours();
    let triangle = icon::WARNING.image(IconSize::Small).tint(ink);
    ui.add(
        Button::new((triangle, RichText::new(text).color(ink)))
            .fill(fill)
            .stroke(Stroke::NONE)
            .corner_radius(RADIUS_PILL),
    )
}
