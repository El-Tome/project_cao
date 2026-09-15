//! The two values shown as a shape is drawn, and editable on the spot.

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

    /// Reads a field the user has just changed. An emptied field goes back to being a readout.
    pub fn read(text: &str) -> Option<f64> {
        text.trim()
            .replace(',', ".")
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
    }
}
