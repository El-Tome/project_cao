use cao_prefs::Profiles;
use cao_prefs::settings::{DEFAULT_PROFILE, PROFILE_EXTENSION, Profile};

use crate::adapters::files::DiskFiles;
use crate::lang::Catalogue;
use crate::ui::text_edit::text_edit;
use crate::wording::settings::profile;

use super::SettingsEditor;

pub(super) fn section(
    ui: &mut egui::Ui,
    profiles: &mut Profiles,
    editor: &mut SettingsEditor,
    lang: &Catalogue,
) -> bool {
    let mut touched = false;

    ui.label(lang.t("settings.profiles.intro"));
    ui.add_space(6.0);

    let names: Vec<String> = profiles.names().map(str::to_string).collect();
    let active = profiles.active_name().to_string();
    ui.horizontal_wrapped(|ui| {
        for key in &names {
            if ui
                .selectable_label(*key == active, profile(lang, key))
                .clicked()
            {
                touched |= profiles.switch_to(key);
            }
        }
    });

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        text_edit(
            ui,
            &mut editor.new_profile_name,
            160.0,
            &lang.t("settings.profiles.name_hint"),
        );
        if ui.button(lang.t("settings.profiles.duplicate")).clicked() {
            let name = if editor.new_profile_name.trim().is_empty() {
                lang.t_with(
                    "settings.profiles.copy_of",
                    &[("name", &profile(lang, &active))],
                )
            } else {
                editor.new_profile_name.trim().to_string()
            };
            let created = profiles.duplicate_active(&name);
            editor.notice = Some(lang.t_with(
                "settings.profiles.created",
                &[("name", &profile(lang, &created))],
            ));
            editor.new_profile_name.clear();
            touched = true;
        }
        ui.add_enabled_ui(active != DEFAULT_PROFILE, |ui| {
            if ui.button(lang.t("settings.profiles.remove")).clicked() {
                touched |= profiles.remove(&active);
            }
        });
    });

    ui.add_space(12.0);
    ui.heading(lang.t("settings.profiles.share"));
    ui.horizontal(|ui| {
        text_edit(
            ui,
            &mut editor.profile_path,
            320.0,
            &lang.t_with(
                "settings.profiles.path_hint",
                &[("extension", PROFILE_EXTENSION)],
            ),
        );
        if ui.button(lang.t("settings.profiles.export")).clicked() {
            let path = std::path::PathBuf::from(editor.profile_path.trim());
            match profiles.active_profile().export(&DiskFiles, &path) {
                Ok(()) => {
                    editor.notice = Some(lang.t_with(
                        "settings.profiles.written",
                        &[("path", &path.display().to_string())],
                    ));
                }
                Err(err) => editor.notice = Some(crate::wording::storage::say(lang, &err)),
            }
        }
        if ui.button(lang.t("settings.profiles.import")).clicked() {
            let path = std::path::PathBuf::from(editor.profile_path.trim());
            match Profile::import(&DiskFiles, &path) {
                Ok(profile) => {
                    let name = profiles.add(profile);
                    editor.notice =
                        Some(lang.t_with("settings.profiles.imported", &[("name", &name)]));
                    touched = true;
                }
                Err(err) => editor.notice = Some(crate::wording::storage::say(lang, &err)),
            }
        }
    });

    ui.add_space(12.0);
    ui.separator();
    if ui
        .button(lang.t("settings.profiles.reset"))
        .on_hover_text(lang.t("settings.profiles.reset_hint"))
        .clicked()
    {
        profiles.reset_active();
        let named = profile(lang, &active);
        editor.notice = Some(lang.t_with("settings.profiles.reset_done", &[("name", &named)]));
        touched = true;
    }

    touched
}
