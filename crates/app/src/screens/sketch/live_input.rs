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

    /// Starts a fresh set of fields, each standing on the value a tool
    /// suggests for it — the ranks it leaves empty stay empty.
    ///
    /// A value shown is a value decided: the field holds exactly what it says,
    /// so what the drawing is laid to is what the user reads.
    pub fn open_on(&mut self, values: &[Option<f64>]) {
        self.open();
        for (rank, value) in values.iter().enumerate() {
            let Some(value) = value else { continue };
            let text = shown(*value);
            let field = self.field(rank);
            field.locked = Self::read(&text);
            field.text = text;
        }
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

/// A suggested value as its field shows it: whole where it can be, to the
/// hundredth where it cannot.
fn shown(value: f64) -> String {
    match value.fract() == 0.0 {
        true => format!("{value:.0}"),
        false => format!("{value:.2}"),
    }
}
