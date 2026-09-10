use cao_part::PartDocument;
use cao_prefs::Command;

use crate::screens::extrusion::ExtrusionState;
use crate::wording::constraints;

/// The values an extrusion needs, shown only while one is being set up.
///
/// These are not commands: they are numbers being typed, and a toolbar entry
/// cannot stand for a field the user is in the middle of filling in.
pub(super) fn extrusion_row(
    ui: &mut egui::Ui,
    document: &PartDocument,
    extrusion: &mut ExtrusionState,
    asked: &mut Vec<Command>,
) {
    if extrusion.sketch.is_none() {
        if document.sketches().is_empty() {
            return;
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("Extruder l'esquisse :");
            for index in 0..document.sketches().len() {
                if ui.button(format!("{}", index + 1)).clicked() {
                    extrusion.offer(index);
                }
            }
        });
        return;
    }
    if !extrusion.is_active() {
        return;
    }

    ui.horizontal_wrapped(|ui| {
        if extrusion.is_revolving() {
            ui.label("Angle :");
            ui.add(
                egui::TextEdit::singleline(&mut extrusion.angle_input)
                    .desired_width(60.0)
                    .hint_text("°"),
            );
            ui.label("Autour de :");
            for axis in [cao_sketch::SketchAxis::U, cao_sketch::SketchAxis::V] {
                let held = extrusion.axis == cao_part::RevolutionAxis::Sketch(axis);
                if ui.selectable_label(held, constraints::axis(axis)).clicked() {
                    extrusion.axis = cao_part::RevolutionAxis::Sketch(axis);
                }
            }
            if let cao_part::RevolutionAxis::Segment(segment) = extrusion.axis {
                ui.selectable_label(true, format!("trait {}", segment.0))
                    .on_hover_text("Cliquer un autre trait de l'esquisse pour en changer");
            } else {
                ui.weak("ou cliquer un trait");
            }
        } else {
            ui.label("Hauteur :");
            ui.add(
                egui::TextEdit::singleline(&mut extrusion.distance_input)
                    .desired_width(70.0)
                    .hint_text("mm"),
            );
        }
        ui.checkbox(&mut extrusion.reversed, "Sens inverse")
            .on_hover_text("Pousser la matière de l'autre côté du plan");

        ui.separator();
        ui.weak(format!("{} aire(s)", extrusion.picks.len()));
        ui.add_enabled_ui(extrusion.is_ready(), |ui| {
            if ui.button("Appliquer").clicked() {
                asked.push(Command::ExtrusionApply);
            }
        });
        if ui.button("Annuler").clicked() {
            asked.push(Command::ExtrusionCancel);
        }
    });
}
