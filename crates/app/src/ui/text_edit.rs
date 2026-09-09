/// A single-line text field with its width and its placeholder baked in.
pub fn text_edit(ui: &mut egui::Ui, value: &mut String, width: f32, hint: &str) -> egui::Response {
    ui.add(
        egui::TextEdit::singleline(value)
            .desired_width(width)
            .hint_text(hint),
    )
}
