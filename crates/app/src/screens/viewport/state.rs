//! What the canvas knows between frames, and the values it works its
//! geometry out in.
//!
//! Nothing here draws: the presenter answers what the view asks, and the
//! drawing lives in [`super::view`].

use cao_prefs::{Modifier, Shortcuts, ViewportConfig, config::ViewportCorner, theme::Theme};
use cao_render::camera::{CubeZone, view_angles_towards};
use cao_render::{OrbitCamera, ViewTransition, adaptive_step};
use cao_sketch::{SnapSettings, WorkPlane};
use cao_solid::EdgeName;
use glam::DVec3;

use crate::screens::SketchContext;
use crate::ui::floating_bar::TOP_ABOVE_THE_EDGE as BAR_TOP;

use super::edges::DrawnEdges;
use super::kept_sections::KeptSections;
use super::navigation::Drag;
use super::reading::Reading;
use super::section::Section;
use super::{matter::Sections, visibility::Visibility};
use blinking::Blinking;

mod blinking;

/// How close, in pixels, the cursor has to be to take hold of something.
///
/// These are physical pixels, so a high-density screen halves them: at ten, a
/// point had to be hit within four points of the mouse, which is a good deal
/// finer than anyone aims.
pub(crate) const PICK_PIXELS: f64 = 18.0;

/// How far, in pixels, the cursor has to pass the line between two quarters
/// of an angle before the angle being put down turns to face the other one.
/// Less than that, and a hand wavering on the line flicks it back and forth.
pub(crate) const HOLD_PIXELS: f64 = 8.0;

/// What the canvas is showing: the bare world axes, or a work plane with its
/// grid. Landing on a plane shows the grid; orbiting leaves it, since the view
/// is no longer aligned with any plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewMode {
    Free,
    Plane(WorkPlane),
}

impl ViewMode {
    pub fn plane(self) -> Option<WorkPlane> {
        match self {
            Self::Free => None,
            Self::Plane(plane) => Some(plane),
        }
    }
}

pub struct ViewportState {
    pub config: ViewportConfig,
    /// Every colour the viewport draws with, copied from the profile in use.
    pub theme: Theme,
    /// The key held to pull a point off what holds it, copied from the profile
    /// the same way.
    pub let_go: Modifier,
    pub(super) camera: OrbitCamera,
    pub(super) mode: ViewMode,
    pub(super) transition: Option<ViewTransition>,
    pub(super) hovered_zone: Option<CubeZone>,
    pub(super) drag: Option<Drag>,
    /// Where the copy tools' panel was last let go, as the settings hold it.
    pub copy_panel_at: Option<[f32; 2]>,
    /// The canvas as the last frame drew it, remembered so that framing asked
    /// for from a toolbar button uses the viewport's shape, not the button's.
    pub(super) canvas: Option<egui::Rect>,
    pub(super) sections: Sections,
    /// The section view, while it is on.
    pub(super) section: Option<Section>,
    pub(crate) kept_sections: KeptSections,
    pub(super) visibility: Visibility,
    pub(super) reading: Reading,
    blinking: Option<Blinking>,
    /// The edge of the part the hand is on, while the edge panel is open.
    pub edge_under_the_hand: Option<EdgeName>,
    /// The edge the edge panel holds as chosen, drawn wherever the hand is.
    pub edge_chosen: Option<EdgeName>,
    pub(super) edges: DrawnEdges,
}

impl Default for ViewportState {
    fn default() -> Self {
        let config = ViewportConfig::default();
        let mut camera = OrbitCamera::default();
        camera.set_distance_limits(config.min_distance, config.max_distance);
        Self {
            config,
            theme: Theme::default(),
            let_go: Shortcuts::default().let_go,
            camera,
            mode: ViewMode::Free,
            transition: None,
            hovered_zone: None,
            drag: None,
            copy_panel_at: None,
            canvas: None,
            sections: Sections::default(),
            section: None,
            kept_sections: KeptSections::default(),
            visibility: Visibility::default(),
            reading: Reading::default(),
            blinking: None,
            edge_under_the_hand: None,
            edge_chosen: None,
            edges: DrawnEdges::default(),
        }
    }
}

impl ViewportState {
    pub(crate) fn section(&self) -> Option<&Section> {
        self.section.as_ref()
    }

    /// Turns the section view on, along XY through the middle of the part, or
    /// off when it is on.
    pub(crate) fn toggle_section(&mut self, body: &cao_solid::Body) {
        self.section = match self.section {
            Some(_) => None,
            None => Some(Section::through_the_middle(WorkPlane::XY, body)),
        };
    }

    pub(crate) fn close_section(&mut self) {
        self.section = None;
    }

    /// The section the frame cuts with: the section view's while it is open,
    /// otherwise the kept one switched on.
    pub(crate) fn section_shown(&self) -> Option<&Section> {
        self.section.as_ref().or(self.kept_sections.shown())
    }

    /// Opens the section view on a section the part keeps, to change it.
    pub(crate) fn reopen_section(&mut self, kept: &cao_part::SavedSection) {
        self.section = Some(Section::reopened(kept));
    }
}

impl ViewportState {
    pub fn mode(&self) -> ViewMode {
        self.mode
    }

    /// Turns the camera to look straight at a plane and frames `radius` around `center`, which is
    /// what both starting a sketch and the re-align button do.
    pub fn look_at_plane(&mut self, plane: WorkPlane, center: DVec3, radius: f64) {
        let (yaw, pitch) = view_angles_towards(plane.normal().as_vec3());
        self.transition = Some(ViewTransition::to_angles(&self.camera, yaw, pitch));
        self.camera
            .focus_on(center.as_vec3(), radius as f32, self.aspect());
        self.mode = ViewMode::Plane(plane);
    }

    /// Width over height of the canvas, square until a frame has drawn it.
    fn aspect(&self) -> f32 {
        self.canvas.map_or(1.0, |canvas| canvas.aspect_ratio())
    }

    /// Turns to an oblique view and frames the whole part, which is how a freshly extruded volume
    /// is actually seen: straight down on its own sketch plane, a prism is indistinguishable from
    /// the drawing it came from.
    pub fn look_at_part(&mut self, center: DVec3, radius: f64) {
        self.transition = Some(ViewTransition::to_zone(
            &self.camera,
            CubeZone::opening_corner(),
        ));
        self.camera
            .focus_on(center.as_vec3(), radius as f32, self.aspect());
        self.mode = ViewMode::Free;
    }

    /// Frames the whole part from the direction it is seen from; with no
    /// volume to frame, turns back to the view a new part opens with.
    pub(crate) fn frame_the_part(&mut self, body: &cao_solid::Body) {
        match body.bounds() {
            Some(bounds) => self.fit(bounds),
            None => self.turn_to_the_opening_view(body),
        }
    }

    /// Stands the camera so that these bounds fill the view, from wherever it
    /// is looking.
    pub(super) fn fit(&mut self, (min, max): (DVec3, DVec3)) {
        let radius = (max - min).length() * 0.5;
        self.camera
            .focus_on(((min + max) * 0.5).as_vec3(), radius as f32, self.aspect());
    }

    /// A face lands on its work plane and shows the grid; an edge or a corner
    /// is an oblique view, which stays in wireframe mode.
    pub(super) fn snap_to_zone(&mut self, zone: CubeZone) {
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

    pub(super) fn orbit(&mut self, delta: glam::Vec2, sensitivity: f32) {
        self.camera.orbit(delta, sensitivity);
        self.mode = ViewMode::Free;
        self.transition = None;
    }
}

/// Who the gesture on the canvas belongs to this frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum GestureGoesTo {
    TheCube,
    PickingAnArea,
    TheToolInHand,
}

/// The cube sits on top of the canvas, so it is asked first. Setting up an
/// extrusion then takes the whole canvas: no drawing tool is in hand while the
/// areas are being picked.
pub(super) fn gesture_goes_to(cube_took_it: bool, sketch: &SketchContext<'_>) -> GestureGoesTo {
    if cube_took_it {
        GestureGoesTo::TheCube
    } else if sketch.extrusion.is_active() {
        GestureGoesTo::PickingAnArea
    } else {
        GestureGoesTo::TheToolInHand
    }
}

/// How much of the world one pixel covers right now, and the grid step that
/// follows from it. Shared by the grid and the scale bar so they can never
/// disagree.
#[derive(Clone, Copy)]
pub(crate) struct ViewScale {
    pub(super) units_per_pixel: f64,
    /// Grid step in world units, for drawing.
    pub(super) step: f64,
    /// The same step in millimetres, for the label.
    pub(super) step_millimeters: f64,
    pub(super) height_px: f32,
    pub(super) diagonal_px: f32,
    pub(super) aspect: f32,
}

impl ViewScale {
    pub(super) fn of(
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
            units_per_pixel * millimeters_per_unit,
            f64::from(config.grid_pixel_spacing),
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
    pub(super) fn step_in_points(&self, pixels_per_point: f32) -> f32 {
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
    pub(super) fn snapping(&self, config: &ViewportConfig) -> SnapSettings {
        SnapSettings {
            point_reach: self.world_size_of(PICK_PIXELS),
            curve_reach: self.world_size_of(config.segment_snap_pixels as f64),
            grid_step: config
                .grid_snap
                .then(|| self.step / config.grid_snap_divisions.max(1) as f64),
            grid_reach: self.world_size_of(config.grid_snap_pixels as f64),
        }
    }
}

/// Where a thing of `size` stands in `corner`, `margin` in from the view's
/// edges — above the view bar in the bottom right, where the bar stands.
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
            viewport.right_bottom() + egui::vec2(-margin - size.x, -margin - size.y - BAR_TOP)
        }
    }
}

pub(crate) fn cube_rect(viewport: egui::Rect, config: &ViewportConfig) -> egui::Rect {
    let size = egui::vec2(config.cube_size, config.cube_size);
    let origin = corner_origin(viewport, config.cube_corner, size, config.cube_margin);
    egui::Rect::from_min_size(origin, size)
}

/// How much of the plane to show when a sketch has no geometry to frame yet.
pub const DEFAULT_SKETCH_RADIUS: f64 = 100.0;

/// Work planes are drawn a fixed fraction of the view across, so they stay
/// clickable however far the camera is.
pub(crate) fn plane_half_size(state: &ViewportState) -> f64 {
    state.camera.distance() as f64 * 0.3
}

/// Position inside `rect` as normalized device coordinates: -1..1 with y up.
pub(crate) fn to_ndc(position: egui::Pos2, rect: egui::Rect) -> glam::Vec2 {
    glam::Vec2::new(
        (position.x - rect.center().x) / (rect.width() * 0.5),
        (rect.center().y - position.y) / (rect.height() * 0.5),
    )
}

#[cfg(test)]
mod tests;
