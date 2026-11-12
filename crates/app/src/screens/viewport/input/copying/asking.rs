//! What the drawing says the panel waits on: the circular pattern names its
//! current step, the mirror and the rectangular pattern their active slot.

use crate::screens::SketchContext;
use crate::screens::sketch::{CopySlot, CopyStep, Tool};
use crate::wording::copy_panel::{self as said, Asked};

/// A slot clicked in the panel becomes active, and when it belongs to a step
/// the drawing says what that step asks for. A panel laid out in no steps —
/// the mirror's, the rectangular pattern's — leaves the message as it was.
pub(crate) fn activate_in_the_panel(context: &mut SketchContext<'_>, slot: CopySlot) {
    let Some(panel) = context.editor.copy_panel_mut() else {
        return;
    };
    panel.activate(slot);
    if panel.current_step().is_some() {
        say_what_is_asked(context);
    }
}

/// What the drawing says while the panel waits on its active slot.
pub(super) fn say_what_is_asked(context: &mut SketchContext<'_>) {
    let Some(panel) = context.editor.copy_panel() else {
        return;
    };
    if let Some((rank, step)) = panel.current_step() {
        let asked = match step {
            CopyStep::Elements => Asked::Elements,
            CopyStep::Centre => Asked::Centre,
            CopyStep::CountAndStep => Asked::Count,
            CopyStep::Rotation => Asked::Rotation,
        };
        context.editor.message = Some(said::circular_step(context.lang, rank + 1, asked));
        return;
    }
    let said = match (panel.active(), context.editor.tool) {
        (CopySlot::Elements, Tool::Mirror) => "sketch.mirror_take_elements",
        (CopySlot::Elements, _) => "sketch.pattern_take_elements",
        (CopySlot::Target, Tool::RectangularPattern) => "sketch.pattern_click_the_direction",
        (CopySlot::Target, _) => "sketch.mirror_click_the_axis",
        _ => {
            context.editor.message = None;
            return;
        }
    };
    context.editor.message = Some(context.lang.t(said));
}
