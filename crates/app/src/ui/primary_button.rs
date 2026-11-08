use egui::{Button, Response, RichText, Ui, vec2};

use crate::ui::theme::tokens::{ACCENT_AMBER, RADIUS_PILL, TEXT_ON_ACCENT};

/// How tall the button stands, in points: `docs/redesign/source/part.html`,
/// which the tokens do not carry.
pub const HEIGHT: f32 = 28.0;

/// The one action a place leads to, in amber with round ends, the key that
/// does it recalled in its label (`docs/redesign/components.md` C15). Its fill
/// stays amber under the pointer: only its border answers it.
pub fn primary_button(ui: &mut Ui, label: &str) -> Response {
    ui.add(
        Button::new(RichText::new(label).color(TEXT_ON_ACCENT))
            .fill(ACCENT_AMBER)
            .corner_radius(RADIUS_PILL)
            .min_size(vec2(0.0, HEIGHT)),
    )
}
