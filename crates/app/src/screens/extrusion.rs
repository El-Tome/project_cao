use cao_core::ExtrusionMode;
use glam::Vec2;

/// Everything the extrusion workflow remembers between frames.
///
/// It is deliberately its own module rather than more fields on the sketch
/// editor: turning areas into matter is a different job from drawing them, and
/// the two only meet through the sketch they share.
#[derive(Default)]
pub struct ExtrusionState {
    /// The mode in hand, if the tool is armed at all.
    pub mode: Option<ExtrusionMode>,
    /// The sketch whose areas are being picked.
    pub sketch: Option<usize>,
    /// One point inside each chosen area, in the sketch's coordinates.
    pub picks: Vec<Vec2>,
    /// The area under the cursor, as an index into the sketch's areas.
    pub hovered: Option<usize>,
    /// Height typed by the user, in millimetres.
    pub distance_input: String,
    /// Whether the matter goes the other way along the plane.
    pub reversed: bool,
    pub message: Option<String>,
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
            ..Self::default()
        };
    }

    pub fn arm(&mut self, mode: ExtrusionMode) {
        self.mode = Some(mode);
        self.message = None;
        if self.distance_input.trim().is_empty() {
            self.distance_input = "10".to_string();
        }
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }

    /// The value typed, in millimetres, signed by the chosen direction.
    pub fn distance(&self) -> Option<f32> {
        let value = self
            .distance_input
            .trim()
            .replace(',', ".")
            .parse::<f32>()
            .ok()
            .filter(|value| value.abs() > 1e-6)?;
        Some(if self.reversed { -value } else { value })
    }
}
