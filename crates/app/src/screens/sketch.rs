use crate::wording::dimension::REDUNDANT_WARNING;
use cao_part::{DimensionOutcome, PartDocument, history::Operation};
use cao_sketch::{DimensionTarget, LengthOutcome, PointId, Rule, Selection, ToolState, WorkPlane};
use glam::DVec2;

pub use cao_sketch::{CircleMode, DimensionMode};

/// One of the two values that can be typed while a shape is being drawn.
///
/// A value left alone is only a readout of what the cursor is doing. A value
/// typed becomes a decision: the shape can no longer take another, and the
/// dimension is placed on it when the shape is validated.
#[derive(Default)]
pub struct LiveField {
    pub text: String,
    pub locked: Option<f64>,
}

/// The two values shown as a shape is drawn, editable on the spot: length and
/// angle for a line, width and height for a rectangle.
///
/// Fixing one of the two still leaves the other free — an angle alone lets the
/// line be lengthened, a length alone lets it turn.
#[derive(Default)]
pub struct LiveInput {
    pub first: LiveField,
    pub second: LiveField,
    /// Set when the fields appear, so the first one takes the keyboard on its
    /// own: reaching it with Tab means walking through the toolbar first.
    pub focus: bool,
}

impl LiveInput {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Starts a fresh pair of fields with the keyboard on the first one.
    pub fn open(&mut self) {
        self.clear();
        self.focus = true;
    }

    /// The two decisions, as the drawing reads them.
    pub fn locked(&self) -> cao_sketch::LockedInput {
        cao_sketch::LockedInput {
            first: self.first.locked,
            second: self.second.locked,
        }
    }

    /// Reads a field the user has just changed. An emptied field goes back to
    /// being a readout.
    pub fn read(text: &str) -> Option<f64> {
        text.trim()
            .replace(',', ".")
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
    }
}

/// The drawing tool in hand. New tools are added here and to the Esquisse
/// menu; nothing else needs to know about them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Tool {
    #[default]
    None,
    /// Pick points and drag them.
    Select,
    Line,
    Rectangle,
    Circle,
    Point,
    /// Smart dimension: measures whatever is clicked.
    Dimension,
    /// Lays down a rule with no value.
    Constrain(Rule),
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
    /// Which kind of measurement the dimension tool is forcing. Not reset
    /// between shapes, for the same reason.
    pub dimension_mode: DimensionMode,
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
    /// A flat face of the part.
    Face(WorkPlane),
}

impl PlaneChoice {
    pub fn plane(self) -> WorkPlane {
        match self {
            Self::Origin(index) => WorkPlane::ORIGIN_PLANES[index],
            Self::Face(plane) => plane,
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

    pub fn start_choosing_plane(&mut self) {
        self.phase = SketchPhase::ChoosingPlane;
        self.tool = Tool::None;
        self.reset_pending();
        self.message = Some("Choisissez un plan d'esquisse".to_string());
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
        self.selection().contains(&what)
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

/// `placement: None` leaves the annotation where it was put down, not reset.
pub(crate) fn apply_dimension_value(
    document: &mut PartDocument,
    editor: &mut SketchEditor,
    index: usize,
    target: DimensionTarget,
) -> bool {
    let Some(typed) = editor.editing.as_ref().map(|editing| editing.input.clone()) else {
        return false;
    };
    let Ok(value) = typed.trim().replace(',', ".").parse::<f64>() else {
        editor.message = Some("Valeur invalide".to_string());
        return false;
    };

    // The same value twice must not repeat an identical step in the history.
    if document.sketches()[index]
        .dimension_of(target)
        .is_some_and(|dimension| (dimension.value - value).abs() < 1e-4)
    {
        editor.message = None;
        return false;
    }

    match document.apply(Operation::SetDimension {
        sketch: index,
        target,
        value,
        placement: None,
    }) {
        Some(DimensionOutcome::ScaleDefined {
            millimeters_per_unit: mm,
        }) => {
            editor.message = Some(format!("Échelle définie : 1 unité = {mm:.4} mm"));
            true
        }
        Some(DimensionOutcome::Geometry(LengthOutcome::Exact)) => {
            editor.message = None;
            true
        }
        Some(DimensionOutcome::Geometry(LengthOutcome::BestEffort)) => {
            editor.message = Some("Contour fermé : seul le point d'arrivée a bougé".to_string());
            true
        }
        Some(DimensionOutcome::Reference) => {
            editor.message = Some(REDUNDANT_WARNING.to_string());
            true
        }
        _ => {
            editor.message = Some("Cote impossible ici".to_string());
            false
        }
    }
}
