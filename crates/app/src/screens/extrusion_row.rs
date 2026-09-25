use cao_part::{ExtrusionMode, PartDocument};
use cao_prefs::Command;

use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::variables::{NAMING, offered};
use crate::ui::formula_field::formula_field;
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
    lang: &Catalogue,
) {
    if extrusion.sketch.is_none() {
        if document.sketches().is_empty() {
            return;
        }
        ui.horizontal_wrapped(|ui| {
            ui.label(lang.t("extrusion.pick_a_sketch"));
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

    let offers = offered(document.variables());
    ui.horizontal_wrapped(|ui| {
        if extrusion.is_revolving() {
            ui.label(lang.t("extrusion.angle"));
            formula_field(
                ui,
                egui::Id::new("extrusion_angle"),
                &mut extrusion.angle_input,
                (60.0, "°"),
                &offers,
                NAMING,
            );
            ui.label(lang.t("extrusion.around"));
            for axis in [cao_sketch::SketchAxis::U, cao_sketch::SketchAxis::V] {
                let held = extrusion.axis == cao_part::RevolutionAxis::Sketch(axis);
                let name = constraints::axis(lang, axis);
                if ui.selectable_label(held, name).clicked() {
                    extrusion.axis = cao_part::RevolutionAxis::Sketch(axis);
                }
            }
            if let cao_part::RevolutionAxis::Segment(segment) = extrusion.axis {
                let segment = segment.0.to_string();
                ui.selectable_label(
                    true,
                    lang.t_with("extrusion.segment_axis", &[("segment", &segment)]),
                )
                .on_hover_text(lang.t("extrusion.change_the_axis_trait"));
            } else {
                ui.weak(lang.t("extrusion.or_click_a_trait"));
            }
        } else {
            let key = match extrusion.mode {
                Some(ExtrusionMode::Cut) => "extrusion.depth",
                _ => "extrusion.height",
            };
            ui.label(lang.t(key));
            formula_field(
                ui,
                egui::Id::new("extrusion_distance"),
                &mut extrusion.distance_input,
                (70.0, "mm"),
                &offers,
                NAMING,
            );
        }
        ui.checkbox(&mut extrusion.reversed, lang.t("extrusion.reversed"))
            .on_hover_text(lang.t("extrusion.reversed_hint"));
        if let Some(wrong) = extrusion.wrong(document.variables()) {
            ui.colored_label(
                ui.visuals().error_fg_color,
                crate::wording::formula::unusable(lang, &wrong),
            );
        }

        ui.separator();
        let count = extrusion.picks.len().to_string();
        ui.weak(lang.t_with("extrusion.areas_chosen", &[("count", &count)]));
        ui.add_enabled_ui(extrusion.is_ready(document.variables()), |ui| {
            if ui.button(lang.t("extrusion.apply")).clicked() {
                asked.push(Command::ExtrusionApply);
            }
        });
        if ui.button(lang.t("extrusion.cancel")).clicked() {
            asked.push(Command::ExtrusionCancel);
        }
    });
}
