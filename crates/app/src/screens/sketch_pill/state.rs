use cao_part::PartDocument;
use cao_prefs::{Chord, Command, Key};
use cao_sketch::{PlaneKind, Sketch};

use crate::lang::Catalogue;
use crate::screens::sketch::SketchEditor;
use crate::wording::{command, plane, shortcuts, view};

/// What the pill reads while a sketch is open: the sketch's name, where it is
/// drawn, what the solver leaves it to set, and its button.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SketchPill {
    pub name: String,
    pub plane: String,
    pub left: Left,
    /// How `left` reads: nothing while nothing is drawn.
    pub left_read: Option<String>,
    pub finish: String,
}

/// What the solver leaves the sketch to set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Left {
    /// Nothing laid but the origin: the solver's nought would announce an
    /// empty sketch as set.
    NothingDrawn,
    /// Degrees of freedom still to set, one or more.
    Free(usize),
    /// None left.
    Settled,
}

impl Left {
    /// The solver's count, `Sketch::freedom`, at the part's scale.
    pub fn of(sketch: &Sketch, millimeters_per_unit: f64) -> Self {
        if !sketch.holds_a_drawing() {
            return Self::NothingDrawn;
        }
        match sketch.freedom(millimeters_per_unit).degrees_of_freedom {
            0 => Self::Settled,
            free => Self::Free(free),
        }
    }

    fn read(self, lang: &Catalogue) -> Option<String> {
        let count = |key: &str, free: usize| lang.t_with(key, &[("count", &free.to_string())]);
        match self {
            Self::NothingDrawn => None,
            Self::Free(1) => Some(count("sketch_pill.free_one", 1)),
            Self::Free(free) => Some(count("sketch_pill.free_many", free)),
            Self::Settled => Some(lang.t("sketch_pill.settled")),
        }
    }
}

impl SketchPill {
    /// Nothing outside a sketch. The sketch is ranked by its place among the
    /// part's drawings, from one, as the Part tree names it; `finish_on` is
    /// what the settings bind finishing it to.
    pub fn of(
        document: &PartDocument,
        editor: &SketchEditor,
        finish_on: Option<Chord>,
        lang: &Catalogue,
    ) -> Option<Self> {
        let index = editor.active_sketch()?;
        let sketch = document.sketches().get(index)?;
        let rank = (index + 1).to_string();
        let left = Left::of(sketch, document.scale());
        Some(Self {
            name: lang.t_with("part_tree.sketch", &[("rank", &rank)]),
            plane: laid_on(document, index, sketch, lang, plane::label),
            left,
            left_read: left.read(lang),
            finish: finish(finish_on, lang),
        })
    }
}

/// The button's name, and the key that finishes too: a glyph for Enter alone,
/// the chord as the ribbon writes it for any other, nothing when none does.
fn finish(on: Option<Chord>, lang: &Catalogue) -> String {
    let name = command::label(lang, Command::FinishSketch);
    match on {
        Some(chord) if chord == Chord::new(Key::Enter) => {
            lang.t_with("sketch_pill.finish", &[("name", &name)])
        }
        Some(chord) => lang.t_with(
            "sketch_pill.finish_on",
            &[("name", &name), ("keys", &shortcuts::chord(lang, chord))],
        ),
        None => name,
    }
}

/// Where the sketch is drawn: the section it was laid on while the part still
/// keeps it, else its plane, as `named` words it.
pub fn laid_on(
    document: &PartDocument,
    index: usize,
    sketch: &Sketch,
    lang: &Catalogue,
    named: fn(&Catalogue, PlaneKind) -> String,
) -> String {
    match kept_section_under(document, index) {
        Some(id) => view::kept_section(lang, id + 1),
        None => named(lang, sketch.plane.kind()),
    }
}

/// A deleted section is only marked so, and the drawing laid on it still names
/// its id: the part's sections are asked whether it stands, and only for a
/// drawing laid on one.
fn kept_section_under(document: &PartDocument, index: usize) -> Option<usize> {
    let under = document.section_under(index)?;
    document
        .sections()
        .iter()
        .any(|kept| kept.id == under)
        .then_some(under)
}
