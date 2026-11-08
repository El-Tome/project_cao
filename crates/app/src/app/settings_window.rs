use super::CaoApp;
use crate::screens;

impl CaoApp {
    /// The preferences, in a window over whatever is open. Kept out of the
    /// screen routing on purpose: settings apply to the whole application, and
    /// closing them must not close the part being drawn.
    pub(super) fn show_settings(&mut self, ui: &mut egui::Ui) {
        if !self.settings_open {
            return;
        }
        let mut open = true;
        let mut touched = false;

        egui::Window::new(self.remembered.lang().t("app.settings_window"))
            .open(&mut open)
            .default_size([720.0, 560.0])
            .vscroll(true)
            .show(ui.ctx(), |ui| {
                let (profiles, lang) = self.remembered.profiles_and_lang();
                touched = screens::settings::show(ui, profiles, &mut self.settings_editor, lang);
            });

        self.settings_open = open;
        if touched {
            self.save_settings();
        }
    }
}
