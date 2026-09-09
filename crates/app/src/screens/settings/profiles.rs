use cao_prefs::Profiles;
use cao_prefs::settings::{DEFAULT_PROFILE, PROFILE_EXTENSION, Profile};

use crate::adapters::files::DiskFiles;
use crate::ui::text_edit::text_edit;
use crate::wording::settings::profile;

use super::SettingsEditor;

pub(super) fn section(
    ui: &mut egui::Ui,
    profiles: &mut Profiles,
    editor: &mut SettingsEditor,
) -> bool {
    let mut touched = false;

    ui.label("Chaque profil est un jeu de réglages complet. Ce qui est modifié ailleurs va dans le profil actif.");
    ui.add_space(6.0);

    let names: Vec<String> = profiles.names().map(str::to_string).collect();
    let active = profiles.active_name().to_string();
    ui.horizontal_wrapped(|ui| {
        for key in &names {
            if ui.selectable_label(*key == active, profile(key)).clicked() {
                touched |= profiles.switch_to(key);
            }
        }
    });

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        text_edit(ui, &mut editor.new_profile_name, 160.0, "nom du profil");
        if ui.button("Dupliquer l'actif").clicked() {
            let name = if editor.new_profile_name.trim().is_empty() {
                format!("{} (copie)", profile(&active))
            } else {
                editor.new_profile_name.trim().to_string()
            };
            let created = profiles.duplicate_active(&name);
            editor.notice = Some(format!("Profil « {} » créé.", profile(&created)));
            editor.new_profile_name.clear();
            touched = true;
        }
        ui.add_enabled_ui(active != DEFAULT_PROFILE, |ui| {
            if ui.button("Supprimer l'actif").clicked() {
                touched |= profiles.remove(&active);
            }
        });
    });

    ui.add_space(12.0);
    ui.heading("Partager");
    ui.horizontal(|ui| {
        text_edit(
            ui,
            &mut editor.profile_path,
            320.0,
            &format!("chemin d'un fichier .{PROFILE_EXTENSION}"),
        );
        if ui.button("Exporter").clicked() {
            let path = std::path::PathBuf::from(editor.profile_path.trim());
            match profiles.active_profile().export(&DiskFiles, &path) {
                Ok(()) => editor.notice = Some(format!("Écrit dans {}", path.display())),
                Err(err) => editor.notice = Some(crate::wording::storage::say(&err)),
            }
        }
        if ui.button("Importer").clicked() {
            let path = std::path::PathBuf::from(editor.profile_path.trim());
            match Profile::import(&DiskFiles, &path) {
                Ok(profile) => {
                    let name = profiles.add(profile);
                    editor.notice = Some(format!("Profil « {name} » importé."));
                    touched = true;
                }
                Err(err) => editor.notice = Some(crate::wording::storage::say(&err)),
            }
        }
    });

    ui.add_space(12.0);
    ui.separator();
    if ui
        .button("↺ Tout remettre par défaut")
        .on_hover_text("Ne touche que le profil actif")
        .clicked()
    {
        profiles.reset_active();
        let named = profile(&active);
        editor.notice = Some(format!("Profil « {named} » remis à ses valeurs d'origine."));
        touched = true;
    }

    touched
}
