use crate::commands::Asked;
use crate::screens::view_bar::offered_button;
use crate::ui::floating_bar::floating_row;

use super::state::OriginViewButton;

/// Draws the button over `view`, and says what was asked of it.
pub fn show(ctx: &egui::Context, view: egui::Rect, button: &OriginViewButton) -> Option<Asked> {
    let id = egui::Id::new("origin_view");
    floating_row(ctx, id, view, (button.pivot, button.at), |ui| {
        offered_button(ui, &button.offered)
            .clicked()
            .then_some(Asked::Command(button.offered.command))
    })
}
