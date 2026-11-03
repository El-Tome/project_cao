//! What a click or a box takes hold of, and what deleting a selection takes
//! with it.

use cao_part::Operation;
use cao_part::history::CutChange;
use cao_sketch::{Selection, ToolState};
use glam::DVec2;

use super::marks::Marks;
use crate::screens::SketchContext;

/// Pulls a box across the drawing and takes everything inside it.
///
/// Whole elements only: a trait counts when both its ends are in the box. Half
/// a trait cannot be deleted, so letting the box claim it would say something
/// the drawing cannot do.
pub(super) fn band_select(
    context: &mut SketchContext<'_>,
    index: usize,
    to: DVec2,
    response: &egui::Response,
    adding: bool,
    marks: &Marks,
    pixel: f64,
) -> bool {
    // Where the box started is kept from the frame the drag began: egui lets go
    // of the press position on the very frame the button comes up, which is the
    // frame that matters here.
    let Some((from, _)) = context.editor.select_state().and_then(|state| state.band) else {
        return false;
    };
    if let Some(state) = context.editor.select_state() {
        state.band = Some((from, to));
    }
    if !response.drag_stopped() {
        return false;
    }
    if let Some(state) = context.editor.select_state() {
        state.band = None;
    }

    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let mut caught = sketch.inside_band(
        from,
        to,
        crate::screens::annotations::metrics(pixel, DVec2::ZERO),
    );
    caught.extend(marks.inside(from, to));

    if !adding && let Some(state) = context.editor.select_state() {
        state.held.clear();
    }
    for what in caught {
        if !context.editor.is_selected(what)
            && let Some(state) = context.editor.select_state()
        {
            state.held.push(what);
        }
    }
    false
}

/// Takes what a click landed on, alone or into the selection.
pub(super) fn take(context: &mut SketchContext<'_>, picked: Option<Selection>, adding: bool) {
    match (picked, adding) {
        // Holding the modifier gathers things up one by one, which is how one
        // picks out three traits that no box can enclose alone.
        (Some(what), true) => context.editor.toggle(what),
        (Some(what), false) => {
            if let ToolState::Select(select) = &mut context.editor.tool_state {
                select.held = vec![what];
            }
        }
        (None, false) => {
            if let ToolState::Select(select) = &mut context.editor.tool_state {
                select.held.clear();
            }
        }
        (None, true) => {}
    }
}

/// What the cursor is over. The order and the reaches are the drawing's own
/// rule; all this adds is where each annotation was drawn.
pub(super) fn pick(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> Option<Selection> {
    let sketch = context.document.sketches().get(index)?;
    sketch.pick(
        cursor,
        snap,
        crate::screens::annotations::metrics(pixel, DVec2::ZERO),
    )
}

/// Deletes what the selection tool is holding, and lets go of it.
pub(super) fn erase_what_is_held(context: &mut SketchContext<'_>, index: usize) -> bool {
    let held = match &mut context.editor.tool_state {
        ToolState::Select(select) => std::mem::take(&mut select.held),
        _ => Vec::new(),
    };
    erase(context, index, &held)
}

/// Deletes what the selection tool is holding, in one step. A fillet or a
/// chamfer held is taken off, its corner sharp again.
pub(super) fn erase(
    context: &mut SketchContext<'_>,
    index: usize,
    selection: &[Selection],
) -> bool {
    if selection.is_empty() {
        return false;
    }
    let mut elements = Vec::new();
    let mut dimensions = Vec::new();
    let mut constraints = Vec::new();
    let mut cuts = Vec::new();
    for held in selection {
        match held {
            Selection::Element(element) => elements.push(*element),
            Selection::Dimension(target) => dimensions.push(*target),
            Selection::Rule(constraint) => constraints.push(*constraint),
            Selection::Cut(cut) => cuts.push(*cut),
        }
    }
    // The cuts first: erasing a side would take its cut away with it, and the
    // step would then break on a cut no longer there to take off.
    let mut done = Vec::new();
    if !cuts.is_empty() {
        done.push(Operation::CutChange(CutChange::TakeOff {
            sketch: index,
            cuts,
        }));
    }
    if !elements.is_empty() || !dimensions.is_empty() || !constraints.is_empty() {
        done.push(Operation::EraseMany {
            sketch: index,
            elements,
            dimensions,
            constraints,
        });
    }
    context.document.apply(match done.len() {
        1 => done.remove(0),
        _ => Operation::Gesture(done),
    });
    true
}
