//! The values shown as a shape is drawn, and editable on the spot.

/// One of the values that can be typed while a shape is being drawn.
///
/// A value left alone is only a readout of what the cursor is doing. A value
/// typed becomes a decision: the shape can no longer take another, and the
/// dimension is placed on it when the shape is validated.
#[derive(Default)]
pub struct LiveField {
    pub text: String,
    pub locked: Option<f64>,
}

/// The values shown as a shape is drawn, editable on the spot: length and
/// angle for a line, width and height for a rectangle, the two steps and the
/// two counts of a rectangular pattern.
///
/// Fixing one of them still leaves the others free — an angle alone lets the
/// line be lengthened, a length alone lets it turn.
#[derive(Default)]
pub struct LiveInput {
    fields: Vec<LiveField>,
    /// Set when the fields appear, so the first one takes the keyboard on its
    /// own: reaching it with Tab means walking through the toolbar first.
    pub focus: bool,
}

impl LiveInput {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Starts a fresh set of fields with the keyboard on the first one.
    pub fn open(&mut self) {
        self.clear();
        self.focus = true;
    }

    /// The field of that rank, made on the spot. How many a tool shows is the
    /// tool's own business: two for a line, four for a rectangular pattern.
    pub fn field(&mut self, rank: usize) -> &mut LiveField {
        if self.fields.len() <= rank {
            self.fields.resize_with(rank + 1, LiveField::default);
        }
        &mut self.fields[rank]
    }

    /// What was typed into the field of that rank, if anything was.
    pub fn typed(&self, rank: usize) -> Option<f64> {
        self.fields.get(rank)?.locked
    }

    /// The first two decisions, as the drawing reads them — the pair every
    /// shape is drawn to.
    pub fn locked(&self) -> cao_sketch::LockedInput {
        cao_sketch::LockedInput {
            first: self.typed(0),
            second: self.typed(1),
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
