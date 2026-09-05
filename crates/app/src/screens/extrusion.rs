use cao_core::{ExtrusionMode, RevolutionAxis};
use cao_sketch::SketchAxis;
use glam::Vec2;

/// How the matter is made from the chosen areas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Shape {
    /// Pushed straight along the plane's normal.
    #[default]
    Straight,
    /// Swept around an axis lying in the plane.
    Revolution,
}

/// Everything the extrusion workflow remembers between frames.
///
/// It is deliberately its own module rather than more fields on the sketch
/// editor: turning areas into matter is a different job from drawing them, and
/// the two only meet through the sketch they share.
pub struct ExtrusionState {
    /// The mode in hand, if the tool is armed at all.
    pub mode: Option<ExtrusionMode>,
    /// The sketch whose areas are being picked.
    pub sketch: Option<usize>,
    /// One point inside each chosen area, in the sketch's coordinates.
    pub picks: Vec<Vec2>,
    /// The area under the cursor, as an index into the sketch's areas.
    pub hovered: Option<usize>,
    /// Straight, or swept around an axis.
    pub shape: Shape,
    /// What a revolution turns around.
    pub axis: RevolutionAxis,
    /// Sweep typed by the user, in degrees.
    pub angle_input: String,
    /// Height typed by the user, in millimetres.
    pub distance_input: String,
    /// Whether the matter goes the other way along the plane.
    pub reversed: bool,
    pub message: Option<String>,
}

impl Default for ExtrusionState {
    fn default() -> Self {
        Self {
            mode: None,
            sketch: None,
            picks: Vec::new(),
            hovered: None,
            shape: Shape::default(),
            // The vertical axis is the one a profile is usually drawn beside.
            axis: RevolutionAxis::Sketch(SketchAxis::V),
            angle_input: String::new(),
            distance_input: String::new(),
            reversed: false,
            message: None,
        }
    }
}

impl ExtrusionState {
    pub fn is_active(&self) -> bool {
        self.mode.is_some() && self.sketch.is_some()
    }

    /// Offers the tool on a sketch that has just been finished, without arming
    /// it: the user may simply want to be done.
    pub fn offer(&mut self, sketch: usize) {
        *self = Self {
            sketch: Some(sketch),
            distance_input: "10".to_string(),
            angle_input: "360".to_string(),
            axis: RevolutionAxis::Sketch(SketchAxis::V),
            ..Self::default()
        };
    }

    pub fn is_revolving(&self) -> bool {
        self.shape == Shape::Revolution
    }

    pub fn arm(&mut self, mode: ExtrusionMode) {
        self.mode = Some(mode);
        self.message = None;
        if self.distance_input.trim().is_empty() {
            self.distance_input = "10".to_string();
        }
        if self.angle_input.trim().is_empty() {
            self.angle_input = "360".to_string();
        }
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }

    /// The value typed, in millimetres, signed by the chosen direction.
    pub fn distance(&self) -> Option<f32> {
        signed(&self.distance_input, self.reversed)
    }

    /// The sweep typed, in degrees, signed by the chosen direction.
    pub fn angle(&self) -> Option<f32> {
        let value = signed(&self.angle_input, self.reversed)?;
        (value.abs() <= 360.0).then_some(value)
    }

    /// Whether there is enough to apply: areas chosen, and a usable value.
    pub fn is_ready(&self) -> bool {
        if self.picks.is_empty() {
            return false;
        }
        match self.shape {
            Shape::Straight => self.distance().is_some(),
            Shape::Revolution => self.angle().is_some(),
        }
    }
}

fn signed(input: &str, reversed: bool) -> Option<f32> {
    let value = input
        .trim()
        .replace(',', ".")
        .parse::<f32>()
        .ok()
        .filter(|value| value.abs() > 1e-6)?;
    Some(if reversed { -value } else { value })
}
