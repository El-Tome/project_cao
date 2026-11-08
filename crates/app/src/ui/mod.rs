pub mod dropping;
pub mod eye;
pub mod floating_panel;
pub mod formula_field;
pub mod icon;
pub mod logo;
pub mod panel_toggle;
pub mod question;
pub mod slider;
pub mod text_edit;
pub mod theme;
pub mod tool_button;

/// What every primitive draws with: the theme, and the icons.
pub fn install(ctx: &egui::Context) {
    theme::install(ctx);
    icon::install(ctx);
}
