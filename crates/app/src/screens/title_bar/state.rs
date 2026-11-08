use std::path::Path;

use cao_part::PartDocument;

use crate::lang::Catalogue;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;
use crate::wording::view::{Standing, standing_on};

/// What the title bar reads: the part's name and the file it is kept in, the
/// name of where the view stands, and the message of the last gesture.
pub struct TitleBar<'a> {
    pub name: &'a str,
    pub path: &'a Path,
    pub view: String,
    pub message: Option<&'a str>,
}

impl<'a> TitleBar<'a> {
    /// A sketch is ranked by its place among the part's drawings and a kept
    /// section by its id, both from one, as the Part tree names them.
    pub fn of(
        document: &'a PartDocument,
        path: &'a Path,
        editor: &'a SketchEditor,
        viewport: &ViewportState,
        lang: &Catalogue,
    ) -> Self {
        let standing = Standing {
            open_sketch_rank: editor.active_sketch().map(|sketch| sketch + 1),
            section_view_on: viewport.section().is_some(),
            kept_section_rank: viewport.kept_sections.cutting().map(|id| id + 1),
            plane: viewport.mode().plane(),
        };
        Self {
            name: document.name(),
            path,
            view: standing_on(lang, standing),
            message: editor.message.as_deref(),
        }
    }
}
