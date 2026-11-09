//! Everything egui draws on top of the scene: the labels on the orientation
//! cube, the marks of the rules a drawing carries, the selection band, and the
//! scale bar.
//!
//! These paint straight into the frame rather than handing back vertices: they
//! are the part of the viewport egui owns, not the part the GPU does.

use cao_prefs::theme::{Rgba, Theme};
use cao_render::camera::CubeZone;
use cao_render::cube;
use cao_sketch::{Constraint, Going, Selection, Sketch};
use glam::{DVec2, DVec3};

use crate::screens::viewport::cube_labels;

/// How far apart two marks are set when they would otherwise land on top of each other, in points.
const MARK_SPACING: f32 = 16.0;

/// How far from its centre a mark answers a click, in points: the glyph's own
/// size, short of the next mark set beside it.
const MARK_REACH: f32 = 7.0;
use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::viewport::{ViewScale, ViewportState, corner_origin};
use crate::ui::theme::tokens;
use crate::wording::constraints;

/// Where a world point lands on screen, or `None` when it is behind the camera.
pub(crate) fn to_screen(
    point: DVec3,
    view_projection: glam::Mat4,
    rect: egui::Rect,
) -> Option<egui::Pos2> {
    let clip = view_projection * point.as_vec3().extend(1.0);
    if clip.w <= 1e-6 {
        return None;
    }
    let ndc = clip.truncate() / clip.w;
    Some(egui::pos2(
        rect.center().x + ndc.x * rect.width() * 0.5,
        rect.center().y - ndc.y * rect.height() * 0.5,
    ))
}

#[cfg(feature = "test-support")]
impl ViewportState {
    /// Where a world point lands on screen in the last frame drawn: nothing
    /// before the first frame, or for a point behind the camera.
    pub fn where_it_shows(&self, point: DVec3) -> Option<egui::Pos2> {
        let canvas = self.canvas?;
        let view_projection = self.camera.view_projection(canvas.aspect_ratio());
        to_screen(point, view_projection, canvas)
    }
}

/// The cube's labels are drawn by egui rather than the GPU: text needs a font
/// atlas, and egui already has one. Laying each on the face it names is
/// [`super::cube_labels`]'s job.
pub(crate) fn paint_face_labels(
    ui: &egui::Ui,
    state: &ViewportState,
    cube_rect: egui::Rect,
    lang: &Catalogue,
) {
    let painter = ui.painter_at(cube_rect);
    let view_projection = cube::view_projection(state.camera.rotation());
    let forward = state.camera.forward();
    let preferred = (state.config.cube_size * 0.11).max(cube_labels::MINIMUM_READABLE_FONT);

    for face in cao_render::CubeFace::ALL {
        if !cube::is_visible(face, forward) {
            continue;
        }
        let text = crate::wording::cube::face(lang, face);
        let ink = if state.hovered_zone == Some(CubeZone::Face(face)) {
            tokens::TEXT_TITLE
        } else {
            tokens::TEXT_ON_ACCENT
        };
        let name = cube_labels::face_label(
            &painter,
            view_projection,
            cube_rect,
            face,
            &text,
            preferred,
            ink,
        );
        if let Some(name) = name {
            painter.add(name);
        }
    }
}

/// The marks of the rules, written beside what they hold.
///
/// Text rather than drawn symbols: a rule has no size and no direction of its
/// own, so there is nothing to draw it *at* — only something to write next to.
pub(crate) fn paint_rule_marks(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
    going: Option<&Going>,
) {
    let Some(index) = context.editor.active_sketch() else {
        return;
    };
    // Mid-drag the marks are read off the settled preview, like the values of
    // the dimensions: read from the recorded drawing they would stay behind
    // and only catch up when the button came up.
    let Some(sketch) = context
        .editor
        .drag_preview()
        .or_else(|| context.document.sketches().get(index))
    else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));
    let painter = ui.painter_at(rect);

    for (constraint, position) in placed_marks(sketch, view_projection, rect) {
        let rule = Selection::Rule(constraint);
        let held = context.editor.is_selected(rule) || context.editor.hovered == Some(rule);
        painter.text(
            position,
            egui::Align2::CENTER_CENTER,
            constraints::mark(constraint),
            egui::FontId::proportional(13.0),
            mark_shade(&state.theme, going, &constraint, held),
        );
    }
}

/// Where each rule's mark is written on screen, one entry per mark.
///
/// Several rules can hold the same corner — a middle and a right angle, say —
/// so marks that would land on each other are set side by side. The painter
/// and the pick both read it, so a mark is taken where it is seen.
pub(crate) fn placed_marks(
    sketch: &Sketch,
    view_projection: glam::Mat4,
    rect: egui::Rect,
) -> Vec<(Constraint, egui::Pos2)> {
    let mut placed: Vec<(Constraint, egui::Pos2)> = Vec::new();
    for constraint in sketch.constraints() {
        for at in sketch.rule_marks(*constraint) {
            let Some(position) = to_screen(sketch.plane.to_world(at), view_projection, rect) else {
                continue;
            };
            let mut position = position + egui::vec2(9.0, -9.0);
            while placed
                .iter()
                .any(|(_, held)| held.distance(position) < MARK_SPACING)
            {
                position.x += MARK_SPACING;
            }
            placed.push((*constraint, position));
        }
    }
    placed
}

/// The rule whose mark is under `pointer`: on the surface the mark is drawn
/// on, and no further, so that just beside it is still the trait.
pub(crate) fn mark_under(
    placed: &[(Constraint, egui::Pos2)],
    pointer: egui::Pos2,
) -> Option<Constraint> {
    placed
        .iter()
        .find(|(_, at)| {
            (at.x - pointer.x).abs() <= MARK_REACH && (at.y - pointer.y).abs() <= MARK_REACH
        })
        .map(|(rule, _)| *rule)
}

/// The colour a rule's mark is written in.
///
/// A rule the click is about to take away is said in the alert colour even
/// when it is the one being pointed at: of the two things to say, that is the
/// one the user cannot undo.
///
/// A lit mark takes the selection blue made opaque and taken four fifths of the
/// way to white: a glyph over a dark canvas, unlike a surface tint, has to read
/// brighter to stand out.
fn mark_shade(
    theme: &Theme,
    going: Option<&Going>,
    rule: &Constraint,
    held: bool,
) -> egui::Color32 {
    match (going.is_some_and(|going| going.rules.contains(rule)), held) {
        (true, _) => tint_to_color(theme.going),
        (false, true) => tint_to_color(
            theme
                .highlight
                .with_alpha(1.0)
                .mix(Rgba::opaque(1.0, 1.0, 1.0), 0.8),
        ),
        (false, false) => tint_to_color(theme.rule),
    }
}

/// The box being pulled across the drawing./// The box being pulled across the drawing.
///
/// Drawn on the sketch's own axes rather than the screen's: seen at an angle,
/// a box that stayed square on screen would take in a different area than the
/// one it appears to cover.
pub(crate) fn paint_band(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
) {
    let (Some((from, to)), Some(index)) = (context.editor.band(), context.editor.active_sketch())
    else {
        return;
    };
    let Some(sketch) = context.document.sketches().get(index) else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));

    let corners: Option<Vec<egui::Pos2>> =
        [from, DVec2::new(to.x, from.y), to, DVec2::new(from.x, to.y)]
            .into_iter()
            .map(|corner| to_screen(sketch.plane.to_world(corner), view_projection, rect))
            .collect();
    let Some(corners) = corners else {
        return;
    };

    let edge = tint_to_color(state.theme.highlight);
    ui.painter_at(rect).add(egui::Shape::convex_polygon(
        corners,
        edge.gamma_multiply(0.12),
        egui::Stroke::new(1.0, edge),
    ));
}

/// A colour of the theme as egui paints it.
pub(crate) fn tint_to_color(color: Rgba) -> egui::Color32 {
    let channel = |value: f32| (value.clamp(0.0, 1.0) * 255.0) as u8;
    egui::Color32::from_rgba_unmultiplied(
        channel(color.r),
        channel(color.g),
        channel(color.b),
        channel(color.a),
    )
}

/// A scale bar: one grid step long, labelled with the length it represents.
/// It answers "how big is a square, and how fast am I zooming" at a glance.
pub(crate) fn paint_ruler(
    ui: &egui::Ui,
    state: &ViewportState,
    viewport: egui::Rect,
    scale: ViewScale,
) {
    let config = &state.config;
    let length = scale.step_in_points(ui.ctx().pixels_per_point());
    let label = config.unit.format(scale.step_millimeters);

    let tick = 5.0;
    let text_height = 16.0;
    let size = egui::vec2(length, tick + text_height);
    let origin = corner_origin(viewport, config.ruler_corner, size, config.cube_margin);

    let painter = ui.painter_at(viewport);
    let color = tokens::TEXT_SECONDARY;
    let stroke = egui::Stroke::new(1.5, color);
    let baseline = origin.y + size.y;
    let (left, right) = (origin.x, origin.x + length);

    painter.line_segment(
        [egui::pos2(left, baseline), egui::pos2(right, baseline)],
        stroke,
    );
    for x in [left, right] {
        painter.line_segment(
            [egui::pos2(x, baseline), egui::pos2(x, baseline - tick)],
            stroke,
        );
    }
    painter.text(
        egui::pos2((left + right) * 0.5, baseline - tick - 2.0),
        egui::Align2::CENTER_BOTTOM,
        label,
        egui::FontId::monospace(12.0),
        color,
    );
}

#[cfg(test)]
mod tests;
