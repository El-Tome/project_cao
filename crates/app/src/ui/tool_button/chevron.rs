use egui::{Response, Stroke, Ui, vec2};

use super::{ink, pressable, undo_the_fade};

/// The 8 points of the chevron, and a little room either side of it to aim at.
pub const CHEVRON_WIDTH: f32 = 12.0;
const HALF_CHEVRON: f32 = 4.0;
const HALF_RISE: f32 = 2.0;
const STROKE_WIDTH: f32 = 1.5;

/// The part of a split button that opens its menu, as tall as its body.
pub fn chevron_button(ui: &mut Ui, enabled: bool, height: f32, name: &str) -> Response {
    ui.add_enabled(enabled, move |ui: &mut Ui| {
        undo_the_fade(ui);
        let stroke = Stroke::new(STROKE_WIDTH, ink(ui.is_enabled(), false));
        let (rect, response) = pressable(ui, vec2(CHEVRON_WIDTH, height), false, name, None);
        let tip = rect.center() + vec2(0.0, HALF_RISE);
        for side in [-HALF_CHEVRON, HALF_CHEVRON] {
            let end = rect.center() + vec2(side, -HALF_RISE);
            ui.painter().line_segment([end, tip], stroke);
        }
        response
    })
}
