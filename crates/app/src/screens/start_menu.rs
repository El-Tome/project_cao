use std::path::PathBuf;

use cao_core::RecentEntry;

pub enum StartMenuAction {
    None,
    CreateNew(String),
    Open(PathBuf),
}

pub fn show(ui: &mut egui::Ui, recents: &[RecentEntry], new_part_name: &mut String) -> StartMenuAction {
    let mut action = StartMenuAction::None;

    ui.vertical_centered(|ui| {
        ui.add_space(24.0);
        ui.heading("CAO");
        ui.label("Nouvelle pièce ou reprise d'une pièce récente");
        ui.add_space(24.0);
    });

    ui.separator();

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            ui.heading("Nouvelle pièce");
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Nom :");
                ui.text_edit_singleline(new_part_name);
            });
            ui.add_space(8.0);
            let can_create = !new_part_name.trim().is_empty();
            if ui.add_enabled(can_create, egui::Button::new("Créer")).clicked() {
                action = StartMenuAction::CreateNew(new_part_name.trim().to_string());
                new_part_name.clear();
            }
        });

        columns[1].vertical(|ui| {
            ui.heading("Pièces récentes");
            ui.add_space(8.0);
            if recents.is_empty() {
                ui.weak("Aucune pièce ouverte pour le moment.");
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for entry in recents {
                        ui.horizontal(|ui| {
                            if ui.button(&entry.name).clicked() {
                                action = StartMenuAction::Open(entry.path.clone());
                            }
                            ui.weak(entry.opened_at.format("%Y-%m-%d %H:%M").to_string());
                        });
                    }
                });
            }
        });
    });

    action
}
