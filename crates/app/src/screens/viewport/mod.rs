//! The canvas: camera navigation, the state a session of sketching keeps
//! between frames, and the frame loop that ties input to drawing together.
//!
//! What a click does with a tool lives in [`input`]; what ends up painted
//! lives in [`render`].

mod input;
mod render;

use cao_part::PartDocument;
use cao_prefs::ViewportConfig;
use cao_prefs::config::{Binding, TrackpadGesture, ViewportCorner};
use cao_prefs::theme::Theme;
use cao_render::camera::{CubeZone, view_angles_towards};
use cao_render::{OrbitCamera, SceneFrame, ViewTransition, adaptive_step, cube};
use cao_sketch::{SnapSettings, ToolState, WorkPlane};
use glam::DVec3;

use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, Tool};
use input::{draw_circle, draw_line_point, handle_sketch_input, pick_areas, two_click_shape};
use render::{
    build_frame, paint_band, paint_dimension_field, paint_dimension_labels, paint_face_labels,
    paint_live_input, paint_rule_marks, paint_ruler,
};

pub use input::DEFAULT_SKETCH_RADIUS;

/// How close, in pixels, the cursor has to be to take hold of something.
///
/// These are physical pixels, so a high-density screen halves them: at ten, a
/// point had to be hit within four points of the mouse, which is a good deal
/// finer than anyone aims.
pub(crate) const PICK_PIXELS: f64 = 18.0;

/// What the canvas is showing: the bare world axes, or a work plane with its
/// grid. Landing on a plane shows the grid; orbiting leaves it, since the view
/// is no longer aligned with any plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewMode {
    Free,
    Plane(WorkPlane),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Drag {
    Orbit,
    Pan,
}

pub struct ViewportState {
    pub config: ViewportConfig,
    /// Every colour the viewport draws with, copied from the profile in use.
    pub theme: Theme,
    camera: OrbitCamera,
    mode: ViewMode,
    transition: Option<ViewTransition>,
    hovered_zone: Option<CubeZone>,
    drag: Option<Drag>,
    /// Width over height of the canvas, remembered so that framing asked for
    /// from a toolbar button uses the viewport's shape, not the button's.
    aspect: f32,
}

impl Default for ViewportState {
    fn default() -> Self {
        let config = ViewportConfig::default();
        let mut camera = OrbitCamera::default();
        camera.set_distance_limits(config.min_distance, config.max_distance);
        Self {
            config,
            theme: Theme::default(),
            camera,
            mode: ViewMode::Free,
            transition: None,
            hovered_zone: None,
            drag: None,
            aspect: 1.0,
        }
    }
}

impl ViewportState {
    pub fn mode(&self) -> ViewMode {
        self.mode
    }

    /// Turns the camera to look straight at a plane and frames `radius` around
    /// `center`, which is what both starting a sketch and the re-align button
    /// do.
    pub fn look_at_plane(&mut self, plane: WorkPlane, center: DVec3, radius: f64) {
        let (yaw, pitch) = view_angles_towards(plane.normal().as_vec3());
        self.transition = Some(ViewTransition::to_angles(&self.camera, yaw, pitch));
        self.camera
            .focus_on(center.as_vec3(), radius as f32, self.aspect);
        self.mode = ViewMode::Plane(plane);
    }

    /// Turns to an oblique view and frames the whole part, which is how a
    /// freshly extruded volume is actually seen: straight down on its own
    /// sketch plane, a prism is indistinguishable from the drawing it came
    /// from.
    pub fn look_at_part(&mut self, center: DVec3, radius: f64) {
        let corner = CubeZone::corner(
            cao_render::CubeFace::PlusX,
            cao_render::CubeFace::MinusY,
            cao_render::CubeFace::PlusZ,
        );
        self.transition = Some(ViewTransition::to_zone(&self.camera, corner));
        self.camera
            .focus_on(center.as_vec3(), radius as f32, self.aspect);
        self.mode = ViewMode::Free;
    }

    /// A face lands on its work plane and shows the grid; an edge or a corner
    /// is an oblique view, which stays in wireframe mode.
    fn snap_to_zone(&mut self, zone: CubeZone) {
        self.transition = Some(ViewTransition::to_zone(&self.camera, zone));
        self.mode = match zone.face() {
            Some(face) => {
                let (u, v) = face.plane_basis();
                ViewMode::Plane(WorkPlane {
                    origin: DVec3::ZERO,
                    u: u.as_dvec3(),
                    v: v.as_dvec3(),
                })
            }
            None => ViewMode::Free,
        };
    }

    fn orbit(&mut self, delta: glam::Vec2, sensitivity: f32) {
        self.camera.orbit(delta, sensitivity);
        self.mode = ViewMode::Free;
        self.transition = None;
    }
}

/// What the viewport is allowed to read and change about the part while the
/// user draws on it.
pub struct SketchContext<'a> {
    pub document: &'a mut PartDocument,
    pub editor: &'a mut SketchEditor,
    pub extrusion: &'a mut ExtrusionState,
}

/// Returns true when the part was modified and should be saved.
pub fn show(ui: &mut egui::Ui, state: &mut ViewportState, sketch: &mut SketchContext<'_>) -> bool {
    let (rect, response) =
        ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
    if rect.width() < 1.0 || rect.height() < 1.0 {
        return false;
    }

    let cube_rect = cube_rect(rect, &state.config);
    state.aspect = rect.width() / rect.height();

    advance_transition(ui, state);
    handle_escape(ui, sketch);
    let handled_cube = handle_navigation(ui, state, &response, cube_rect);
    let scale = ViewScale::of(
        &state.camera,
        rect,
        ui.ctx().pixels_per_point(),
        &state.config,
        sketch.document.scale(),
    );
    let changed = if handled_cube {
        false
    } else if sketch.extrusion.is_active() {
        // Picking areas takes the whole canvas: no drawing tool is in hand
        // while the extrusion is being set up.
        pick_areas(state, &response, rect, scale, sketch);
        false
    } else {
        handle_sketch_input(ui, state, &response, rect, scale, sketch)
    };

    // The scene goes down first. Everything egui paints — the values of the
    // dimensions, the scale bar, the labels — is added to the same layer, in
    // order, and the scene now fills the viewport with its background: put it
    // last and it wipes all of them out.
    let frame = build_frame(state, rect, cube_rect, scale, sketch);
    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        rect,
        ViewportCallback { frame },
    ));

    paint_face_labels(ui, state, cube_rect);
    paint_band(ui, state, rect, sketch);
    paint_rule_marks(ui, state, rect, sketch);
    paint_dimension_labels(ui, state, rect, sketch);
    if state.config.ruler_visible {
        paint_ruler(ui, state, rect, scale);
    }
    let mut changed = changed | paint_dimension_field(ui, state, rect, sketch);

    let drawing = matches!(
        sketch.editor.tool_state,
        ToolState::Line { .. } | ToolState::Rectangle { .. } | ToolState::Circle { .. }
    );
    if drawing && paint_live_input(ui, sketch) {
        // Enter finishes the shape from the keyboard, without having to find
        // the canvas again with the mouse.
        if let Some(index) = sketch.editor.active_sketch() {
            let cursor = sketch
                .editor
                .aimed
                .map(|aimed| aimed.position)
                .or(sketch.editor.cursor)
                .unwrap_or_default();
            let snap = scale.world_size_of(PICK_PIXELS);
            changed |= match sketch.editor.tool {
                Tool::Line => draw_line_point(sketch, index, cursor, snap, scale.units_per_pixel),
                Tool::Circle => draw_circle(sketch, index, cursor, snap, scale.units_per_pixel),
                _ => two_click_shape(sketch, index, cursor, snap, scale.units_per_pixel),
            };
        }
    }

    changed
}

/// Escape steps back out of whatever is going on: the shape in progress, the
/// dimension being placed, and then the tool itself.
///
/// A tool that stays in hand after its work is done is a tool that draws a
/// stray line on the next click; falling back to the selection tool is the
/// habit every CAD package has taught.
fn handle_escape(ui: &egui::Ui, context: &mut SketchContext<'_>) {
    if context.editor.active_sketch().is_none()
        || !ui.input(|input| input.key_pressed(egui::Key::Escape))
    {
        return;
    }
    let editor = &mut context.editor;
    let busy = editor.editing.is_some() || editor.tool_state.is_busy();
    editor.reset_pending();
    if !busy {
        editor.tool = Tool::Select;
    }
}

/// How much of the world one pixel covers right now, and the grid step that
/// follows from it. Shared by the grid and the scale bar so they can never
/// disagree.
#[derive(Clone, Copy)]
pub(crate) struct ViewScale {
    units_per_pixel: f64,
    /// Grid step in world units, for drawing.
    step: f64,
    /// The same step in millimetres, for the label.
    step_millimeters: f64,
    height_px: f32,
    diagonal_px: f32,
    aspect: f32,
}

impl ViewScale {
    fn of(
        camera: &OrbitCamera,
        rect: egui::Rect,
        pixels_per_point: f32,
        config: &ViewportConfig,
        millimeters_per_unit: f64,
    ) -> Self {
        let height_px = rect.height() * pixels_per_point;
        let units_per_pixel = camera.world_units_per_pixel(height_px) as f64;
        let width_px = rect.width() * pixels_per_point;

        // The step is chosen in millimetres, not in world units: those are what
        // the ruler shows, so they are what must land on round values. Once the
        // first dimension has set the scale, a world unit is no longer a
        // millimetre, and picking the step in units would put the ruler out by
        // exactly that factor.
        let millimeters_per_unit = if millimeters_per_unit > 1e-9 {
            millimeters_per_unit
        } else {
            1.0
        };
        let step_millimeters = adaptive_step(
            (units_per_pixel * millimeters_per_unit) as f32,
            config.grid_pixel_spacing,
        ) as f64;

        Self {
            units_per_pixel,
            step: step_millimeters / millimeters_per_unit,
            step_millimeters,
            height_px,
            diagonal_px: (width_px * width_px + height_px * height_px).sqrt(),
            aspect: rect.width() / rect.height(),
        }
    }

    /// Length of one grid step on screen, in logical points.
    fn step_in_points(&self, pixels_per_point: f32) -> f32 {
        (self.step / self.units_per_pixel) as f32 / pixels_per_point
    }

    /// World size of something that should stay a fixed size on screen.
    pub(crate) fn world_size_of(&self, pixels: f64) -> f64 {
        self.units_per_pixel * pixels
    }

    /// How far each magnet reaches, in the units the drawing reasons in.
    ///
    /// The tolerances are chosen in physical pixels — a magnet that widened as
    /// one zoomed in would be unusable — and the drawing knows nothing of
    /// pixels, so the conversion happens here and nowhere lower. The grid is
    /// drawn every `step`; snapping to a fraction of it keeps the magnet useful
    /// without forcing everything onto the coarse lines.
    fn snapping(&self, config: &ViewportConfig) -> SnapSettings {
        SnapSettings {
            point_reach: self.world_size_of(PICK_PIXELS),
            segment_reach: self.world_size_of(config.segment_snap_pixels as f64),
            grid_step: config
                .grid_snap
                .then(|| self.step / config.grid_snap_divisions.max(1) as f64),
            grid_reach: self.world_size_of(config.grid_snap_pixels as f64),
        }
    }
}

pub(crate) fn corner_origin(
    viewport: egui::Rect,
    corner: ViewportCorner,
    size: egui::Vec2,
    margin: f32,
) -> egui::Pos2 {
    match corner {
        ViewportCorner::TopLeft => viewport.left_top() + egui::vec2(margin, margin),
        ViewportCorner::TopRight => viewport.right_top() + egui::vec2(-margin - size.x, margin),
        ViewportCorner::BottomLeft => viewport.left_bottom() + egui::vec2(margin, -margin - size.y),
        ViewportCorner::BottomRight => {
            viewport.right_bottom() + egui::vec2(-margin - size.x, -margin - size.y)
        }
    }
}

pub(crate) fn cube_rect(viewport: egui::Rect, config: &ViewportConfig) -> egui::Rect {
    let size = egui::vec2(config.cube_size, config.cube_size);
    let origin = corner_origin(viewport, config.cube_corner, size, config.cube_margin);
    egui::Rect::from_min_size(origin, size)
}

fn advance_transition(ui: &egui::Ui, state: &mut ViewportState) {
    let Some(transition) = state.transition.as_mut() else {
        return;
    };
    let dt = ui.input(|input| input.stable_dt);
    if transition.advance(&mut state.camera, dt) {
        ui.ctx().request_repaint();
    } else {
        state.transition = None;
    }
}

/// Camera navigation. Returns true when the click belonged to the orientation
/// cube, so the sketch tools do not also act on it.
fn handle_navigation(
    ui: &egui::Ui,
    state: &mut ViewportState,
    response: &egui::Response,
    cube_rect: egui::Rect,
) -> bool {
    let pointer = response.hover_pos();
    let over_cube = pointer.is_some_and(|position| cube_rect.contains(position));

    state.hovered_zone = match (state.drag, pointer) {
        (None, Some(position)) if over_cube => {
            cube::pick_zone(state.camera.rotation(), to_ndc(position, cube_rect))
        }
        _ => None,
    };

    if response.clicked()
        && let Some(zone) = state.hovered_zone
    {
        state.snap_to_zone(zone);
        return true;
    }

    let (delta, scroll, drag) = ui.input(|input| {
        let matches = |bindings: &[Binding]| {
            bindings.iter().any(|binding| {
                input.pointer.button_down(to_egui_button(binding.button))
                    && input.modifiers.shift == binding.shift
                    && input.modifiers.command == binding.ctrl
                    && input.modifiers.alt == binding.alt
            })
        };
        let navigation = state.config.navigation;
        let drag = if matches(navigation.orbit()) {
            Some(Drag::Orbit)
        } else if matches(navigation.pan()) {
            Some(Drag::Pan)
        } else {
            None
        };
        (
            glam::Vec2::new(input.pointer.delta().x, input.pointer.delta().y),
            ScrollInput::read(input),
            drag,
        )
    });

    // Only a drag that started over the canvas keeps control, but once it has
    // it survives the cursor leaving the canvas.
    state.drag = match (state.drag, drag) {
        (Some(_), None) => None,
        (Some(active), Some(_)) => Some(active),
        (None, Some(new)) if response.hovered() && !over_cube => Some(new),
        (None, _) => None,
    };

    let height_px = rect_height_px(ui, response.rect);

    match state.drag {
        Some(Drag::Orbit) => state.orbit(delta, state.config.orbit_sensitivity),
        Some(Drag::Pan) => state.camera.pan(delta, height_px),
        None => {}
    }

    if !response.hovered() {
        return over_cube;
    }

    // A mouse wheel zooms, as in every CAD package. A trackpad's two-finger
    // scroll is a different gesture arriving in the same event stream, so it
    // gets its own mapping and its own sensitivity.
    if scroll.wheel_notches != 0.0 {
        state
            .camera
            .zoom(scroll.wheel_notches, state.config.wheel_zoom_sensitivity);
    }

    if scroll.zoom_points != 0.0 {
        state
            .camera
            .zoom(scroll.zoom_points, state.config.zoom_sensitivity);
    }

    if scroll.trackpad != glam::Vec2::ZERO {
        let trackpad = state.config.trackpad;
        let shift = ui.input(|input| input.modifiers.shift);
        let gesture = if shift {
            trackpad.shift_scroll
        } else {
            trackpad.scroll
        };
        let delta = scroll.trackpad * trackpad.scroll_sensitivity;

        match gesture {
            TrackpadGesture::Pan => state.camera.pan(delta, height_px),
            TrackpadGesture::Orbit => state.orbit(-delta, state.config.orbit_sensitivity),
            TrackpadGesture::Zoom => state.camera.zoom(delta.y, state.config.zoom_sensitivity),
            TrackpadGesture::Ignore => {}
        }
    }

    if state.config.trackpad.pinch_zooms && scroll.pinch != 1.0 {
        state.camera.zoom_by_factor(scroll.pinch);
    }

    over_cube
}

/// Work planes are drawn a fixed fraction of the view across, so they stay
/// clickable however far the camera is.
pub(crate) fn plane_half_size(state: &ViewportState) -> f64 {
    state.camera.distance() as f64 * 0.3
}

fn to_egui_button(button: cao_prefs::config::PointerButton) -> egui::PointerButton {
    match button {
        cao_prefs::config::PointerButton::Primary => egui::PointerButton::Primary,
        cao_prefs::config::PointerButton::Middle => egui::PointerButton::Middle,
        cao_prefs::config::PointerButton::Secondary => egui::PointerButton::Secondary,
    }
}

/// Position inside `rect` as normalized device coordinates: -1..1 with y up.
pub(crate) fn to_ndc(position: egui::Pos2, rect: egui::Rect) -> glam::Vec2 {
    glam::Vec2::new(
        (position.x - rect.center().x) / (rect.width() * 0.5),
        (rect.center().y - position.y) / (rect.height() * 0.5),
    )
}

fn rect_height_px(ui: &egui::Ui, rect: egui::Rect) -> f32 {
    rect.height() * ui.ctx().pixels_per_point()
}

/// The three ways a scroll gesture arrives, kept apart because they mean
/// different things.
///
/// They cannot be read from `smooth_scroll_delta`/`zoom_delta`, which merge a
/// wheel, a two-finger scroll and a pinch into common values: a wheel notch is
/// one *line* while a trackpad reports *pixels*, so sharing a sensitivity makes
/// one of them crawl and the other bolt.
#[derive(Default)]
struct ScrollInput {
    /// Mouse wheel, in notches.
    wheel_notches: f32,
    /// Two-finger scroll, in points.
    trackpad: glam::Vec2,
    /// Two-finger scroll held with the zoom modifier, in points.
    zoom_points: f32,
    /// Pinch, as a scale factor (1.0 = no change).
    pinch: f32,
}

impl ScrollInput {
    fn read(input: &egui::InputState) -> Self {
        let mut scroll = Self {
            pinch: 1.0,
            ..Self::default()
        };

        for event in &input.events {
            match event {
                egui::Event::MouseWheel {
                    unit,
                    delta,
                    modifiers,
                    ..
                } => match unit {
                    egui::MouseWheelUnit::Point if modifiers.command => {
                        scroll.zoom_points += delta.y;
                    }
                    egui::MouseWheelUnit::Point => {
                        scroll.trackpad += glam::Vec2::new(delta.x, delta.y);
                    }
                    egui::MouseWheelUnit::Line | egui::MouseWheelUnit::Page => {
                        scroll.wheel_notches += delta.y;
                    }
                },
                egui::Event::Zoom(factor) => scroll.pinch *= factor,
                _ => {}
            }
        }

        if let Some(touch) = input.multi_touch() {
            scroll.pinch *= touch.zoom_delta;
        }

        scroll
    }
}

struct ViewportCallback {
    frame: SceneFrame,
}

impl egui_wgpu::CallbackTrait for ViewportCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some(renderer) = resources.get_mut::<cao_render::SceneRenderer>() {
            renderer.prepare(device, queue, &self.frame);
        }
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        if let Some(renderer) = resources.get::<cao_render::SceneRenderer>() {
            renderer.paint(render_pass);
        }
    }
}
