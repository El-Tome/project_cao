use cao_core::PartDocument;
use cao_core::ViewportConfig;
use cao_core::config::{Binding, PointerButton, TrackpadGesture, ViewportCorner};
use cao_core::history::{Operation, PointRef};
use cao_render::camera::{CubeZone, view_angles_towards};
use cao_render::{
    AxisStyle, GridStyle, OrbitCamera, SceneFrame, SceneRenderer, ViewTransition, ViewportRect,
    adaptive_step, cube, push_axes, push_grid, push_plane_outline, push_plane_quad, srgb,
};
use cao_sketch::{DimensionTarget, PointId, Sketch, WorkPlane};
use glam::{Vec2, Vec3};

use crate::screens::sketch::{ChainAnchor, DimensionMode, SketchEditor, Tool};

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
    let cursor = magnetise(cursor, scale, &state.config, context, index, snap);

    context.editor.cursor = Some(cursor);
    context.editor.hovered_point = context.document.sketches()[index].nearest_point(cursor, snap);

    // Dragging a point is a gesture, not a click, so it comes before the
    // click-based tools.
    if context.editor.tool == Tool::Select {
        return drag_point(context, index, cursor, response, snap);
    }

    if !response.clicked() {
        return false;
    }

    match context.editor.tool {
        Tool::Line => draw_line_point(context, index, cursor, snap),
        Tool::Point => {
            context.document.apply(Operation::AddPoint {
                sketch: index,
                position: cursor,
            });
            true
        }
        Tool::Rectangle | Tool::Circle => two_click_shape(context, index, cursor, snap),
        Tool::Dimension => {
            measure(context, index, cursor, snap);
            false
        }
        Tool::Select | Tool::None => false,
    }
}

/// Moving a point by hand. The drawing settles around it afterwards, so the
/// values already given stay true.
fn drag_point(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: Vec2,
    response: &egui::Response,
    snap: f32,
) -> bool {
    let sketch = &context.document.sketches()[index];

    if response.drag_started() {
        context.editor.dragged_point = sketch
            .nearest_point(cursor, snap)
            .filter(|point| !sketch.is_origin(*point));
    }
    let Some(point) = context.editor.dragged_point else {
        return false;
    };
    let finished = response.drag_stopped();
    if finished {
        context.editor.dragged_point = None;
    }
    if !response.dragged() && !finished {
        return false;
    }

    context.document.apply(Operation::MovePoint {
        sketch: index,
        point,
        position: cursor,
    });
    true
}

/// A point already there, or a new one where the cursor is.
fn point_ref_at(context: &SketchContext<'_>, index: usize, position: Vec2, snap: f32) -> PointRef {
    match context.document.sketches()[index].nearest_point(position, snap) {
        Some(point) => PointRef::Existing(point),
        None => PointRef::New(position),
    }
}

fn two_click_shape(context: &mut SketchContext<'_>, index: usize, cursor: Vec2, snap: f32) -> bool {
    let Some(start) = context.editor.pending_start else {
        context.editor.pending_start = Some(cursor);
        return false;
    };
    // A shape with no extent is a stray click, not a drawing.
    if start.distance(cursor) < 1e-6 {
        return false;
    }
    context.editor.pending_start = None;

    // Corners and centres reuse a point already drawn when one is under the
    // cursor, so shapes hang together instead of stacking points on top of
    // each other. Nothing forces the user to place those points first.
    let anchor = point_ref_at(context, index, start, snap);
    let operation = match context.editor.tool {
        Tool::Rectangle => Operation::AddRectangle {
            sketch: index,
            corner: anchor,
            opposite: point_ref_at(context, index, cursor, snap),
        },
        _ => Operation::AddCircle {
            sketch: index,
            center: anchor,
            radius: start.distance(cursor),
        },
    };

    context.document.apply(operation);
    true
}

/// The smart dimension tool: works out what is under the cursor and measures
/// it, unless a mode is forcing one kind.
///
/// Two-step measurements — point to point, angle — collect their first half and
/// wait; everything else is settled in a single click.
fn measure(context: &mut SketchContext<'_>, index: usize, cursor: Vec2, snap: f32) {
    let mode = context.editor.dimension_mode;
    let sketch = &context.document.sketches()[index];

    // A point wins over a segment under the same cursor: it is the smaller
    // target, so aiming at it is the deliberate act.
    if mode.takes_points()
        && let Some(point) = sketch.nearest_point(cursor, snap * 0.8)
    {
        return measure_from_point(context, index, point);
    }
    if mode == DimensionMode::PointToPoint {
        return;
    }

    if matches!(mode, DimensionMode::Auto | DimensionMode::Angle)
        && context.editor.first_angle_segment.is_some()
    {
        return continue_angle(context, index, cursor, snap);
    }

    // An axis picked first waits for the segment to measure against it.
    if matches!(mode, DimensionMode::Auto | DimensionMode::Angle)
        && context.editor.first_angle_segment.is_none()
        && let Some(axis) = axis_under(cursor, snap)
        && sketch.nearest_segment(cursor, snap).is_none()
    {
        context.editor.first_axis = Some(axis);
        context.editor.message = Some(format!(
            "{} choisi, cliquez maintenant un trait",
            axis.label()
        ));
        return;
    }

    if mode != DimensionMode::Radius
        && let Some(segment) = sketch.nearest_segment(cursor, snap)
    {
        if let Some(axis) = context.editor.first_axis.take() {
            return select_target(context, index, DimensionTarget::AxisAngle { segment, axis });
        }
        if mode == DimensionMode::Angle {
            context.editor.first_angle_segment = Some(segment);
            context.editor.message =
                Some("Choisissez le second trait, ou un axe de l'esquisse".to_string());
            return;
        }
        return select_target(context, index, DimensionTarget::Length(segment));
    }

    if mode != DimensionMode::Length
        && let Some(circle) = sketch.nearest_circle(cursor, snap)
    {
        return select_target(context, index, DimensionTarget::Radius(circle));
    }

    context.editor.select(None, None);
    context.editor.message = Some("Rien à mesurer ici".to_string());
}

/// First click on a point remembers it; the second gives the distance between
/// the two. Measuring from the sketch origin is how a shape gets positioned.
fn measure_from_point(context: &mut SketchContext<'_>, index: usize, point: PointId) {
    let Some(first) = context.editor.first_point else {
        context.editor.first_point = Some(point);
        context.editor.message = Some("Choisissez le second point".to_string());
        return;
    };
    if first == point {
        return;
    }
    context.editor.first_point = None;
    select_target(
        context,
        index,
        DimensionTarget::Distance {
            from: first,
            to: point,
        },
    );
}

/// Second half of an angle: another segment, or one of the sketch axes.
fn continue_angle(context: &mut SketchContext<'_>, index: usize, cursor: Vec2, snap: f32) {
    let Some(first) = context.editor.first_angle_segment else {
        return;
    };
    let sketch = &context.document.sketches()[index];

    // A segment lying along an axis is found before the axis itself, so
    // clicking the same segment twice is read as "and now the axis it sits on"
    // rather than ignored — which is what made a rectangle drawn on the axes
    // impossible to pin down.
    if let Some(second) = sketch.nearest_segment(cursor, snap)
        && second != first
    {
        context.editor.first_angle_segment = None;
        if sketch.angle_between(first, second).is_none() {
            context.editor.select(None, None);
            context.editor.message = Some("Ces deux traits ne se touchent pas".to_string());
            return;
        }
        return select_target(context, index, DimensionTarget::Angle { first, second });
    }

    // An axis rather than a second segment gives the drawing a fixed direction
    // to lean on — the only way to stop it turning about its origin.
    if let Some(axis) = axis_under(cursor, snap) {
        context.editor.first_angle_segment = None;
        select_target(
            context,
            index,
            DimensionTarget::AxisAngle {
                segment: first,
                axis,
            },
        );
    }
}

/// Selects a target and fills the value field, warning when the value would add
/// nothing.
fn select_target(context: &mut SketchContext<'_>, index: usize, target: DimensionTarget) {
    let measured = context.document.measured(index, target);
    context.editor.select(Some(target), measured);

    let scale = context.document.scale();
    context.editor.message = context.document.sketches()[index]
        .would_be_redundant(target, scale)
        .then(|| REDUNDANT_WARNING.to_string());
}

/// Which sketch axis the cursor is on, if either. The axes are drawn as lines
/// through the origin, so they are picked the same way a segment is.
fn axis_under(cursor: Vec2, tolerance: f32) -> Option<cao_sketch::SketchAxis> {
    if cursor.y.abs() <= tolerance {
        return Some(cao_sketch::SketchAxis::U);
    }
    if cursor.x.abs() <= tolerance {
        return Some(cao_sketch::SketchAxis::V);
    }
    None
}

/// Shown when a value would add nothing to a shape that is already settled.
pub const REDUNDANT_WARNING: &str = "Cette cote n'apporte rien : la forme est déjà entièrement contrainte. Elle sera posée en simple lecture.";

/// Pulls the cursor onto whatever it is near: an existing point first, then the
/// grid.
///
/// The grid magnet is what makes drawing on the origin, or a right angle by
/// following the lines, a matter of aiming roughly rather than exactly. It only
/// bites within a few pixels, so a deliberate free position is still possible.
fn magnetise(
    cursor: Vec2,
    scale: ViewScale,
    config: &ViewportConfig,
    context: &SketchContext<'_>,
    index: usize,
    snap: f32,
) -> Vec2 {
    let sketch = &context.document.sketches()[index];
    if let Some(point) = sketch.nearest_point(cursor, snap) {
        return sketch.point(point);
    }
    if !config.grid_snap {
        return cursor;
    }

    // The grid is drawn every `step`; snapping to a fraction of it keeps the
    // magnet useful without forcing everything onto the coarse lines.
    let step = scale.step / config.grid_snap_divisions.max(1) as f32;
    if step <= 0.0 {
        return cursor;
    }
    let snapped = Vec2::new(
        (cursor.x / step).round() * step,
        (cursor.y / step).round() * step,
    );
    if snapped.distance(cursor) <= scale.world_size_of(config.grid_snap_pixels) {
        snapped
    } else {
        cursor
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

/// Colours saying how settled the drawing is: a shape that still has freedom
/// left is drawn one way, one that is fully determined another. It is the
/// quickest possible answer to "is my part pinned down yet?".
fn sketch_colors(active: bool, constrained: bool) -> ([f32; 4], f32) {
    match (active, constrained) {
        (true, false) => (srgb(0.98, 0.85, 0.35, 1.0), 2.5),
        (true, true) => (srgb(0.45, 0.85, 0.55, 1.0), 2.5),
        (false, _) => (srgb(0.70, 0.72, 0.76, 0.8), 1.5),
    }
}

/// Where a point is shown: at the cursor while it is being dragged, at its
/// recorded place otherwise.
fn shown_position(sketch: &Sketch, point: PointId, context: &SketchContext<'_>) -> Vec2 {
    match (context.editor.dragged_point, context.editor.drag_position) {
        (Some(dragged), Some(position)) if dragged == point && !sketch.is_origin(point) => position,
        _ => sketch.point(point),
    }
}

fn push_sketch(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    scale: ViewScale,
    active: bool,
    context: &SketchContext<'_>,
) {
    let settled = sketch.is_settled(context.document.scale());
    for segment in sketch.segments() {
        let (color, width) = sketch_colors(active, settled);
        let start = sketch
            .plane
            .to_world(shown_position(sketch, segment.start, context));
        let end = sketch
            .plane
            .to_world(shown_position(sketch, segment.end, context));
        out.push(cao_render::Vertex::line(start, color, width));
        out.push(cao_render::Vertex::line(end, color, width));
    }

    for circle in sketch.circles() {
        let (color, width) = sketch_colors(active, settled);
        push_circle(out, sketch, circle.center, circle.radius, color, width);
    }

    if !active {
        return;
    }

    push_point_markers(out, sketch, scale, settled, context);

    // Every dimension is drawn where it applies, with extension lines, arrows
    // and arcs, so the drawing says what holds it rather than just carrying a
    // number.
    for dimension in sketch.dimensions() {
        let style = if dimension.driven {
            crate::screens::annotations::Style::driven()
        } else {
            crate::screens::annotations::Style::driving()
        };
        crate::screens::annotations::push(
            out,
            sketch,
            dimension.target,
            &style,
            scale.units_per_pixel,
        );
    }

    push_preview(out, sketch, context);

    if let Some(cao_sketch::DimensionTarget::Length(selected)) = context.editor.selected
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

/// Points are drawn as small squares kept at a constant size on screen, so they
/// stay visible and clickable at any zoom. The sketch origin gets a diamond
/// instead: it is always there, cannot be moved, and everything can be measured
/// from it, so it should not look like an ordinary point.
fn push_point_markers(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    scale: ViewScale,
    settled: bool,
    context: &SketchContext<'_>,
) {
    let half = scale.world_size_of(4.0);
    let (base, _) = sketch_colors(true, settled);
    let highlight = srgb(0.30, 0.75, 1.0, 1.0);

    for index in 0..sketch.points().len() {
        let point = PointId(index);
        let center = sketch
            .plane
            .to_world(shown_position(sketch, point, context));
        let hovered = context.editor.hovered_point == Some(point)
            || context.editor.first_point == Some(point);
        let color = if hovered { highlight } else { base };
        let width = if hovered { 2.5 } else { 1.5 };

        let (u, v) = if sketch.is_origin(point) {
            // Turned a quarter: a diamond reads differently at a glance.
            (
                (sketch.plane.u + sketch.plane.v) * std::f32::consts::FRAC_1_SQRT_2,
                (sketch.plane.v - sketch.plane.u) * std::f32::consts::FRAC_1_SQRT_2,
            )
        } else {
            (sketch.plane.u, sketch.plane.v)
        };
        let size = if sketch.is_origin(point) {
            half * 1.6
        } else {
            half
        };

        let corners = [
            center - u * size - v * size,
            center + u * size - v * size,
            center + u * size + v * size,
            center - u * size + v * size,
        ];
        for corner in 0..4 {
            out.push(cao_render::Vertex::line(corners[corner], color, width));
            out.push(cao_render::Vertex::line(
                corners[(corner + 1) % 4],
                color,
                width,
            ));
        }
    }
}

/// The shape about to be drawn, following the cursor: placing a point blind and
/// only then seeing where it went is needlessly uncomfortable.
fn push_preview(out: &mut Vec<cao_render::Vertex>, sketch: &Sketch, context: &SketchContext<'_>) {
    let Some(cursor) = context.editor.cursor else {
        return;
    };
    let preview = srgb(0.98, 0.85, 0.35, 0.55);

    if let Some(anchor) = context.editor.chain {
        let from = match anchor {
            ChainAnchor::Pending(position) => position,
            ChainAnchor::Point(id) if id.0 < sketch.points().len() => sketch.point(id),
            ChainAnchor::Point(_) => cursor,
        };
        push_preview_line(out, sketch, from, cursor, preview);
    }

    let Some(start) = context.editor.pending_start else {
        return;
    };
    match context.editor.tool {
        Tool::Rectangle => {
            let corners = [
                start,
                Vec2::new(cursor.x, start.y),
                cursor,
                Vec2::new(start.x, cursor.y),
            ];
            for index in 0..4 {
                push_preview_line(
                    out,
                    sketch,
                    corners[index],
                    corners[(index + 1) % 4],
                    preview,
                );
            }
        }
        Tool::Circle => {
            let radius = start.distance(cursor);
            push_circle_at(out, sketch, start, radius, preview, 1.5);
        }
        _ => {}
    }
}

fn push_preview_line(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    from: Vec2,
    to: Vec2,
    color: [f32; 4],
) {
    out.push(cao_render::Vertex::line(
        sketch.plane.to_world(from),
        color,
        1.5,
    ));
    out.push(cao_render::Vertex::line(
        sketch.plane.to_world(to),
        color,
        1.5,
    ));
}

fn push_circle(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    center: PointId,
    radius: f32,
    color: [f32; 4],
    width: f32,
) {
    push_circle_at(out, sketch, sketch.point(center), radius, color, width);
}

/// Circles are drawn as a many-sided polygon: the line renderer only knows
/// about segments, and at this many sides the corners are invisible.
fn push_circle_at(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    center: Vec2,
    radius: f32,
    color: [f32; 4],
    width: f32,
) {
    const SIDES: usize = 96;
    if radius <= 0.0 {
        return;
    }
    let mut previous = None;
    for step in 0..=SIDES {
        let angle = step as f32 / SIDES as f32 * std::f32::consts::TAU;
        let point = center + Vec2::new(angle.cos(), angle.sin()) * radius;
        let world = sketch.plane.to_world(point);
        if let Some(previous) = previous {
            out.push(cao_render::Vertex::line(previous, color, width));
            out.push(cao_render::Vertex::line(world, color, width));
        }
        previous = Some(world);
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

/// Each dimension is drawn where it applies, with the value it stands for.
/// A readout shows what the geometry measures rather than a stored number, so
/// it stays true however the drawing moves.
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

    let pixel = state
        .camera
        .world_units_per_pixel(rect.height() * ui.ctx().pixels_per_point());

    for dimension in sketch.dimensions() {
        // Asking the annotation where its value belongs keeps the text on the
        // dimension line instead of floating near the geometry.
        let mut ignored = Vec::new();
        let style = crate::screens::annotations::Style::driving();
        let Some(placement) = crate::screens::annotations::push(
            &mut ignored,
            sketch,
            dimension.target,
            &style,
            pixel,
        ) else {
            continue;
        };
        let Some(position) = to_screen(
            sketch.plane.to_world(placement.text_at),
            view_projection,
            rect,
        ) else {
            continue;
        };
        let value = if dimension.driven {
            context
                .document
                .measured(index, dimension.target)
                .unwrap_or(dimension.value)
        } else {
            dimension.value
        };
        let text = if dimension.is_angle() {
            format!("{value:.1}°")
        } else {
            state.config.unit.format(value)
        };
        let color = if dimension.driven {
            egui::Color32::from_gray(170)
        } else {
            egui::Color32::from_rgb(250, 220, 120)
        };
        painter.text(
            position,
            egui::Align2::CENTER_CENTER,
            if dimension.driven {
                format!("({text})")
            } else {
                text
            },
            egui::FontId::proportional(13.0),
            color,
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
