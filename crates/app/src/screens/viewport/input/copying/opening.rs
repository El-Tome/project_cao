//! What a pattern's fields stand on when they appear.
//!
//! A value there to be typed over: the first thing anyone types is then a
//! number being corrected rather than one guessed at.

use cao_sketch::{Element, ToolState};

use crate::screens::SketchContext;
use crate::screens::sketch::Tool;

/// A circular pattern owes nothing to what is held: the quarter turn four times
/// round is the pattern anyone draws when they draw one at all.
const A_QUARTER_TURN: f64 = 90.0;
const FOUR_TIMES_ROUND: f64 = 4.0;

/// Both counts of a grid open on the original and one copy — the smallest
/// pattern that lays anything.
const THE_ORIGINAL_AND_ONE: f64 = 2.0;

/// Opens the fields the tool in hand asks for, each on the value it opens on.
///
/// A grid steps by how wide what is held stands, in the millimetres its field
/// is labelled with. The click that names the direction comes after this, so the
/// extent along it cannot be read yet, and the widest way round is the one
/// number no direction can make too small.
///
/// The mirror asks for no values and opens nothing.
pub(super) fn fields_open_on(context: &mut SketchContext<'_>) {
    let ToolState::Copying { held, .. } = &context.editor.tool_state else {
        return;
    };
    let values = match context.editor.tool {
        Tool::CircularPattern => [Some(A_QUARTER_TURN), Some(FOUR_TIMES_ROUND), None, None],
        Tool::RectangularPattern => {
            let span = widest_span(context, held);
            [
                span,
                Some(THE_ORIGINAL_AND_ONE),
                span,
                Some(THE_ORIGINAL_AND_ONE),
            ]
        }
        _ => return,
    };
    context.editor.live.open_on(&values);
}

/// How wide what is held stands, when there is a drawing to read it from.
fn widest_span(context: &SketchContext<'_>, held: &[Element]) -> Option<f64> {
    let index = context.editor.active_sketch()?;
    let span = context.document.sketches().get(index)?.widest_span(held)?;
    Some(context.document.to_millimeters(span))
}

#[cfg(test)]
mod tests;
