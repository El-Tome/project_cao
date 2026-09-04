use cao_core::ViewportConfig;
use cao_core::config::{Binding, PointerButton, ViewportCorner};
use cao_render::camera::{CubeFace, GridPlane};
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
    hovered_face: Option<CubeFace>,
    drag: Option<Drag>,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            config: ViewportConfig::default(),
            camera: OrbitCamera::default(),
            mode: ViewMode::Free,
            transition: None,
            hovered_face: None,
            drag: None,
        }
    }
}

impl ViewportState {
    pub fn mode(&self) -> ViewMode {
        self.mode
    }

    fn snap_to_face(&mut self, face: CubeFace) {
        self.transition = Some(ViewTransition::to_face(&self.camera, face));
        self.mode = ViewMode::Plane(face.plane());
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

    let frame = build_frame(ui, state, rect, cube_rect);
    paint_face_labels(ui, state, cube_rect);

    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        rect,
        ViewportCallback { frame },
    ));
}

fn cube_rect(viewport: egui::Rect, config: &ViewportConfig) -> egui::Rect {
    let size = egui::vec2(config.cube_size, config.cube_size);
    let margin = config.cube_margin;
    let corner = match config.cube_corner {
        ViewportCorner::TopLeft => viewport.left_top() + egui::vec2(margin, margin),
        ViewportCorner::TopRight => viewport.right_top() + egui::vec2(-margin - size.x, margin),
        ViewportCorner::BottomLeft => viewport.left_bottom() + egui::vec2(margin, -margin - size.y),
        ViewportCorner::BottomRight => {
            viewport.right_bottom() + egui::vec2(-margin - size.x, -margin - size.y)
        }
    };
    egui::Rect::from_min_size(corner, size)
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

    state.hovered_face = match (state.drag, pointer) {
        (None, Some(position)) if over_cube => {
            cube::pick_face(state.camera.rotation(), to_ndc(position, cube_rect))
        }
        _ => None,
    };

    if response.clicked()
        && let Some(face) = state.hovered_face
    {
        state.snap_to_face(face);
        return;
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
            Vec2::new(input.pointer.delta().x, input.pointer.delta().y),
            input.smooth_scroll_delta.y,
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

    match state.drag {
        Some(Drag::Orbit) => {
            state.camera.orbit(delta, state.config.orbit_sensitivity);
            state.mode = ViewMode::Free;
            state.transition = None;
        }
        Some(Drag::Pan) => state.camera.pan(delta, rect_height_px(ui, response.rect)),
        None => {}
    }

    if response.hovered() && scroll != 0.0 {
        state.camera.zoom(scroll, state.config.zoom_sensitivity);
    }
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
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    cube_rect: egui::Rect,
) -> SceneFrame {
    let camera = &state.camera;
    let pixels_per_point = ui.ctx().pixels_per_point();
    let height_px = rect.height() * pixels_per_point;

    let mut lines = Vec::new();

    if let ViewMode::Plane(plane) = state.mode {
        let step = adaptive_step(
            camera.world_units_per_pixel(height_px),
            state.config.grid_pixel_spacing,
        );
        let visible_world_height = camera.world_units_per_pixel(height_px) * height_px;
        let half_extent = visible_world_height * 1.5;
        let (_, _, normal) = plane.basis();
        let center = camera.target() - normal * camera.target().dot(normal);
        push_grid(
            &mut lines,
            plane,
            center,
            step,
            half_extent,
            &GridStyle::default(),
        );
    }

    push_axes(&mut lines, camera.distance() * 50.0, &AxisStyle::default());

    let mut cube_triangles = Vec::new();
    let mut cube_edges = Vec::new();
    cube::push_faces(&mut cube_triangles, state.hovered_face);
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
        let color = if state.hovered_face == Some(face) {
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
