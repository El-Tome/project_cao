/// The eye beside a row: bright while what the row names is shown, faded while
/// it is not. Says whether it was clicked.
pub fn eye(ui: &mut egui::Ui, lit: bool, hover: String) -> bool {
    let mark = egui::RichText::new("👁");
    let mark = if lit { mark.strong() } else { mark.weak() };
    ui.add(egui::Button::selectable(lit, mark))
        .on_hover_text(hover)
        .clicked()
}
