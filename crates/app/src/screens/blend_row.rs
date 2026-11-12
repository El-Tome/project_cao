use cao_part::PartDocument;
use cao_part::history::BlendShape;
use cao_prefs::Command;

use crate::lang::Catalogue;
use crate::screens::blend::BlendState;
use crate::screens::variables::{NAMING, offered};
use crate::ui::formula_field::formula_field;
use crate::wording::outcome;

/// The size an edge is chamfered or rounded by, shown while the edge panel is
/// open, in the place the extrusion's values take.
pub(super) fn blend_row(
    ui: &mut egui::Ui,
    document: &PartDocument,
    blend: &mut BlendState,
    asked: &mut Vec<Command>,
    lang: &Catalogue,
) {
    let Some(shape) = blend.shape else {
        return;
    };
    let offers = offered(document.variables());
    ui.horizontal_wrapped(|ui| {
        ui.label(lang.t(match shape {
            BlendShape::Chamfer => "blend.distance",
            BlendShape::Fillet => "blend.radius",
        }));
        formula_field(
            ui,
            egui::Id::new("blend_size"),
            &mut blend.size_input,
            (70.0, "mm"),
            &offers,
            NAMING,
        );
        if let Some(reason) = blend.declined {
            ui.colored_label(
                ui.visuals().error_fg_color,
                outcome::blend_declined(lang, shape, reason),
            );
        }
        ui.separator();
        ui.add_enabled_ui(blend.is_ready(document.variables()), |ui| {
            if ui.button(lang.t("extrusion.apply")).clicked() {
                asked.push(Command::BlendApply);
            }
        });
        if ui.button(lang.t("extrusion.cancel")).clicked() {
            blend.close();
        }
    });
}
