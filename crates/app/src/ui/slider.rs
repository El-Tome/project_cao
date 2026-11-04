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

/// The same, each value said by `say` rather than as a bare number.
pub fn slider_saying<Num: Numeric>(
    ui: &mut egui::Ui,
    value: &mut Num,
    range: RangeInclusive<Num>,
    label: &str,
    say: impl Fn(f64) -> String,
) -> egui::Response {
    let slider = egui::Slider::new(value, range).text(label);
    ui.add(slider.custom_formatter(move |value, _| say(value)))
}
