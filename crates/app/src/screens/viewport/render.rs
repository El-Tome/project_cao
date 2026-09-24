//! What ends up painted: the wgpu scene (grid, axes, the sketch, the solid),
//! and everything egui draws on top of it — dimensions, rule marks, the band,
//! the live fields, the orientation cube's labels.
//!
//! What a click decides lives in [`super::input`].

use cao_prefs::theme::{Background, Theme};
use cao_render::{AxisStyle, BackgroundShape, SceneFrame, ViewportRect, cube, push_axes, srgb};
use cao_sketch::{DimensionTarget, Going};
use glam::DVec2;

mod arc;
mod circle;
mod curves;
mod dimensions;
mod drawing;
mod ellipse;
mod emphasis;
mod extrusion;
mod grid;
mod live_fields;
mod marks;
mod overlays;
mod planes;
mod preview;
mod reading;
mod symmetric_line;
mod trim;

pub(crate) use dimensions::{paint_dimension_field, paint_dimension_labels};
use drawing::{Shown, push_sketch, what_would_be_laid};
use extrusion::push_chosen_areas;
pub(crate) use live_fields::paint_live_input;
pub(crate) use overlays::{paint_band, paint_face_labels, paint_rule_marks, paint_ruler};
use planes::{push_choosable_planes, push_hovered_face};
use preview::pending_annotation;
use reading::push_measure;
pub(crate) use reading::{Measuring, paint_measure, what_is_measured};
pub(crate) use trim::what_would_go;

use super::matter;
use super::{ViewMode, ViewScale, ViewportState};
use crate::screens::SketchContext;

/// A colour from the theme, turned into the space the shader blends in.
fn tint(color: cao_prefs::theme::Rgba) -> [f32; 4] {
    srgb(color.r, color.g, color.b, color.a)
}

/// The same, with the opacity replaced.
fn tint_at(color: cao_prefs::theme::Rgba, alpha: f32) -> [f32; 4] {
    tint(color.with_alpha(alpha))
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
    going: Option<&Going>,
    measuring: Option<&Measuring>,
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
            going: active.then_some(going).flatten(),
            active,
        };
        push_sketch(&mut lines, &mut surfaces, &shown, theme, scale, context);
    }

    if let Some(index) = context.editor.active_sketch()
        && let Some(sketch) = context.document.sketches().get(index)
    {
        push_measure(
            &mut lines,
            &mut surfaces,
            sketch,
            context,
            theme,
            scale,
            measuring,
        );
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

/// Lights the faces of the part a refusal named, while they are lit, so that
/// the matter a step made is seen where it stands.
pub(crate) fn push_blinking_matter(
    surfaces: &mut Vec<cao_render::Vertex>,
    state: &ViewportState,
    context: &SketchContext<'_>,
    now: f64,
) {
    let Some((faces, true)) = state.faces_blinking(now) else {
        return;
    };
    let fill = tint(state.theme.refused);
    for face in faces {
        push_hovered_face(surfaces, fill, context, *face);
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

#[cfg(test)]
mod tests;
