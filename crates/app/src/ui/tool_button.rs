/// A button that shows whether its tool is the one in hand, greyed out while
/// it cannot be pressed.
///
/// Added straight to the row rather than through `add_enabled_ui`: a child ui
/// has no size before it is laid, so a wrapping row never moves it down.
pub fn tool_button(ui: &mut egui::Ui, enabled: bool, on: bool, label: String) -> egui::Response {
    ui.add_enabled(enabled, egui::Button::selectable(on, label))
}
