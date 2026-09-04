use cao_core::ViewportConfig;
use cao_core::config::{Binding, PointerButton, TrackpadGesture, ViewportCorner};
use cao_render::camera::{CubeFace, CubeZone, GridPlane};
use cao_render::{
    AxisStyle, GridStyle, OrbitCamera, SceneFrame, SceneRenderer, ViewTransition, ViewportRect,
    adaptive_step, cube, push_axes, push_grid,
};
use glam::Vec2;

/// What the canvas is showing: the bare world axes, or a work plane with its
/// grid. Clicking a face of the orientation cube enters a plane; orbiting
/// leaves it, since the view is no longer aligned with any plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewMode {
    Free,
    Plane(GridPlane),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Drag {
    Orbit,
    Pan,
}

pub struct ViewportState {
    pub config: ViewportConfig,
    camera: OrbitCamera,
    mode: ViewMode,
    transition: Option<ViewTransition>,
    hovered_zone: Option<CubeZone>,
    drag: Option<Drag>,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            config: ViewportConfig::default(),
            camera: OrbitCamera::default(),
            mode: ViewMode::Free,
            transition: None,
            hovered_zone: None,
            drag: None,
        }
    }
}

impl ViewportState {
    pub fn mode(&self) -> ViewMode {
        self.mode
    }

    /// A face lands on its work plane and shows the grid; an edge or a corner
    /// is an oblique view, which stays in wireframe mode.
    fn snap_to_zone(&mut self, zone: CubeZone) {
        self.transition = Some(ViewTransition::to_zone(&self.camera, zone));
        self.mode = match zone.plane() {
            Some(plane) => ViewMode::Plane(plane),
            None => ViewMode::Free,
        };
    }

    fn orbit(&mut self, delta: Vec2, sensitivity: f32) {
        self.camera.orbit(delta, sensitivity);
        self.mode = ViewMode::Free;
        self.transition = None;
    }
}

pub fn show(ui: &mut egui::Ui, state: &mut ViewportState) {
    let (rect, response) =
        ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
    if rect.width() < 1.0 || rect.height() < 1.0 {
        return;
    }

    let cube_rect = cube_rect(rect, &state.config);

    advance_transition(ui, state);
    handle_input(ui, state, &response, cube_rect);

    let scale = ViewScale::of(
        &state.camera,
        rect,
        ui.ctx().pixels_per_point(),
        &state.config,
    );
    let frame = build_frame(state, rect, cube_rect, scale);
    paint_face_labels(ui, state, cube_rect);
    if state.config.ruler_visible {
        paint_ruler(ui, state, rect, scale);
    }

    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        rect,
        ViewportCallback { frame },
    ));
}

/// How much of the world one pixel covers right now, and the grid step that
/// follows from it. Shared by the grid and the scale bar so they can never
/// disagree.
#[derive(Clone, Copy)]
struct ViewScale {
    units_per_pixel: f32,
    step: f32,
    height_px: f32,
}

impl ViewScale {
    fn of(
        camera: &OrbitCamera,
        rect: egui::Rect,
        pixels_per_point: f32,
        config: &ViewportConfig,
    ) -> Self {
        let height_px = rect.height() * pixels_per_point;
        let units_per_pixel = camera.world_units_per_pixel(height_px);
        Self {
            units_per_pixel,
            step: adaptive_step(units_per_pixel, config.grid_pixel_spacing),
            height_px,
        }
    }

    /// Length of one grid step on screen, in logical points.
    fn step_in_points(&self, pixels_per_point: f32) -> f32 {
        self.step / self.units_per_pixel / pixels_per_point
    }
}

fn corner_origin(
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

fn cube_rect(viewport: egui::Rect, config: &ViewportConfig) -> egui::Rect {
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

fn handle_input(
    ui: &egui::Ui,
    state: &mut ViewportState,
    response: &egui::Response,
    cube_rect: egui::Rect,
) {
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
        return;
    }

    let (delta, wheel, scroll, pinch, drag) = ui.input(|input| {
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
        let (wheel, scroll) = split_scroll(input);
        (
            Vec2::new(input.pointer.delta().x, input.pointer.delta().y),
            wheel,
            scroll,
            input.zoom_delta(),
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
        return;
    }

    // A mouse wheel zooms, as in every CAD package. A trackpad's two-finger
    // scroll is a different gesture in the same event stream, so it gets its
    // own mapping.
    if wheel != 0.0 {
        state.camera.zoom(wheel, state.config.zoom_sensitivity);
    }

    if scroll != Vec2::ZERO {
        let trackpad = state.config.trackpad;
        let shift = ui.input(|input| input.modifiers.shift);
        let gesture = if shift {
            trackpad.shift_scroll
        } else {
            trackpad.scroll
        };
        let scroll = scroll * trackpad.scroll_sensitivity;

        match gesture {
            TrackpadGesture::Pan => state.camera.pan(scroll, height_px),
            TrackpadGesture::Orbit => {
                state.orbit(-scroll, state.config.orbit_sensitivity);
            }
            TrackpadGesture::Zoom => {
                state.camera.zoom(scroll.y, state.config.zoom_sensitivity);
            }
            TrackpadGesture::Ignore => {}
        }
    }

    if state.config.trackpad.pinch_zooms && pinch != 1.0 {
        state.camera.zoom_by_factor(pinch);
    }
}

/// Splits scroll events into the mouse wheel (reported in lines or pages) and
/// a trackpad's two-finger scroll (reported in points). egui merges both into
/// `smooth_scroll_delta`, which would make them indistinguishable.
fn split_scroll(input: &egui::InputState) -> (f32, Vec2) {
    let mut wheel = 0.0;
    let mut scroll = Vec2::ZERO;

    for event in &input.events {
        let egui::Event::MouseWheel {
            unit,
            delta,
            modifiers,
            ..
        } = event
        else {
            continue;
        };
        // egui turns a scroll with the zoom modifier into a zoom gesture; it
        // must not also count as a scroll here.
        if modifiers.command {
            continue;
        }
        match unit {
            egui::MouseWheelUnit::Point => scroll += Vec2::new(delta.x, delta.y),
            egui::MouseWheelUnit::Line | egui::MouseWheelUnit::Page => wheel += delta.y,
        }
    }

    (wheel, scroll)
}

fn to_egui_button(button: PointerButton) -> egui::PointerButton {
    match button {
        PointerButton::Primary => egui::PointerButton::Primary,
        PointerButton::Middle => egui::PointerButton::Middle,
        PointerButton::Secondary => egui::PointerButton::Secondary,
    }
}

/// Position inside `rect` as normalized device coordinates: -1..1 with y up.
fn to_ndc(position: egui::Pos2, rect: egui::Rect) -> Vec2 {
    Vec2::new(
        (position.x - rect.center().x) / (rect.width() * 0.5),
        (rect.center().y - position.y) / (rect.height() * 0.5),
    )
}

fn rect_height_px(ui: &egui::Ui, rect: egui::Rect) -> f32 {
    rect.height() * ui.ctx().pixels_per_point()
}

fn build_frame(
    state: &ViewportState,
    rect: egui::Rect,
    cube_rect: egui::Rect,
    scale: ViewScale,
) -> SceneFrame {
    let camera = &state.camera;
    let pixels_per_point = scale.height_px / rect.height();

    let mut lines = Vec::new();

    if let ViewMode::Plane(plane) = state.mode {
        let half_extent = scale.units_per_pixel * scale.height_px * 1.5;
        let (_, _, normal) = plane.basis();
        let center = camera.target() - normal * camera.target().dot(normal);
        push_grid(
            &mut lines,
            plane,
            center,
            scale.step,
            half_extent,
            &GridStyle::default(),
        );
    }

    push_axes(&mut lines, camera.distance() * 50.0, &AxisStyle::default());

    let mut cube_triangles = Vec::new();
    let mut cube_edges = Vec::new();
    cube::push_faces(&mut cube_triangles, state.hovered_zone);
    cube::push_edges(&mut cube_edges, camera.forward(), 1.5);

    SceneFrame {
        scene_view_projection: camera.view_projection(rect.width() / rect.height()),
        scene_viewport: to_physical(rect, pixels_per_point),
        scene_lines: lines,
        cube_view_projection: cube::view_projection(camera.rotation()),
        cube_triangles,
        cube_edges,
        cube_viewport: to_physical(cube_rect, pixels_per_point),
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
/// atlas, and egui already has one.
fn paint_face_labels(ui: &egui::Ui, state: &ViewportState, cube_rect: egui::Rect) {
    let painter = ui.painter_at(cube_rect);
    let view_projection = cube::view_projection(state.camera.rotation());
    let forward = state.camera.forward();
    let font = egui::FontId::proportional((state.config.cube_size * 0.11).max(8.0));

    for face in CubeFace::ALL {
        if !cube::is_visible(face, forward) {
            continue;
        }
        let clip = view_projection * cube::face_center(face).extend(1.0);
        let position = egui::pos2(
            cube_rect.center().x + clip.x / clip.w * cube_rect.width() * 0.5,
            cube_rect.center().y - clip.y / clip.w * cube_rect.height() * 0.5,
        );
        let color = if state.hovered_zone == Some(CubeZone::Face(face)) {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_gray(60)
        };
        painter.text(
            position,
            egui::Align2::CENTER_CENTER,
            face_label(face),
            font.clone(),
            color,
        );
    }
}

fn face_label(face: CubeFace) -> &'static str {
    match face {
        CubeFace::PlusX => "DROITE",
        CubeFace::MinusX => "GAUCHE",
        CubeFace::PlusY => "ARRIÈRE",
        CubeFace::MinusY => "FACE",
        CubeFace::PlusZ => "DESSUS",
        CubeFace::MinusZ => "DESSOUS",
    }
}

/// A scale bar: one grid step long, labelled with the length it represents.
/// It answers "how big is a square, and how fast am I zooming" at a glance.
fn paint_ruler(ui: &egui::Ui, state: &ViewportState, viewport: egui::Rect, scale: ViewScale) {
    let config = &state.config;
    let length = scale.step_in_points(ui.ctx().pixels_per_point());
    let label = config.unit.format(scale.step);

    let tick = 5.0;
    let text_height = 16.0;
    let size = egui::vec2(length, tick + text_height);
    let origin = corner_origin(viewport, config.ruler_corner, size, config.cube_margin);

    let painter = ui.painter();
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
        if let Some(renderer) = resources.get_mut::<SceneRenderer>() {
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
        if let Some(renderer) = resources.get::<SceneRenderer>() {
            renderer.paint(render_pass);
        }
    }
}
