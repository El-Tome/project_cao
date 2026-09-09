use std::ops::RangeInclusive;

use egui::emath::Numeric;

/// A slider with its label baked in, the way every screen already wants it.
pub fn slider<Num: Numeric>(
    ui: &mut egui::Ui,
    value: &mut Num,
    range: RangeInclusive<Num>,
    label: &str,
) -> egui::Response {
    ui.add(egui::Slider::new(value, range).text(label))
}
