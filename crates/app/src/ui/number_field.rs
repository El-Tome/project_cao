/// A number typed or dragged, followed whenever something else moves it.
pub fn number_field(ui: &mut egui::Ui, value: &mut f64, step: f64) -> egui::Response {
    ui.add(egui::DragValue::new(value).speed(step))
}
