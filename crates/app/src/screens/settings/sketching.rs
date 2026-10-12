use cao_prefs::Profiles;

use crate::lang::Catalogue;

pub(super) fn section(ui: &mut egui::Ui, profiles: &mut Profiles, lang: &Catalogue) -> bool {
    let before = profiles.active().sketching;
    let config = &mut profiles.active_mut().sketching;

    ui.checkbox(
        &mut config.exact_angles_lay_rules,
        lang.t("settings.sketching.exact_angles_lay_rules"),
    )
    .on_hover_text(lang.t("settings.sketching.exact_angles_lay_rules_hint"));

    before != profiles.active().sketching
}
