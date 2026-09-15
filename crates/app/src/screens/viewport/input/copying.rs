use cao_part::Operation;
use cao_sketch::{ChosenAxis, Element, LockedInput, Selection, Sketch, ToolState, axis_under};
use glam::DVec2;

use crate::screens::sketch::Tool;
use crate::screens::viewport::SketchContext;

use super::pick;

/// One click of a tool that lays a copy down — the mirror, or the circular
/// pattern. Both gather what is held until `Entrée` says the selection is done,
/// and then name the one thing they each need: an axis, or a centre.
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
/// right thing whichever of the two is in hand.
fn gathering(context: &SketchContext<'_>) -> &'static str {
    match context.editor.tool {
        Tool::CircularPattern => "sketch.pattern_take_elements",
        _ => "sketch.mirror_take_elements",
    }
}

fn asking(context: &SketchContext<'_>) -> &'static str {
    match context.editor.tool {
        Tool::CircularPattern => "sketch.pattern_click_the_centre",
        _ => "sketch.mirror_click_the_axis",
    }
}

/// Starts the tool holding what it is given — whatever the selection tool had
/// in hand when the mirror was reached for.
fn take_hold(context: &mut SketchContext<'_>, held: Vec<Element>) {
    context.editor.tool_state = ToolState::Copying {
        held,
        naming_the_target: false,
    };
    let said = gathering(context);
    context.editor.message = Some(context.lang.t(said));
}

/// `Entrée`: what is held is what will be copied, and the next click names the
/// axis. Nothing held is nothing to mirror, so the tool stays where it is.
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
    if context.editor.tool == Tool::CircularPattern {
        context.editor.live.open();
    }
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
mod tests {
    use cao_sketch::{SketchAxis, WorkPlane};

    use super::*;

    /// A trait standing well clear of both sketch axes.
    fn a_drawing() -> (Sketch, cao_sketch::SegmentId) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let low = sketch.add_point(DVec2::new(4.0, -5.0));
        let high = sketch.add_point(DVec2::new(4.0, 5.0));
        let trait_ = sketch.add_segment(low, high);
        (sketch, trait_)
    }

    #[test]
    fn a_click_on_a_trait_names_that_trait_as_the_axis() {
        let (sketch, trait_) = a_drawing();

        assert_eq!(
            axis_at(&sketch, DVec2::new(4.0, 1.0), 0.5),
            Some(ChosenAxis::Trait(trait_))
        );
    }

    #[test]
    fn a_click_on_nothing_but_a_sketch_axis_names_that_axis() {
        let (sketch, _) = a_drawing();

        assert_eq!(
            axis_at(&sketch, DVec2::new(20.0, 0.0), 0.5),
            Some(ChosenAxis::Sketch(SketchAxis::U))
        );
    }

    #[test]
    fn a_trait_wins_over_the_axis_it_is_lying_on() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let west = sketch.add_point(DVec2::new(-5.0, 0.0));
        let east = sketch.add_point(DVec2::new(5.0, 0.0));
        let along = sketch.add_segment(west, east);

        assert_eq!(
            axis_at(&sketch, DVec2::new(1.0, 0.0), 0.5),
            Some(ChosenAxis::Trait(along)),
            "the trait is the smaller target, and the one the user drew"
        );
    }

    #[test]
    fn a_click_far_from_anything_names_no_axis() {
        let (sketch, _) = a_drawing();

        assert_eq!(axis_at(&sketch, DVec2::new(20.0, 20.0), 0.5), None);
    }

    #[test]
    fn a_pattern_waits_for_both_its_values() {
        let some = |first, second| LockedInput { first, second };

        assert_eq!(turned(some(Some(30.0), None)), None);
        assert_eq!(turned(some(None, Some(6.0))), None);
        assert_eq!(turned(some(Some(30.0), Some(6.0))), Some((30.0, 6)));
    }

    #[test]
    fn a_count_under_two_is_no_pattern_at_all() {
        let some = |first, second| LockedInput { first, second };

        assert_eq!(turned(some(Some(30.0), Some(1.0))), None);
        assert_eq!(
            turned(some(Some(30.0), Some(1.6))),
            Some((30.0, 2)),
            "a count is a whole number of copies, so what was typed is rounded to one"
        );
    }
}
