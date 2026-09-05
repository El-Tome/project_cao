use cao_core::PartDocument;
use cao_core::ViewportConfig;
use cao_core::config::{Binding, PointerButton, TrackpadGesture, ViewportCorner};
use cao_core::history::{Operation, PointRef};
use cao_render::camera::{CubeZone, view_angles_towards};
use cao_render::{
    AxisStyle, GridStyle, OrbitCamera, SceneFrame, SceneRenderer, ViewTransition, ViewportRect,
    adaptive_step, cube, push_axes, push_grid, push_plane_outline, push_plane_quad, srgb,
};
use cao_sketch::{PointId, Sketch, WorkPlane};
use glam::{Vec2, Vec3};

use crate::screens::sketch::{ChainAnchor, SketchEditor, Tool};

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
    pub fn look_at_plane(&mut self, plane: WorkPlane, center: Vec3, radius: f32) {
        let (yaw, pitch) = view_angles_towards(plane.normal());
        self.transition = Some(ViewTransition::to_angles(&self.camera, yaw, pitch));
        self.camera.focus_on(center, radius, self.aspect);
        self.mode = ViewMode::Plane(plane);
    }

    /// A face lands on its work plane and shows the grid; an edge or a corner
    /// is an oblique view, which stays in wireframe mode.
    fn snap_to_zone(&mut self, zone: CubeZone) {
        self.transition = Some(ViewTransition::to_zone(&self.camera, zone));
        self.mode = match zone.face() {
            Some(face) => {
                let (u, v) = face.plane_basis();
                ViewMode::Plane(WorkPlane {
                    origin: Vec3::ZERO,
                    u,
                    v,
                })
            }
            None => ViewMode::Free,
        };
    }

    fn orbit(&mut self, delta: Vec2, sensitivity: f32) {
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
    } else {
        handle_sketch_input(ui, state, &response, rect, scale, sketch)
    };

    let frame = build_frame(state, rect, cube_rect, scale, sketch);
    paint_face_labels(ui, state, cube_rect);
    paint_dimension_labels(ui, state, rect, sketch);
    if state.config.ruler_visible {
        paint_ruler(ui, state, rect, scale);
    }

    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        rect,
        ViewportCallback { frame },
    ));

    changed
}

/// How much of the world one pixel covers right now, and the grid step that
/// follows from it. Shared by the grid and the scale bar so they can never
/// disagree.
#[derive(Clone, Copy)]
struct ViewScale {
    units_per_pixel: f32,
    /// Grid step in world units, for drawing.
    step: f32,
    /// The same step in millimetres, for the label.
    step_millimeters: f32,
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
        millimeters_per_unit: f32,
    ) -> Self {
        let height_px = rect.height() * pixels_per_point;
        let units_per_pixel = camera.world_units_per_pixel(height_px);
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
            units_per_pixel * millimeters_per_unit,
            config.grid_pixel_spacing,
        );

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
        self.step / self.units_per_pixel / pixels_per_point
    }

    /// World size of something that should stay a fixed size on screen.
    fn world_size_of(&self, pixels: f32) -> f32 {
        self.units_per_pixel * pixels
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
            Vec2::new(input.pointer.delta().x, input.pointer.delta().y),
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

    if scroll.trackpad != Vec2::ZERO {
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

/// Picking a plane, drawing lines, selecting a segment to dimension.
/// Returns true when the part changed.
fn handle_sketch_input(
    ui: &egui::Ui,
    state: &mut ViewportState,
    response: &egui::Response,
    rect: egui::Rect,
    scale: ViewScale,
    context: &mut SketchContext<'_>,
) -> bool {
    let Some(pointer) = response.hover_pos() else {
        context.editor.hovered_plane = None;
        context.editor.cursor = None;
        return false;
    };
    let (origin, direction) = state
        .camera
        .ray(to_ndc(pointer, rect), rect.width() / rect.height());

    if context.editor.is_choosing_plane() {
        let half_size = plane_half_size(state);
        context.editor.hovered_plane = WorkPlane::ORIGIN_PLANES
            .iter()
            .position(|plane| plane_hit(plane, origin, direction, half_size).is_some());

        if response.clicked()
            && let Some(index) = context.editor.hovered_plane
        {
            let plane = WorkPlane::ORIGIN_PLANES[index];
            context.document.apply(Operation::CreateSketch { plane });
            let sketch = context.document.sketches().len() - 1;
            context.editor.begin_editing(sketch, plane);
            // A fresh sketch has nothing to frame yet, so we show a patch of
            // plane big enough to draw in. The scale is meaningless until the
            // first dimension anyway.
            state.look_at_plane(plane, plane.origin, DEFAULT_SKETCH_RADIUS);
            return true;
        }
        return false;
    }

    let Some(index) = context.editor.active_sketch() else {
        return false;
    };
    let Some(plane) = context
        .document
        .sketches()
        .get(index)
        .map(|sketch| sketch.plane)
    else {
        return false;
    };
    let Some(cursor) = plane.ray_intersection(origin, direction) else {
        return false;
    };

    if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
        context.editor.end_chain();
    }

    // Snapping to an existing point is what lets a contour actually close.
    let snap = scale.world_size_of(10.0);

    context.editor.cursor = Some(snap_position(context, index, cursor, snap));

    match context.editor.tool {
        Tool::Line if response.clicked() => draw_line_point(context, index, cursor, snap),
        Tool::Dimension if response.clicked() => {
            let sketch = &context.document.sketches()[index];
            let segment = sketch.nearest_segment(cursor, snap);
            let length = segment.map(|id| {
                sketch
                    .dimension_of(id)
                    .map(|dimension| dimension.millimeters)
                    .unwrap_or_else(|| context.document.to_millimeters(sketch.segment_length(id)))
            });
            context.editor.select_segment(segment, length);
            false
        }
        _ => false,
    }
}

/// Where the next point would actually land, snapping onto an existing one when
/// the cursor is near it. Shown live so the drawing never surprises the user.
fn snap_position(context: &SketchContext<'_>, index: usize, cursor: Vec2, snap: f32) -> Vec2 {
    let sketch = &context.document.sketches()[index];
    match sketch.nearest_point(cursor, snap) {
        Some(id) => sketch.point(id),
        None => cursor,
    }
}

/// One click of the line tool. The first click only remembers where the chain
/// starts; the second turns the pair into a segment in the history.
fn draw_line_point(context: &mut SketchContext<'_>, index: usize, cursor: Vec2, snap: f32) -> bool {
    let sketch = &context.document.sketches()[index];
    let end = match sketch.nearest_point(cursor, snap) {
        Some(id) => PointRef::Existing(id),
        None => PointRef::New(cursor),
    };

    let Some(anchor) = context.editor.chain else {
        context.editor.chain = Some(match end {
            PointRef::Existing(id) => ChainAnchor::Point(id),
            PointRef::New(position) => ChainAnchor::Pending(position),
        });
        return false;
    };

    let start = match anchor {
        ChainAnchor::Point(id) => PointRef::Existing(id),
        ChainAnchor::Pending(position) => PointRef::New(position),
    };
    if start == end {
        return false;
    }

    context.document.apply(Operation::AddSegment {
        sketch: index,
        start,
        end,
    });

    // The far end of the segment just drawn becomes the next anchor. A point
    // created by the operation is the last one in the sketch.
    let sketch = &context.document.sketches()[index];
    context.editor.chain = Some(ChainAnchor::Point(match end {
        PointRef::Existing(id) => id,
        PointRef::New(_) => PointId(sketch.points().len().saturating_sub(1)),
    }));
    true
}

/// How much of the plane to show when a sketch has no geometry to frame yet.
pub const DEFAULT_SKETCH_RADIUS: f32 = 100.0;

/// Work planes are drawn a fixed fraction of the view across, so they stay
/// clickable however far the camera is.
fn plane_half_size(state: &ViewportState) -> f32 {
    state.camera.distance() * 0.3
}

/// Where a ray crosses a plane patch, in plane coordinates, if it lands inside
/// the square actually drawn.
fn plane_hit(plane: &WorkPlane, origin: Vec3, direction: Vec3, half_size: f32) -> Option<Vec2> {
    let hit = plane.ray_intersection(origin, direction)?;
    (hit.x.abs() <= half_size && hit.y.abs() <= half_size).then_some(hit)
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

/// Where a world point lands on screen, or `None` when it is behind the camera.
fn to_screen(point: Vec3, view_projection: glam::Mat4, rect: egui::Rect) -> Option<egui::Pos2> {
    let clip = view_projection * point.extend(1.0);
    if clip.w <= 1e-6 {
        return None;
    }
    let ndc = clip.truncate() / clip.w;
    Some(egui::pos2(
        rect.center().x + ndc.x * rect.width() * 0.5,
        rect.center().y - ndc.y * rect.height() * 0.5,
    ))
}

fn rect_height_px(ui: &egui::Ui, rect: egui::Rect) -> f32 {
    rect.height() * ui.ctx().pixels_per_point()
}

fn build_frame(
    state: &ViewportState,
    rect: egui::Rect,
    cube_rect: egui::Rect,
    scale: ViewScale,
    context: &SketchContext<'_>,
) -> SceneFrame {
    let camera = &state.camera;
    let pixels_per_point = scale.height_px / rect.height();

    let mut lines = Vec::new();
    let mut surfaces = Vec::new();

    // The grid is only drawn once the view has actually landed on the plane.
    // Mid-animation the view is oblique, and a grid of finite size seen at an
    // angle reads as a disc floating in the middle of the screen.
    if let (ViewMode::Plane(plane), None) = (state.mode, &state.transition) {
        // It also has to reach past the corners of the screen, or its outer
        // fade shows up as that same disc.
        let half_extent = scale.units_per_pixel * scale.diagonal_px * 1.5;
        let normal = plane.normal();
        let center = camera.target() - normal * (camera.target() - plane.origin).dot(normal);
        push_grid(
            &mut lines,
            plane.u,
            plane.v,
            center,
            scale.step,
            half_extent,
            &GridStyle::default(),
        );
    }

    let facing = match state.mode {
        ViewMode::Plane(plane) => Some(plane.normal()),
        ViewMode::Free => None,
    };
    push_axes(
        &mut lines,
        camera.distance() * 50.0,
        &AxisStyle::default(),
        facing,
    );

    if context.editor.is_choosing_plane() {
        push_choosable_planes(&mut surfaces, &mut lines, state, context);
    }

    for (index, sketch) in context.document.sketches().iter().enumerate() {
        let active = context.editor.active_sketch() == Some(index);
        push_sketch(&mut lines, sketch, scale, active, context);
    }

    let mut cube_triangles = Vec::new();
    let mut cube_edges = Vec::new();
    cube::push_faces(&mut cube_triangles, state.hovered_zone);
    cube::push_edges(&mut cube_edges, camera.forward(), 1.5);

    SceneFrame {
        scene_view_projection: camera.view_projection(scale.aspect),
        scene_viewport: to_physical(rect, pixels_per_point),
        scene_surfaces: surfaces,
        scene_lines: lines,
        cube_view_projection: cube::view_projection(camera.rotation()),
        cube_triangles,
        cube_edges,
        cube_viewport: to_physical(cube_rect, pixels_per_point),
    }
}

fn push_choosable_planes(
    surfaces: &mut Vec<cao_render::Vertex>,
    lines: &mut Vec<cao_render::Vertex>,
    state: &ViewportState,
    context: &SketchContext<'_>,
) {
    let half_size = plane_half_size(state);
    for (index, plane) in WorkPlane::ORIGIN_PLANES.iter().enumerate() {
        let hovered = context.editor.hovered_plane == Some(index);
        let fill = if hovered {
            srgb(0.30, 0.60, 0.95, 0.35)
        } else {
            srgb(0.55, 0.60, 0.68, 0.12)
        };
        let outline = if hovered {
            srgb(0.45, 0.75, 1.0, 1.0)
        } else {
            srgb(0.65, 0.70, 0.78, 0.7)
        };
        push_plane_quad(surfaces, plane.origin, plane.u, plane.v, half_size, fill);
        push_plane_outline(
            lines,
            plane.origin,
            plane.u,
            plane.v,
            half_size,
            outline,
            if hovered { 2.5 } else { 1.5 },
        );
    }
}

fn push_sketch(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    scale: ViewScale,
    active: bool,
    context: &SketchContext<'_>,
) {
    let color = if active {
        srgb(0.98, 0.85, 0.35, 1.0)
    } else {
        srgb(0.70, 0.72, 0.76, 0.8)
    };
    let width = if active { 2.5 } else { 1.5 };

    for segment in sketch.segments() {
        let start = sketch.plane.to_world(sketch.point(segment.start));
        let end = sketch.plane.to_world(sketch.point(segment.end));
        out.push(cao_render::Vertex::line(start, color, width));
        out.push(cao_render::Vertex::line(end, color, width));
    }

    if !active {
        return;
    }

    // Points are drawn as small crosses kept at a constant size on screen, so
    // they stay clickable at any zoom.
    let arm = scale.world_size_of(4.0);
    for point in sketch.points() {
        let center = sketch.plane.to_world(*point);
        for axis in [sketch.plane.u, sketch.plane.v] {
            out.push(cao_render::Vertex::line(center - axis * arm, color, 1.5));
            out.push(cao_render::Vertex::line(center + axis * arm, color, 1.5));
        }
    }

    // The segment about to be drawn, following the cursor: placing a point
    // blind and only then seeing where it went is needlessly uncomfortable.
    if let (Some(anchor), Some(cursor)) = (context.editor.chain, context.editor.cursor) {
        let from = match anchor {
            ChainAnchor::Pending(position) => position,
            ChainAnchor::Point(id) if id.0 < sketch.points().len() => sketch.point(id),
            ChainAnchor::Point(_) => cursor,
        };
        let preview = srgb(0.98, 0.85, 0.35, 0.55);
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(from),
            preview,
            1.5,
        ));
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(cursor),
            preview,
            1.5,
        ));
    }

    if let Some(selected) = context.editor.selected_segment
        && selected.0 < sketch.segments().len()
    {
        let (start, end) = sketch.endpoints(selected);
        let highlight = srgb(0.30, 0.75, 1.0, 1.0);
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(start),
            highlight,
            4.0,
        ));
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(end),
            highlight,
            4.0,
        ));
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

    for face in cao_render::CubeFace::ALL {
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

/// Each dimensioned segment carries its length, drawn beside its middle.
fn paint_dimension_labels(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
) {
    let Some(index) = context.editor.active_sketch() else {
        return;
    };
    let Some(sketch) = context.document.sketches().get(index) else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));
    let painter = ui.painter_at(rect);

    for dimension in sketch.dimensions() {
        if dimension.segment.0 >= sketch.segments().len() {
            continue;
        }
        let (start, end) = sketch.endpoints(dimension.segment);
        let middle = sketch.plane.to_world((start + end) * 0.5);
        let Some(position) = to_screen(middle, view_projection, rect) else {
            continue;
        };
        painter.text(
            position - egui::vec2(0.0, 12.0),
            egui::Align2::CENTER_BOTTOM,
            state.config.unit.format(dimension.millimeters),
            egui::FontId::proportional(13.0),
            egui::Color32::from_rgb(250, 220, 120),
        );
    }
}

fn face_label(face: cao_render::CubeFace) -> &'static str {
    match face {
        cao_render::CubeFace::PlusX => "DROITE",
        cao_render::CubeFace::MinusX => "GAUCHE",
        cao_render::CubeFace::PlusY => "ARRIÈRE",
        cao_render::CubeFace::MinusY => "FACE",
        cao_render::CubeFace::PlusZ => "DESSUS",
        cao_render::CubeFace::MinusZ => "DESSOUS",
    }
}

/// A scale bar: one grid step long, labelled with the length it represents.
/// It answers "how big is a square, and how fast am I zooming" at a glance.
fn paint_ruler(ui: &egui::Ui, state: &ViewportState, viewport: egui::Rect, scale: ViewScale) {
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

/// The scroll-like gestures of one frame, kept apart because they mean
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
    trackpad: Vec2,
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
                        scroll.trackpad += Vec2::new(delta.x, delta.y);
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
