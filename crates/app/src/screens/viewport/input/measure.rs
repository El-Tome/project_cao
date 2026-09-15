//! What a click of the smart dimension tool does: what it measures, where the
//! annotation lands, what it shows before the click, and reopening one already
//! on the drawing.

use cao_part::Operation;
use cao_sketch::{DimensionMode, DimensionTarget, ToolState};
use glam::DVec2;

use crate::screens::viewport::SketchContext;
use crate::wording::{constraints, dimension, outcome};

use super::{annotation_position, nearest_annotation, refine};

/// The smart dimension tool: works out what is under the cursor and measures
/// it, unless a mode is forcing one kind.
///
/// Two-step measurements — point to point, angle — collect their first half and
/// wait; everything else is settled in a single click.
pub(super) fn measure(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let mode = context.editor.dimension_mode;

    // A dimension already chosen is waiting to be put down. This click either
    // adds the second half of a pair — the segment that makes it an angle, the
    // point that makes it a distance to a line — or says where it goes.
    let placing = match &context.editor.tool_state {
        ToolState::Dimension { placing, .. } => *placing,
        _ => None,
    };
    if let Some(target) = placing {
        if mode == DimensionMode::Auto
            && let Some(refined) = refine(context, index, target, cursor, snap)
        {
            if let ToolState::Dimension { placing, .. } = &mut context.editor.tool_state {
                *placing = Some(refined);
            }
            context.editor.message = Some(context.lang.t("sketch.place_the_dimension"));
            return false;
        }
        if let ToolState::Dimension { placing, .. } = &mut context.editor.tool_state {
            *placing = None;
        }
        return place_dimension(context, index, target, cursor, pixel);
    }

    let sketch = &context.document.sketches()[index];

    // An annotation under the cursor, with no geometry there to take the click
    // first, means changing its value — never laying a second copy over it.
    let on_geometry = sketch.nearest_point(cursor, snap * 0.8).is_some()
        || sketch.nearest_segment(cursor, snap).is_some()
        || sketch.nearest_circle(cursor, snap).is_some();
    if !on_geometry
        && let Some(target) = nearest_annotation(context, index, cursor, snap * 1.5, pixel)
    {
        edit_dimension(context, index, target);
        return false;
    }

    let picks = match &context.editor.tool_state {
        ToolState::Dimension { picks, .. } => *picks,
        _ => cao_sketch::DimensionPicks::default(),
    };
    let sketch = &context.document.sketches()[index];
    let (picks, outcome) = cao_sketch::measure_pick(sketch, mode, picks, cursor, snap);
    context.editor.tool_state = ToolState::Dimension {
        placing: None,
        picks,
    };

    match outcome {
        cao_sketch::DimensionPick::Target(target) => select_target(context, index, target),
        cao_sketch::DimensionPick::WaitingForSecondPoint => {
            context.editor.message = Some(context.lang.t("sketch.choose_second_point"));
        }
        cao_sketch::DimensionPick::WaitingForSecondTraitOrAxis => {
            context.editor.message = Some(context.lang.t("sketch.choose_second_trait_or_axis"));
        }
        cao_sketch::DimensionPick::WaitingForTraitAfterAxis(axis) => {
            let axis = constraints::axis(context.lang, axis);
            context.editor.message = Some(
                context
                    .lang
                    .t_with("sketch.axis_chosen", &[("axis", &axis)]),
            );
        }
        cao_sketch::DimensionPick::TraitsDoNotTouch => {
            context.editor.select(None, None);
            context.editor.message = Some(context.lang.t("sketch.traits_do_not_touch"));
        }
        cao_sketch::DimensionPick::Nothing => {
            context.editor.select(None, None);
            context.editor.message = Some(context.lang.t("sketch.nothing_to_measure"));
        }
        cao_sketch::DimensionPick::Unchanged => {}
    }
    false
}

/// Puts down the dimension that was waiting, where the click landed.
///
/// The value it starts with is what the geometry already measures, so placing
/// one never moves the drawing; typing another is what does.
pub(super) fn place_dimension(
    context: &mut SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
    cursor: DVec2,
    pixel: f64,
) -> bool {
    // Where the cursor is says which of the three readings of a slanted trait
    // is wanted, so it is settled here, at the click that puts the cote down.
    let target = context
        .document
        .sketches()
        .get(index)
        .map_or(target, |sketch| sketch.oriented(target, cursor));

    // The reading asked for is already on the drawing: show its value rather
    // than lay a second copy over it.
    if context.document.sketches()[index]
        .dimension_of(target)
        .is_some()
    {
        edit_dimension(context, index, target);
        return false;
    }

    let Some(value) = context.document.measured(index, target) else {
        return false;
    };
    let applied = context.document.apply(Operation::SetDimension {
        sketch: index,
        target,
        value,
        // What the annotation has to be moved by for its value to land on the
        // cursor: a linear or angular annotation follows its offset exactly,
        // so the gap between where the value is and where the cursor is *is*
        // the movement.
        placement: Some(
            annotation_position(context, index, target, pixel)
                .map(|placement| placement.offset + (cursor - placement.text_at))
                .unwrap_or_default(),
        ),
    });

    context.editor.select(Some(target), Some(value));
    context.editor.message = outcome::message(context.lang, applied);
    true
}

/// The dimension a click would place right now, without placing it.
///
/// The same reading of the cursor as `measure`, so what is shown in advance is
/// what will actually be recorded — two separate readings would eventually
/// disagree, and a preview that lies is worse than none.
pub(crate) fn measure_preview(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
) -> Option<DimensionTarget> {
    let mode = context.editor.dimension_mode;
    let sketch = context.document.sketches().get(index)?;
    let picks = match &context.editor.tool_state {
        ToolState::Dimension { picks, .. } => *picks,
        _ => cao_sketch::DimensionPicks::default(),
    };
    match cao_sketch::measure_pick(sketch, mode, picks, cursor, snap).1 {
        cao_sketch::DimensionPick::Target(target) => Some(target),
        _ => None,
    }
}

/// Takes hold of what was clicked; the annotation then follows the cursor until
/// a second click says where it goes.
///
/// Two clicks rather than one because a dimension dropped on top of the shape
/// it measures has to be dragged off it anyway — this way it lands where it
/// belongs from the start.
fn select_target(context: &mut SketchContext<'_>, index: usize, target: DimensionTarget) {
    let target = target.normalised();

    // The same measurement clicked again is the one already there: showing its
    // value to be retyped is what the user is after, not a second copy of it
    // laid over the first. A slanted trait is the exception — it has a width
    // and a height to offer besides its length, and which one is wanted is only
    // known once the cote is placed.
    if !context.document.sketches()[index].is_slanted(target)
        && context.document.sketches()[index]
            .dimension_of(target)
            .is_some()
    {
        return edit_dimension(context, index, target);
    }

    context.editor.select(None, None);
    match &mut context.editor.tool_state {
        ToolState::Dimension { placing, .. } => *placing = Some(target),
        _ => {
            context.editor.tool_state = ToolState::Dimension {
                placing: Some(target),
                picks: cao_sketch::DimensionPicks::default(),
            }
        }
    }

    let scale = context.document.scale();
    context.editor.message = Some(
        if context.document.sketches()[index].would_be_redundant(target, scale) {
            dimension::redundant_warning(context.lang)
        } else {
            context.lang.t("sketch.place_the_dimension")
        },
    );
}

/// Opens a dimension already on the drawing for editing, its value in the field
/// ready to be replaced.
pub(super) fn edit_dimension(
    context: &mut SketchContext<'_>,
    index: usize,
    target: DimensionTarget,
) {
    let value = context.document.sketches()[index]
        .dimension_of(target)
        .map(|dimension| dimension.value)
        .or_else(|| context.document.measured(index, target));
    // Only the dimension tool's own progress is dropped: this is also reached
    // from the selection tool, whose held selection must survive it.
    if let ToolState::Dimension { .. } = &context.editor.tool_state {
        context.editor.tool_state = ToolState::None;
    }
    context.editor.select(Some(target), value);
    context.editor.message = None;
}
