//! The section view's panel — the plane, the distance, each side's state and
//! « Inverser » — and its handle on the cutting plane.

use cao_prefs::Command;
use cao_sketch::WorkPlane;

use super::overlays::{tint_to_color, to_screen};
use crate::screens::SketchContext;
use crate::screens::viewport::section::{Section, to_the_middle};
use crate::screens::viewport::{ViewScale, ViewportState};
use crate::ui::floating_panel::{floating_panel, panel_row};
use crate::ui::number_field::number_field;
use crate::wording::section::{self as said, Side};

/// Draws the panel while the section is on, and carries out what was done in
/// it. Échap closes it, and with it the section.
pub(crate) fn paint_section_panel(
    ui: &egui::Ui,
    state: &mut ViewportState,
    view: egui::Rect,
    context: &SketchContext<'_>,
) {
    let escaped = ui.input(|input| input.key_pressed(egui::Key::Escape))
        && !crate::ui::formula_field::keeps_escape(ui.ctx());
    if escaped {
        state.section = None;
    }
    let Some(section) = state.section.as_mut() else {
        return;
    };
    let lang = context.lang;
    let title = crate::wording::command::label(lang, Command::SectionView);
    let left_at = view.min + egui::vec2(16.0, 16.0);
    let id = egui::Id::new("section_panel");
    floating_panel(ui.ctx(), id, &title, (left_at, view), |ui| {
        rows(ui, section, context)
    });
}

fn rows(ui: &mut egui::Ui, section: &mut Section, context: &SketchContext<'_>) {
    let (lang, body) = (context.lang, context.document.body());
    let waiting = section.is_waiting_for_a_face();
    let mut on_an_origin_plane = false;
    for plane in WorkPlane::ORIGIN_PLANES {
        let chosen = !waiting && section.plane == plane;
        on_an_origin_plane |= chosen;
        let said = crate::wording::plane::label(lang, plane.kind());
        if panel_row(ui, said, chosen, false).clicked() {
            section.choose_plane(plane);
            section.offset = to_the_middle(plane, body);
        }
    }
    let on_a_face = !on_an_origin_plane;
    if panel_row(ui, lang.t("section.clicked_face"), on_a_face, waiting).clicked() {
        section.wait_for_a_face();
    }
    ui.horizontal(|ui| {
        ui.label(lang.t("section.distance"));
        number_field(ui, &mut section.offset, 0.1);
    });
    if panel_row(
        ui,
        said::side(lang, Side::Behind, section.behind),
        false,
        false,
    )
    .clicked()
    {
        section.next_behind();
    }
    if panel_row(
        ui,
        said::side(lang, Side::InFront, section.in_front),
        false,
        false,
    )
    .clicked()
    {
        section.next_in_front();
    }
    if ui.button(lang.t("section.invert")).clicked() {
        section.invert();
    }
}

/// The handle: a dot on the cutting plane over the middle of the part, and a
/// stroke along the normal it is pulled along.
pub(crate) fn paint_section_handle(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    scale: ViewScale,
    context: &SketchContext<'_>,
) {
    let Some(section) = state.section() else {
        return;
    };
    let handle = section.handle(context.document.body());
    let reach = section.plane.normal() * scale.world_size_of(HANDLE_PIXELS);
    let view_projection = state.camera.view_projection(rect.width() / rect.height());
    let color = tint_to_color(state.theme.highlight);
    let (Some(at), Some(tip)) = (
        to_screen(handle, view_projection, rect),
        to_screen(handle + reach, view_projection, rect),
    ) else {
        return;
    };
    let painter = ui.painter_at(rect);
    painter.line_segment([at, tip], egui::Stroke::new(2.0, color));
    painter.circle_filled(at, 6.0, color);
}

const HANDLE_PIXELS: f64 = 60.0;
