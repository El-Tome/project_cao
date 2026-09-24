//! The panels beside a part: what it is made of, what was done to it, and
//! the variables its sizes are written from.

use cao_part::PartDocument;

use crate::commands;
use crate::lang::Catalogue;
use crate::screens;
use crate::screens::history_tree::HistoryAction;
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::SketchEditor;
use crate::screens::variables::VariablesPanel;
use crate::screens::viewport::ViewportState;

/// What the panels keep between frames: which of them are open, and what the
/// panel of variables is part-way through.
pub(crate) struct Beside<'a> {
    pub ribbon: &'a mut Ribbon,
    pub variables: &'a mut VariablesPanel,
}

/// Draws the panels the ribbon has open beside the part, and carries out what
/// was asked in them. Returns true when the part was changed.
pub(crate) fn beside_the_part(
    ui: &mut egui::Ui,
    doc: &mut PartDocument,
    editor: &mut SketchEditor,
    viewport: &mut ViewportState,
    Beside { ribbon, variables }: Beside<'_>,
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

    if ribbon.variables_open {
        let asked = screens::variables::run(variables, ui, doc, lang);
        if asked.changed {
            // The part was built again from the new sizes, so what a tool was
            // half-way through may stand on geometry that has moved.
            commands::clamp_editor_to_document(editor, doc);
            changed = true;
        }
        let named = asked.named;
        if !named.is_empty() {
            let now = ui.input(|input| input.time);
            let faces = named
                .steps
                .iter()
                .flat_map(|step| doc.faces_made_by(*step))
                .collect();
            viewport.blink(named.values, faces, now);
            variables.blink(named.variables, now);
        }
    }
    changed
}
