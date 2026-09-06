use cao_sketch::{DimensionTarget, Element, PointId, SegmentId, SketchAxis, WorkPlane};
use glam::DVec2;

/// What the selection tool is holding, and what pressing Suppr would delete.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Selection {
    Element(Element),
    Dimension(DimensionTarget),
}

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

    pub fn is_locked(&self) -> bool {
        self.first.locked.is_some() || self.second.locked.is_some()
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
}

/// What the smart dimension tool is allowed to measure.
///
/// `Auto` takes whatever is under the cursor, which covers most of the work.
/// The others force one kind, for when two things overlap and the wrong one
/// keeps winning.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DimensionMode {
    #[default]
    Auto,
    /// Between two points, joined or not.
    PointToPoint,
    /// The length of a segment, by clicking the segment itself.
    Length,
    /// Between two segments, or a segment and a sketch axis.
    Angle,
    /// The radius of a circle.
    Radius,
}

impl DimensionMode {
    /// Whether this mode may pick a point.
    pub fn takes_points(self) -> bool {
        matches!(self, Self::Auto | Self::PointToPoint)
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

/// Everything the sketch workflow needs to remember between frames. The sketch
/// data itself lives in the part document; this is only the editing state.
#[derive(Default)]
pub struct SketchEditor {
    pub phase: SketchPhase,
    pub tool: Tool,
    /// The plane a sketch is being drawn on, kept here so the view can be
    /// re-aligned with it at any time.
    pub plane: Option<WorkPlane>,
    /// Where the polyline in progress carries on from.
    pub chain: Option<ChainAnchor>,
    pub hovered_plane: Option<PlaneChoice>,
    /// What the dimension tool is pointing at, once it is placed.
    pub selected: Option<DimensionTarget>,
    /// The dimension chosen but not yet put down: it follows the cursor until
    /// a second click says where it goes.
    pub placing: Option<DimensionTarget>,
    /// Which kind of measurement the dimension tool is forcing.
    pub dimension_mode: DimensionMode,
    /// First segment picked by the angle mode, waiting for the second.
    pub first_angle_segment: Option<SegmentId>,
    /// First point picked by the point-to-point mode.
    pub first_point: Option<PointId>,
    /// Point being dragged with the selection tool, and where it currently
    /// sits. Nothing is recorded until it is let go: a drag produces one entry
    /// in the history, not one per frame.
    pub dragged_point: Option<PointId>,
    /// Annotation being dragged out of the way.
    pub dragged_dimension: Option<DimensionTarget>,
    /// Where the drag began, to measure how far it has travelled.
    pub drag_origin: Option<DVec2>,
    pub drag_position: Option<DVec2>,
    /// Axis picked first by the dimension tool, waiting for a segment.
    pub first_axis: Option<SketchAxis>,
    /// Point under the cursor, highlighted so it is clear what a click takes.
    pub hovered_point: Option<PointId>,
    /// What the cursor has been pulled onto, so the drawing can say so.
    pub snap: Option<crate::screens::viewport::Snap>,
    /// The whole sketch as it would settle if the point were let go here.
    ///
    /// Drawing only the point under the cursor and leaving the rest where it
    /// was showed a shape torn out of shape, and nothing of where it was
    /// actually going to land.
    pub drag_preview: Option<cao_sketch::Sketch>,
    /// Everything the selection tool is holding, ready to be deleted.
    pub selection: Vec<Selection>,
    /// The box being pulled across the drawing, in sketch coordinates: where it
    /// started and where the cursor is now.
    pub band: Option<(DVec2, DVec2)>,
    /// The last segment the line tool drew, which the next one may square up
    /// against.
    pub chain_previous: Option<SegmentId>,
    /// The corner where a right angle is about to be made, so it can be shown
    /// before it is committed to.
    pub square_corner: Option<DVec2>,
    /// Where the line being drawn would actually end, once what the user typed
    /// and the right-angle snap have had their say. The preview shows this and
    /// not the raw cursor, so what is drawn is what a click would record.
    pub aimed: Option<DVec2>,
    pub live: LiveInput,
    /// First corner of a rectangle, or the centre of a circle.
    pub pending_start: Option<DVec2>,
    /// Text being typed into the dimension field.
    pub dimension_input: String,
    /// Set when a dimension has just been picked, so its field takes the
    /// keyboard on its own: reaching it with Tab means walking through every
    /// button of the toolbar first.
    pub focus_dimension_field: bool,
    /// Where the next point would land, snapped. Drives the preview line.
    pub cursor: Option<DVec2>,
    pub message: Option<String>,
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

/// The point a polyline continues from.
///
/// The first click of a chain has nothing to attach to yet, and creating a
/// lone point would put a step in the history that draws nothing. So it is held
/// here until the second click, which turns the pair into one segment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChainAnchor {
    Pending(DVec2),
    Point(PointId),
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

    /// Drops everything half-finished: the polyline in progress, the first
    /// corner of a shape, the segment waiting for its partner.
    pub fn reset_pending(&mut self) {
        self.chain = None;
        self.chain_previous = None;
        self.square_corner = None;
        self.aimed = None;
        self.live.clear();
        self.selection.clear();
        self.band = None;
        self.pending_start = None;
        self.first_angle_segment = None;
        self.first_point = None;
        self.first_axis = None;
        self.placing = None;
        self.dragged_point = None;
        self.drag_preview = None;
        self.dragged_dimension = None;
        self.drag_origin = None;
        self.drag_position = None;
        self.selected = None;
    }

    pub fn begin_editing(&mut self, sketch: usize, plane: WorkPlane) {
        self.phase = SketchPhase::Editing(sketch);
        self.plane = Some(plane);
        self.tool = Tool::Line;
        self.reset_pending();
        self.hovered_plane = None;
        self.message = None;
    }

    /// Whether something is part of what the selection tool is holding.
    pub fn is_selected(&self, what: Selection) -> bool {
        self.selection.contains(&what)
    }

    /// Adds or removes one thing, the way holding the modifier does.
    pub fn toggle(&mut self, what: Selection) {
        match self.selection.iter().position(|held| *held == what) {
            Some(index) => {
                self.selection.remove(index);
            }
            None => self.selection.push(what),
        }
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }

    pub fn select(&mut self, target: Option<DimensionTarget>, measured: Option<f64>) {
        self.focus_dimension_field = target.is_some() && target != self.selected;
        self.selected = target;
        self.dimension_input = match measured {
            Some(length) => format!("{length:.2}")
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
            None => String::new(),
        };
    }
}
