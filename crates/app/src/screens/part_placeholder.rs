use std::path::Path;

use cao_core::PartDocument;

/// Shown right after a part is created/opened, before any real editor
/// (sketch mode, etc.) exists. Returns true when the user asks to go back
/// to the start menu.
pub fn show(ui: &mut egui::Ui, doc: &PartDocument, path: &Path) -> bool {
    let mut back = false;

    ui.add_space(24.0);
    ui.heading(&doc.name);
    ui.label(format!("Créée le {}", doc.created_at.format("%Y-%m-%d %H:%M")));
    ui.weak(path.display().to_string());
    ui.add_space(16.0);
    ui.weak("L'éditeur (croquis / extrusion / assemblage) n'est pas encore implémenté.");
    ui.add_space(16.0);
    if ui.button("Retour au menu").clicked() {
        back = true;
    }

    back
}
