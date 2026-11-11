//! The first row of the part screen: the way home, the part's name, where the
//! view stands, the message, and the count of the part's alerts, which opens
//! their list.

mod state;
mod view;

use std::path::Path;

use cao_part::PartDocument;

pub use state::{Held, TitleBar, settle};
pub use view::show;

use crate::lang::Catalogue;
use crate::screens::part_tree::TreeAction;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;

/// Puts the title bar on screen from what the shell holds for it: whether the
/// way home was clicked, and what an alert's button asked of the tree.
pub fn run(
    ui: &mut egui::Ui,
    document: &PartDocument,
    path: &Path,
    editor: &SketchEditor,
    viewport: &ViewportState,
    held: &mut Held,
    lang: &Catalogue,
) -> (bool, TreeAction) {
    let error = held.covering.as_ref().map(|lying| lying.error.as_str());
    let bar = TitleBar::of(document, path, editor, viewport, error, lang);
    show(ui, &bar, &mut held.alerts_open, (&viewport.theme, lang))
}
