//! What a click of the measure tool does: reads what is under the cursor and
//! shows it, and writes nothing anywhere.
//!
//! Every path here hands back `false`. That single fact is what "leaves no
//! trace" comes down to: the flag is what tells the canvas an operation was
//! recorded, and it is what marks the part as modified further up.

use cao_sketch::{
    DimensionMode, DimensionPick, DimensionPicks, DimensionTarget, SegmentId, Sketch, ToolState,
};
use glam::DVec2;

use crate::screens::viewport::SketchContext;
use crate::wording::constraints;

/// The measure tool. It shares the dimension tool's aim — `measure_pick` and
/// `refine` already know what a click means — and parts from it at the
/// answer: a dimension is placed and recorded, a measure is only shown.
pub(crate) fn read(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> bool {
    let picks = match &context.editor.tool_state {
        ToolState::Measure { picks, .. } => *picks,
        _ => DimensionPicks::default(),
    };
    let shown = showing(context);
    let Some(sketch) = context.document.sketches().get(index) else {
        return false;
    };

    // A second click on what is already shown says more about it: another
    // trait makes an angle of a length, a point makes a distance to the line,
    // a centre turns a diameter into a radius. This is the whole of what makes
    // the dimension tool smart, and the measure is no less so for reading
    // rather than recording.
    if let Some(target) = shown
        && let Some(refined) = refined(sketch, target, cursor, snap)
    {
        return show(context, picks, Some(refined), None);
    }

    let (picks, outcome) =
        cao_sketch::measure_pick(sketch, DimensionMode::Auto, picks, cursor, snap);

    // A measure left standing while the next one is gathered would be showing
    // a value nobody is asking about any more. Only `Unchanged` — a click that
    // landed on nothing new — leaves what is on screen alone.
    match outcome {
        DimensionPick::Target(target) => show(context, picks, Some(target), None),
        DimensionPick::TraitsAreParallel { first, second } => {
            show(context, picks, across_to(sketch, first, second), None)
        }
        DimensionPick::WaitingForSecondPoint => {
            show(context, picks, None, Some("sketch.choose_second_point"))
        }
        DimensionPick::WaitingForSecondTraitOrAxis => show(
            context,
            picks,
            None,
            Some("sketch.choose_second_trait_or_axis"),
        ),
        DimensionPick::WaitingForTraitAfterAxis(axis) => {
            let axis = constraints::axis(context.lang, axis);
            let said = context
                .lang
                .t_with("sketch.axis_chosen", &[("axis", &axis)]);
            show(context, picks, None, None);
            context.editor.message = Some(said);
            false
        }
        DimensionPick::PointAlreadyOnTheTrait => show(
            context,
            picks,
            None,
            Some("sketch.point_already_on_the_trait"),
        ),
        DimensionPick::PointInLineWithTheTrait => show(
            context,
            picks,
            None,
            Some("sketch.point_in_line_with_the_trait"),
        ),
        DimensionPick::Nothing => show(context, picks, None, Some("sketch.nothing_to_measure")),
        DimensionPick::Unchanged => show(context, picks, shown, None),
    }
}

/// Puts the measure on screen, and says `false` — a measure never changes the
/// part, and this is the one place that answer is written.
fn show(
    context: &mut SketchContext<'_>,
    picks: DimensionPicks,
    showing: Option<DimensionTarget>,
    message: Option<&str>,
) -> bool {
    context.editor.tool_state = ToolState::Measure { picks, showing };
    context.editor.message = message.map(|key| context.lang.t(key));
    false
}

/// What a second click adds to the measure already on screen.
///
/// The drawing's own `refine` answers for everything the dimension tool can
/// also say. What it stops at is two traits running the same way: they open no
/// angle, so a dimension has nothing to put down there. A measure does — how
/// far apart they stand — and that is one end of the first taken square onto
/// the second, which is the same reading as a point to a trait and so needs no
/// machinery of its own.
fn refined(
    sketch: &Sketch,
    target: DimensionTarget,
    cursor: DVec2,
    snap: f64,
) -> Option<DimensionTarget> {
    if let Some(refined) = sketch.refine(target, cursor, snap) {
        return Some(refined);
    }
    let DimensionTarget::Length(first) = target else {
        return None;
    };
    let second = sketch.nearest_segment(cursor, snap)?;
    if second == first || !sketch.run_the_same_way(first, second) {
        return None;
    }
    across_to(sketch, first, second)
}

/// The gap between two traits that run the same way. Which end of the first is
/// taken says nothing about the answer — that is what parallel means — so the
/// one it starts at is as good as any.
fn across_to(sketch: &Sketch, first: SegmentId, second: SegmentId) -> Option<DimensionTarget> {
    let point = sketch.segments().get(first.0)?.start;
    let target = DimensionTarget::PointToSegment {
        point,
        segment: second,
    };
    sketch.read(target).is_some().then_some(target)
}

/// What the measure tool has on screen right now, and nothing when another
/// tool is in hand.
pub(crate) fn showing(context: &SketchContext<'_>) -> Option<DimensionTarget> {
    match &context.editor.tool_state {
        ToolState::Measure { showing, .. } => *showing,
        _ => None,
    }
}

#[cfg(test)]
mod tests;
