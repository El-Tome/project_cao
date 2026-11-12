//! The panels beside a part: what it is made of and what was done to it, and
//! the variables its sizes are written from.

use cao_part::PartDocument;

use crate::commands;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::part_tree::TreeAction;
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::SketchEditor;
use crate::screens::variables::VariablesPanel;
use crate::screens::viewport::ViewportState;
use crate::screens::{self, SketchContext};

/// What the panels keep between frames: which of them are open, and what the
/// panel of variables is part-way through.
pub(crate) struct Beside<'a> {
    pub ribbon: &'a mut Ribbon,
    pub variables: &'a mut VariablesPanel,
}

/// Draws the panels the ribbon has open beside the part, and carries out what
/// was asked in them, and what an alert's button asked of the tree. Returns
/// true when the part was changed.
pub(crate) fn beside_the_part(
    ui: &mut egui::Ui,
    context: &mut SketchContext<'_>,
    viewport: &mut ViewportState,
    Beside { ribbon, variables }: Beside<'_>,
    alerted: TreeAction,
) -> bool {
    let (doc, editor, extrusion, lang) = (
        &mut *context.document,
        &mut *context.editor,
        &mut *context.extrusion,
        context.lang,
    );
    let mut changed = carry_out(alerted, doc, editor, extrusion, viewport, ribbon);
    if ribbon.part_tree_open {
        let unfolded = ribbon.history_unfolded;
        let (action, hovered) =
            screens::part_tree::panel(ui, doc, editor, viewport, unfolded, lang);
        screens::part_tree::state::hover(doc, editor, viewport, hovered);
        changed |= carry_out(action, doc, editor, extrusion, viewport, ribbon);
    } else {
        screens::part_tree::state::hover(doc, editor, viewport, None);
        viewport.light_out();
    }
    if let Some(section) = ribbon.section_delete_confirm
        && let Some(confirmed) = screens::part_tree::view::section_delete_confirm(ui, lang)
    {
        ribbon.section_delete_confirm = None;
        if confirmed {
            changed |= delete_section(doc, section);
        }
    }

    if ribbon.history_compact_confirm
        && let Some(confirmed) = screens::part_tree::view::compact_confirm(ui, lang)
    {
        ribbon.history_compact_confirm = false;
        if confirmed {
            commands::compact_history(doc, editor, viewport);
            extrusion.history_moved();
            changed = true;
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
    if viewport
        .kept_sections
        .keep_up_with(doc, editor.active_sketch())
    {
        ui.ctx().request_repaint();
    }
    changed
}

/// Carries out what was asked of the tree, from its panel or from an alert's
/// buttons. Returns true when the part was changed.
pub(crate) fn carry_out(
    action: TreeAction,
    doc: &mut PartDocument,
    editor: &mut SketchEditor,
    extrusion: &mut ExtrusionState,
    viewport: &mut ViewportState,
    ribbon: &mut Ribbon,
) -> bool {
    let mut changed = false;
    // What « Montrer » marks is seen on its row of the Part panel, and a panel
    // shut puts the mark out.
    ribbon.part_tree_open |= matches!(
        action,
        TreeAction::MarkSketch(_) | TreeAction::MarkVolume(_) | TreeAction::SwitchSectionOn(_)
    );
    match action {
        TreeAction::EditSketch(sketch) => {
            if let Some(plane) = doc.sketches().get(sketch).map(|s| s.plane) {
                viewport.light_out();
                editor.begin_editing(sketch, plane);
                let (center, radius) = commands::sketch_framing(doc, Some(sketch), plane);
                viewport.look_at_plane(plane, center, radius);
            }
        }
        TreeAction::MarkSketch(sketch) => viewport.mark_sketch(sketch),
        TreeAction::MarkVolume(step) => viewport.mark_volume(step),
        TreeAction::EditExtrusion(step) => {
            if let Some(reopened) = ExtrusionState::reopened(doc, step) {
                editor.close();
                *extrusion = reopened;
                ribbon.tab = 1;
            }
        }
        TreeAction::ToggleEye(sketch) => viewport.toggle_eye(doc, sketch),
        TreeAction::Select { what, adding } => {
            screens::part_tree::state::take(editor, what, adding);
        }
        TreeAction::Light(row) => viewport.light(row),
        TreeAction::NextVolumeMode(step) => {
            viewport.next_volume_mode(doc, step);
        }
        TreeAction::MakeAllOpaque => viewport.make_all_opaque(),
        TreeAction::ToggleSection(section) => {
            viewport.kept_sections.toggle(section);
        }
        TreeAction::SwitchSectionOn(section) => viewport.kept_sections.switch_on(section),
        TreeAction::EditSection(section) => {
            if let Some(kept) = doc.sections().iter().find(|kept| kept.id == section) {
                viewport.reopen_section(kept);
            }
        }
        TreeAction::DeleteSection(section) => match doc.is_stood_on(section) {
            true => ribbon.section_delete_confirm = Some(section),
            false => changed |= delete_section(doc, section),
        },
        TreeAction::ToggleHistory => {
            ribbon.history_unfolded = !ribbon.history_unfolded;
        }
        TreeAction::RewindTo(step) => {
            doc.rewind_to(step);
            viewport.light_out();
            commands::clamp_editor_to_document(editor, doc);
            extrusion.history_moved();
            changed = true;
        }
        TreeAction::AskToCompact => ribbon.history_compact_confirm = true,
        TreeAction::ReadTo(rank, boxes) => viewport.read_to(rank, boxes),
        TreeAction::ReadPrevious(boxes) => viewport.read_previous(boxes),
        TreeAction::ReadNext(boxes) => viewport.read_next(boxes),
        TreeAction::None => {}
    }
    changed
}

fn delete_section(doc: &mut PartDocument, section: usize) -> bool {
    doc.apply(cao_part::Operation::DeleteSection { section });
    true
}
