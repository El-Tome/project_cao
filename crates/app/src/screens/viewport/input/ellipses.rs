//! What one click of the ellipse tool does, and the ellipse the clicks so far
//! make.

use cao_part::{Operation, PointRef};
use cao_sketch::{
    ELLIPSE_PLACES, EllipseDraft, EllipseId, LockedInput, ToolState, ellipse_aimed,
    ellipse_dimensions, ellipse_from,
};
use glam::DVec2;

use super::{annotation_position, point_ref_at};
use crate::screens::viewport::SketchContext;
use crate::wording::outcome;

/// One click of the ellipse tool: the centre, then the end of the first axis,
/// then how far the second reaches — which draws the ellipse, its two axes,
/// and whatever was typed on the way as dimensions on those axes.
pub(crate) fn draw_ellipse(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let (mut places, first_typed) = places_so_far(context);
    let cursor = aimed(context, &places, cursor);

    if places.len() + 1 < ELLIPSE_PLACES {
        // The fields that decided the first axis are cleared and handed to the
        // second: what was typed into them is carried here, or the ellipse
        // could never be dimensioned for it once drawn.
        let first_typed = match places.len() {
            1 => context.editor.live.locked(),
            _ => first_typed,
        };
        places.push(cursor);
        context.editor.tool_state = ToolState::Ellipse {
            places,
            first_typed,
        };
        context.editor.live.open();
        context.editor.message = Some(context.lang.t("ellipse.asks_for"));
        return false;
    }

    let Some(drawn) = ellipse_from(&places, cursor) else {
        context.editor.message = Some(context.lang.t("sketch.no_ellipse_from_these"));
        return false;
    };
    let second_width = context.editor.live.typed(0);

    // Only the two places clicked reuse what is under them. The other three
    // are worked out rather than aimed at, and a point that happens to lie
    // near one of them would tie the ellipse to whatever holds it.
    let across = drawn.second_axis();
    let center = point_ref_at(context, index, drawn.centre, snap);
    let first = [
        PointRef::New(drawn.centre - drawn.first),
        point_ref_at(context, index, drawn.centre + drawn.first, snap),
    ];
    let second = [drawn.centre - across, drawn.centre + across].map(PointRef::New);

    context.editor.tool_state = ToolState::None;
    context.document.apply(Operation::AddEllipse {
        sketch: index,
        center,
        first,
        second,
        construction: context.editor.construction,
    });
    let ellipse = EllipseId(
        context.document.sketches()[index]
            .ellipses()
            .len()
            .saturating_sub(1),
    );
    dimension_the_ellipse(context, index, ellipse, first_typed, second_width, pixel);
    context.editor.live.clear();
    context.editor.message = Some(context.lang.t("ellipse.asks_for"));
    true
}

/// The ellipse a click right now would draw, for the canvas to show first.
pub(crate) fn ellipse_preview(context: &SketchContext<'_>, cursor: DVec2) -> Option<EllipseDraft> {
    let (places, _) = places_so_far(context);
    ellipse_from(&places, aimed(context, &places, cursor))
}

/// The cursor, once the values typed at it have had their say.
pub(crate) fn ellipse_aimed_at(context: &SketchContext<'_>, cursor: DVec2) -> DVec2 {
    let (places, _) = places_so_far(context);
    aimed(context, &places, cursor)
}

fn aimed(context: &SketchContext<'_>, places: &[DVec2], cursor: DVec2) -> DVec2 {
    ellipse_aimed(
        places,
        cursor,
        context.editor.live.locked(),
        context.document.scale(),
    )
}

fn places_so_far(context: &SketchContext<'_>) -> (Vec<DVec2>, LockedInput) {
    match &context.editor.tool_state {
        ToolState::Ellipse {
            places,
            first_typed,
        } => (places.clone(), *first_typed),
        _ => (Vec::new(), LockedInput::default()),
    }
}

/// Places on the axes of the ellipse just drawn the widths and the angle typed
/// for it.
fn dimension_the_ellipse(
    context: &mut SketchContext<'_>,
    index: usize,
    ellipse: EllipseId,
    first_typed: LockedInput,
    second_width: Option<f64>,
    pixel: f64,
) {
    let scale = context.document.scale();
    let wanted = ellipse_dimensions(
        &context.document.sketches()[index],
        ellipse,
        first_typed,
        second_width,
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

#[cfg(test)]
mod tests;
