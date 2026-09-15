//! What one click of the arc tool does, and what the canvas shows in between.

use cao_part::{Operation, PointRef};
use cao_sketch::{ArcDraft, ArcId, ArcMode, PointId, Sketch, ToolState, arc_aimed, arc_from};
use glam::DVec2;

use super::{annotation_position, point_ref_at};
use crate::screens::viewport::SketchContext;
use crate::wording::outcome;

/// One click of the arc tool: takes the place pointed at, and draws the arc as
/// soon as enough of it is known.
pub(crate) fn draw_arc(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let mode = context.editor.arc_mode;
    let (mut places, first_typed) = places_so_far(context);
    let asks_for = crate::wording::arc::asks_for(context.lang, mode);
    let cursor = aimed(context, &places, cursor);

    if places.len() + 1 < mode.wants() {
        // The field that decides this leg is about to be cleared and handed
        // to the next: whether it was typed has to be carried forward by
        // hand, or the arc could never be dimensioned for it once settled.
        let first_typed = context.editor.live.typed(0).is_some();
        places.push(cursor);
        context.editor.tool_state = ToolState::Arc {
            places,
            first_typed,
        };
        context.editor.live.open();
        context.editor.message = Some(asks_for);
        return false;
    }

    let Some(drawn) = arc_from(mode, &places, cursor) else {
        context.editor.message = Some(context.lang.t("sketch.no_arc_from_these"));
        return false;
    };
    let second_typed = context.editor.live.typed(0).is_some();

    // Each of the three reuses a point already drawn when one is under it, as
    // everywhere else, so an arc hangs off what is there instead of stacking
    // points on top of it.
    let center = point_ref_at(context, index, drawn.centre, snap);
    let start = point_ref_at(context, index, drawn.start, snap);
    let end = point_ref_at(context, index, drawn.end, snap);

    context.editor.tool_state = ToolState::None;
    context.document.apply(Operation::AddArc {
        sketch: index,
        center,
        start,
        end,
        construction: context.editor.construction,
    });
    let arc = ArcId(
        context.document.sketches()[index]
            .arcs()
            .len()
            .saturating_sub(1),
    );
    dimension_the_arc(context, index, arc, mode, first_typed, second_typed, pixel);
    // The angle's two arms have nothing else to open from: unlike a radius,
    // which draws its own leader out to the rim, a swept angle draws only the
    // arc between them, so they are left behind as construction geometry when
    // the sweep was the value being pinned down.
    if mode == ArcMode::ByCenter && second_typed {
        construct_the_sector(context, index, arc);
    }
    context.editor.live.clear();
    context.editor.message = Some(asks_for);
    true
}

/// The arc a click right now would draw, for the canvas to show first.
pub(crate) fn arc_preview(context: &SketchContext<'_>, cursor: DVec2) -> Option<ArcDraft> {
    let (places, _) = places_so_far(context);
    let cursor = aimed(context, &places, cursor);
    arc_from(context.editor.arc_mode, &places, cursor)
}

fn places_so_far(context: &SketchContext<'_>) -> (Vec<DVec2>, bool) {
    match &context.editor.tool_state {
        ToolState::Arc {
            places,
            first_typed,
        } => (places.clone(), *first_typed),
        _ => (Vec::new(), false),
    }
}

/// Places on the arc just drawn whatever radius, sweep or distance the user
/// typed for it.
fn dimension_the_arc(
    context: &mut SketchContext<'_>,
    index: usize,
    arc: ArcId,
    mode: ArcMode,
    first_typed: bool,
    second_typed: bool,
    pixel: f64,
) {
    let scale = context.document.scale();
    let wanted = cao_sketch::arc_dimensions(
        &context.document.sketches()[index],
        arc,
        mode,
        first_typed,
        second_typed,
        scale,
    );
    for (target, value) in wanted {
        let applied = context.document.apply(Operation::SetDimension {
            sketch: index,
            target,
            value,
            placement: annotation_position(context, index, target, pixel)
                .map(|placement| placement.offset),
        });
        if let Some(message) = outcome::message(context.lang, applied) {
            context.editor.message = Some(message);
        }
    }
}

/// The two radii a swept angle opens between, left on the drawing as
/// construction geometry so the angle it was dimensioned with has something
/// to show for it.
fn construct_the_sector(context: &mut SketchContext<'_>, index: usize, arc: ArcId) {
    let drawn = context.document.sketches()[index].arc(arc);
    for end in [drawn.start, drawn.end] {
        context.document.apply(Operation::AddSegment {
            sketch: index,
            start: PointRef::Existing(drawn.center),
            end: PointRef::Existing(end),
            construction: true,
        });
    }
}

/// The cursor, once a value typed into the live field has had its say. What
/// that value means at each stage is `cao_sketch`'s to decide; this only
/// hands over what has been picked so far and what was typed.
pub(crate) fn aimed(context: &SketchContext<'_>, places: &[DVec2], cursor: DVec2) -> DVec2 {
    arc_aimed(
        context.editor.arc_mode,
        places,
        cursor,
        context.editor.live.typed(0),
        context.document.scale(),
    )
}

/// The points a drag on `point` would carry along, when it is the centre of
/// an arc: the centre and its two ends move as one, the way a circle's rim
/// follows a dragged centre. `None` for a point that is nobody's centre, so a
/// plain drag of it stays a plain drag.
pub(crate) fn arc_centre_group(sketch: &Sketch, point: PointId) -> Option<Vec<PointId>> {
    let ends = sketch.arc_ends_around(point);
    (!ends.is_empty()).then(|| {
        let mut group = vec![point];
        group.extend(ends);
        group
    })
}
