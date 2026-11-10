//! The tree as the view stands: which sketch is being drawn on, and which are
//! hidden.

use cao_part::PartDocument;

use super::PartTree;
use crate::lang::Catalogue;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;

impl PartTree {
    pub fn seen(
        document: &PartDocument,
        editor: &SketchEditor,
        viewport: &ViewportState,
        lang: &Catalogue,
    ) -> Self {
        let mut tree = Self::of(document, lang);
        for drawing in &mut tree.sketches {
            drawing.open = editor.active_sketch() == Some(drawing.sketch);
            drawing.dimmed = !drawing.open && !viewport.eye_lit(document, drawing.sketch);
        }
        tree
    }
}
