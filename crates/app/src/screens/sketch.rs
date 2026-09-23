use cao_sketch::{DimensionTarget, PointId, Rule, RulePick, Selection, ToolState, WorkPlane};
use glam::DVec2;

mod live_input;
mod typed_dimension;

pub use cao_sketch::{ArcMode, ChamferMode, CircleMode, DimensionMode, EllipseMode};
pub use live_input::{LiveField, LiveInput};
pub(crate) use typed_dimension::apply_dimension_value;

/// The drawing tool in hand. New tools are added here and to the Esquisse
/// menu; nothing else needs to know about them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Tool {
    #[default]
    None,
    /// Pick points and drag them.
    Select,
    Line,
    LineSymmetric,
    Rectangle,
    Circle,
    Arc,
    Ellipse,
    Point,
    /// Smart dimension: measures whatever is clicked.
    Dimension,
    /// Reads the drawing without touching it: a distance, a length, a radius
    /// or an angle, shown until the next measure and written down nowhere.
    Measure,
    /// Takes the stretch of a trait a click falls in out of it.
    Trim,
    /// Drops a point where traits cross and cuts each of them there.
    Split,
    /// Cuts the corner two traits share with a straight line.
    Chamfer,
    /// Rounds the corner two traits share into a curve tangent to both.
    Fillet,
    /// Lays a copy of what is held on the other side of an axis.
    Mirror,
    /// Repeats what is held around a chosen centre.
    CircularPattern,
    /// Repeats what is held in rows square to a chosen direction.
    RectangularPattern,
    /// Lays down a rule with no value.
    Constrain(Rule),
}

impl Tool {
    /// Whether pointing this tool at something lights the whole of it.
    ///
    /// Only the tool that takes the whole thing. A tool that takes a stretch
    /// shows the stretch instead — lighting the whole curve would promise to
    /// take all of it, which is the one thing the click will not do.
    pub(crate) fn lights_the_whole_of_it(self) -> bool {
        matches!(self, Self::Select)
    }
}

/// Where the sketch workflow currently stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SketchPhase {
    /// Nothing going on: the viewport is just a 3D view.
    #[default]
    Idle,
    /// The user asked for a new sketch and must now click a plane.
    ChoosingPlane,
    /// A sketch is open, at the given index in the part document.
    Editing(usize),
}

/// A dimension already on the drawing, opened for editing: its value, typed
/// into the field beside it.
pub struct DimensionEdit {
    pub target: DimensionTarget,
    pub input: String,
    /// Set when the dimension has just been opened, so its field takes the
    /// keyboard on its own: reaching it with Tab means walking through every
    /// button of the toolbar first.
    pub focus: bool,
}

/// Everything the sketch workflow needs to remember between frames. The sketch
/// data itself lives in the part document; this is only the editing state.
#[derive(Default)]
pub struct SketchEditor {
    pub phase: SketchPhase,
    pub tool: Tool,
    /// The plane a sketch is being drawn on, kept here so the view can be
    /// re-aligned with it at any time.
    pub plane: Option<WorkPlane>,
    pub hovered_plane: Option<PlaneChoice>,
    /// Point under the cursor, highlighted so it is clear what a click takes.
    pub hovered_point: Option<PointId>,
    pub hovered: Option<Selection>,
    /// What the cursor has been pulled onto, so the drawing can say so.
    pub snap: Option<cao_sketch::Snap>,
    /// Where the next point would land, snapped. Drives the preview line.
    pub cursor: Option<DVec2>,
    pub message: Option<String>,
    /// How the circle tool is drawing. Not reset between shapes: a mode
    /// chosen stays chosen until another is.
    pub circle_mode: CircleMode,
    /// How the arc tool is drawing, kept across shapes for the same reason.
    pub arc_mode: ArcMode,
    /// How the ellipse tool is drawing, kept across shapes for the same reason.
    pub ellipse_mode: EllipseMode,
    /// Which kind of measurement the dimension tool is forcing. Not reset
    /// between shapes, for the same reason.
    pub dimension_mode: DimensionMode,
    /// How the chamfer tool is saying what it takes, kept across corners for
    /// the same reason.
    pub chamfer_mode: ChamferMode,
    pub construction: bool,
    pub live: LiveInput,
    /// Where the shape being drawn actually ends, once what was typed and the
    /// right-angle snap have had their say. Recomputed every frame from
    /// `tool_state` and `cursor`; the preview shows this and not the raw
    /// cursor, so what is drawn is what a click would record.
    pub aimed: Option<cao_sketch::Aim>,
    /// A dimension already on the drawing, opened for editing.
    pub editing: Option<DimensionEdit>,
    /// How far the shape, dimension or rule the tool in hand is drawing has
    /// gotten. See `cao_sketch::ToolState`.
    pub tool_state: ToolState,
}

/// What the cursor is offering to sketch on.
///
/// A face of the part and one of the three planes of the origin are the same
/// choice as far as the user is concerned, so they are the same value here.
/// Which one wins is decided by whichever is nearer the camera, so a face in
/// front of a plane takes it — without ever making the planes unreachable.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlaneChoice {
    /// One of the three planes through the origin, by rank.
    Origin(usize),
    /// A flat face of the part: the plane it offers, and which face it is.
    Face { plane: WorkPlane, face: usize },
    /// A face of the part no drawing can be laid on, because it is not flat.
    Curved(usize),
}

impl PlaneChoice {
    /// The plane it offers to draw on, or nothing when it offers none.
    pub fn plane(self) -> Option<WorkPlane> {
        match self {
            Self::Origin(index) => Some(WorkPlane::ORIGIN_PLANES[index]),
            Self::Face { plane, .. } => Some(plane),
            Self::Curved(_) => None,
        }
    }
}

impl SketchEditor {
    pub fn active_sketch(&self) -> Option<usize> {
        match self.phase {
            SketchPhase::Editing(index) => Some(index),
            _ => None,
        }
    }

    pub fn is_choosing_plane(&self) -> bool {
        self.phase == SketchPhase::ChoosingPlane
    }

    pub fn start_choosing_plane(&mut self, lang: &crate::lang::Catalogue) {
        self.phase = SketchPhase::ChoosingPlane;
        self.tool = Tool::None;
        self.reset_pending();
        self.message = Some(lang.t("sketch.choose_a_plane"));
    }

    /// Drops everything half-finished: the shape in progress, the dimension
    /// being placed, the rule waiting for its next pick.
    pub fn reset_pending(&mut self) {
        self.tool_state = match self.tool {
            Tool::Select => ToolState::Select(Box::default()),
            _ => ToolState::None,
        };
        self.live.clear();
        self.editing = None;
    }

    /// Drops the measure on screen, and nothing else.
    ///
    /// A measure is a value read off the drawing. The moment the drawing
    /// moves, that value is a claim about something that is no longer there,
    /// and a readout that is quietly wrong is worse than none — so it goes,
    /// rather than waiting to be noticed.
    pub fn forget_the_measure(&mut self) {
        if matches!(self.tool_state, ToolState::Measure { .. }) {
            self.tool_state = ToolState::None;
        }
    }

    pub fn begin_editing(&mut self, sketch: usize, plane: WorkPlane) {
        self.phase = SketchPhase::Editing(sketch);
        self.plane = Some(plane);
        self.tool = Tool::Line;
        self.reset_pending();
        self.hovered_plane = None;
        self.message = None;
    }

    /// What the selection tool is holding, ready to be deleted.
    pub fn selection(&self) -> &[Selection] {
        match &self.tool_state {
            ToolState::Select(state) => &state.held,
            _ => &[],
        }
    }

    /// The elements the selection tool is holding, which is what the mirror
    /// carries over when it is reached for: taking things and then saying what
    /// to do with them is the gesture the drawing already has.
    pub fn held_elements(&self) -> Vec<cao_sketch::Element> {
        self.selection()
            .iter()
            .filter_map(|held| match held {
                Selection::Element(element) => Some(*element),
                _ => None,
            })
            .collect()
    }

    /// The selection tool's own state, when it is the tool in hand.
    pub fn select_state(&mut self) -> Option<&mut cao_sketch::SelectState> {
        match &mut self.tool_state {
            ToolState::Select(state) => Some(state),
            _ => None,
        }
    }

    /// The whole drawing settled as it would be if a drag in progress were
    /// let go right now, standing in for the recorded sketch while it lasts.
    pub fn drag_preview(&self) -> Option<&cao_sketch::Sketch> {
        match &self.tool_state {
            ToolState::Select(state) => state.drag_preview.as_ref(),
            _ => None,
        }
    }

    pub fn dragged_point(&self) -> Option<PointId> {
        match &self.tool_state {
            ToolState::Select(state) => state.dragged_point,
            _ => None,
        }
    }

    pub fn drag_position(&self) -> Option<DVec2> {
        match &self.tool_state {
            ToolState::Select(state) => state.drag_position,
            _ => None,
        }
    }

    pub fn dragged_dimension(&self) -> Option<DimensionTarget> {
        match &self.tool_state {
            ToolState::Select(state) => state.dragged_dimension,
            _ => None,
        }
    }

    pub fn drag_origin(&self) -> Option<DVec2> {
        match &self.tool_state {
            ToolState::Select(state) => state.drag_origin,
            _ => None,
        }
    }

    pub fn band(&self) -> Option<(DVec2, DVec2)> {
        match &self.tool_state {
            ToolState::Select(state) => state.band,
            _ => None,
        }
    }

    /// The dimension chosen but not yet put down: it follows the cursor until
    /// a second click says where it goes.
    pub fn placing(&self) -> Option<DimensionTarget> {
        match &self.tool_state {
            ToolState::Dimension { placing, .. } => *placing,
            _ => None,
        }
    }

    /// The first point picked by the point-to-point mode of the dimension
    /// tool, waiting for the second.
    pub fn first_point(&self) -> Option<PointId> {
        match &self.tool_state {
            ToolState::Dimension { picks, .. } => picks.first_point,
            _ => None,
        }
    }

    /// Where the polyline in progress carries on from.
    pub fn chain(&self) -> Option<cao_sketch::ChainAnchor> {
        match &self.tool_state {
            ToolState::Line { anchor, .. } => Some(*anchor),
            _ => None,
        }
    }

    /// What the rule being laid down has already been pointed at.
    pub fn rule_picks(&self) -> &[RulePick] {
        self.tool_state.rule_picks()
    }

    /// The places the arc being drawn has been given so far.
    pub fn arc_places(&self) -> &[DVec2] {
        match &self.tool_state {
            ToolState::Arc { places, .. } => places,
            _ => &[],
        }
    }

    pub fn ellipse_places(&self) -> &[DVec2] {
        match &self.tool_state {
            ToolState::Ellipse { places, .. } => places,
            _ => &[],
        }
    }

    /// First corner of a rectangle being drawn.
    pub fn pending_start(&self) -> Option<DVec2> {
        match &self.tool_state {
            ToolState::Rectangle { start } => Some(*start),
            _ => None,
        }
    }

    /// The dimension already on the drawing, open for editing.
    pub fn selected(&self) -> Option<DimensionTarget> {
        self.editing.as_ref().map(|editing| editing.target)
    }

    /// Whether something is part of what the selection tool is holding.
    pub fn is_selected(&self, what: Selection) -> bool {
        if self.selection().contains(&what) {
            return true;
        }
        // The mirror holds its own, so what it has taken is drawn as taken.
        match (what, &self.tool_state) {
            (Selection::Element(element), ToolState::Copying { held, .. }) => {
                held.contains(&element)
            }
            _ => false,
        }
    }

    /// Adds or removes one thing, the way holding the modifier does.
    pub fn toggle(&mut self, what: Selection) {
        let ToolState::Select(state) = &mut self.tool_state else {
            return;
        };
        match state.held.iter().position(|held| *held == what) {
            Some(index) => {
                state.held.remove(index);
            }
            None => state.held.push(what),
        }
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }

    pub fn select(&mut self, target: Option<DimensionTarget>, measured: Option<f64>) {
        let Some(target) = target else {
            self.editing = None;
            return;
        };
        let same = self
            .editing
            .as_ref()
            .is_some_and(|editing| editing.target == target);
        self.editing = Some(DimensionEdit {
            target,
            input: match measured {
                Some(length) => format!("{length:.2}")
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string(),
                None => String::new(),
            },
            focus: !same,
        });
    }
}

#[cfg(test)]
mod tests;
