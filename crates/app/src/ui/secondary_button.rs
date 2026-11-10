use egui::{Button, Response, Ui};

/// An action beside the main one, as C15 of `docs/redesign/components.md`
/// draws its secondary button: transparent within a border, which the theme
/// gives every button at rest. Greyed, and deaf to a click, while it would do
/// nothing.
pub fn secondary_button(ui: &mut Ui, enabled: bool, label: &str) -> Response {
    ui.add_enabled(enabled, Button::new(label))
}
