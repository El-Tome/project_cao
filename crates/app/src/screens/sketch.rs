use cao_sketch::{DimensionTarget, PointId, SegmentId, SketchAxis, WorkPlane};
use glam::Vec2;

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

impl Tool {
    /// The tools offered in the Esquisse category, in order.
    pub const SKETCH_TOOLS: [Self; 6] = [
        Self::Select,
        Self::Line,
        Self::Rectangle,
        Self::Circle,
        Self::Point,
        Self::Dimension,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::None => "Aucun outil",
            Self::Select => "Sélection",
            Self::Line => "Ligne",
            Self::Rectangle => "Rectangle",
            Self::Circle => "Cercle",
            Self::Point => "Point",
            Self::Dimension => "Cote",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Select => "Cliquer-glisser un point pour le déplacer",
            Self::Line => "Clics successifs, Échap pour terminer la chaîne",
            Self::Rectangle => "Deux clics : deux coins opposés",
            Self::Circle => "Deux clics : centre puis rayon",
            Self::Point => "Un clic pose un point",
            Self::Dimension => "Cote intelligente : cliquer ce qu'on veut mesurer",
        }
    }
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
    pub const ALL: [Self; 5] = [
        Self::Auto,
        Self::PointToPoint,
        Self::Length,
        Self::Angle,
        Self::Radius,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Auto => "Intelligente",
            Self::PointToPoint => "Point à point",
            Self::Length => "Trait",
            Self::Angle => "Angle",
            Self::Radius => "Rayon",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::Auto => "Mesure ce qui est sous le curseur",
            Self::PointToPoint => "Deux points, reliés ou non",
            Self::Length => "Un trait, mesuré sur toute sa longueur",
            Self::Angle => "Deux traits qui se touchent, ou un trait et un axe",
            Self::Radius => "Un cercle",
        }
    }

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
    pub hovered_plane: Option<usize>,
    /// What the dimension tool is pointing at.
    pub selected: Option<DimensionTarget>,
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
    pub drag_origin: Option<Vec2>,
    pub drag_position: Option<Vec2>,
    /// Axis picked first by the dimension tool, waiting for a segment.
    pub first_axis: Option<SketchAxis>,
    /// Point under the cursor, highlighted so it is clear what a click takes.
    pub hovered_point: Option<PointId>,
    /// First corner of a rectangle, or the centre of a circle.
    pub pending_start: Option<Vec2>,
    /// Text being typed into the dimension field.
    pub dimension_input: String,
    /// Where the next point would land, snapped. Drives the preview line.
    pub cursor: Option<Vec2>,
    pub message: Option<String>,
}

/// The point a polyline continues from.
///
/// The first click of a chain has nothing to attach to yet, and creating a
/// lone point would put a step in the history that draws nothing. So it is held
/// here until the second click, which turns the pair into one segment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChainAnchor {
    Pending(Vec2),
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
        self.pending_start = None;
        self.first_angle_segment = None;
        self.first_point = None;
        self.first_axis = None;
        self.dragged_point = None;
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

    /// Ends the polyline in progress without leaving the sketch.
    pub fn end_chain(&mut self) {
        self.chain = None;
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }

    pub fn select(&mut self, target: Option<DimensionTarget>, measured: Option<f32>) {
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
