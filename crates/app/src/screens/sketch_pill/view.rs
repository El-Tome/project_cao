use crate::ui::pill::{floating_pill, name_and_detail, note};
use crate::ui::primary_button::{self, primary_button};
use crate::ui::theme::tokens::{STATE_SUCCESS_TEXT, TEXT_QUIET};

use super::state::{Left, SketchPill};

/// Draws the pill at the top centre of `view`, and says whether its button was
/// clicked.
pub fn show(ctx: &egui::Context, view: egui::Rect, pill: &SketchPill) -> bool {
    let id = egui::Id::new("sketch_pill");
    floating_pill(ctx, id, view, primary_button::HEIGHT, |ui| {
        name_and_detail(ui, &pill.name, &pill.plane);
        if let Some(left) = &pill.left_read {
            let ink = match pill.left {
                Left::Settled => STATE_SUCCESS_TEXT,
                Left::NothingDrawn | Left::Free(_) => TEXT_QUIET,
            };
            note(ui, left, ink);
        }
        primary_button(ui, &pill.finish).clicked()
    })
}
