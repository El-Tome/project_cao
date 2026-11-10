use cao_part::PartDocument;

use super::clamp_editor_to_document;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;

/// Compacts the history once its warning is confirmed. Every operation is
/// numbered anew, so no eye switched by hand, no light and no volume's state
/// still names its operation: they go back to their defaults.
pub(crate) fn compact_history(
    doc: &mut PartDocument,
    editor: &mut SketchEditor,
    viewport: &mut ViewportState,
) {
    doc.compact_history();
    clamp_editor_to_document(editor, doc);
    viewport.forget_the_numbers();
}
