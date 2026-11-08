use cao_part::PartDocument;
use cao_sketch::Sketch;

use crate::lang::Catalogue;
use crate::screens::sketch::SketchEditor;
use crate::wording::plane;

/// What the pill reads while a sketch is open: the sketch's name, the plane it
/// is drawn on, and its button.
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
            plane: plane_of(sketch, lang),
            finish: lang.t("sketch_pill.finish"),
        })
    }
}

/// How the plane the sketch is drawn on reads.
fn plane_of(sketch: &Sketch, lang: &Catalogue) -> String {
    plane::label(lang, sketch.plane.kind())
}
