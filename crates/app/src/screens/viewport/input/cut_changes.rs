//! A fillet or a chamfer changed after it was laid, from the selection: its
//! values opened again in the tool that laid it, the cut taken off, a
//! chamfer's sides swapped, or the cut switched to construction.

use cao_part::history::CutChange;
use cao_part::{Operation, PartDocument};
use cao_sketch::{Chamfer, ChamferMode, Corner, CornerCutId, CutShape, Selection, ToolState};

use crate::screens::SketchContext;
use crate::screens::sketch::{SketchEditor, Tool};

/// The cuts the selection tool is holding.
pub(crate) fn held_cuts(editor: &SketchEditor) -> Vec<CornerCutId> {
    editor
        .selection()
        .iter()
        .filter_map(|held| match held {
            Selection::Cut(cut) => Some(*cut),
            _ => None,
        })
        .collect()
}

/// Opens the values of a cut again, in the tool that lays its kind: every
/// corner laid with it taken, the fields holding what it is worth now. What
/// `Entrée` then lays changes the group in place.
pub(crate) fn open_cut_values(context: &mut SketchContext<'_>, index: usize, cut: CornerCutId) {
    let Some(sketch) = context.document.sketches().get(index) else {
        return;
    };
    let Some(laid) = sketch.corner_cuts().get(cut.0).copied() else {
        return;
    };
    let taken: Vec<Corner> = sketch
        .cuts_grouped_with(cut)
        .into_iter()
        .map(|id| {
            let [first, second] = sketch.corner_cuts()[id.0].sides;
            Corner::Between(first, second)
        })
        .collect();
    let mm = context.document.scale();
    let (tool, mode, values) = match laid.shape {
        CutShape::Fillet { radius } => (Tool::Fillet, None, [Some(radius * mm), None]),
        CutShape::Chamfer(Chamfer::Equal(reach)) => (
            Tool::Chamfer,
            Some(ChamferMode::Equal),
            [Some(reach * mm), None],
        ),
        CutShape::Chamfer(Chamfer::Sided { first, second }) => (
            Tool::Chamfer,
            Some(ChamferMode::Sided),
            [Some(first * mm), Some(second * mm)],
        ),
        CutShape::Chamfer(Chamfer::Angled { along, degrees }) => (
            Tool::Chamfer,
            Some(ChamferMode::Angled),
            [Some(along * mm), Some(degrees)],
        ),
    };
    let written = context.document.cut_written(index, cut);
    let editor = &mut *context.editor;
    editor.tool = tool;
    if let Some(mode) = mode {
        editor.chamfer_mode = mode;
    }
    editor.reset_pending();
    editor.tool_state = ToolState::Corner { taken, half: None };
    editor.live.open_on(&values);
    // Each value is opened on what it was written as — a formula of the
    // variables, or the number to its last digit — so that Entrée on it
    // unchanged keeps what it follows and what it is worth.
    for (rank, text) in written.into_iter().enumerate() {
        if let Some(text) = text {
            let field = editor.live.field(rank);
            field.text = text;
            field.take(context.document.variables());
        }
    }
    editor.message = Some(context.lang.t("sketch.cut_values_opened"));
}

/// Switches every cut held between construction and the outline, each on its
/// own, as one step. Says whether there was a cut held to switch: the toggle
/// otherwise only says what the next shape is drawn as.
pub(crate) fn switch_held_cuts(context: &mut SketchContext<'_>) -> bool {
    let Some(index) = context.editor.active_sketch() else {
        return false;
    };
    let held = held_cuts(context.editor);
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let changes: Vec<Operation> = held
        .iter()
        .filter_map(|cut| sketch.corner_cuts().get(cut.0).map(|laid| (*cut, laid)))
        .map(|(cut, laid)| {
            Operation::CutChange(CutChange::Construction {
                sketch: index,
                cut,
                construction: !laid.construction,
            })
        })
        .collect();
    if changes.is_empty() {
        return false;
    }
    context.document.apply(Operation::Gesture(changes));
    true
}

/// The chamfer held whose two sides can trade places — one measured from
/// its first side, in two distances or a distance and an angle.
pub(crate) fn chamfer_to_swap(
    document: &PartDocument,
    editor: &SketchEditor,
) -> Option<CornerCutId> {
    let index = editor.active_sketch()?;
    let sketch = document.sketches().get(index)?;
    let [cut] = held_cuts(editor)[..] else {
        return None;
    };
    match sketch.corner_cuts().get(cut.0)?.shape {
        CutShape::Chamfer(Chamfer::Sided { .. } | Chamfer::Angled { .. }) => Some(cut),
        _ => None,
    }
}

/// Swaps the sides of the chamfer held, as one step.
pub(crate) fn swap_held_chamfer(context: &mut SketchContext<'_>) -> bool {
    let (Some(index), Some(cut)) = (
        context.editor.active_sketch(),
        chamfer_to_swap(context.document, context.editor),
    ) else {
        return false;
    };
    context
        .document
        .apply(Operation::CutChange(CutChange::SwapSides {
            sketch: index,
            cut,
        }));
    true
}

#[cfg(test)]
mod tests;
