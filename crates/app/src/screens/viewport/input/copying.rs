use cao_part::Operation;
use cao_sketch::{
    ChosenAxis, Element, LockedInput, Repeats, Selection, Sketch, ToolState, axis_under,
};
use glam::DVec2;

use crate::screens::sketch::Tool;
use crate::screens::viewport::SketchContext;

use super::pick;

mod opening;
mod preview;
pub(crate) use preview::previewed;

/// One click of a tool that lays a copy down — the mirror, or either pattern.
/// All three gather what is held until `Entrée` says the selection is done, and
/// then name the one thing they each need: an axis, a centre, a direction.
pub(crate) fn copy(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let ToolState::Copying {
        naming_the_target, ..
    } = context.editor.tool_state
    else {
        take_hold(context, Vec::new());
        return false;
    };
    match naming_the_target {
        true => lay_the_copy(context, index, cursor, snap),
        false => gather(context, index, cursor, snap, pixel),
    }
}

/// What a tool laying copies calls the stage it is at, so the drawing says the
/// right thing whichever of the three is in hand.
fn gathering(context: &SketchContext<'_>) -> &'static str {
    match context.editor.tool {
        Tool::CircularPattern | Tool::RectangularPattern => "sketch.pattern_take_elements",
        _ => "sketch.mirror_take_elements",
    }
}

fn asking(context: &SketchContext<'_>) -> &'static str {
    match context.editor.tool {
        Tool::CircularPattern => "sketch.pattern_click_the_centre",
        Tool::RectangularPattern => "sketch.pattern_click_the_direction",
        _ => "sketch.mirror_click_the_axis",
    }
}

/// Starts the tool holding what it is given — whatever the selection tool had
/// in hand when the tool was reached for.
fn take_hold(context: &mut SketchContext<'_>, held: Vec<Element>) {
    context.editor.tool_state = ToolState::Copying {
        held,
        naming_the_target: false,
    };
    let said = gathering(context);
    context.editor.message = Some(context.lang.t(said));
}

/// `Entrée`: what is held is what will be copied, and the next click names
/// where it goes. Nothing held is nothing to copy, so the tool stays where it
/// is.
pub(crate) fn hold_is_done(context: &mut SketchContext<'_>) -> bool {
    let ToolState::Copying { held, .. } = &context.editor.tool_state else {
        return false;
    };
    if held.is_empty() {
        return false;
    }
    context.editor.tool_state = ToolState::Copying {
        held: held.clone(),
        naming_the_target: true,
    };
    opening::fields_open_on(context);
    let said = asking(context);
    context.editor.message = Some(context.lang.t(said));
    true
}

fn gather(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let Some(Selection::Element(element)) = pick(context, index, cursor, snap, pixel) else {
        return false;
    };
    let ToolState::Copying { held, .. } = &mut context.editor.tool_state else {
        return false;
    };
    match held.iter().position(|already| *already == element) {
        Some(rank) => {
            held.remove(rank);
        }
        None => held.push(element),
    }
    let said = gathering(context);
    context.editor.message = Some(context.lang.t(said));
    true
}

fn lay_the_copy(context: &mut SketchContext<'_>, index: usize, cursor: DVec2, snap: f64) -> bool {
    let ToolState::Copying { held, .. } = &context.editor.tool_state else {
        return false;
    };
    let elements = held.clone();
    let asked = match context.editor.tool {
        Tool::CircularPattern => around(context, index, elements, cursor, snap),
        Tool::RectangularPattern => in_rows(context, index, elements, cursor, snap),
        _ => across(context, index, elements, cursor, snap),
    };
    let Some(operation) = asked else {
        return false;
    };

    context.document.apply(operation);
    take_hold(context, Vec::new());
    context.editor.live.clear();
    true
}

/// The mirror's own reading of the last click: the axis to lay the copy across.
fn across(
    context: &mut SketchContext<'_>,
    index: usize,
    elements: Vec<Element>,
    cursor: DVec2,
    snap: f64,
) -> Option<Operation> {
    let sketch = context.document.sketches().get(index)?;
    let Some(axis) = axis_at(sketch, cursor, snap) else {
        context.editor.message = Some(context.lang.t("sketch.mirror_needs_an_axis"));
        return None;
    };
    Some(Operation::Mirror {
        sketch: index,
        elements,
        axis,
    })
}

/// The circular pattern's: a point of the drawing to turn about, and the two
/// values typed for it.
fn around(
    context: &mut SketchContext<'_>,
    index: usize,
    elements: Vec<Element>,
    cursor: DVec2,
    snap: f64,
) -> Option<Operation> {
    let sketch = context.document.sketches().get(index)?;
    let Some(centre) = sketch.nearest_point(cursor, snap) else {
        context.editor.message = Some(context.lang.t("sketch.pattern_needs_a_centre"));
        return None;
    };
    let Some((degrees, count)) = turned(context.editor.live.locked()) else {
        context.editor.message = Some(context.lang.t("sketch.pattern_needs_its_values"));
        return None;
    };
    Some(Operation::CircularPattern {
        sketch: index,
        elements,
        centre,
        degrees,
        count,
    })
}

/// The rectangular pattern's: a direction of the drawing to run along, and the
/// four values typed for it. The second direction is that one square.
fn in_rows(
    context: &mut SketchContext<'_>,
    index: usize,
    elements: Vec<Element>,
    cursor: DVec2,
    snap: f64,
) -> Option<Operation> {
    let sketch = context.document.sketches().get(index)?;
    let Some(direction) = axis_at(sketch, cursor, snap) else {
        context.editor.message = Some(context.lang.t("sketch.pattern_needs_a_direction"));
        return None;
    };
    let live = &context.editor.live;
    let typed = [live.typed(0), live.typed(1), live.typed(2), live.typed(3)];
    let Some((along, across)) = filled(typed) else {
        context.editor.message = Some(context.lang.t("sketch.pattern_needs_its_steps"));
        return None;
    };
    Some(Operation::RectangularPattern {
        sketch: index,
        elements,
        direction,
        along,
        across,
    })
}

/// The two steps and the two counts a rectangular pattern was given, once all
/// four have been typed.
///
/// A count is a whole number of copies, so what was typed is rounded to one. A
/// count of one is a direction the pattern does not run in, which leaves a
/// single row; one in both directions is no pattern at all.
fn filled(typed: [Option<f64>; 4]) -> Option<(Repeats, Repeats)> {
    let run = |step: Option<f64>, count: Option<f64>| {
        let count = count?.round();
        (count >= 1.0).then_some(Repeats {
            step: step?,
            count: count as usize,
        })
    };
    let along = run(typed[0], typed[1])?;
    let across = run(typed[2], typed[3])?;
    (along.count * across.count >= 2).then_some((along, across))
}

/// The step and the count a pattern was given, once both have been typed.
///
/// A count is a whole number of copies, so what was typed is rounded to one;
/// under two there is nothing to lay, and the drawing says so rather than
/// recording a step that does nothing.
fn turned(locked: LockedInput) -> Option<(f64, usize)> {
    let count = locked.second?.round();
    (count >= 2.0).then_some((locked.first?, count as usize))
}

/// Which axis a click names: a trait of the drawing first, then one of the
/// sketch's own two — the trait is the smaller target, and the axes run right
/// through the drawing.
fn axis_at(sketch: &Sketch, cursor: DVec2, snap: f64) -> Option<ChosenAxis> {
    if let Some(segment) = sketch.nearest_segment(cursor, snap) {
        return Some(ChosenAxis::Trait(segment));
    }
    axis_under(cursor, snap).map(ChosenAxis::Sketch)
}

#[cfg(test)]
mod tests;
