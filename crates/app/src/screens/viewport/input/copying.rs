use cao_part::history::RepeatsAsked;
use cao_part::{Formula, Operation, Variables};
use cao_sketch::{
    ChosenAxis, Element, LockedInput, Repeats, Selection, Sketch, axis_under, largest_count_around,
};
use glam::DVec2;

use crate::screens::SketchContext;
use crate::screens::sketch::{CopySlot, CopyTarget, LiveInput, Tool};

use super::super::values::refused_for_what_is_typed;
use super::{ViewScale, ViewportState, pick};

mod opening;
mod preview;
pub(crate) use preview::previewed;

/// One click of a tool that lays a copy down — the mirror, or either pattern.
/// It fills the active slot of the panel, and only that one: the tool never
/// guesses from what was clicked.
pub(crate) fn copy(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let Some(panel) = context.editor.copy_panel() else {
        take_up(context, Vec::new());
        return false;
    };
    match panel.active() {
        CopySlot::Elements => gather(context, index, cursor, snap, pixel),
        CopySlot::Target => name_the_target(context, index, cursor, snap),
        CopySlot::Value(_) | CopySlot::Validate => false,
    }
}

/// A box pulled across the drawing while Elements is active adds what it
/// takes. True while the box is what the pointer is doing.
///
/// A hand moving the view pulls none, as with the Selection tool: the button
/// that pans, or the one that orbits with its key held.
pub(crate) fn gather_in_a_box(
    view: &ViewportState,
    context: &mut SketchContext<'_>,
    (pressed, cursor): (DVec2, DVec2),
    response: &egui::Response,
    scale: ViewScale,
) -> bool {
    let editor = &mut *context.editor;
    let Some(panel) = editor.copy_panel.as_mut() else {
        return false;
    };
    if panel.active() != CopySlot::Elements {
        return false;
    }
    if response.drag_started() {
        let navigating = view.drag.is_some();
        if navigating || !response.drag_started_by(egui::PointerButton::Primary) {
            return false;
        }
        panel.band = Some((pressed, cursor));
        return true;
    }
    if response.dragged()
        && let Some((from, _)) = panel.band
    {
        panel.band = Some((from, cursor));
        return true;
    }
    let Some((from, _)) = panel.band.take().filter(|_| response.drag_stopped()) else {
        return false;
    };
    let index = editor.active_sketch();
    let Some(sketch) = index.and_then(|index| context.document.sketches().get(index)) else {
        return false;
    };
    let metrics = crate::screens::annotations::metrics(scale.units_per_pixel, DVec2::ZERO);
    let caught = sketch.inside_band(from, cursor, metrics);
    let editor = &mut *context.editor;
    let Some(panel) = editor.copy_panel.as_mut() else {
        return false;
    };
    panel.take_in(caught.into_iter().filter_map(|caught| match caught {
        Selection::Element(element) => Some(element),
        _ => None,
    }));
    panel.unmark_if_filled(&editor.live);
    true
}

/// `Entrée` in the panel: the slot below becomes active, and on Validate the
/// copy is laid — or, with a slot still empty, that slot is activated and
/// marked.
pub(crate) fn enter_in_the_panel(context: &mut SketchContext<'_>) -> bool {
    let Some(leaving) = context.editor.copy_panel().map(|panel| panel.active()) else {
        return false;
    };
    if leaving == CopySlot::Validate {
        if refused_for_what_is_typed(context) {
            return false;
        }
        let editor = &mut *context.editor;
        let Some(panel) = editor.copy_panel.as_mut() else {
            return false;
        };
        if !panel.mark_the_first_empty(&editor.live) {
            let laid = editor
                .active_sketch()
                .and_then(|index| asked(context, index));
            return lay(context, laid);
        }
    } else if let Some(panel) = context.editor.copy_panel.as_mut() {
        panel.move_down();
    }
    let editor = &*context.editor;
    let untyped = editor.copy_panel().is_some_and(|panel| {
        (0..panel.value_kinds().len()).all(|rank| editor.live.typed(rank).is_none())
    });
    if leaving == CopySlot::Elements && untyped {
        opening::fields_open_on(context);
    }
    say_what_is_asked(context);
    false
}

/// Opens the panel of the tool in hand, holding what it is given — whatever
/// the selection tool had in hand when the tool was reached for. With
/// elements already held, the values open on them and the target is next.
pub(crate) fn take_up(context: &mut SketchContext<'_>, held: Vec<cao_sketch::Element>) {
    let holding = !held.is_empty();
    context.editor.open_copy_panel(held);
    if holding {
        opening::fields_open_on(context);
    }
    say_what_is_asked(context);
}

/// What the drawing says while the panel waits on its active slot.
fn say_what_is_asked(context: &mut SketchContext<'_>) {
    let Some(active) = context.editor.copy_panel().map(|panel| panel.active()) else {
        return;
    };
    let said = match (active, context.editor.tool) {
        (CopySlot::Elements, Tool::Mirror) => "sketch.mirror_take_elements",
        (CopySlot::Elements, _) => "sketch.pattern_take_elements",
        (CopySlot::Target, Tool::CircularPattern) => "sketch.pattern_click_the_centre",
        (CopySlot::Target, Tool::RectangularPattern) => "sketch.pattern_click_the_direction",
        (CopySlot::Target, _) => "sketch.mirror_click_the_axis",
        _ => {
            context.editor.message = None;
            return;
        }
    };
    context.editor.message = Some(context.lang.t(said));
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
    let (Some(sketch), Some(panel)) = (
        context.document.sketches().get(index),
        context.editor.copy_panel.as_mut(),
    ) else {
        return false;
    };
    let ends = |held: Element| match held {
        Element::Segment(side) => sketch
            .segments()
            .get(side.0)
            .map_or_else(Vec::new, |side| vec![side.start, side.end]),
        _ => Vec::new(),
    };
    panel.toggle(element, ends);
    panel.unmark_if_filled(&context.editor.live);
    false
}

/// The target is kept from the click that names it, whatever the cursor is
/// over when the copy is laid: a point for the circular pattern, a trait or an
/// axis for the other two.
fn name_the_target(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> bool {
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };
    let (named, refused) = match context.editor.tool {
        Tool::CircularPattern => (
            sketch.nearest_point(cursor, snap).map(CopyTarget::Centre),
            "sketch.pattern_needs_a_centre",
        ),
        Tool::RectangularPattern => (
            axis_at(sketch, cursor, snap).map(CopyTarget::Axis),
            "sketch.pattern_needs_a_direction",
        ),
        _ => (
            axis_at(sketch, cursor, snap).map(CopyTarget::Axis),
            "sketch.mirror_needs_an_axis",
        ),
    };
    let editor = &mut *context.editor;
    let (Some(named), Some(panel)) = (named, editor.copy_panel.as_mut()) else {
        editor.message = Some(context.lang.t(refused));
        return false;
    };
    panel.name_the_target(named);
    panel.unmark_if_filled(&editor.live);
    say_what_is_asked(context);
    false
}

/// Lays the copy as one step, and leaves the tool in hand with its panel
/// emptied — elements, target and values.
fn lay(context: &mut SketchContext<'_>, asked: Option<Operation>) -> bool {
    let Some(operation) = asked else {
        return false;
    };
    context.document.apply(operation);
    context.editor.live.clear();
    take_up(context, Vec::new());
    true
}

/// The operation the panel asks for, once every slot holds something.
fn asked(context: &mut SketchContext<'_>, index: usize) -> Option<Operation> {
    let panel = context.editor.copy_panel()?;
    let elements = panel.elements().to_vec();
    match (context.editor.tool, panel.target()?) {
        (Tool::CircularPattern, CopyTarget::Centre(centre)) => {
            around(context, index, elements, centre)
        }
        (Tool::RectangularPattern, CopyTarget::Axis(direction)) => {
            in_rows(context, index, elements, direction)
        }
        (Tool::Mirror, CopyTarget::Axis(axis)) => Some(Operation::Mirror {
            sketch: index,
            elements,
            axis,
        }),
        _ => None,
    }
}

/// The circular pattern's: the point to turn about, and the two values typed
/// for it.
fn around(
    context: &mut SketchContext<'_>,
    index: usize,
    elements: Vec<cao_sketch::Element>,
    centre: cao_sketch::PointId,
) -> Option<Operation> {
    let Some((degrees, count)) = turned(context.editor.live.locked()) else {
        context.editor.message = Some(context.lang.t("sketch.pattern_needs_its_values"));
        return None;
    };
    let live = &context.editor.live;
    let degrees = as_typed(live.typed_as_written(0), degrees);
    let Some(count) = count_as_typed(live.typed_as_written(1), count) else {
        context.editor.message = Some(crate::wording::formula::not_a_count(context.lang));
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

/// The rectangular pattern's: the direction to run along, and the four values
/// typed for it. The second direction is that one square.
fn in_rows(
    context: &mut SketchContext<'_>,
    index: usize,
    elements: Vec<cao_sketch::Element>,
    direction: ChosenAxis,
) -> Option<Operation> {
    let live = &context.editor.live;
    let typed = [live.typed(0), live.typed(1), live.typed(2), live.typed(3)];
    let Some((along, across)) = filled(typed) else {
        context.editor.message = Some(context.lang.t("sketch.pattern_needs_its_steps"));
        return None;
    };
    let run = |run: Repeats, ranks: [usize; 2]| {
        Some(RepeatsAsked {
            step: as_typed(live.typed_as_written(ranks[0]), run.step),
            count: count_as_typed(live.typed_as_written(ranks[1]), run.count)?,
        })
    };
    let (Some(along), Some(across)) = (run(along, [0, 1]), run(across, [2, 3])) else {
        context.editor.message = Some(crate::wording::formula::not_a_count(context.lang));
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

/// A value a pattern was given, as it was typed.
fn as_typed(typed: Option<(Formula, f64)>, number: f64) -> Formula {
    match typed {
        Some((written, _)) if written.as_number().is_none() => written,
        _ => Formula::Number(number),
    }
}

/// A count as it was typed — nothing when a formula typed for it comes to
/// anything but a whole number. A plain number is rounded to one, as it always
/// was.
fn count_as_typed(typed: Option<(Formula, f64)>, count: usize) -> Option<Formula> {
    match typed {
        Some((written, value)) if written.as_number().is_none() => {
            ((value - value.round()).abs() < 1e-6).then_some(written)
        }
        _ => Some(Formula::Number(count as f64)),
    }
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

/// Brings a circular pattern's count down to what a full turn of its step
/// holds, the moment the two no longer agree, and answers that full turn for
/// as long as the count stands brought down to it.
pub(crate) fn count_brought_down(live: &mut LiveInput, variables: &Variables) -> Option<usize> {
    let largest = live.typed(0).and_then(largest_count_around);
    match (largest, live.typed(1).map(f64::round)) {
        (Some(largest), Some(count)) if count > largest as f64 => {
            live.field(1).text = largest.to_string();
            live.take(1, variables, None);
            live.brought_down = Some(largest);
        }
        (Some(largest), Some(count)) if count == largest as f64 => {}
        _ => live.brought_down = None,
    }
    live.brought_down
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
