use crate::ui::pill::{floating_pill, name_and_detail};
use crate::ui::primary_button::primary_button;

use super::state::SketchPill;

/// Draws the pill at the top centre of `view`, and says whether its button was
/// clicked.
pub fn show(ctx: &egui::Context, view: egui::Rect, pill: &SketchPill) -> bool {
    floating_pill(ctx, egui::Id::new("sketch_pill"), view, |ui| {
        name_and_detail(ui, &pill.name, &pill.plane);
        primary_button(ui, &pill.finish).clicked()
    })
}
