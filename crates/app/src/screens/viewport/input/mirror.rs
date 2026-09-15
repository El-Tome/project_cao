use cao_part::Operation;
use cao_sketch::{Element, MirrorAxis, Selection, Sketch, ToolState, axis_under};
use glam::DVec2;

use crate::screens::viewport::SketchContext;

use super::pick;

/// One click of the mirror tool: it adds to what is held until `Entrée` says
/// the selection is done, and then names the axis to lay the copy across.
pub(crate) fn mirror(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let ToolState::Mirror {
        naming_the_axis, ..
    } = context.editor.tool_state
    else {
        take_hold(context, Vec::new());
        return false;
    };
    match naming_the_axis {
        true => lay_the_copy(context, index, cursor, snap),
        false => gather(context, index, cursor, snap, pixel),
    }
}

/// Starts the tool holding what it is given — whatever the selection tool had
/// in hand when the mirror was reached for.
fn take_hold(context: &mut SketchContext<'_>, held: Vec<Element>) {
    context.editor.tool_state = ToolState::Mirror {
        held,
        naming_the_axis: false,
    };
    context.editor.message = Some(context.lang.t("sketch.mirror_take_elements"));
}

/// `Entrée`: what is held is what will be copied, and the next click names the
/// axis. Nothing held is nothing to mirror, so the tool stays where it is.
pub(crate) fn hold_is_done(context: &mut SketchContext<'_>) -> bool {
    let ToolState::Mirror { held, .. } = &context.editor.tool_state else {
        return false;
    };
    if held.is_empty() {
        return false;
    }
    context.editor.tool_state = ToolState::Mirror {
        held: held.clone(),
        naming_the_axis: true,
    };
    context.editor.message = Some(context.lang.t("sketch.mirror_click_the_axis"));
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
    let ToolState::Mirror { held, .. } = &mut context.editor.tool_state else {
        return false;
    };
    match held.iter().position(|already| *already == element) {
        Some(rank) => {
            held.remove(rank);
        }
        None => held.push(element),
    }
    context.editor.message = Some(context.lang.t("sketch.mirror_take_elements"));
    true
}

fn lay_the_copy(context: &mut SketchContext<'_>, index: usize, cursor: DVec2, snap: f64) -> bool {
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let Some(axis) = axis_at(sketch, cursor, snap) else {
        context.editor.message = Some(context.lang.t("sketch.mirror_needs_an_axis"));
        return false;
    };
    let ToolState::Mirror { held, .. } = &context.editor.tool_state else {
        return false;
    };
    let elements = held.clone();

    context.document.apply(Operation::Mirror {
        sketch: index,
        elements,
        axis,
    });
    take_hold(context, Vec::new());
    true
}

/// Which axis a click names: a trait of the drawing first, then one of the
/// sketch's own two — the trait is the smaller target, and the axes run right
/// through the drawing.
fn axis_at(sketch: &Sketch, cursor: DVec2, snap: f64) -> Option<MirrorAxis> {
    if let Some(segment) = sketch.nearest_segment(cursor, snap) {
        return Some(MirrorAxis::Trait(segment));
    }
    axis_under(cursor, snap).map(MirrorAxis::Sketch)
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
            Some(MirrorAxis::Trait(trait_))
        );
    }

    #[test]
    fn a_click_on_nothing_but_a_sketch_axis_names_that_axis() {
        let (sketch, _) = a_drawing();

        assert_eq!(
            axis_at(&sketch, DVec2::new(20.0, 0.0), 0.5),
            Some(MirrorAxis::Sketch(SketchAxis::U))
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
            Some(MirrorAxis::Trait(along)),
            "the trait is the smaller target, and the one the user drew"
        );
    }

    #[test]
    fn a_click_far_from_anything_names_no_axis() {
        let (sketch, _) = a_drawing();

        assert_eq!(axis_at(&sketch, DVec2::new(20.0, 20.0), 0.5), None);
    }
}
