//! The values shown as a shape is drawn, and editable on the spot.

use cao_part::{Formula, Unusable, Variables};

/// One of the values that can be typed while a shape is being drawn.
///
/// A value left alone is only a readout of what the cursor is doing. A value
/// typed becomes a decision: the shape can no longer take another, and the
/// dimension is placed on it when the shape is validated — written as it was
/// typed, so that a size typed from the part's variables keeps following them.
#[derive(Default)]
pub struct LiveField {
    pub text: String,
    /// What was typed, worked out: the number the shape is drawn to.
    pub locked: Option<f64>,
    /// What was typed, as written.
    pub written: Option<Formula>,
    /// Why what was typed cannot be used, while it cannot.
    pub wrong: Option<Unusable>,
}

impl LiveField {
    /// Takes in what the field holds now, read against the part's variables.
    /// An emptied field goes back to being a readout.
    pub fn take(&mut self, variables: &Variables) {
        let read = match self.text.trim().is_empty() {
            true => Ok(None),
            false => variables.size_of(&self.text).map(Some),
        };
        (self.written, self.locked, self.wrong) = match read {
            Ok(Some((formula, value))) => (Some(formula), Some(value), None),
            Ok(None) => (None, None, None),
            Err(wrong) => (None, None, Some(wrong)),
        };
    }
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
    /// What was typed at an earlier stage of the shape, carried over when the
    /// fields opened again for the next: an ellipse's first axis, an arc's
    /// first leg.
    carried: [Option<(Formula, f64)>; 2],
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

    /// Starts a fresh set of fields for the next stage of a shape, carrying
    /// what was typed into these as written, so that the dimension it leaves
    /// once drawn keeps it.
    pub fn open_for_the_next_stage(&mut self) {
        let carried = [self.typed_as_written(0), self.typed_as_written(1)];
        self.open();
        self.carried = carried;
    }

    /// What was typed at the stage before, in the field of that rank.
    pub fn carried(&self, rank: usize) -> Option<(Formula, f64)> {
        self.carried.get(rank)?.clone()
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
            let field = self.field(rank);
            field.text = shown(*value);
            field.take(&Variables::default());
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

    /// What was typed into the field of that rank, as written.
    pub fn written(&self, rank: usize) -> Option<Formula> {
        self.fields.get(rank)?.written.clone()
    }

    /// What was typed into the field of that rank, as written and as the
    /// number it comes to.
    pub fn typed_as_written(&self, rank: usize) -> Option<(Formula, f64)> {
        let field = self.fields.get(rank)?;
        Some((field.written.clone()?, field.locked?))
    }

    /// Why a field holds something that cannot be used — the first one that
    /// does. A shape is not laid while one does: what was typed would be
    /// dropped without a word.
    pub fn wrong(&self) -> Option<&Unusable> {
        self.fields.iter().find_map(|field| field.wrong.as_ref())
    }

    /// The first two decisions, as the drawing reads them — the pair every
    /// shape is drawn to.
    pub fn locked(&self) -> cao_sketch::LockedInput {
        cao_sketch::LockedInput {
            first: self.typed(0),
            second: self.typed(1),
        }
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
