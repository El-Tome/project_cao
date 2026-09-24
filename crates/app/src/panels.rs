//! The panels beside a part: what it is made of, and what was done to it.

use cao_part::PartDocument;

use crate::commands;
use crate::lang::Catalogue;
use crate::screens;
use crate::screens::history_tree::HistoryAction;
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;

/// Draws the panels the ribbon has open beside the part, and carries out what
/// was asked in them. Returns true when the part was changed.
pub(crate) fn beside_the_part(
    ui: &mut egui::Ui,
    doc: &mut PartDocument,
    editor: &mut SketchEditor,
    viewport: &mut ViewportState,
    ribbon: &mut Ribbon,
    lang: &Catalogue,
) -> bool {
    let mut changed = false;
    if ribbon.part_tree_open {
        match screens::part_tree::panel(ui, doc, lang) {
            screens::part_tree::TreeAction::EditSketch(sketch) => {
                if let Some(plane) = doc.sketches().get(sketch).map(|s| s.plane) {
                    editor.begin_editing(sketch, plane);
                    let (center, radius) = commands::sketch_framing(doc, Some(sketch), plane);
                    viewport.look_at_plane(plane, center, radius);
                }
            }
            screens::part_tree::TreeAction::None => {}
        }
    }

    if ribbon.history_open {
        match screens::history_tree::panel(ui, doc, &mut ribbon.history_compact_confirm, lang) {
            HistoryAction::RewindTo(step) => {
                doc.rewind_to(step);
                commands::clamp_editor_to_document(editor, doc);
                changed = true;
            }
            HistoryAction::EditSketch(sketch) => {
                if let Some(plane) = doc.sketches().get(sketch).map(|s| s.plane) {
                    editor.begin_editing(sketch, plane);
                    let (center, radius) = commands::sketch_framing(doc, Some(sketch), plane);
                    viewport.look_at_plane(plane, center, radius);
                }
            }
            HistoryAction::CompactHistory => {
                doc.compact_history();
                commands::clamp_editor_to_document(editor, doc);
                changed = true;
            }
            HistoryAction::None => {}
        }
    }
    changed
}
