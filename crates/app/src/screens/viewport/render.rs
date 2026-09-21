//! What ends up painted: the wgpu scene (grid, axes, the sketch, the solid),
//! and everything egui draws on top of it — dimensions, rule marks, the band,
//! the live fields, the orientation cube's labels.
//!
//! What a click decides lives in [`super::input`].

use cao_prefs::theme::{Background, Rgba, Theme};
use cao_render::camera::CubeZone;
use cao_render::{AxisStyle, BackgroundShape, SceneFrame, ViewportRect, cube, push_axes, srgb};
use cao_sketch::{DimensionTarget, Selection};
use glam::{DVec2, DVec3};

use crate::lang::Catalogue;
use crate::wording::constraints;

mod arc;
mod circle;
mod curves;
mod dimensions;
mod drawing;
mod emphasis;
mod extrusion;
mod grid;
mod live_fields;
mod marks;
mod planes;
mod preview;
mod symmetric_line;

pub(crate) use dimensions::{paint_dimension_field, paint_dimension_labels};
use drawing::{Shown, push_sketch, what_would_be_laid};
use extrusion::push_chosen_areas;
pub(crate) use live_fields::paint_live_input;
use planes::push_choosable_planes;
use preview::pending_annotation;

use super::cube_labels;
use super::matter;
use super::{SketchContext, ViewMode, ViewScale, ViewportState, corner_origin};

/// A colour from the theme, turned into the space the shader blends in.
fn tint(color: cao_prefs::theme::Rgba) -> [f32; 4] {
    srgb(color.r, color.g, color.b, color.a)
}

/// The same, with the opacity replaced.
fn tint_at(color: cao_prefs::theme::Rgba, alpha: f32) -> [f32; 4] {
    tint(color.with_alpha(alpha))
}

/// Where a world point lands on screen, or `None` when it is behind the camera.
fn to_screen(point: DVec3, view_projection: glam::Mat4, rect: egui::Rect) -> Option<egui::Pos2> {
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

fn axis_style(theme: &Theme) -> AxisStyle {
    AxisStyle {
        x: tint(theme.axis_x),
        y: tint(theme.axis_y),
        z: tint(theme.axis_z),
        width: theme.axis_width,
    }
}

pub(crate) fn build_frame(
    state: &ViewportState,
    rect: egui::Rect,
    cube_rect: egui::Rect,
    scale: ViewScale,
    context: &SketchContext<'_>,
) -> SceneFrame {
    let camera = &state.camera;
    let pixels_per_point = scale.height_px / rect.height();

    let theme = &state.theme;
    let mut lines = Vec::new();
    let mut world_lines = Vec::new();
    let mut surfaces = Vec::new();

    let mut background = Vec::new();
    let (shape, sample) = background_shape(&theme.background);
    cao_render::push_background(&mut background, shape, sample);

    // The grid and the drawing's own axes are only drawn once the view has
    // actually landed on the plane. Mid-animation the view is oblique, and a
    // grid of finite size seen at an angle reads as a disc floating in the
    // middle of the screen.
    let landed = match (state.mode, &state.transition) {
        (ViewMode::Plane(plane), None) => Some(plane),
        _ => None,
    };
    if let Some(plane) = landed {
        grid::push(
            &mut world_lines,
            plane,
            camera.target(),
            camera.distance(),
            scale,
            theme,
        );
    }

    // A drawing has one origin on screen, its own. The world's three are the
    // part's reference and belong to the view of the part — and to the swing
    // onto a plane, which would otherwise show nothing at all until it lands.
    if landed.is_none() {
        let facing = match state.mode {
            ViewMode::Plane(plane) => Some(plane.normal().as_vec3()),
            ViewMode::Free => None,
        };
        push_axes(
            &mut world_lines,
            camera.distance() * 50.0,
            &axis_style(theme),
            facing,
        );
    }

    if context.editor.is_choosing_plane() {
        push_choosable_planes(&mut surfaces, &mut lines, state, context);
    }

    for (index, sketch) in context.document.sketches().iter().enumerate() {
        let active = context.editor.active_sketch() == Some(index);
        // What a tool is offering to lay stands in for the recorded drawing the
        // same way a drag does, and what is new in it is drawn as a promise.
        let offered = active
            .then(|| what_would_be_laid(sketch, context, scale))
            .flatten();
        // Mid-drag, the settled preview stands in for the recorded drawing.
        let shown = match (offered.as_ref(), active, context.editor.drag_preview()) {
            (Some(preview), ..) => &preview.sketch,
            (None, true, Some(preview)) => preview,
            _ => sketch,
        };
        let shown = Shown {
            sketch: shown,
            laid: offered.as_ref().map_or(&[][..], |preview| &preview.laid),
            active,
        };
        push_sketch(&mut lines, &mut surfaces, &shown, theme, scale, context);
    }

    push_chosen_areas(&mut surfaces, &mut lines, theme, context);

    let mut solids = Vec::new();
    let shown = matter::shown_body(context.editor, context.document.body(), camera.eye());
    cao_render::push_solid(
        &mut solids,
        &shown
            .triangles()
            .iter()
            .map(|corners| corners.map(|corner| corner.as_vec3()))
            .collect::<Vec<_>>(),
        tint(theme.solid),
        camera.forward(),
    );

    let mut cube_triangles = Vec::new();
    let mut cube_edges = Vec::new();
    cube::push_faces(&mut cube_triangles, state.hovered_zone);
    cube::push_edges(&mut cube_edges, camera.forward(), 1.5);

    SceneFrame {
        scene_view_projection: camera.view_projection(scale.aspect),
        scene_viewport: to_physical(rect, pixels_per_point),
        scene_background: background,
        scene_world_lines: world_lines,
        scene_surfaces: surfaces,
        scene_solids: solids,
        scene_lines: lines,
        cube_view_projection: cube::view_projection(camera.rotation()),
        cube_triangles,
        cube_edges,
        cube_viewport: to_physical(cube_rect, pixels_per_point),
    }
}

/// Turns the background the user described into what the renderer draws.
fn background_shape(background: &Background) -> (BackgroundShape, impl Fn(f32) -> [f32; 4] + '_) {
    let shape = match background {
        Background::Solid(_) => BackgroundShape::Flat,
        Background::Linear { angle_degrees, .. } => BackgroundShape::Linear {
            angle_degrees: *angle_degrees,
        },
        Background::Radial { center, radius, .. } => BackgroundShape::Radial {
            center: *center,
            radius: *radius,
        },
    };
    (shape, move |t: f32| tint(background.sample(t)))
}

/// How far an annotation has been dragged so far, before anything is recorded.
///
/// Nothing goes into the history until the button is let go, so without this
/// the annotation would sit still through the whole gesture and only jump at
/// the end — the drag would look like it had done nothing.
fn live_offset(context: &SketchContext<'_>, target: DimensionTarget) -> DVec2 {
    let editor = &context.editor;
    match (
        editor.dragged_dimension(),
        editor.drag_origin(),
        editor.drag_position(),
    ) {
        (Some(dragged), Some(origin), Some(position)) if dragged == target => position - origin,
        _ => DVec2::ZERO,
    }
}

fn to_physical(rect: egui::Rect, pixels_per_point: f32) -> ViewportRect {
    ViewportRect {
        x: rect.left() * pixels_per_point,
        y: rect.top() * pixels_per_point,
        width: rect.width() * pixels_per_point,
        height: rect.height() * pixels_per_point,
    }
}

/// The cube's labels are drawn by egui rather than the GPU: text needs a font
/// atlas, and egui already has one. Sizing them to the face they sit on,
/// rather than to the cube, is [`super::cube_labels`]'s job.
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
        let label =
            cube_labels::face_label(&painter, view_projection, cube_rect, face, &text, preferred);
        let Some((position, font_size)) = label else {
            continue;
        };
        let color = if state.hovered_zone == Some(CubeZone::Face(face)) {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_gray(60)
        };
        painter.text(
            position,
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(font_size),
            color,
        );
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

    // Several rules can hold the same corner — a middle and a right angle, say
    // — so marks that would land on each other are set side by side.
    let mut taken: Vec<egui::Pos2> = Vec::new();
    for constraint in sketch.constraints() {
        let rule = Selection::Rule(*constraint);
        let held = context.editor.is_selected(rule) || context.editor.hovered == Some(rule);
        for at in sketch.rule_marks(*constraint) {
            let Some(position) = to_screen(sketch.plane.to_world(at), view_projection, rect) else {
                continue;
            };
            let mut position = position + egui::vec2(9.0, -9.0);
            while taken
                .iter()
                .any(|held| held.distance(position) < MARK_SPACING)
            {
                position.x += MARK_SPACING;
            }
            taken.push(position);
            painter.text(
                position,
                egui::Align2::CENTER_CENTER,
                constraints::mark(*constraint),
                egui::FontId::proportional(13.0),
                match held {
                    true => tint_to_color(state.theme.highlight),
                    false => tint_to_color(state.theme.rule),
                },
            );
        }
    }
}

/// How far apart two marks are set when they would otherwise land on top of each other, in points.
const MARK_SPACING: f32 = 16.0;

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
fn tint_to_color(color: Rgba) -> egui::Color32 {
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
    let color = egui::Color32::from_gray(200);
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
        egui::FontId::proportional(12.0),
        color,
    );
}
