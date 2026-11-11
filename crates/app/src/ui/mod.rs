pub mod alert_line;
pub mod alerts_button;
pub mod badge;
pub mod dropping;
pub mod eye;
pub mod floating_bar;
pub mod floating_panel;
pub mod formula_field;
pub mod history_line;
pub mod icon;
pub mod logo;
pub mod panel_toggle;
pub mod pill;
pub mod primary_button;
pub mod question;
pub mod quiet_link;
pub mod reader_card;
pub mod secondary_button;
pub mod sheet;
pub mod slider;
pub mod step_box;
pub mod text_edit;
pub mod theme;
pub mod tool_button;
pub mod tree_row;

/// What every primitive draws with: the theme, and the icons.
pub fn install(ctx: &egui::Context) {
    theme::install(ctx);
    icon::install(ctx);
}
