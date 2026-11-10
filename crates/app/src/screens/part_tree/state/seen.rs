//! The tree as the view stands: which sketch is being drawn on, which are
//! hidden, and how each volume is drawn.

use cao_part::PartDocument;
use cao_render::MatterInFront;

use super::PartTree;
use crate::lang::Catalogue;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;
use crate::wording::part_tree as said;

/// An action the panel offers, and whether it would do anything now.
#[derive(Clone, Debug, PartialEq)]
pub struct Offered {
    pub label: String,
    pub enabled: bool,
}

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
        for body in &mut tree.bodies {
            body.shown = viewport.volume_mode(document, body.step);
            body.state = said::volume_state(lang, body.shown);
            body.visibility_button = said::visibility_button(lang, body.shown);
            body.dimmed = body.shown == MatterInFront::Hidden;
        }
        tree.make_all_opaque = (!tree.bodies.is_empty()).then(|| Offered {
            label: lang.t("part_tree.make_all_opaque"),
            enabled: !viewport.all_opaque(document),
        });
        tree
    }
}
