use egui::{Button, Response, Ui};

use crate::ui::icon::{Icon, IconSize};

/// A toggle of the bar's header: its icon beside its name, pressed while the
/// panel it shows is shown.
pub fn panel_toggle(ui: &mut Ui, shown: &mut bool, icon: Icon, name: String) -> Response {
    let mut response = panel_button(ui, *shown, icon, name);
    if response.clicked() {
        *shown = !*shown;
        response.mark_changed();
    }
    response
}

/// The same face, for a button of the header that reports its click rather
/// than holding a state.
pub fn panel_button(ui: &mut Ui, on: bool, icon: Icon, name: String) -> Response {
    ui.add(
        Button::selectable(on, (icon.image(IconSize::Small), name))
            .image_tint_follows_text_color(true),
    )
}
