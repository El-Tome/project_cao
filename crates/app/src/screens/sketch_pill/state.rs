use cao_part::PartDocument;
use cao_sketch::Sketch;

use crate::lang::Catalogue;
use crate::screens::sketch::SketchEditor;
use crate::wording::{plane, view};

/// What the pill reads while a sketch is open: the sketch's name, where it is
/// drawn, and its button.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SketchPill {
    pub name: String,
    pub plane: String,
    pub finish: String,
}

impl SketchPill {
    /// Nothing outside a sketch. The sketch is ranked by its place among the
    /// part's drawings, from one, as the Part tree names it.
    pub fn of(document: &PartDocument, editor: &SketchEditor, lang: &Catalogue) -> Option<Self> {
        let index = editor.active_sketch()?;
        let sketch = document.sketches().get(index)?;
        let rank = (index + 1).to_string();
        Some(Self {
            name: lang.t_with("part_tree.sketch", &[("rank", &rank)]),
            plane: laid_on(document, index, sketch, lang),
            finish: lang.t("sketch_pill.finish"),
        })
    }
}

/// Where the sketch is drawn: the section it was laid on while the part still
/// keeps it, else its plane.
fn laid_on(document: &PartDocument, index: usize, sketch: &Sketch, lang: &Catalogue) -> String {
    match kept_section_under(document, index) {
        Some(id) => view::kept_section(lang, id + 1),
        None => plane::label(lang, sketch.plane.kind()),
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
