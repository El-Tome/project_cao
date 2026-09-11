use std::path::PathBuf;

use cao_prefs::RecentEntry;

use crate::adapters::clock;
use crate::lang::Catalogue;

pub enum StartMenuAction {
    None,
    CreateNew(String),
    Open(PathBuf),
}

pub fn show(
    ui: &mut egui::Ui,
    recents: &[RecentEntry],
    new_part_name: &mut String,
    lang: &Catalogue,
) -> StartMenuAction {
    let mut action = StartMenuAction::None;

    ui.vertical_centered(|ui| {
        ui.add_space(24.0);
        ui.heading(lang.t("start_menu.title"));
        ui.label(lang.t("start_menu.subtitle"));
        ui.add_space(24.0);
    });

    ui.separator();

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            ui.heading(lang.t("start_menu.new_part"));
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(lang.t("start_menu.name"));
                ui.text_edit_singleline(new_part_name);
            });
            ui.add_space(8.0);
            let can_create = !new_part_name.trim().is_empty();
            if ui
                .add_enabled(can_create, egui::Button::new(lang.t("start_menu.create")))
                .clicked()
            {
                action = StartMenuAction::CreateNew(new_part_name.trim().to_string());
                new_part_name.clear();
            }
        });

        columns[1].vertical(|ui| {
            ui.heading(lang.t("start_menu.recents"));
            ui.add_space(8.0);
            if recents.is_empty() {
                ui.weak(lang.t("start_menu.no_recents"));
            } else {
                let zone = clock::reader_zone();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for entry in recents {
                        ui.horizontal(|ui| {
                            if ui.button(&entry.name).clicked() {
                                action = StartMenuAction::Open(entry.path.clone());
                            }
                            ui.weak(lang.t_moment(
                                "start_menu.recent_date",
                                entry.opened_at,
                                &zone,
                            ));
                        });
                    }
                });
            }
        });
    });

    action
}
