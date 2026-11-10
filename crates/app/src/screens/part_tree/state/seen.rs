//! The tree as the view stands: which sketch is being drawn on, which are
//! hidden, and how each volume is drawn.

use cao_part::PartDocument;
use cao_render::MatterInFront;

use super::PartTree;
use crate::lang::Catalogue;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;
use crate::wording::part_tree as said;

/// One segment of « TOUT LE CORPS »: the state it sets every volume to, lit
/// while every volume is in it.
#[derive(Clone, Debug, PartialEq)]
pub struct Segment {
    pub name: String,
    pub mode: MatterInFront,
    pub lit: bool,
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
        let shared = viewport.shared_volume_mode(document);
        let states = [
            MatterInFront::Hidden,
            MatterInFront::SeeThrough,
            MatterInFront::Opaque,
        ];
        tree.whole_body = states
            .into_iter()
            .map(|mode| Segment {
                name: said::whole_body(lang, mode),
                mode,
                lit: shared == Some(mode),
            })
            .collect();
        tree
    }
}
