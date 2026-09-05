use cao_sketch::{PointId, SegmentId, WorkPlane};
use glam::Vec2;

/// The drawing tool in hand. New tools are added here and to the Esquisse
/// menu; nothing else needs to know about them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Tool {
    #[default]
    None,
    Line,
    Dimension,
}

impl Tool {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "Aucun outil",
            Self::Line => "Ligne",
            Self::Dimension => "Cote",
        }
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
    pub selected_segment: Option<SegmentId>,
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
        self.chain = None;
        self.selected_segment = None;
        self.message = Some("Choisissez un plan d'esquisse".to_string());
    }

    pub fn begin_editing(&mut self, sketch: usize, plane: WorkPlane) {
        self.phase = SketchPhase::Editing(sketch);
        self.plane = Some(plane);
        self.tool = Tool::Line;
        self.chain = None;
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

    pub fn select_segment(&mut self, segment: Option<SegmentId>, current_length_mm: Option<f32>) {
        self.selected_segment = segment;
        self.dimension_input = match current_length_mm {
            Some(length) => format!("{length:.2}")
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
            None => String::new(),
        };
    }
}
