use cao_core::PartDocument;
use cao_core::ViewportConfig;
use cao_core::config::{Binding, PointerButton, TrackpadGesture, ViewportCorner};
use cao_core::history::{Operation, PointRef};
use cao_render::camera::{CubeZone, view_angles_towards};
use cao_core::theme::{Background, Rgba, Theme};
use cao_render::{
    AxisStyle, BackgroundShape, GridStyle, OrbitCamera, SceneFrame, SceneRenderer, ViewTransition,
    ViewportRect, adaptive_step, cube, push_axes, push_grid, push_plane_outline, push_plane_quad,
    srgb,
};
use cao_sketch::{
    CircleId, Constraint, DimensionTarget, Element, PointId, SegmentId, Sketch, WorkPlane,
};
use glam::{DVec2, DVec3};

use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{
    ChainAnchor, CircleMode, DimensionMode, LiveField, PlaneChoice, Rule, RulePick, Selection,
    SketchEditor, Tool,
};

/// A colour from the theme, turned into the space the shader blends in.
fn tint(color: Rgba) -> [f32; 4] {
    srgb(color.r, color.g, color.b, color.a)
}

/// The same, with the opacity replaced.
fn tint_at(color: Rgba, alpha: f32) -> [f32; 4] {
    tint(color.with_alpha(alpha))
}

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

    let drawing = sketch.editor.chain.is_some()
        || sketch.editor.pending_start.is_some()
        || !sketch.editor.circle_points.is_empty()
        || !sketch.editor.circle_segments.is_empty();
    if drawing && paint_live_input(ui, sketch) {
        // Enter finishes the shape from the keyboard, without having to find
        // the canvas again with the mouse.
        if let Some(index) = sketch.editor.active_sketch() {
            let cursor = sketch.editor.aimed.or(sketch.editor.cursor).unwrap_or_default();
            let snap = scale.world_size_of(PICK_PIXELS);
            changed |= match sketch.editor.tool {
                Tool::Line => {
                    draw_line_point(sketch, index, cursor, snap, scale.units_per_pixel)
                }
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
    let busy = editor.chain.is_some()
        || editor.pending_start.is_some()
        || editor.placing.is_some()
        || editor.selected.is_some()
        || !editor.selection.is_empty()
        || editor.first_point.is_some()
        || editor.first_angle_segment.is_some()
        || editor.first_axis.is_some();
    editor.reset_pending();
    if !busy {
        editor.tool = Tool::Select;
    }
}

/// How much of the world one pixel covers right now, and the grid step that
/// follows from it. Shared by the grid and the scale bar so they can never
/// disagree.
#[derive(Clone, Copy)]
struct ViewScale {
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
    fn world_size_of(&self, pixels: f64) -> f64 {
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
    // The camera reasons in f32, the drawing in f64: the ray crosses over here,
    // once, rather than at every call that follows.
    let (origin, direction) = state
        .camera
        .ray(to_ndc(pointer, rect), rect.width() / rect.height());
    let (origin, direction) = (origin.as_dvec3(), direction.as_dvec3());

    if context.editor.is_choosing_plane() {
        context.editor.hovered_plane = plane_under(state, context, origin, direction);

        if response.clicked()
            && let Some(choice) = context.editor.hovered_plane
        {
            let plane = choice.plane();
            context.document.apply(Operation::CreateSketch { plane });
            let sketch = context.document.sketches().len() - 1;
            context.editor.begin_editing(sketch, plane);
            // A fresh sketch has nothing to frame yet, so we show a patch of
            // plane big enough to draw in, centred where the click landed. On a
            // face of the part that matters: the plane's own origin is the world
            // origin projected onto it, which can be nowhere near the face.
            let center = plane
                .ray_intersection(origin, direction)
                .map(|local| plane.to_world(local))
                .unwrap_or(plane.origin);
            state.look_at_plane(plane, center, DEFAULT_SKETCH_RADIUS);
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

    // Snapping to an existing point is what lets a contour actually close.
    let snap = scale.world_size_of(PICK_PIXELS);
    let (cursor, snapped_to) = magnetise(cursor, scale, &state.config, context, index, snap);
    context.editor.snap = snapped_to;

    context.editor.cursor = Some(cursor);
    context.editor.hovered_point = context.document.sketches()[index].nearest_point(cursor, snap);

    // Worked out once a frame and shown as the preview, so that what is drawn
    // on screen is exactly what a click would record.
    let (aimed, corner) = match context.editor.tool {
        Tool::Line if context.editor.chain.is_some() => {
            let aimed = aim(context, index, cursor);
            let sketch = &context.document.sketches()[index];
            let corner = aimed
                .square_with
                .and_then(|_| anchor_position(sketch, context));
            (Some(aimed.position), corner)
        }
        Tool::Rectangle if context.editor.pending_start.is_some() => {
            (Some(rectangle_corner(context, cursor)), None)
        }
        _ => (None, None),
    };
    context.editor.aimed = aimed;
    context.editor.square_corner = corner;

    // Dragging a point is a gesture, not a click, so it comes before the
    // click-based tools.
    if context.editor.tool == Tool::Select {
        // What is grabbed is decided where the button went down, not where the
        // cursor is when egui calls it a drag: by then it has already travelled
        // the few pixels of the drag threshold, which was enough to miss the
        // very point being aimed at.
        let pressed = ui
            .input(|input| input.pointer.press_origin())
            .and_then(|position| {
                let (origin, direction) = state
                    .camera
                    .ray(to_ndc(position, rect), rect.width() / rect.height());
                plane.ray_intersection(origin.as_dvec3(), direction.as_dvec3())
            })
            .map(|position| magnetise(position, scale, &state.config, context, index, snap).0)
            .unwrap_or(cursor);

        let adding = ui.input(|input| input.modifiers.command || input.modifiers.shift);
        if response.clicked() {
            let picked = pick(context, index, cursor, snap, scale.units_per_pixel);
            match (picked, adding) {
                // Holding the modifier gathers things up one by one, which is
                // how one picks out three traits that no box can enclose alone.
                (Some(what), true) => context.editor.toggle(what),
                (Some(what), false) => context.editor.selection = vec![what],
                (None, false) => context.editor.selection.clear(),
                (None, true) => {}
            }
            // A dimension picked with the selection tool opens its value too:
            // reaching for the dimension tool again to change a number one is
            // already pointing at is a step for nothing.
            match picked {
                Some(Selection::Dimension(target)) if !adding => {
                    edit_dimension(context, index, target)
                }
                _ => context.editor.select(None, None),
            }
        }

        if !context.editor.selection.is_empty()
            && ui.input(|input| {
                input.key_pressed(egui::Key::Delete) || input.key_pressed(egui::Key::Backspace)
            })
        {
            let held = std::mem::take(&mut context.editor.selection);
            return erase(context, index, &held);
        }

        // A drag that grabbed nothing pulls a box instead, the way a desktop
        // does. Who grabs is settled at the start of the gesture and holds for
        // the whole of it: deciding again every frame would swap gestures
        // mid-drag, as soon as the cursor happened to pass over a point.
        if response.drag_started() {
            let changed = drag_point(
                context,
                index,
                cursor,
                pressed,
                response,
                snap,
                scale.units_per_pixel,
            );
            if context.editor.dragged_point.is_none()
                && context.editor.dragged_dimension.is_none()
                && context.editor.dragged_group.is_empty()
            {
                context.editor.band = Some((pressed, cursor));
            }
            return changed;
        }
        if context.editor.band.is_some() {
            return band_select(context, index, cursor, response, adding, scale.units_per_pixel);
        }

        return drag_point(
            context,
            index,
            cursor,
            pressed,
            response,
            snap,
            scale.units_per_pixel,
        );
    }

    if !response.clicked() {
        return false;
    }

    match context.editor.tool {
        Tool::Line => draw_line_point(context, index, cursor, snap, scale.units_per_pixel),
        Tool::Point => {
            context.document.apply(Operation::AddPoint {
                sketch: index,
                position: cursor,
            });
            true
        }
        Tool::Rectangle => two_click_shape(
            context,
            index,
            context.editor.aimed.unwrap_or(cursor),
            snap,
            scale.units_per_pixel,
        ),
        Tool::Circle => draw_circle(context, index, cursor, snap, scale.units_per_pixel),
        Tool::Dimension => measure(context, index, cursor, snap, scale.units_per_pixel),
        Tool::Constrain(rule) => constrain(context, index, rule, cursor, snap),
        Tool::Select | Tool::None => false,
    }
}

/// What is offered to sketch on under the cursor: a face of the part where
/// there is one, otherwise the nearest of the three planes of the origin.
///
/// The part comes first rather than whatever is nearest the camera. The three
/// planes are unbounded sheets running right through the part, so nearest-wins
/// would leave them covering the very faces one usually wants — while they
/// stay reachable everywhere the part is not.
fn plane_under(
    state: &ViewportState,
    context: &SketchContext<'_>,
    origin: DVec3,
    direction: DVec3,
) -> Option<PlaneChoice> {
    if let Some(hit) = context.document.body().ray_hit(origin, direction) {
        // The sketch's own origin lands where the world origin projects onto
        // the face, so that a drawing on a face is still measured from
        // somewhere the user can point at.
        let normal = hit.polygon.normal();
        let plane = WorkPlane::from_normal(normal * hit.polygon.plane_offset(), normal);
        return Some(PlaneChoice::Face(plane));
    }

    let half_size = plane_half_size(state);
    WorkPlane::ORIGIN_PLANES
        .iter()
        .enumerate()
        .filter_map(|(index, plane)| {
            let local = plane_hit(plane, origin, direction, half_size)?;
            Some(((plane.to_world(local) - origin).length(), index))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, index)| PlaneChoice::Origin(index))
}

/// What the cursor is over, in the order a click should take it.
///
/// A point before a line before a circle before a dimension: the smaller the
/// target, the harder it is to hit on purpose, so the smaller one wins.
fn pick(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> Option<Selection> {
    let sketch = context.document.sketches().get(index)?;

    if let Some(point) = sketch
        .nearest_point(cursor, snap)
        .filter(|point| !sketch.is_origin(*point))
    {
        return Some(Selection::Element(Element::Point(point)));
    }
    if let Some(segment) = sketch.nearest_segment(cursor, snap) {
        return Some(Selection::Element(Element::Segment(segment)));
    }
    if let Some(circle) = sketch.nearest_circle(cursor, snap) {
        return Some(Selection::Element(Element::Circle(circle)));
    }
    if let Some(target) = nearest_annotation(context, index, cursor, snap * 1.5, pixel) {
        return Some(Selection::Dimension(target));
    }
    nearest_rule(sketch, cursor, snap * 1.5).map(Selection::Rule)
}

/// The rule whose nearest mark sits under the cursor.
fn nearest_rule(sketch: &Sketch, cursor: DVec2, tolerance: f64) -> Option<Constraint> {
    sketch
        .constraints()
        .iter()
        .filter_map(|constraint| {
            let nearest = rule_marks(sketch, *constraint)
                .into_iter()
                .map(|at| at.distance(cursor))
                .min_by(f64::total_cmp)?;
            Some((*constraint, nearest))
        })
        .filter(|(_, distance)| *distance <= tolerance)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(constraint, _)| constraint)
}

/// Points the constraint tool at something, and lays the rule down as soon as
/// it has been shown enough.
///
/// The order of the clicks does not matter: a point and a trait make the same
/// coincidence whichever comes first. What matters is what was clicked, so the
/// rule is built from the kinds gathered rather than from their order.
fn constrain(
    context: &mut SketchContext<'_>,
    index: usize,
    rule: Rule,
    cursor: DVec2,
    snap: f64,
) -> bool {
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    // Smallest target first, as everywhere else: a point is harder to hit on
    // purpose than the trait it sits on. The axes of the sketch come last,
    // being the widest thing on screen.
    let picked = sketch
        .nearest_point(cursor, snap * 0.8)
        .filter(|point| !sketch.is_origin(*point) || rule == Rule::Coincident)
        .map(Element::Point)
        .or_else(|| sketch.nearest_segment(cursor, snap).map(Element::Segment))
        .or_else(|| sketch.nearest_circle(cursor, snap).map(Element::Circle))
        .map(RulePick::Element)
        .or_else(|| {
            (rule == Rule::Collinear)
                .then(|| axis_under(cursor, snap).map(RulePick::Axis))
                .flatten()
        });

    let Some(picked) = picked else {
        context.editor.message = Some("Rien à contraindre ici".to_string());
        return false;
    };
    if context.editor.rule_picks.contains(&picked) {
        return false;
    }
    context.editor.rule_picks.push(picked);
    if context.editor.rule_picks.len() < rule.wants() {
        context.editor.message = Some(rule.asks_for().to_string());
        return false;
    }

    let picks = std::mem::take(&mut context.editor.rule_picks);
    if let Some(Operation::Constrain { constraint, .. }) =
        rule_operation(rule, index, &picks, sketch)
        && sketch.constraints().contains(&constraint.normalised())
    {
        // The drawing already carries it; recording the step again would fill
        // the history with entries that change nothing.
        context.editor.message = Some(format!("{} : déjà posée", rule.label()));
        return false;
    }
    let Some(operation) = rule_operation(rule, index, &picks, sketch) else {
        context.editor.message = Some(format!("{} : {}", rule.asks_for(), "pas ces éléments-là"));
        return false;
    };
    context.document.apply(operation);
    context.editor.message = Some(rule.asks_for().to_string());
    true
}

/// The step a rule becomes, once it has been shown what it speaks of.
///
/// Two of them are not rules at all but merges: two points made one, or two
/// circles brought onto a single centre. Holding them apart with an equation
/// would leave two points sitting on top of each other for ever, which is
/// exactly what the drawing does not want.
fn rule_operation(
    rule: Rule,
    index: usize,
    picks: &[RulePick],
    sketch: &Sketch,
) -> Option<Operation> {
    // The order is kept: for an equality, the first trait clicked is the one
    // whose length the other takes.
    let segments: Vec<SegmentId> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Element(Element::Segment(id)) => Some(*id),
            _ => None,
        })
        .collect();
    let points: Vec<PointId> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Element(Element::Point(id)) => Some(*id),
            _ => None,
        })
        .collect();
    let circles: Vec<CircleId> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Element(Element::Circle(id)) => Some(*id),
            _ => None,
        })
        .collect();
    let axes: Vec<cao_sketch::SketchAxis> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Axis(axis) => Some(*axis),
            _ => None,
        })
        .collect();

    let constraint = |constraint: Constraint| {
        Some(Operation::Constrain {
            sketch: index,
            constraint,
        })
    };
    let pair = |list: &[SegmentId]| (list.len() == 2).then(|| (list[0], list[1]));

    match rule {
        Rule::Perpendicular => pair(&segments)
            .and_then(|(first, second)| constraint(Constraint::Perpendicular { first, second })),
        Rule::Parallel => pair(&segments)
            .and_then(|(first, second)| constraint(Constraint::Parallel { first, second })),
        Rule::Collinear => match (pair(&segments), segments.as_slice(), axes.as_slice()) {
            (Some((first, second)), _, _) => constraint(Constraint::Collinear { first, second }),
            // A trait laid on one of the sketch's own axes, which is the same
            // rule against a line that cannot move.
            (None, [segment], [axis]) => constraint(Constraint::AxisCollinear {
                segment: *segment,
                axis: *axis,
            }),
            _ => None,
        },
        Rule::Equal => match (pair(&segments), circles.as_slice()) {
            (Some((first, second)), _) => constraint(Constraint::Equal { first, second }),
            (None, [first, second]) => constraint(Constraint::EqualRadius {
                first: *first,
                second: *second,
            }),
            _ => None,
        },
        Rule::Tangent => match (circles.as_slice(), segments.as_slice()) {
            ([circle], [segment]) => constraint(Constraint::Tangent {
                at: None,
                circle: *circle,
                segment: *segment,
            }),
            _ => None,
        },
        Rule::Midpoint => match (points.as_slice(), segments.as_slice()) {
            ([point], [segment]) => constraint(Constraint::Midpoint {
                point: *point,
                segment: *segment,
            }),
            _ => None,
        },
        Rule::Fixed => match picks {
            [RulePick::Element(element)] => constraint(Constraint::Fixed { element: *element }),
            _ => None,
        },
        Rule::Coincident => match (points.as_slice(), segments.as_slice()) {
            ([point], [segment]) => constraint(Constraint::OnSegment {
                point: *point,
                segment: *segment,
            }),
            // Two points asked to coincide are one point: the origin is never
            // the one that gives way.
            ([first, second], []) => {
                let (kept, dropped) = match sketch.is_origin(*second) {
                    true => (*second, *first),
                    false => (*first, *second),
                };
                Some(Operation::MergePoints {
                    sketch: index,
                    kept,
                    dropped,
                })
            }
            _ => None,
        },
        Rule::Concentric => match circles.as_slice() {
            [first, second] => {
                let (kept, dropped) = (
                    sketch.circle(*first).center,
                    sketch.circle(*second).center,
                );
                (kept != dropped).then_some(Operation::MergePoints {
                    sketch: index,
                    kept,
                    dropped,
                })
            }
            _ => None,
        },
    }
}

/// Pulls a box across the drawing and takes everything inside it.
///
/// Whole elements only: a trait counts when both its ends are in the box. Half
/// a trait cannot be deleted, so letting the box claim it would say something
/// the drawing cannot do.
fn band_select(
    context: &mut SketchContext<'_>,
    index: usize,
    to: DVec2,
    response: &egui::Response,
    adding: bool,
    pixel: f64,
) -> bool {
    // Where the box started is kept from the frame the drag began: egui lets go
    // of the press position on the very frame the button comes up, which is the
    // frame that matters here.
    let Some((from, _)) = context.editor.band else {
        return false;
    };
    context.editor.band = Some((from, to));
    if !response.drag_stopped() {
        return false;
    }
    context.editor.band = None;

    let (low, high) = (from.min(to), from.max(to));
    let inside = |point: DVec2| point.cmpge(low).all() && point.cmple(high).all();
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };

    let mut caught: Vec<Selection> = Vec::new();
    for (id, point) in sketch.live_points() {
        if inside(point) && !sketch.is_origin(id) {
            caught.push(Selection::Element(Element::Point(id)));
        }
    }
    for (id, segment) in sketch.live_segments() {
        if inside(sketch.point(segment.start)) && inside(sketch.point(segment.end)) {
            caught.push(Selection::Element(Element::Segment(id)));
        }
    }
    for (id, circle) in sketch.live_circles() {
        let center = sketch.point(circle.center);
        let reach = DVec2::splat(circle.radius);
        if inside(center - reach) && inside(center + reach) {
            caught.push(Selection::Element(Element::Circle(id)));
        }
    }
    for dimension in sketch.dimensions() {
        if annotation_home(context, index, dimension.target, pixel)
            .is_some_and(|(text_at, _)| inside(text_at))
        {
            caught.push(Selection::Dimension(dimension.target));
        }
    }

    if !adding {
        context.editor.selection.clear();
    }
    for what in caught {
        if !context.editor.is_selected(what) {
            context.editor.selection.push(what);
        }
    }
    false
}

/// Deletes what the selection tool is holding, in one step.
fn erase(context: &mut SketchContext<'_>, index: usize, selection: &[Selection]) -> bool {
    if selection.is_empty() {
        return false;
    }
    let mut elements = Vec::new();
    let mut dimensions = Vec::new();
    let mut constraints = Vec::new();
    for held in selection {
        match held {
            Selection::Element(element) => elements.push(*element),
            Selection::Dimension(target) => dimensions.push(*target),
            Selection::Rule(constraint) => constraints.push(*constraint),
        }
    }
    context.document.apply(Operation::EraseMany {
        sketch: index,
        elements,
        dimensions,
        constraints,
    });
    true
}

/// Choosing which closed areas of a sketch become matter.
///
/// An area is named by a point inside it rather than by its rank, so the choice
/// still means the same thing after the drawing changes. Clicking an area
/// already chosen takes it back out.
fn pick_areas(
    state: &ViewportState,
    response: &egui::Response,
    rect: egui::Rect,
    scale: ViewScale,
    context: &mut SketchContext<'_>,
) {
    context.extrusion.hovered = None;

    let Some(index) = context.extrusion.sketch else {
        return;
    };
    let Some(sketch) = context.document.sketches().get(index) else {
        return;
    };
    let Some(pointer) = response.hover_pos() else {
        return;
    };

    let (origin, direction) = state
        .camera
        .ray(to_ndc(pointer, rect), rect.width() / rect.height());
    let Some(cursor) = sketch
        .plane
        .ray_intersection(origin.as_dvec3(), direction.as_dvec3())
    else {
        return;
    };

    // A line is a much smaller target than an area, so it is offered first:
    // that is how a drawn line becomes the axis a revolution turns around.
    if context.extrusion.is_revolving()
        && response.clicked()
        && let Some(segment) = sketch.nearest_segment(cursor, scale.world_size_of(8.0))
    {
        context.extrusion.axis = cao_core::RevolutionAxis::Segment(segment);
        return;
    }

    let regions = sketch.regions();
    // The innermost area wins: inside a shape drawn within another, the click
    // means the small one, not the one it sits in.
    let Some(under) = regions
        .iter()
        .enumerate()
        .filter(|(_, region)| region.contains(cursor))
        .max_by_key(|(_, region)| region.depth)
        .map(|(index, _)| index)
    else {
        return;
    };
    context.extrusion.hovered = Some(under);

    if !response.clicked() {
        return;
    }
    let already = context
        .extrusion
        .picks
        .iter()
        .position(|pick| regions[under].contains(*pick));
    match already {
        Some(position) => {
            context.extrusion.picks.remove(position);
        }
        None => context.extrusion.picks.push(cursor),
    }
}

/// Moving a point by hand. The drawing settles around it afterwards, so the
/// values already given stay true.
#[allow(clippy::too_many_arguments)]
fn drag_point(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    pressed: DVec2,
    response: &egui::Response,
    snap: f64,
    pixel: f64,
) -> bool {
    let sketch = &context.document.sketches()[index];

    if response.drag_started() {
        // Pressing on something already picked moves the whole selection, the
        // way a desktop moves a group of icons. It comes first: what is held is
        // a deliberate choice, and it would be odd for the drag to take one
        // corner out of it instead.
        context.editor.dragged_group = grabbed_group(context, index, pressed, snap, pixel);
        if !context.editor.dragged_group.is_empty() {
            context.editor.drag_origin = Some(pressed);
            return false;
        }

        // Nothing that is already held in place can be dragged: a value the
        // user typed must not be silently undone by a slip of the mouse. The
        // way to move a settled point is to change what settles it.
        let settled = sketch.settled_points(context.document.scale());

        // A point first, then an annotation: the point is the smaller target
        // and the one a drag is usually after.
        context.editor.dragged_point = sketch
            .nearest_point(pressed, snap)
            .filter(|point| !sketch.is_origin(*point))
            .filter(|point| !settled.get(point.0).copied().unwrap_or(false));

        if context.editor.dragged_point.is_none() {
            context.editor.dragged_dimension =
                nearest_annotation(context, index, pressed, snap * 1.5, pixel);
            context.editor.drag_origin = Some(pressed);
        }
    }

    if !context.editor.dragged_group.is_empty() {
        return drag_group(context, index, cursor, response);
    }
    if let Some(target) = context.editor.dragged_dimension {
        return drag_annotation(context, index, target, cursor, response, pixel);
    }

    let Some(point) = context.editor.dragged_point else {
        return false;
    };

    // While the drag lasts the point is only *shown* at the cursor; the move is
    // recorded once, on release. Recording every frame buried the history under
    // hundreds of entries that all said the same thing.
    //
    // What is shown, though, is the whole drawing settled as if the point had
    // been let go here — the values already given pull the rest of the shape
    // along, and seeing only the point move told nothing of where it was
    // heading.
    if !response.drag_stopped() {
        context.editor.drag_position = Some(cursor);
        let mut settling = context.document.sketches()[index].clone();
        settling.settle_around(point, cursor, context.document.scale());
        context.editor.drag_preview = Some(settling);
        return false;
    }

    context.editor.dragged_point = None;
    context.editor.drag_position = None;
    context.editor.drag_preview = None;
    context.document.apply(Operation::MovePoint {
        sketch: index,
        point,
        position: cursor,
    });

    // Two ends laid on top of each other are one corner, not two. The decision
    // is taken here, at the drop, and recorded: how close is close enough
    // depends on the zoom, so re-deriving it on replay could join a different
    // pair, or none.
    let sketch = &context.document.sketches()[index];
    if let Some(other) = sketch
        .nearest_point(cursor, snap)
        .filter(|other| *other != point)
    {
        context.document.apply(Operation::MergePoints {
            sketch: index,
            kept: other,
            dropped: point,
        });
    }
    true
}

/// The points a drag would carry along, when it starts on something the
/// selection tool is already holding.
///
/// Empty when the press lands anywhere else: a drag beside a selection is a
/// new box, not a move of the old one.
fn grabbed_group(
    context: &SketchContext<'_>,
    index: usize,
    pressed: DVec2,
    snap: f64,
    pixel: f64,
) -> Vec<PointId> {
    let Some(what) = pick(context, index, pressed, snap, pixel) else {
        return Vec::new();
    };
    if !context.editor.is_selected(what) {
        return Vec::new();
    }
    let Some(sketch) = context.document.sketches().get(index) else {
        return Vec::new();
    };

    let mut points: Vec<PointId> = Vec::new();
    let mut take = |point: PointId| {
        if !sketch.is_origin(point) && !points.contains(&point) {
            points.push(point);
        }
    };
    for held in &context.editor.selection {
        let Selection::Element(element) = held else {
            continue;
        };
        match element {
            Element::Point(id) => take(*id),
            Element::Segment(id) => {
                if let Some(segment) = sketch.segments().get(id.0) {
                    take(segment.start);
                    take(segment.end);
                }
            }
            Element::Circle(id) => {
                if let Some(circle) = sketch.circles().get(id.0) {
                    take(circle.center);
                }
            }
        }
    }
    points
}

/// Moving a whole selection at once. Same rule as a point: shown following the
/// cursor, written once on release, as a single entry in the history.
fn drag_group(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    response: &egui::Response,
) -> bool {
    let Some(origin) = context.editor.drag_origin else {
        return false;
    };
    let travelled = cursor - origin;
    let points = context.editor.dragged_group.clone();

    if !response.drag_stopped() {
        context.editor.drag_position = Some(cursor);
        let mut settling = context.document.sketches()[index].clone();
        let dropped: Vec<(PointId, DVec2)> = points
            .iter()
            .filter_map(|point| {
                settling
                    .points()
                    .get(point.0)
                    .map(|place| (*point, *place + travelled))
            })
            .collect();
        settling.settle_around_all(&dropped, context.document.scale());
        context.editor.drag_preview = Some(settling);
        return false;
    }

    context.editor.dragged_group.clear();
    context.editor.drag_origin = None;
    context.editor.drag_position = None;
    context.editor.drag_preview = None;
    if travelled.length() < 1e-9 {
        return false;
    }
    context.document.apply(Operation::MoveMany {
        sketch: index,
        points,
        by: travelled,
    });
    true
}

/// Moving an annotation out of the way. Same rule as a point: shown following
/// the cursor, written once on release.
fn drag_annotation(
    context: &mut SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    cursor: DVec2,
    response: &egui::Response,
    pixel: f64,
) -> bool {
    let Some(origin) = context.editor.drag_origin else {
        return false;
    };
    let travelled = cursor - origin;

    if !response.drag_stopped() {
        context.editor.drag_position = Some(cursor);
        return false;
    }

    // Where the annotation sits right now, whether that was recorded before or
    // is still the standing-off distance it was drawn with.
    let previous = annotation_home(context, index, target, pixel)
        .map(|(_, offset)| offset)
        .unwrap_or_default();

    context.editor.dragged_dimension = None;
    context.editor.drag_origin = None;
    context.editor.drag_position = None;
    context.document.apply(Operation::MoveDimension {
        sketch: index,
        target,
        offset: previous + travelled,
    });
    true
}

/// Which annotation sits under the cursor. Their positions are worked out by
/// the drawing code, so they are asked for rather than guessed.
fn nearest_annotation(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    tolerance: f64,
    pixel: f64,
) -> Option<DimensionTarget> {
    let sketch = context.document.sketches().get(index)?;
    // Only where the annotation lands matters here; its vertices are thrown
    // away, so the theme's colours never come into it. The scale, on the other
    // hand, has to be the real one: an annotation sits a fixed number of pixels
    // off what it measures, so guessing it puts the target somewhere the
    // annotation is not.
    let style = crate::screens::annotations::Style::driving(&Theme::default());
    let mut discarded = Vec::new();

    let anchors: Vec<_> = sketch
        .dimensions()
        .iter()
        .filter_map(|dimension| {
            crate::screens::annotations::push(
                &mut discarded,
                sketch,
                dimension.target,
                &style,
                pixel,
                DVec2::ZERO,
            )
            .map(|placement| (dimension.target, placement.text_at))
        })
        .collect();

    sketch.nearest_dimension(&anchors, cursor, tolerance)
}

/// A point already there, or a new one where the cursor is.
fn point_ref_at(context: &SketchContext<'_>, index: usize, position: DVec2, snap: f64) -> PointRef {
    match context.document.sketches()[index].nearest_point(position, snap) {
        Some(point) => PointRef::Existing(point),
        None => PointRef::New(position),
    }
}

fn two_click_shape(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let Some(start) = context.editor.pending_start else {
        context.editor.pending_start = Some(cursor);
        if context.editor.tool == Tool::Rectangle {
            context.editor.live.open();
        }
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
            rim: Vec::new(),
        },
    };

    let rectangle = context.editor.tool == Tool::Rectangle;
    context.document.apply(operation);
    if rectangle {
        dimension_the_rectangle(context, index, pixel);
    }
    context.editor.live.clear();
    true
}

/// The far corner of the rectangle being drawn, once the sizes typed have had
/// their say. A size left alone follows the cursor; one typed only fixes that
/// side, so the other can still be dragged out.
fn rectangle_corner(context: &SketchContext<'_>, cursor: DVec2) -> DVec2 {
    let Some(start) = context.editor.pending_start else {
        return cursor;
    };
    let scale = context.document.scale().max(1e-9);
    let span = cursor - start;
    // The sign follows the cursor: 40 typed means 40 the way the user is
    // dragging, not 40 the other way.
    let side = |locked: Option<f64>, current: f64| match locked {
        Some(millimeters) => (millimeters / scale).copysign(current),
        None => current,
    };
    start
        + DVec2::new(
            side(context.editor.live.first.locked, span.x),
            side(context.editor.live.second.locked, span.y),
        )
}

/// One click of the circle tool: takes what was pointed at, and draws the
/// circle as soon as enough of it is known.
fn draw_circle(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let mode = context.editor.circle_mode;
    let sketch = &context.document.sketches()[index];

    if mode.touches_traits() && context.editor.circle_segments.len() < mode.wants() - 1 {
        let Some(segment) = sketch.nearest_segment(cursor, snap) else {
            context.editor.message = Some("Cliquez un trait".to_string());
            return false;
        };
        if !context.editor.circle_segments.contains(&segment) {
            context.editor.circle_segments.push(segment);
            context.editor.live.open();
        }
        context.editor.message = Some(mode.asks_for().to_string());
        return false;
    }

    if !mode.touches_traits() && context.editor.circle_points.len() < mode.wants() - 1 {
        context.editor.circle_points.push(cursor);
        context.editor.live.open();
        context.editor.message = Some(mode.asks_for().to_string());
        return false;
    }

    let Some(found) = circle_from(context, index, cursor, snap) else {
        context.editor.message = Some("Ces éléments ne donnent pas de cercle".to_string());
        return false;
    };
    let places = std::mem::take(&mut context.editor.circle_points);
    let mut touched = std::mem::take(&mut context.editor.circle_segments);
    // The last trait of a three-tangent circle is the one under the cursor at
    // the click, and it holds the circle just as much as the other two.
    if mode == CircleMode::ThreeTangents
        && let Some(last) = sketch.nearest_segment(cursor, snap)
        && !touched.contains(&last)
    {
        touched.push(last);
    }

    // The centre reuses a point already drawn when one is under it, as
    // everywhere else, so shapes hang together instead of stacking points.
    let center = point_ref_at(context, index, found.center, snap);
    // The places clicked on the rim stay as points of the drawing, held on the
    // circle: they are what it can afterwards be grabbed and measured by.
    let rim: Vec<DVec2> = match mode {
        CircleMode::Center => vec![cursor],
        CircleMode::TwoPoints => match places.first() {
            Some(first) => vec![*first, found.center * 2.0 - *first],
            None => Vec::new(),
        },
        CircleMode::ThreePoints => places.clone(),
        CircleMode::TwoTangents | CircleMode::ThreeTangents => Vec::new(),
    };
    let rim: Vec<cao_core::PointRef> = rim
        .into_iter()
        .filter(|place| place.distance(found.center) > 1e-6)
        .map(|place| point_ref_at(context, index, place, snap))
        .collect();
    context.document.apply(Operation::AddCircle {
        sketch: index,
        center,
        radius: found.radius,
        rim,
    });
    let drawn = CircleId(context.document.sketches()[index].circles().len().saturating_sub(1));

    // A circle drawn against traits stays against them: the tangency is the
    // whole point of having pointed at them.
    for segment in touched {
        context.document.apply(Operation::Constrain {
            sketch: index,
            constraint: cao_sketch::Constraint::Tangent {
                circle: drawn,
                segment,
                at: None,
            },
        });
    }
    // And a size typed by hand becomes the dimension it deserves.
    if let Some(diameter) = context.editor.live.first.locked {
        let target = DimensionTarget::Diameter(drawn);
        let scale = context.document.scale();
        if !context.document.sketches()[index].would_be_redundant(target, scale) {
            context.document.apply(Operation::SetDimension {
                sketch: index,
                target,
                value: diameter,
                placement: annotation_home(context, index, target, pixel).map(|(_, at)| at),
            });
        }
    }

    context.editor.live.clear();
    context.editor.message = Some(mode.asks_for().to_string());
    true
}

/// The circle the picks so far and the cursor make, if they make one.
///
/// The same reading is used for the preview and for the click, so what is shown
/// is what gets drawn.
fn circle_from(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> Option<Found> {
    let sketch = context.document.sketches().get(index)?;
    let scale = context.document.scale().max(1e-9);
    let wanted = context
        .editor
        .live
        .first
        .locked
        .map(|diameter| diameter / (2.0 * scale));
    let places = &context.editor.circle_points;
    let line = |segment: &SegmentId| sketch.endpoints(*segment);

    let found = match context.editor.circle_mode {
        CircleMode::Center => {
            let center = *places.first()?;
            Found {
                center,
                radius: center.distance(cursor),
            }
        }
        CircleMode::TwoPoints => {
            let first = *places.first()?;
            // A size typed pushes the far point out along the same direction.
            let far = match wanted {
                Some(radius) => first + (cursor - first).normalize_or(DVec2::X) * radius * 2.0,
                None => cursor,
            };
            Found {
                center: (first + far) * 0.5,
                radius: first.distance(far) * 0.5,
            }
        }
        CircleMode::ThreePoints => {
            let (first, second) = (*places.first()?, *places.get(1)?);
            let center = match wanted {
                // A size too small to reach both points is held at the smallest
                // that does. Typing 150 goes through 1 and 15 on the way, and a
                // circle that vanishes at the first keystroke takes the field
                // being typed into with it.
                Some(radius) => cao_sketch::construct::centre_through_at(
                    first,
                    second,
                    cursor,
                    radius.max(first.distance(second) * 0.5),
                )?,
                None => cao_sketch::construct::centre_through(first, second, cursor)?,
            };
            Found {
                center,
                radius: center.distance(first),
            }
        }
        CircleMode::TwoTangents => {
            let mut touching = cao_sketch::construct::centre_touching_two(
                line(context.editor.circle_segments.first()?),
                line(context.editor.circle_segments.get(1)?),
                cursor,
            )?;
            if let Some(radius) = wanted {
                touching = cao_sketch::construct::resize_touching(touching, radius);
            }
            Found {
                center: touching.centre,
                radius: touching.radius,
            }
        }
        CircleMode::ThreeTangents => {
            let (center, radius) = cao_sketch::construct::circle_touching_three(
                line(context.editor.circle_segments.first()?),
                line(context.editor.circle_segments.get(1)?),
                line(&sketch.nearest_segment(cursor, snap)?),
            )?;
            Found { center, radius }
        }
    };
    (found.radius > 1e-9).then_some(found)
}

/// A circle about to be drawn.
#[derive(Clone, Copy)]
struct Found {
    center: DVec2,
    radius: f64,
}

/// Places on a fresh rectangle what makes it a rectangle, and its two sizes.
///
/// Drawing one and then having to say four times that its corners are square is
/// busywork: that is what a rectangle *is*. Three right angles are enough — the
/// fourth follows — plus a length on two neighbouring sides, which is exactly
/// what pins it down.
fn dimension_the_rectangle(context: &mut SketchContext<'_>, index: usize, pixel: f64) {
    let count = context.document.sketches()[index].segments().len();
    let Some(first) = count.checked_sub(4) else {
        return;
    };
    let sides: Vec<SegmentId> = (first..count).map(SegmentId).collect();

    let scale = context.document.scale();
    let mut wanted: Vec<(DimensionTarget, f64)> = Vec::new();
    for corner in 0..3 {
        wanted.push((
            DimensionTarget::Angle {
                first: sides[corner],
                second: sides[corner + 1],
            },
            90.0,
        ));
    }
    for side in sides.iter().take(2) {
        let length = context.document.sketches()[index].segment_length(*side) * scale;
        wanted.push((DimensionTarget::Length(*side), length));
    }

    for (target, value) in wanted {
        let target = target.normalised();
        if context.document.sketches()[index].would_be_redundant(target, scale) {
            continue;
        }
        // Pinned down where it is drawn, in sketch units: left to stand off by
        // a distance in pixels, an annotation slides back over the drawing as
        // soon as one zooms out.
        context.document.apply(Operation::SetDimension {
            sketch: index,
            target,
            value,
            placement: annotation_home(context, index, target, pixel).map(|(_, offset)| offset),
        });
    }
}

/// The smart dimension tool: works out what is under the cursor and measures
/// it, unless a mode is forcing one kind.
///
/// Two-step measurements — point to point, angle — collect their first half and
/// wait; everything else is settled in a single click.
fn measure(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let mode = context.editor.dimension_mode;

    // A dimension already chosen is waiting to be put down. This click either
    // adds the second half of a pair — the segment that makes it an angle, the
    // point that makes it a distance to a line — or says where it goes.
    if let Some(target) = context.editor.placing {
        if mode == DimensionMode::Auto
            && let Some(refined) = refine(context, index, target, cursor, snap)
        {
            context.editor.placing = Some(refined);
            context.editor.message = Some(PLACE_PROMPT.to_string());
            return false;
        }
        context.editor.placing = None;
        return place_dimension(context, index, target, cursor, pixel);
    }

    let sketch = &context.document.sketches()[index];

    // An annotation under the cursor, with no geometry there to take the click
    // first, means changing its value — never laying a second copy over it.
    let on_geometry = sketch.nearest_point(cursor, snap * 0.8).is_some()
        || sketch.nearest_segment(cursor, snap).is_some()
        || sketch.nearest_circle(cursor, snap).is_some();
    if !on_geometry
        && let Some(target) = nearest_annotation(context, index, cursor, snap * 1.5, pixel)
    {
        edit_dimension(context, index, target);
        return false;
    }

    let sketch = &context.document.sketches()[index];

    // A point wins over a segment under the same cursor: it is the smaller
    // target, so aiming at it is the deliberate act.
    if mode.takes_points()
        && let Some(point) = sketch.nearest_point(cursor, snap * 0.8)
    {
        measure_from_point(context, index, point);
        return false;
    }
    // A point already picked, and now a segment: the distance from that point
    // to the line, taken square to it.
    if mode != DimensionMode::PointToPoint
        && let Some(point) = context.editor.first_point
        && let Some(segment) = sketch.nearest_segment(cursor, snap)
    {
        context.editor.first_point = None;
        select_target(
            context,
            index,
            DimensionTarget::PointToSegment { point, segment },
        );
        return false;
    }
    if mode == DimensionMode::PointToPoint {
        return false;
    }

    if matches!(mode, DimensionMode::Auto | DimensionMode::Angle)
        && context.editor.first_angle_segment.is_some()
    {
        continue_angle(context, index, cursor, snap);
        return false;
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
        return false;
    }

    if mode != DimensionMode::Radius
        && let Some(segment) = sketch.nearest_segment(cursor, snap)
    {
        if let Some(axis) = context.editor.first_axis.take() {
            select_target(context, index, DimensionTarget::AxisAngle { segment, axis });
            return false;
        }
        if mode == DimensionMode::Angle {
            context.editor.first_angle_segment = Some(segment);
            context.editor.message =
                Some("Choisissez le second trait, ou un axe de l'esquisse".to_string());
            return false;
        }
        select_target(context, index, DimensionTarget::Length(segment));
        return false;
    }

    if mode != DimensionMode::Length
        && let Some(circle) = sketch.nearest_circle(cursor, snap)
    {
        select_target(context, index, DimensionTarget::Diameter(circle));
        return false;
    }

    context.editor.select(None, None);
    context.editor.message = Some("Rien à mesurer ici".to_string());
    false
}

/// Puts down the dimension that was waiting, where the click landed.
///
/// The value it starts with is what the geometry already measures, so placing
/// one never moves the drawing; typing another is what does.
fn place_dimension(
    context: &mut SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    cursor: DVec2,
    pixel: f64,
) -> bool {
    // Where the cursor is says which of the three readings of a slanted trait
    // is wanted, so it is settled here, at the click that puts the cote down.
    let target = oriented(context, index, target, cursor);

    // The reading asked for is already on the drawing: show its value rather
    // than lay a second copy over it.
    if context.document.sketches()[index]
        .dimension_of(target)
        .is_some()
    {
        edit_dimension(context, index, target);
        return false;
    }

    let Some(value) = context.document.measured(index, target) else {
        return false;
    };
    let outcome = context.document.apply(Operation::SetDimension {
        sketch: index,
        target,
        value,
        placement: Some(annotation_offset(context, index, target, cursor, pixel)),
    });

    context.editor.select(Some(target), Some(value));
    context.editor.message = matches!(outcome, Some(cao_core::DimensionOutcome::Reference))
        .then(|| REDUNDANT_WARNING.to_string());
    true
}

/// Where an annotation currently writes its value, and the offset that holds it
/// there. Asked of the drawing code itself: two ways of working out where an
/// annotation sits would eventually disagree.
fn annotation_home(
    context: &SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    pixel: f64,
) -> Option<(DVec2, DVec2)> {
    let sketch = context.document.sketches().get(index)?;
    let mut ignored = Vec::new();
    // Only the shape of the annotation matters here, never its colours.
    let style = crate::screens::annotations::Style::driving(&Theme::default());
    let placement = crate::screens::annotations::push(
        &mut ignored,
        sketch,
        target,
        &style,
        pixel,
        DVec2::ZERO,
    )?;
    Some((placement.text_at, placement.offset))
}

/// What the annotation has to be moved by for its value to land on the cursor,
/// and the offset that would record that.
///
/// A linear or angular annotation follows its offset exactly, so the gap
/// between where the value is and where the cursor is *is* the movement.
fn annotation_offset(
    context: &SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    cursor: DVec2,
    pixel: f64,
) -> DVec2 {
    match annotation_home(context, index, target, pixel) {
        Some((text_at, offset)) => offset + (cursor - text_at),
        None => DVec2::ZERO,
    }
}

/// The dimension a click would place right now, without placing it.
///
/// The same reading of the cursor as `measure`, so what is shown in advance is
/// what will actually be recorded — two separate readings would eventually
/// disagree, and a preview that lies is worse than none.
fn measure_preview(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> Option<DimensionTarget> {
    let mode = context.editor.dimension_mode;
    let sketch = context.document.sketches().get(index)?;

    if mode.takes_points()
        && let Some(point) = sketch.nearest_point(cursor, snap * 0.8)
    {
        let first = context.editor.first_point?;
        return (first != point).then_some(DimensionTarget::Distance { from: first, to: point });
    }
    if mode == DimensionMode::PointToPoint {
        return None;
    }

    if matches!(mode, DimensionMode::Auto | DimensionMode::Angle)
        && let Some(first) = context.editor.first_angle_segment
    {
        if let Some(second) = sketch.nearest_segment(cursor, snap)
            && second != first
        {
            return sketch
                .angle_between(first, second)
                .map(|_| DimensionTarget::Angle { first, second });
        }
        return axis_under(cursor, snap).map(|axis| DimensionTarget::AxisAngle {
            segment: first,
            axis,
        });
    }

    if mode != DimensionMode::Radius
        && let Some(segment) = sketch.nearest_segment(cursor, snap)
    {
        return match (context.editor.first_axis, mode) {
            (Some(axis), _) => Some(DimensionTarget::AxisAngle { segment, axis }),
            // The angle mode is still waiting for its partner, so there is
            // nothing whole to show yet.
            (None, DimensionMode::Angle) => None,
            (None, _) => Some(DimensionTarget::Length(segment)),
        };
    }

    if mode != DimensionMode::Length {
        return sketch
            .nearest_circle(cursor, snap)
            .map(DimensionTarget::Diameter);
    }
    None
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
fn continue_angle(context: &mut SketchContext<'_>, index: usize, cursor: DVec2, snap: f64) {
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

pub const PLACE_PROMPT: &str = "Cliquez où poser la cote, ou une seconde entité";

/// Takes hold of what was clicked; the annotation then follows the cursor until
/// a second click says where it goes.
///
/// Two clicks rather than one because a dimension dropped on top of the shape
/// it measures has to be dragged off it anyway — this way it lands where it
/// belongs from the start.
fn select_target(context: &mut SketchContext<'_>, index: usize, target: DimensionTarget) {
    let target = target.normalised();

    // The same measurement clicked again is the one already there: showing its
    // value to be retyped is what the user is after, not a second copy of it
    // laid over the first. A slanted trait is the exception — it has a width
    // and a height to offer besides its length, and which one is wanted is only
    // known once the cote is placed.
    if !is_slanted(&context.document.sketches()[index], target)
        && context.document.sketches()[index]
            .dimension_of(target)
            .is_some()
    {
        return edit_dimension(context, index, target);
    }

    context.editor.select(None, None);
    context.editor.placing = Some(target);

    let scale = context.document.scale();
    context.editor.message = Some(
        if context.document.sketches()[index].would_be_redundant(target, scale) {
            REDUNDANT_WARNING.to_string()
        } else {
            PLACE_PROMPT.to_string()
        },
    );
}

/// Opens a dimension already on the drawing for editing, its value in the field
/// ready to be replaced.
fn edit_dimension(context: &mut SketchContext<'_>, index: usize, target: DimensionTarget) {
    let value = context.document.sketches()[index]
        .dimension_of(target)
        .map(|dimension| dimension.value)
        .or_else(|| context.document.measured(index, target));
    context.editor.placing = None;
    context.editor.first_point = None;
    context.editor.first_angle_segment = None;
    context.editor.first_axis = None;
    context.editor.select(Some(target), value);
    context.editor.message = None;
}

/// How far off an axis a trait has to be before its width and its height are
/// worth offering.
///
/// Only a trait sitting square on an axis is left out: its width *is* its
/// length, and two names for one measurement is one too many. Everything else,
/// however slightly leaning, gets the choice.
const SLANT_DEGREES: f64 = 0.5;

/// Which of the three readings of a slanted trait the cursor is asking for.
///
/// The two ends box off the plane: above or below that box the cursor asks for
/// the width, left or right of it the height, and inside it — the triangle the
/// trait closes — or out past a corner, the length itself.
fn oriented(
    context: &SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    cursor: DVec2,
) -> DimensionTarget {
    let sketch = match context.document.sketches().get(index) {
        Some(sketch) => sketch,
        None => return target,
    };
    let Some((from, to)) = ends_of(sketch, target).filter(|_| is_slanted(sketch, target)) else {
        return target;
    };
    let (Some(a), Some(b)) = (
        sketch.points().get(from.0).copied(),
        sketch.points().get(to.0).copied(),
    ) else {
        return target;
    };
    let (low, high) = (a.min(b), a.max(b));
    let within_x = (low.x..=high.x).contains(&cursor.x);
    let within_y = (low.y..=high.y).contains(&cursor.y);
    let axis = match (within_x, within_y) {
        (true, false) => cao_sketch::SketchAxis::U,
        (false, true) => cao_sketch::SketchAxis::V,
        _ => return target,
    };
    DimensionTarget::Projected { from, to, axis }.normalised()
}

/// Whether a trait leans far enough off both axes for its width and its height
/// to be worth offering beside its length.
fn is_slanted(sketch: &Sketch, target: DimensionTarget) -> bool {
    let Some((from, to)) = ends_of(sketch, target) else {
        return false;
    };
    let (Some(start), Some(end)) = (sketch.points().get(from.0), sketch.points().get(to.0)) else {
        return false;
    };
    let span = *end - *start;
    let slant = span.y.atan2(span.x).to_degrees().abs();
    slant.min((slant - 90.0).abs()).min((slant - 180.0).abs()) >= SLANT_DEGREES
}

/// The two ends of what a linear dimension measures, when it has two.
fn ends_of(sketch: &Sketch, target: DimensionTarget) -> Option<(PointId, PointId)> {
    match target {
        DimensionTarget::Length(segment) => {
            let segment = sketch.segments().get(segment.0)?;
            Some((segment.start, segment.end))
        }
        DimensionTarget::Distance { from, to } | DimensionTarget::Projected { from, to, .. } => {
            Some((from, to))
        }
        _ => None,
    }
}

/// The second half a dimension in hand can still take: another segment makes it
/// an angle, a point makes it a distance to a line, an axis a direction.
///
/// Without this, clicking a segment could only ever mean its length, and an
/// angle between two traits had to be asked for through the Mesurer row — which
/// is exactly what one expects the smart dimension to do on its own.
fn refine(
    context: &SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    cursor: DVec2,
    snap: f64,
) -> Option<DimensionTarget> {
    let sketch = context.document.sketches().get(index)?;

    // A diameter taken back to the centre is a radius: it is the one thing the
    // centre can add to a circle already picked.
    if let DimensionTarget::Diameter(circle) = target {
        let center = sketch.circles().get(circle.0)?.center;
        return sketch
            .nearest_point(cursor, snap * 0.8)
            .filter(|point| *point == center)
            .map(|_| DimensionTarget::Radius(circle));
    }

    let DimensionTarget::Length(first) = target else {
        return None;
    };

    if let Some(second) = sketch.nearest_segment(cursor, snap)
        && second != first
        && sketch.angle_between(first, second).is_some()
    {
        return Some(DimensionTarget::Angle { first, second }.normalised());
    }
    if let Some(point) = sketch.nearest_point(cursor, snap * 0.8)
        && !touches(sketch, first, point)
    {
        return Some(DimensionTarget::PointToSegment {
            point,
            segment: first,
        });
    }
    if sketch.nearest_segment(cursor, snap).is_none()
        && let Some(axis) = axis_under(cursor, snap)
    {
        return Some(DimensionTarget::AxisAngle {
            segment: first,
            axis,
        });
    }
    None
}

/// Whether a point is one of a segment's own ends — measuring a segment to its
/// own corner would be a distance of nothing.
fn touches(sketch: &Sketch, segment: SegmentId, point: PointId) -> bool {
    match sketch.segments().get(segment.0) {
        Some(segment) => segment.start == point || segment.end == point,
        None => false,
    }
}

/// Which sketch axis the cursor is on, if either. The axes are drawn as lines
/// through the origin, so they are picked the same way a segment is.
fn axis_under(cursor: DVec2, tolerance: f64) -> Option<cao_sketch::SketchAxis> {
    if cursor.y.abs() <= tolerance {
        return Some(cao_sketch::SketchAxis::U);
    }
    if cursor.x.abs() <= tolerance {
        return Some(cao_sketch::SketchAxis::V);
    }
    None
}

/// Shown when a value would add nothing to a shape that is already settled.
pub const REDUNDANT_WARNING: &str =
    "Cette cote n'apporte rien : ce qu'elle mesure est déjà tenu. Elle sera posée en simple lecture.";

/// Pulls the cursor onto whatever it is near: an existing point first, then the
/// grid.
///
/// The grid magnet is what makes drawing on the origin, or a right angle by
/// following the lines, a matter of aiming roughly rather than exactly. It only
/// bites within a few pixels, so a deliberate free position is still possible.
fn magnetise(
    cursor: DVec2,
    scale: ViewScale,
    config: &ViewportConfig,
    context: &SketchContext<'_>,
    index: usize,
    snap: f64,
) -> (DVec2, Option<Snap>) {
    let sketch = &context.document.sketches()[index];
    if let Some(point) = sketch.nearest_point(cursor, snap) {
        return (sketch.point(point), Some(Snap::Point));
    }

    // A line already drawn pulls harder than the grid, and its middle harder
    // still: joining the middle of a side is a thing one aims at, and landing a
    // hair off it leaves geometry that only looks joined.
    let reach = scale.world_size_of(config.segment_snap_pixels as f64);
    if let Some((_, middle)) = sketch.nearest_midpoint(cursor, reach) {
        return (middle, Some(Snap::Midpoint(middle)));
    }
    if let Some((_, at)) = sketch.nearest_on_segment(cursor, reach) {
        return (at, Some(Snap::OnSegment(at)));
    }

    if !config.grid_snap {
        return (cursor, None);
    }

    // The grid is drawn every `step`; snapping to a fraction of it keeps the
    // magnet useful without forcing everything onto the coarse lines.
    let step = scale.step / config.grid_snap_divisions.max(1) as f64;
    if step <= 0.0 {
        return (cursor, None);
    }
    let snapped = DVec2::new(
        (cursor.x / step).round() * step,
        (cursor.y / step).round() * step,
    );
    if snapped.distance(cursor) <= scale.world_size_of(config.grid_snap_pixels as f64) {
        (snapped, None)
    } else {
        (cursor, None)
    }
}

/// What the cursor has been pulled onto, when it is worth saying so.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Snap {
    Point,
    /// The middle of a line, which needs a mark of its own: nothing else on
    /// screen says the cursor is exactly halfway along.
    Midpoint(DVec2),
    OnSegment(DVec2),
}

/// One click of the line tool. The first click only remembers where the chain
/// starts; the second turns the pair into a segment in the history.
fn draw_line_point(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let sketch = &context.document.sketches()[index];

    let Some(anchor) = context.editor.chain else {
        let end = match sketch.nearest_point(cursor, snap) {
            Some(id) => PointRef::Existing(id),
            None => PointRef::New(cursor),
        };
        context.editor.chain = Some(match end {
            PointRef::Existing(id) => ChainAnchor::Point(id),
            PointRef::New(position) => ChainAnchor::Pending(position),
        });
        context.editor.live.open();
        return false;
    };

    // Where the line actually ends: what the user typed wins over where the
    // cursor is, and a right angle is snapped to before anything is recorded.
    let aimed = aim(context, index, cursor);
    let sketch = &context.document.sketches()[index];
    let end = match sketch.nearest_point(aimed.position, snap) {
        // A value typed is a decision; joining a point that happens to be near
        // would quietly give the line another length.
        Some(id) if !context.editor.live.is_locked() => PointRef::Existing(id),
        _ => PointRef::New(aimed.position),
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
    let drawn = SegmentId(sketch.segments().len().saturating_sub(1));
    context.editor.chain = Some(ChainAnchor::Point(match end {
        PointRef::Existing(id) => id,
        PointRef::New(_) => PointId(sketch.points().len().saturating_sub(1)),
    }));

    dimension_the_line(context, index, drawn, aimed, pixel);
    context.editor.chain_previous = Some(drawn);
    context.editor.live.open();
    true
}

/// Where the line being drawn actually ends, and what that implies.
#[derive(Clone, Copy)]
struct Aim {
    position: DVec2,
    /// The segment this one has just been squared up against.
    square_with: Option<SegmentId>,
}

/// Applies to the cursor everything the user has already decided.
///
/// A locked angle leaves the line free to lengthen along that direction; a
/// locked length leaves it free to turn at that distance; both leave nothing to
/// the cursor at all. That is the point of locking one and not the other.
fn aim(context: &SketchContext<'_>, index: usize, cursor: DVec2) -> Aim {
    let nowhere = Aim {
        position: cursor,
        square_with: None,
    };
    let Some(sketch) = context.document.sketches().get(index) else {
        return nowhere;
    };
    let Some(from) = anchor_position(sketch, context) else {
        return nowhere;
    };

    let scale = context.document.scale().max(1e-9);
    let (locked_length, locked_angle) = (
        context.editor.live.first.locked,
        context.editor.live.second.locked,
    );
    let span = cursor - from;

    let mut direction = span.normalize_or(DVec2::X);
    let mut square_with = None;

    if let Some(degrees) = locked_angle {
        // The sign follows the cursor: 30° typed means the 30° the user is
        // pointing at, not the one below the axis they are not.
        let wanted = DVec2::from_angle(degrees.to_radians());
        direction = if wanted.dot(direction) >= 0.0 {
            wanted
        } else {
            -wanted
        };
    } else if let Some((perpendicular, previous)) = right_angle(sketch, context, from, span) {
        direction = perpendicular;
        square_with = Some(previous);
    }

    let length = match locked_length {
        Some(millimeters) => millimeters / scale,
        None => span.dot(direction).max(0.0),
    };

    Aim {
        position: from + direction * length.max(1e-6),
        square_with,
    }
}

/// Half the width of the band, in degrees, inside which a corner is taken as
/// square. Wide enough to be easy to hit, narrow enough that an angle really
/// meant to be 80° is not stolen.
const SQUARE_TOLERANCE_DEGREES: f64 = 4.0;

/// The direction that squares this line up against the one before it, when the
/// cursor is close enough to it.
fn right_angle(
    sketch: &Sketch,
    context: &SketchContext<'_>,
    from: DVec2,
    span: DVec2,
) -> Option<(DVec2, SegmentId)> {
    let previous = context.editor.chain_previous?;
    if previous.0 >= sketch.segments().len() || sketch.is_erased_segment(previous) {
        return None;
    }
    let (start, end) = sketch.endpoints(previous);
    // The arm runs from the shared corner outwards, whichever way it was drawn.
    let arm = if start.distance(from) < end.distance(from) {
        end - start
    } else {
        start - end
    }
    .normalize_or_zero();
    let direction = span.normalize_or_zero();
    if arm == DVec2::ZERO || direction == DVec2::ZERO {
        return None;
    }

    let off_square = direction.dot(arm).abs().asin().to_degrees();
    if off_square > SQUARE_TOLERANCE_DEGREES {
        return None;
    }
    let square = DVec2::new(-arm.y, arm.x);
    let towards = if square.dot(direction) >= 0.0 {
        square
    } else {
        -square
    };
    Some((towards, previous))
}

fn anchor_position(sketch: &Sketch, context: &SketchContext<'_>) -> Option<DVec2> {
    match context.editor.chain? {
        ChainAnchor::Pending(position) => Some(position),
        ChainAnchor::Point(id) => (id.0 < sketch.points().len()).then(|| sketch.point(id)),
    }
}

/// Places on the line just drawn whatever the user typed, and the right angle
/// they aimed at.
///
/// A value that would say nothing is left out: the drawing already holds it,
/// and a second copy could only be redundant.
fn dimension_the_line(
    context: &mut SketchContext<'_>,
    index: usize,
    segment: SegmentId,
    aimed: Aim,
    pixel: f64,
) {
    let live = &context.editor.live;
    let mut wanted: Vec<(DimensionTarget, f64)> = Vec::new();

    if let Some(length) = live.first.locked {
        wanted.push((DimensionTarget::Length(segment), length));
    }
    if let Some(angle) = live.second.locked {
        wanted.push((
            DimensionTarget::AxisAngle {
                segment,
                axis: cao_sketch::SketchAxis::U,
            },
            angle.abs(),
        ));
    }
    if let Some(first) = aimed.square_with {
        wanted.push((DimensionTarget::Angle { first, second: segment }, 90.0));
    }

    let scale = context.document.scale();
    for (target, value) in wanted {
        let target = target.normalised();
        if context.document.sketches()[index].would_be_redundant(target, scale) {
            continue;
        }
        // Pinned down where it is drawn, in sketch units: left to stand off by
        // a distance in pixels, an annotation slides back over the drawing as
        // soon as one zooms out.
        context.document.apply(Operation::SetDimension {
            sketch: index,
            target,
            value,
            placement: annotation_home(context, index, target, pixel).map(|(_, offset)| offset),
        });
    }
}

/// How close, in pixels, the cursor has to be to take hold of something.
///
/// These are physical pixels, so a high-density screen halves them: at ten, a
/// point had to be hit within four points of the mouse, which is a good deal
/// finer than anyone aims.
const PICK_PIXELS: f64 = 18.0;

/// How much of the plane to show when a sketch has no geometry to frame yet.
pub const DEFAULT_SKETCH_RADIUS: f64 = 100.0;

/// Work planes are drawn a fixed fraction of the view across, so they stay
/// clickable however far the camera is.
fn plane_half_size(state: &ViewportState) -> f64 {
    state.camera.distance() as f64 * 0.3
}

/// Where a ray crosses a plane patch, in plane coordinates, if it lands inside
/// the square actually drawn.
fn plane_hit(plane: &WorkPlane, origin: DVec3, direction: DVec3, half_size: f64) -> Option<DVec2> {
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
fn to_ndc(position: egui::Pos2, rect: egui::Rect) -> glam::Vec2 {
    glam::Vec2::new(
        (position.x - rect.center().x) / (rect.width() * 0.5),
        (rect.center().y - position.y) / (rect.height() * 0.5),
    )
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

    let theme = &state.theme;
    let mut lines = Vec::new();
    let mut world_lines = Vec::new();
    let mut surfaces = Vec::new();

    let mut background = Vec::new();
    let (shape, sample) = background_shape(&theme.background);
    cao_render::push_background(&mut background, shape, sample);

    // The grid is only drawn once the view has actually landed on the plane.
    // Mid-animation the view is oblique, and a grid of finite size seen at an
    // angle reads as a disc floating in the middle of the screen.
    if let (ViewMode::Plane(plane), None) = (state.mode, &state.transition) {
        // It also has to reach past the corners of the screen, or its outer
        // fade shows up as that same disc.
        let half_extent = (scale.units_per_pixel * scale.diagonal_px as f64 * 1.5) as f32;
        let normal = plane.normal().as_vec3();
        let origin = plane.origin.as_vec3();
        let center = camera.target() - normal * (camera.target() - origin).dot(normal);
        push_grid(
            &mut world_lines,
            plane.u.as_vec3(),
            plane.v.as_vec3(),
            center,
            scale.step as f32,
            half_extent,
            &GridStyle {
                minor: tint(theme.grid_minor),
                major: tint(theme.grid_major),
                minor_width: theme.grid_minor_width,
                major_width: theme.grid_major_width,
                major_every: theme.grid_major_every.max(1),
                ..GridStyle::default()
            },
        );
    }

    let facing = match state.mode {
        ViewMode::Plane(plane) => Some(plane.normal().as_vec3()),
        ViewMode::Free => None,
    };
    push_axes(
        &mut world_lines,
        camera.distance() * 50.0,
        &AxisStyle {
            x: tint(theme.axis_x),
            y: tint(theme.axis_y),
            z: tint(theme.axis_z),
            width: theme.axis_width,
        },
        facing,
    );

    if context.editor.is_choosing_plane() {
        push_choosable_planes(&mut surfaces, &mut lines, state, context);
    }

    for (index, sketch) in context.document.sketches().iter().enumerate() {
        let active = context.editor.active_sketch() == Some(index);
        // Mid-drag, the settled preview stands in for the recorded drawing.
        let shown = match (active, &context.editor.drag_preview) {
            (true, Some(preview)) => preview,
            _ => sketch,
        };
        push_sketch(&mut lines, &mut surfaces, shown, theme, scale, active, context);
    }

    push_chosen_areas(&mut surfaces, &mut lines, theme, context);

    let mut solids = Vec::new();
    cao_render::push_solid(
        &mut solids,
        &context
            .document
            .body()
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

fn push_choosable_planes(
    surfaces: &mut Vec<cao_render::Vertex>,
    lines: &mut Vec<cao_render::Vertex>,
    state: &ViewportState,
    context: &SketchContext<'_>,
) {
    let theme = &state.theme;
    let half_size = plane_half_size(state);
    // Once there is a part, the three planes step back: they are still there to
    // be picked, but they no longer hide the faces one usually wants.
    let has_body = !context.document.body().is_empty();
    let faded = if has_body { 0.35 } else { 1.0 };

    if let Some(PlaneChoice::Face(plane)) = context.editor.hovered_plane {
        push_hovered_face(surfaces, theme, context, plane);
    }

    for (index, plane) in WorkPlane::ORIGIN_PLANES.iter().enumerate() {
        let hovered = context.editor.hovered_plane == Some(PlaneChoice::Origin(index));
        let fill = if hovered {
            tint(theme.highlight)
        } else {
            tint_at(theme.sketch_inactive, 0.12 * faded)
        };
        let outline = if hovered {
            tint_at(theme.highlight, 1.0)
        } else {
            tint_at(theme.sketch_inactive, 0.7 * faded)
        };
        push_plane_quad(
            surfaces,
            plane.origin.as_vec3(),
            plane.u.as_vec3(),
            plane.v.as_vec3(),
            half_size as f32,
            fill,
        );
        push_plane_outline(
            lines,
            plane.origin.as_vec3(),
            plane.u.as_vec3(),
            plane.v.as_vec3(),
            half_size as f32,
            outline,
            if hovered { 2.5 } else { 1.5 },
        );
    }
}

/// Marks the areas picked for an extrusion, and the one under the cursor.
///
/// A chosen area is filled with the colour of the matter it is about to become,
/// which is the only preview needed before the height is typed.
fn push_chosen_areas(
    surfaces: &mut Vec<cao_render::Vertex>,
    lines: &mut Vec<cao_render::Vertex>,
    theme: &Theme,
    context: &SketchContext<'_>,
) {
    if !context.extrusion.is_active() {
        return;
    }
    let Some(sketch) = context
        .extrusion
        .sketch
        .and_then(|index| context.document.sketches().get(index))
    else {
        return;
    };

    let cutting = context.extrusion.mode == Some(cao_core::ExtrusionMode::Cut);
    let chosen = if cutting {
        tint(theme.extrusion_cut)
    } else {
        tint(theme.extrusion_add)
    };

    if context.extrusion.is_revolving() {
        push_revolution_axis(lines, sketch, theme, context);
    }

    for (index, region) in sketch.regions().iter().enumerate() {
        let picked = context
            .extrusion
            .picks
            .iter()
            .any(|pick| region.contains(*pick));
        let hovered = context.extrusion.hovered == Some(index);
        if !picked && !hovered {
            continue;
        }
        let color = if picked {
            chosen
        } else {
            tint_at(theme.highlight, theme.highlight.a * 0.5)
        };

        // Holes stay empty here too: what is shown filled is exactly what will
        // become matter.
        for [a, b, c] in region.face_triangles() {
            for corner in [a, b, c] {
                surfaces.push(cao_render::Vertex::solid(
                    sketch.plane.to_world(corner).as_vec3(),
                    color,
                ));
            }
        }
    }
}

/// Draws the axis a revolution turns around, well past the drawing so it reads
/// as an axis rather than as one more line of the sketch.
fn push_revolution_axis(
    lines: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    context: &SketchContext<'_>,
) {
    let (origin, direction) = match context.extrusion.axis {
        cao_core::RevolutionAxis::Sketch(axis) => (DVec2::ZERO, axis.direction()),
        cao_core::RevolutionAxis::Segment(segment) => {
            if segment.0 >= sketch.segments().len() {
                return;
            }
            let (start, end) = sketch.endpoints(segment);
            (start, (end - start).normalize_or(DVec2::X))
        }
    };

    let reach = sketch
        .bounds()
        .map(|(min, max)| (max - min).length())
        .unwrap_or(1.0)
        .max(1.0);
    let color = tint_at(theme.sketch_free, 0.9);
    lines.push(cao_render::Vertex::line(
        sketch.plane.to_world(origin - direction * reach).as_vec3(),
        color,
        2.0,
    ));
    lines.push(cao_render::Vertex::line(
        sketch.plane.to_world(origin + direction * reach).as_vec3(),
        color,
        2.0,
    ));
}

/// Tints the areas the drawing encloses, so a closed contour reads as a face
/// rather than four separate lines.
///
/// A shape drawn inside another is tinted more heavily: without that, an
/// outline and the pocket in it wash into one another and the eye cannot tell
/// which is which.
fn push_regions(
    surfaces: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    active: bool,
) {
    for region in sketch.regions() {
        // Deeper areas take more of the tint, which is what tells a shape
        // drawn inside another from the one it sits in.
        let shade = theme.region_fill.a * (1.0 + 0.6 * region.depth.min(4) as f32);
        let color = if active {
            tint_at(theme.region_fill, shade)
        } else {
            tint_at(theme.sketch_inactive, shade * 0.6)
        };
        for [a, b, c] in region.triangles {
            for corner in [a, b, c] {
                surfaces.push(cao_render::Vertex::solid(sketch.plane.to_world(corner).as_vec3(), color));
            }
        }
    }
}

/// Lights up the face of the part under the cursor, so it is clear what a click
/// would sketch on.
fn push_hovered_face(
    surfaces: &mut Vec<cao_render::Vertex>,
    theme: &Theme,
    context: &SketchContext<'_>,
    plane: WorkPlane,
) {
    let normal = plane.normal();
    let offset = plane.origin.dot(normal);
    let fill = tint(theme.highlight);

    // Every face lying on the same plane lights up together: a curved surface
    // and a cut one are both stored as many flat pieces, and lighting only the
    // piece under the cursor would read as picking a fragment of it. Only the
    // fill is drawn — outlining each piece would show the seams between them,
    // which are not something the user drew.
    for polygon in &context.document.body().polygons {
        if polygon.normal().dot(normal) < 0.999 || (polygon.plane_offset() - offset).abs() > 1e-4 {
            continue;
        }
        for [a, b, c] in polygon.triangles() {
            for corner in [a, b, c] {
                surfaces.push(cao_render::Vertex::solid(corner.as_vec3(), fill));
            }
        }
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

/// Colours saying how settled the drawing is: a shape that still has freedom
/// left is drawn one way, one that is fully determined another. It is the
/// quickest possible answer to "is my part pinned down yet?".
fn sketch_colors(theme: &Theme, active: bool, constrained: bool) -> ([f32; 4], f32) {
    match (active, constrained) {
        (true, false) => (tint(theme.sketch_free), theme.sketch_width),
        (true, true) => (tint(theme.sketch_settled), theme.sketch_width),
        (false, _) => (tint(theme.sketch_inactive), theme.sketch_width * 0.6),
    }
}

/// Where a point is shown: at the cursor while it is being dragged, at its
/// recorded place otherwise.
fn shown_position(sketch: &Sketch, point: PointId, context: &SketchContext<'_>) -> DVec2 {
    // The settled preview already has the point where the cursor put it, and
    // everything else where it followed; moving it again would put it twice.
    if context.editor.drag_preview.is_some() {
        return sketch.point(point);
    }
    match (context.editor.dragged_point, context.editor.drag_position) {
        (Some(dragged), Some(position)) if dragged == point && !sketch.is_origin(point) => position,
        _ => sketch.point(point),
    }
}

#[allow(clippy::too_many_arguments)]
fn push_sketch(
    out: &mut Vec<cao_render::Vertex>,
    surfaces: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    scale: ViewScale,
    active: bool,
    context: &SketchContext<'_>,
) {
    push_regions(surfaces, sketch, theme, active);

    // Per element, not one verdict for the whole drawing: a contour can be
    // nailed down while its neighbour is still floating, and that is exactly
    // what tells the user what is left to do.
    let settled = sketch.settled_points(context.document.scale());
    let holds = |point: PointId| settled.get(point.0).copied().unwrap_or(false);

    for (id, segment) in sketch.live_segments() {
        let held = holds(segment.start) && holds(segment.end);
        let (color, width) = match sketch.is_held(Element::Segment(id)) {
            true => (tint(theme.fixed), theme.sketch_width),
            false => sketch_colors(theme, active, held),
        };
        let (color, width) = mark_selected(
            context,
            theme,
            Selection::Element(Element::Segment(id)),
            color,
            width,
        );
        let start = sketch
            .plane
            .to_world(shown_position(sketch, segment.start, context));
        let end = sketch
            .plane
            .to_world(shown_position(sketch, segment.end, context));
        out.push(cao_render::Vertex::line(start.as_vec3(), color, width));
        out.push(cao_render::Vertex::line(end.as_vec3(), color, width));
    }

    for (id, circle) in sketch.live_circles() {
        let (color, width) = match sketch.is_held(Element::Circle(id)) {
            true => (tint(theme.fixed), theme.sketch_width),
            false => sketch_colors(theme, active, holds(circle.center)),
        };
        let (color, width) = mark_selected(
            context,
            theme,
            Selection::Element(Element::Circle(id)),
            color,
            width,
        );
        push_circle(out, sketch, circle.center, circle.radius, color, width);
    }

    if !active {
        return;
    }

    push_point_markers(out, sketch, theme, scale, &settled, context);

    // Every dimension is drawn where it applies, with extension lines, arrows
    // and arcs, so the drawing says what holds it rather than just carrying a
    // number.
    for dimension in sketch.dimensions() {
        let mut style = if dimension.driven {
            crate::screens::annotations::Style::driven(theme)
        } else {
            crate::screens::annotations::Style::driving(theme)
        };
        if context.editor.is_selected(Selection::Dimension(dimension.target)) {
            style.color = tint_at(theme.highlight, 1.0);
            style.width *= 2.0;
        }
        crate::screens::annotations::push(
            out,
            sketch,
            dimension.target,
            &style,
            scale.units_per_pixel,
            live_offset(context, dimension.target),
        );
    }

    push_preview(out, sketch, theme, scale, context);

    if let Some(cao_sketch::DimensionTarget::Length(selected)) = context.editor.selected
        && selected.0 < sketch.segments().len()
    {
        let (start, end) = sketch.endpoints(selected);
        let highlight = tint_at(theme.highlight, 1.0);
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(start).as_vec3(),
            highlight,
            4.0,
        ));
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(end).as_vec3(),
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
    theme: &Theme,
    scale: ViewScale,
    settled: &[bool],
    context: &SketchContext<'_>,
) {
    let half = scale.world_size_of(4.0);
    let highlight = tint_at(theme.highlight, 1.0);

    for index in 0..sketch.points().len() {
        let point = PointId(index);
        if sketch.is_erased_point(point) {
            continue;
        }
        let (base, _) = match sketch.is_held(Element::Point(point)) {
            true => (tint(theme.fixed), theme.sketch_width),
            false => sketch_colors(theme, true, settled.get(index).copied().unwrap_or(false)),
        };
        let center = sketch
            .plane
            .to_world(shown_position(sketch, point, context));
        let hovered = context.editor.hovered_point == Some(point)
            || context.editor.first_point == Some(point);
        let color = if hovered { highlight } else { base };
        let width = if hovered { 2.5 } else { 1.5 };
        let (color, width) = mark_selected(
            context,
            theme,
            Selection::Element(Element::Point(point)),
            color,
            width,
        );

        let (u, v) = if sketch.is_origin(point) {
            // Turned a quarter: a diamond reads differently at a glance.
            (
                (sketch.plane.u + sketch.plane.v) * std::f64::consts::FRAC_1_SQRT_2,
                (sketch.plane.v - sketch.plane.u) * std::f64::consts::FRAC_1_SQRT_2,
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
            out.push(cao_render::Vertex::line(corners[corner].as_vec3(), color, width));
            out.push(cao_render::Vertex::line(
                corners[(corner + 1) % 4].as_vec3(),
                color,
                width,
            ));
        }
    }
}

/// How far an annotation has been dragged so far, before anything is recorded.
///
/// Nothing goes into the history until the button is let go, so without this
/// the annotation would sit still through the whole gesture and only jump at
/// the end — the drag would look like it had done nothing.
fn live_offset(context: &SketchContext<'_>, target: DimensionTarget) -> DVec2 {
    let editor = &context.editor;
    match (editor.dragged_dimension, editor.drag_origin, editor.drag_position) {
        (Some(dragged), Some(origin), Some(position)) if dragged == target => position - origin,
        _ => DVec2::ZERO,
    }
}

/// Draws what the selection tool is holding differently, so it is clear what
/// pressing Suppr would take away.
fn mark_selected(
    context: &SketchContext<'_>,
    theme: &Theme,
    element: Selection,
    color: [f32; 4],
    width: f32,
) -> ([f32; 4], f32) {
    if context.editor.is_selected(element) {
        (tint_at(theme.highlight, 1.0), width * 1.8)
    } else {
        (color, width)
    }
}

/// A small square outline on the plane, which is what a point looks like.
fn push_point_marker(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    at: DVec2,
    size: f64,
    color: [f32; 4],
    width: f32,
) {
    let center = sketch.plane.to_world(at);
    let (u, v) = (sketch.plane.u, sketch.plane.v);
    let corners = [
        center - u * size - v * size,
        center + u * size - v * size,
        center + u * size + v * size,
        center - u * size + v * size,
    ];
    for corner in 0..4 {
        out.push(cao_render::Vertex::line(corners[corner].as_vec3(), color, width));
        out.push(cao_render::Vertex::line(
            corners[(corner + 1) % 4].as_vec3(),
            color,
            width,
        ));
    }
}

/// The mark for the middle of a line: a small triangle, as on a drawing.
///
/// A shape of its own rather than the square used for a point — the two mean
/// different things and must not be told apart by size alone.
fn push_midpoint_mark(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    at: DVec2,
    scale: ViewScale,
    color: [f32; 4],
) {
    let size = scale.world_size_of(6.0);
    let corners = [
        at + DVec2::new(0.0, size),
        at + DVec2::new(-size, -size * 0.7),
        at + DVec2::new(size, -size * 0.7),
    ];
    for index in 0..3 {
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(corners[index]).as_vec3(),
            color,
            2.0,
        ));
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(corners[(index + 1) % 3]).as_vec3(),
            color,
            2.0,
        ));
    }
}

/// The draughtsman's mark for a right angle: a small square tucked into the
/// corner, on the inside of the two arms.
fn push_square_mark(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    corner: DVec2,
    along: DVec2,
    across: DVec2,
    scale: ViewScale,
    color: [f32; 4],
) {
    let (along, across) = (along.normalize_or_zero(), across.normalize_or_zero());
    if along == DVec2::ZERO || across == DVec2::ZERO {
        return;
    }
    // Kept the same size on screen: it marks a corner, it does not measure it.
    let side = scale.world_size_of(11.0);
    let (a, b) = (along * side, across * side);

    for (start, end) in [(a, a + b), (a + b, b)] {
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(corner + start).as_vec3(),
            color,
            2.0,
        ));
        out.push(cao_render::Vertex::line(
            sketch.plane.to_world(corner + end).as_vec3(),
            color,
            2.0,
        ));
    }
}

/// The annotation the dimension tool is showing in advance: the one a click
/// would choose, or the one already chosen and looking for its place.
fn pending_annotation(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> Option<(DimensionTarget, DVec2)> {
    if let Some(target) = context.editor.placing {
        // A second entity under the cursor turns the dimension into another
        // one; it is shown where it would land, not dragged to the cursor.
        if context.editor.dimension_mode == DimensionMode::Auto
            && let Some(refined) = refine(context, index, target, cursor, snap)
        {
            return Some((refined, DVec2::ZERO));
        }
        let target = oriented(context, index, target, cursor);
        // The preview is nudged from where the annotation stands today, not
        // moved to an absolute offset: `push` adds a nudge on top of whatever
        // the dimension already carries.
        let nudge = annotation_home(context, index, target, pixel)
            .map(|(text_at, _)| cursor - text_at)
            .unwrap_or_default();
        return Some((target, nudge));
    }
    let target = measure_preview(context, index, cursor, snap)?;
    Some((target, DVec2::ZERO))
}

/// The shape about to be drawn, following the cursor: placing a point blind and
/// only then seeing where it went is needlessly uncomfortable.
fn push_preview(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    theme: &Theme,
    scale: ViewScale,
    context: &SketchContext<'_>,
) {
    let Some(cursor) = context.editor.cursor else {
        return;
    };
    let preview = tint_at(theme.sketch_free, 0.55);

    if let Some(anchor) = context.editor.chain {
        let from = match anchor {
            ChainAnchor::Pending(position) => position,
            ChainAnchor::Point(id) if id.0 < sketch.points().len() => sketch.point(id),
            ChainAnchor::Point(_) => cursor,
        };
        let to = context.editor.aimed.unwrap_or(cursor);
        push_preview_line(out, sketch, from, to, preview);

        // The little square of a right angle, drawn before it is committed to
        // so the constraint is never a surprise. Its two arms are the line
        // being drawn and the one it is squaring up against.
        if let (Some(corner), Some(previous)) =
            (context.editor.square_corner, context.editor.chain_previous)
            && previous.0 < sketch.segments().len()
        {
            let (start, end) = sketch.endpoints(previous);
            let arm = if start.distance(corner) < end.distance(corner) {
                end - start
            } else {
                start - end
            };
            push_square_mark(out, sketch, corner, to - from, -arm, scale, preview);
        }
    }

    // The point tool has nothing pending, yet placing a point blind is exactly
    // as uncomfortable as the rest.
    if context.editor.tool == Tool::Point {
        push_point_marker(out, sketch, cursor, scale.world_size_of(4.0), preview, 1.5);
    }

    // The dimension a click would place, drawn faintly where it would land —
    // then, once it is chosen, the same annotation following the cursor to the
    // spot it will sit on.
    if context.editor.tool == Tool::Dimension
        && let Some(index) = context.editor.active_sketch()
        && let Some((target, nudge)) = pending_annotation(
            context,
            index,
            cursor,
            scale.world_size_of(PICK_PIXELS),
            scale.units_per_pixel,
        )
    {
        let mut style = crate::screens::annotations::Style::driving(theme);
        style.color = preview;
        style.width *= 0.9;
        crate::screens::annotations::push(
            out,
            sketch,
            target,
            &style,
            scale.units_per_pixel,
            nudge,
        );
    }

    // What the cursor has been caught by. The middle of a line is the one that
    // has to be said out loud: nothing else on screen tells you that you are
    // exactly halfway along it.
    match context.editor.snap {
        Some(Snap::Midpoint(at)) => {
            push_midpoint_mark(out, sketch, at, scale, tint_at(theme.highlight, 1.0))
        }
        Some(Snap::OnSegment(at)) => push_point_marker(
            out,
            sketch,
            at,
            scale.world_size_of(3.0),
            tint_at(theme.highlight, 1.0),
            2.0,
        ),
        _ => {}
    }

    if context.editor.tool == Tool::Circle
        && let Some(index) = context.editor.active_sketch()
        && let Some(found) = circle_from(context, index, cursor, scale.world_size_of(PICK_PIXELS))
    {
        push_circle_at(out, sketch, found.center, found.radius, preview, 1.5);
        push_point_marker(out, sketch, found.center, scale.world_size_of(3.0), preview, 1.5);
    }

    let Some(start) = context.editor.pending_start else {
        return;
    };
    if context.editor.tool == Tool::Rectangle {
        let far = context.editor.aimed.unwrap_or(cursor);
        let corners = [
            start,
            DVec2::new(far.x, start.y),
            far,
            DVec2::new(start.x, far.y),
        ];
        for index in 0..4 {
            push_preview_line(out, sketch, corners[index], corners[(index + 1) % 4], preview);
        }
    }
}

fn push_preview_line(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    from: DVec2,
    to: DVec2,
    color: [f32; 4],
) {
    out.push(cao_render::Vertex::line(
            sketch.plane.to_world(from).as_vec3(),
        color,
        1.5,
    ));
    out.push(cao_render::Vertex::line(
            sketch.plane.to_world(to).as_vec3(),
        color,
        1.5,
    ));
}

fn push_circle(
    out: &mut Vec<cao_render::Vertex>,
    sketch: &Sketch,
    center: PointId,
    radius: f64,
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
    center: DVec2,
    radius: f64,
    color: [f32; 4],
    width: f32,
) {
    const SIDES: usize = 96;
    if radius <= 0.0 {
        return;
    }
    let mut previous = None;
    for step in 0..=SIDES {
        let angle = step as f64 / SIDES as f64 * std::f64::consts::TAU;
        let point = center + DVec2::new(angle.cos(), angle.sin()) * radius;
        let world = sketch.plane.to_world(point);
        if let Some(previous) = previous {
            out.push(cao_render::Vertex::line(previous, color, width));
            out.push(cao_render::Vertex::line(world.as_vec3(), color, width));
        }
        previous = Some(world.as_vec3());
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

/// The length and the angle of the line being drawn, editable on the spot.
///
/// Left alone they only report. Typed into, they become constraints and are
/// placed as dimensions when the line is validated — which is the whole point:
/// a line drawn to a value should not have to be measured afterwards.
///
/// Returns true when the user pressed Enter to finish the line from the
/// keyboard.
fn paint_live_input(ui: &mut egui::Ui, context: &mut SketchContext<'_>) -> bool {
    paint_live_fields(ui, context).unwrap_or(false)
}

/// The same, written where a missing piece simply means there is nothing to
/// show yet.
fn paint_live_fields(ui: &mut egui::Ui, context: &mut SketchContext<'_>) -> Option<bool> {
    let index = context.editor.active_sketch()?;
    let sketch = context.document.sketches().get(index)?;
    let cursor = context.editor.aimed.or(context.editor.cursor)?;
    let scale = context.document.scale();

    // A line is a length and an angle, a rectangle its two sides, a circle its
    // diameter and nothing else — so its second field is left out.
    let (labels, measured): ([&str; 2], [f64; 2]) = match context.editor.tool {
        Tool::Circle => {
            // Shown whether or not the picks make a circle just now: a field
            // that disappears while being typed into cannot be typed into.
            let across = circle_from(context, index, cursor, f64::MAX)
                .map(|found| found.radius * 2.0 * scale)
                .unwrap_or_default();
            (["mm", ""], [across, 0.0])
        }
        Tool::Line => {
            let from = anchor_position(sketch, context)?;
            let span = cursor - from;
            (
                ["mm", "°"],
                [span.length() * scale, span.y.atan2(span.x).to_degrees()],
            )
        }
        Tool::Rectangle => {
            let start = context.editor.pending_start?;
            let span = cursor - start;
            (["mm", "mm"], [span.x.abs() * scale, span.y.abs() * scale])
        }
        _ => return None,
    };

    // Hung off the pointer itself, down and to the right: anchored on the
    // drawing, the fields ended up under the cursor, and a cursor over them is
    // a cursor no longer over the canvas — the shape stopped following it.
    let at = ui.ctx().pointer_latest_pos()?;

    let live = &mut context.editor.live;
    let mut validated = false;
    let focus = std::mem::take(&mut live.focus);
    egui::Area::new(egui::Id::new("live_input"))
        .fixed_pos(at + egui::vec2(20.0, 20.0))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    validated |= live_field(ui, labels[0], measured[0], &mut live.first, focus);
                    if !labels[1].is_empty() {
                        validated |=
                            live_field(ui, labels[1], measured[1], &mut live.second, false);
                    }
                });
            });
        });
    Some(validated)
}

/// A number field that takes the keyboard on demand, whole value selected.
///
/// Selecting it matters: the field arrives holding the value already there, and
/// without it the first keystroke lands after it — 40 typed over 60.88 read
/// 60.8840.
fn value_field(ui: &mut egui::Ui, text: &mut String, hint: &str, focus: bool) -> egui::Response {
    let output = egui::TextEdit::singleline(text)
        .desired_width(72.0)
        .hint_text(hint)
        .show(ui);
    let response = output.response.response;
    if focus {
        response.request_focus();
        let mut state = output.state;
        state.cursor.set_char_range(Some(egui::text::CCursorRange::two(
            egui::text::CCursor::new(0),
            egui::text::CCursor::new(text.chars().count()),
        )));
        state.store(ui.ctx(), response.id);
    }
    response
}

/// One of the two fields. Returns true when Enter was pressed in it.
///
/// An untouched field stays empty and shows what the cursor is doing as a hint:
/// keeping the readout in the field itself meant the first keystroke landed
/// after it, and "40" typed over "0.000" read 0.00040.
fn live_field(
    ui: &mut egui::Ui,
    suffix: &str,
    measured: f64,
    field: &mut LiveField,
    focus: bool,
) -> bool {
    // The keyboard goes to the first field as soon as the fields appear: the
    // value is the next thing the user types, and Tab from the canvas walks
    // through the whole toolbar to get here.
    let response = value_field(ui, &mut field.text, &format!("{measured:.2}"), focus);
    ui.label(suffix);

    // Typing is what turns a readout into a decision. Emptying the field takes
    // the decision back.
    if response.changed() {
        field.locked = crate::screens::sketch::LiveInput::read(&field.text);
    }
    // Enter is consumed rather than merely read: the field has just given the
    // keyboard back, so the shortcut bound to that key would fire too.
    response.lost_focus()
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
}

/// Where a rule's marks are written: on each of the things it holds, so that
/// pointing at one of them says what it is caught up in.
///
/// A right angle is the exception: its mark belongs *in* the corner, which is
/// the only place it reads as an angle rather than as a note about two traits.
fn rule_marks(sketch: &Sketch, constraint: Constraint) -> Vec<DVec2> {
    let middle = |segment: SegmentId| {
        (segment.0 < sketch.segments().len() && !sketch.is_erased_segment(segment)).then(|| {
            let (start, end) = sketch.endpoints(segment);
            (start + end) * 0.5
        })
    };
    let point = |id: PointId| (id.0 < sketch.points().len()).then(|| sketch.point(id));
    let circle = |id: CircleId| {
        (id.0 < sketch.circles().len()).then(|| {
            let round = sketch.circle(id);
            sketch.point(round.center) + DVec2::splat(round.radius * 0.7)
        })
    };
    let both = |first: SegmentId, second: SegmentId| {
        [middle(first), middle(second)].into_iter().flatten().collect()
    };

    match constraint {
        Constraint::Perpendicular { first, second } => match corner_of(sketch, first, second) {
            Some(at) => vec![at],
            None => both(first, second),
        },
        Constraint::Parallel { first, second }
        | Constraint::Equal { first, second }
        | Constraint::Collinear { first, second } => both(first, second),
        Constraint::EqualRadius { first, second } => {
            [circle(first), circle(second)].into_iter().flatten().collect()
        }
        Constraint::AxisCollinear { segment, .. } => middle(segment).into_iter().collect(),
        Constraint::OnSegment { point: held, .. } | Constraint::Midpoint { point: held, .. } => {
            point(held).into_iter().collect()
        }
        // Where the circle actually touches, not somewhere beside it: three
        // tangencies of one circle would otherwise all land on the same spot.
        Constraint::Tangent {
            circle: round,
            segment,
            at,
        } => at
            .and_then(point)
            .or_else(|| {
                (round.0 < sketch.circles().len())
                    .then(|| sketch.foot_on_segment(sketch.circle(round).center, segment))
                    .flatten()
            })
            .into_iter()
            .collect(),
        Constraint::OnCircle { point: held, .. } => point(held).into_iter().collect(),
        Constraint::Fixed { element } => match element {
            Element::Point(held) => point(held).into_iter().collect(),
            Element::Segment(held) => middle(held).into_iter().collect(),
            Element::Circle(held) => circle(held).into_iter().collect(),
        },
    }
}

/// Just inside the corner two traits make, along the bisector.
///
/// They have to actually meet: two traits held square without touching have no
/// corner to write in, and the marks then go on the traits themselves.
fn corner_of(sketch: &Sketch, first: SegmentId, second: SegmentId) -> Option<DVec2> {
    let (pivot, a, b) = sketch.corner_points(first, second)?;
    let reach = (a.distance(pivot).min(b.distance(pivot))) * 0.25;
    let inward = ((a - pivot).normalize_or_zero() + (b - pivot).normalize_or_zero())
        .normalize_or(DVec2::X);
    Some(pivot + inward * reach)
}

/// The marks of the rules, written beside what they hold.
///
/// Text rather than drawn symbols: a rule has no size and no direction of its
/// own, so there is nothing to draw it *at* — only something to write next to.
fn paint_rule_marks(
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
        .drag_preview
        .as_ref()
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
        let held = context.editor.is_selected(Selection::Rule(*constraint));
        for at in rule_marks(sketch, *constraint) {
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
                constraint.mark(),
                egui::FontId::proportional(13.0),
                match held {
                    true => tint_to_color(state.theme.highlight),
                    false => tint_to_color(state.theme.rule),
                },
            );
        }
    }
}

/// How far apart two marks are set when they would otherwise land on top of
/// each other, in points.
const MARK_SPACING: f32 = 16.0;

/// The box being pulled across the drawing./// The box being pulled across the drawing.
///
/// Drawn on the sketch's own axes rather than the screen's: seen at an angle,
/// a box that stayed square on screen would take in a different area than the
/// one it appears to cover.
fn paint_band(
    ui: &egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
) {
    let (Some((from, to)), Some(index)) = (context.editor.band, context.editor.active_sketch())
    else {
        return;
    };
    let Some(sketch) = context.document.sketches().get(index) else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));

    let corners: Option<Vec<egui::Pos2>> = [
        from,
        DVec2::new(to.x, from.y),
        to,
        DVec2::new(from.x, to.y),
    ]
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
    // Mid-drag the annotations are drawn from the settled preview, so their
    // values have to be read from the same drawing — otherwise the numbers stay
    // behind while the lines they belong to move away.
    let Some(sketch) = context
        .editor
        .drag_preview
        .as_ref()
        .or_else(|| context.document.sketches().get(index))
    else {
        return;
    };
    let view_projection = state
        .camera
        .view_projection(rect.width() / rect.height().max(1.0));
    let painter = ui.painter_at(rect);

    let pixel = state
        .camera
        .world_units_per_pixel(rect.height() * ui.ctx().pixels_per_point()) as f64;

    for dimension in sketch.dimensions() {
        // Asking the annotation where its value belongs keeps the text on the
        // dimension line instead of floating near the geometry.
        let mut ignored = Vec::new();
        let style = crate::screens::annotations::Style::driving(&state.theme);
        let Some(placement) = crate::screens::annotations::push(
            &mut ignored,
            sketch,
            dimension.target,
            &style,
            pixel,
            live_offset(context, dimension.target),
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

    // The dimension being placed carries its value with it: a bare pair of
    // arrows says nothing about what is being measured.
    if context.editor.placing.is_some()
        && let Some(cursor) = context.editor.cursor
        && let Some((target, nudge)) =
            pending_annotation(context, index, cursor, pixel * PICK_PIXELS, pixel)
        && let Some(value) = context.document.measured(index, target)
        && let Some(placement) = crate::screens::annotations::push(
            &mut Vec::new(),
            sketch,
            target,
            &crate::screens::annotations::Style::driving(&state.theme),
            pixel,
            nudge,
        )
        && let Some(position) = to_screen(
            sketch.plane.to_world(placement.text_at),
            view_projection,
            rect,
        )
    {
        painter.text(
            position,
            egui::Align2::CENTER_CENTER,
            if matches!(
                target,
                DimensionTarget::Angle { .. } | DimensionTarget::AxisAngle { .. }
            ) {
                format!("{value:.1}°")
            } else {
                state.config.unit.format(value)
            },
            egui::FontId::proportional(13.0),
            egui::Color32::from_rgb(250, 220, 120),
        );
    }
}

/// The value of the dimension in hand, written right where that dimension is.
///
/// It used to sit in the title bar, an arm's length from the drawing: the eyes
/// had to leave the shape being measured to find the number belonging to it.
/// Returns true when a value was applied.
fn paint_dimension_field(
    ui: &mut egui::Ui,
    state: &ViewportState,
    rect: egui::Rect,
    context: &mut SketchContext<'_>,
) -> bool {
    let (Some(index), Some(target)) = (context.editor.active_sketch(), context.editor.selected)
    else {
        return false;
    };
    let pixel = state
        .camera
        .world_units_per_pixel(rect.height() * ui.ctx().pixels_per_point()) as f64;
    let Some(at) = annotation_screen_position(state, rect, context, index, target, pixel) else {
        return false;
    };

    let driven = context.document.sketches()[index]
        .dimension_of(target)
        .is_some_and(|dimension| dimension.driven);
    let angle = matches!(
        target,
        DimensionTarget::Angle { .. } | DimensionTarget::AxisAngle { .. }
    );

    let mut applied = false;
    egui::Area::new(egui::Id::new("dimension_field"))
        .fixed_pos(at + egui::vec2(16.0, 12.0))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    if driven {
                        // A readout cannot be edited: changing it would mean
                        // nothing, since it reports the geometry rather than
                        // deciding it.
                        let measured = context.document.measured(index, target).unwrap_or_default();
                        let suffix = if angle { "°" } else { "mm" };
                        ui.weak(format!("{measured:.2} {suffix} (lecture seule)"));
                        return;
                    }
                    let field = value_field(
                        ui,
                        &mut context.editor.dimension_input,
                        if angle { "degrés" } else { "mm" },
                        std::mem::take(&mut context.editor.focus_dimension_field),
                    );
                    // Enter is eaten here: the field has just given the keyboard
                    // back, so the same press would otherwise also fire the
                    // shortcut bound to it — and end the sketch.
                    let submitted = field.lost_focus()
                        && ui.input_mut(|input| {
                            input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                        });
                    applied = ui.button("✔").on_hover_text("Appliquer").clicked() || submitted;
                });
            });
        });

    applied && apply_dimension_value(context, index, target)
}

/// Where an annotation writes its value, on screen.
fn annotation_screen_position(
    state: &ViewportState,
    rect: egui::Rect,
    context: &SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    pixel: f64,
) -> Option<egui::Pos2> {
    let sketch = context.document.sketches().get(index)?;
    let mut ignored = Vec::new();
    let placement = crate::screens::annotations::push(
        &mut ignored,
        sketch,
        target,
        &crate::screens::annotations::Style::driving(&state.theme),
        pixel,
        live_offset(context, target),
    )?;
    to_screen(
        sketch.plane.to_world(placement.text_at),
        state
            .camera
            .view_projection(rect.width() / rect.height().max(1.0)),
        rect,
    )
}

/// Records what the user typed into the value field.
fn apply_dimension_value(
    context: &mut SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
) -> bool {
    let Ok(value) = context
        .editor
        .dimension_input
        .trim()
        .replace(',', ".")
        .parse::<f64>()
    else {
        context.editor.message = Some("Valeur invalide".to_string());
        return false;
    };

    // Only the value changes here: where the annotation sits was decided when it
    // was put down, and retyping a number must not send it back to its default.
    // A value that is already the one in force changes nothing, and clicking
    // ✔ twice must not leave two identical steps in the history.
    if context.document.sketches()[index]
        .dimension_of(target)
        .is_some_and(|dimension| (dimension.value - value).abs() < 1e-4)
    {
        context.editor.message = None;
        return false;
    }

    match context.document.apply(Operation::SetDimension {
        sketch: index,
        target,
        value,
        placement: None,
    }) {
        Some(cao_core::DimensionOutcome::ScaleDefined {
            millimeters_per_unit,
        }) => {
            context.editor.message = Some(format!(
                "Échelle définie : 1 unité = {millimeters_per_unit:.4} mm"
            ));
            true
        }
        Some(cao_core::DimensionOutcome::Geometry(cao_sketch::LengthOutcome::Exact)) => {
            context.editor.message = None;
            true
        }
        Some(cao_core::DimensionOutcome::Geometry(cao_sketch::LengthOutcome::BestEffort)) => {
            context.editor.message =
                Some("Contour fermé : seul le point d'arrivée a bougé".to_string());
            true
        }
        Some(cao_core::DimensionOutcome::Reference) => {
            context.editor.message = Some(REDUNDANT_WARNING.to_string());
            true
        }
        _ => {
            context.editor.message = Some("Cote impossible ici".to_string());
            false
        }
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
