//! The section view's panel — the plane, the distance, the two tilts, each
//! side's state, « Inverser » and « Enregistrer la coupe » — and, on the
//! cutting plane, its square, its arrow and its handle.

use cao_prefs::Command;
use cao_render::{push_plane_outline, push_plane_quad};
use cao_sketch::WorkPlane;

use super::live_fields::value_field;
use super::overlays::{tint_to_color, to_screen};
use super::{tint, tint_at};
use crate::screens::SketchContext;
use crate::screens::viewport::section::Section;
use crate::screens::viewport::{ViewScale, ViewportState};
use crate::ui::floating_panel::{floating_panel, panel_row};
use crate::wording::section::{self as said, Side};

/// Draws the panel while the section is on, and carries out what was done in
/// it. Échap or the control in its title closes it, and with it the section.
/// Returns true when the section was kept in the part.
pub(crate) fn paint_section_panel(
    ui: &egui::Ui,
    state: &mut ViewportState,
    view: egui::Rect,
    context: &mut SketchContext<'_>,
) -> bool {
    let escaped = ui.input(|input| input.key_pressed(egui::Key::Escape))
        && !crate::ui::formula_field::keeps_escape(ui.ctx());
    if escaped {
        state.section = None;
    }
    let Some(section) = state.section.as_mut() else {
        return false;
    };
    let lang = context.lang;
    let title = crate::wording::command::label(lang, Command::SectionView);
    let left_at = view.min + egui::vec2(16.0, 16.0);
    let id = egui::Id::new("section_panel");
    let close = lang.t("panel.close");
    let mut keep = false;
    let floated = floating_panel(ui.ctx(), id, (&title, &close), (left_at, view), |ui| {
        keep = rows(ui, section, context);
    });
    if keep {
        section.keep_in(context.document);
    }
    if floated.closed {
        state.section = None;
    }
    keep
}

/// Draws the rows, and says whether the section was asked to be kept.
fn rows(ui: &mut egui::Ui, section: &mut Section, context: &SketchContext<'_>) -> bool {
    let (lang, body) = (context.lang, context.document.body());
    let waiting = section.is_waiting_for_a_face();
    let mut on_an_origin_plane = false;
    for plane in WorkPlane::ORIGIN_PLANES {
        let chosen = !waiting && section.plane == plane;
        on_an_origin_plane |= chosen;
        let said = crate::wording::plane::label(lang, plane.kind());
        if panel_row(ui, said, chosen, false).clicked() {
            section.choose_origin_plane(plane, body);
        }
    }
    let on_a_face = !on_an_origin_plane;
    if panel_row(ui, lang.t("section.clicked_face"), on_a_face, waiting).clicked() {
        section.wait_for_a_face();
    }
    let millimetres = context.document.scale();
    let now = section.distance_text(millimetres);
    let distance = ("section_distance", lang.t("section.distance"), "mm");
    if let Some(typed) = ui
        .horizontal(|ui| typed_row(ui, distance, now, context))
        .inner
    {
        section.type_distance(typed, millimetres);
    }
    for (id, key, tilt) in [
        (
            "section_tilt_u",
            "section.tilt_u",
            &mut section.tilt_about_u_degrees,
        ),
        (
            "section_tilt_v",
            "section.tilt_v",
            &mut section.tilt_about_v_degrees,
        ),
    ] {
        let now = crate::wording::dimension::short(*tilt);
        if let Some(typed) = ui
            .horizontal(|ui| typed_row(ui, (id, lang.t(key), "°"), now, context))
            .inner
        {
            *tilt = typed;
        }
    }
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
    ui.button(lang.t("section.save")).clicked()
}

/// A field after its label, taking a number typed or read from the formula
/// field: while it is not being typed in, it reads `now`, which follows the
/// handle for the distance. Returns what was typed.
fn typed_row(
    ui: &mut egui::Ui,
    (id, label, unit): (&str, String, &str),
    now: String,
    context: &SketchContext<'_>,
) -> Option<f64> {
    let variables = context.document.variables();
    let id = egui::Id::new(id);
    let typing = ui.memory(|memory| memory.has_focus(id));
    let mut text = match typing {
        true => ui.data(|data| data.get_temp::<String>(id.with("text"))),
        false => None,
    }
    .unwrap_or(now);
    ui.label(label);
    let field = value_field(ui, id, &mut text, "", false, variables);
    ui.label(unit);
    let typed = match field.response.response.changed() {
        true => variables.size_of(&text).ok().map(|(_, typed)| typed),
        false => None,
    };
    ui.data_mut(|data| data.insert_temp(id.with("text"), text));
    typed
}

/// The handle: a dot on the cutting plane over the middle of the part.
pub(crate) fn paint_section_handle(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
) {
    let Some(section) = state.section() else {
        return;
    };
    let handle = section.handle(context.document.body());
    let view_projection = state.camera.view_projection(rect.width() / rect.height());
    if let Some(at) = to_screen(handle, view_projection, rect) {
        let color = tint_to_color(state.theme.highlight);
        ui.painter_at(rect).circle_filled(at, 6.0, color);
    }
}

/// While the section tool is open, the cutting plane as a see-through square
/// centred on the handle, outlined, and the arrow the way it faces, both the
/// same size on screen at any zoom.
pub(super) fn push_section_plane(
    surfaces: &mut Vec<cao_render::Vertex>,
    lines: &mut Vec<cao_render::Vertex>,
    state: &ViewportState,
    scale: ViewScale,
    context: &SketchContext<'_>,
) {
    let Some(section) = state.section() else {
        return;
    };
    let cut = section.cutting_plane();
    let centre = section.handle(context.document.body());
    let (u, v) = (cut.u.as_vec3(), cut.v.as_vec3());
    let half_size = scale.world_size_of(SQUARE_PIXELS) as f32;
    let highlight = state.theme.highlight;
    push_plane_quad(
        surfaces,
        centre.as_vec3(),
        u,
        v,
        half_size,
        tint_at(highlight, 0.15),
    );
    let outline = tint(highlight);
    push_plane_outline(lines, centre.as_vec3(), u, v, half_size, outline, 1.5);
    let facing = section.facing();
    let length = scale.world_size_of(ARROW_PIXELS);
    let tip = centre + facing * length;
    let barb = |side: glam::DVec3| tip + (side * 0.5 - facing) * length * 0.2;
    for (from, to) in [(centre, tip), (tip, barb(cut.u)), (tip, barb(-cut.u))] {
        lines.push(cao_render::Vertex::line(from.as_vec3(), outline, 2.0));
        lines.push(cao_render::Vertex::line(to.as_vec3(), outline, 2.0));
    }
}

const SQUARE_PIXELS: f64 = 80.0;
const ARROW_PIXELS: f64 = 60.0;
