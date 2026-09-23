//! What one click of the ellipse tool does, and the ellipse the clicks so far
//! make.

use cao_part::{Operation, PointRef};
use cao_sketch::{
    EllipseDraft, EllipseId, EllipseMode, LockedInput, ToolState, ellipse_aimed,
    ellipse_dimensions, ellipse_from, half_between,
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
    let mode = context.editor.ellipse_mode;
    let (mut places, first_typed) = places_so_far(context);
    let cursor = aimed(context, &places, cursor);

    if places.len() + 1 < mode.wants() {
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
        context.editor.message = Some(crate::wording::ellipse::asks_for(context.lang, mode));
        return false;
    }

    let Some(drawn) = ellipse_from(mode, &places, cursor) else {
        context.editor.message = Some(context.lang.t("sketch.no_ellipse_from_these"));
        return false;
    };
    // Placed from its two ends, what is typed at the third click is the rise,
    // which is half of what the second axis measures across.
    let second_width = context.editor.live.typed(0).map(|typed| match mode {
        EllipseMode::ByCentre => typed,
        EllipseMode::ByEnds => typed * 2.0,
    });

    // Only the places actually clicked reuse what is under them. The others
    // are worked out rather than aimed at, and a point that happens to lie
    // near one of them would tie the ellipse to whatever holds it.
    let across = drawn.second_axis();
    let (center, first) = match mode {
        EllipseMode::ByCentre => (
            point_ref_at(context, index, drawn.centre, snap),
            [
                PointRef::New(drawn.centre - drawn.first),
                point_ref_at(context, index, drawn.centre + drawn.first, snap),
            ],
        ),
        EllipseMode::ByEnds => (
            PointRef::New(drawn.centre),
            [
                point_ref_at(context, index, drawn.centre - drawn.first, snap),
                point_ref_at(context, index, drawn.centre + drawn.first, snap),
            ],
        ),
    };
    let second = [drawn.centre - across, drawn.centre + across].map(PointRef::New);
    // Only the half the rise fell on is drawn, and the stretch runs between the
    // very points the first axis stands on — named by the same references, so
    // that laying it does not put a second point on top of each end.
    let stretch = match mode {
        EllipseMode::ByCentre => None,
        EllipseMode::ByEnds => Some(half_between(drawn, cursor).map(|rank| first[rank].clone())),
    };

    context.editor.tool_state = ToolState::None;
    context.document.apply(Operation::AddEllipse {
        sketch: index,
        center,
        first,
        second,
        construction: context.editor.construction,
        drawn: stretch,
    });
    let ellipse = EllipseId(
        context.document.sketches()[index]
            .ellipses()
            .len()
            .saturating_sub(1),
    );
    dimension_the_ellipse(context, index, ellipse, first_typed, second_width, pixel);
    context.editor.live.clear();
    context.editor.message = Some(crate::wording::ellipse::asks_for(context.lang, mode));
    true
}

/// The ellipse a click right now would draw, for the canvas to show first.
pub(crate) fn ellipse_preview(context: &SketchContext<'_>, cursor: DVec2) -> Option<EllipseDraft> {
    let (places, _) = places_so_far(context);
    ellipse_from(
        context.editor.ellipse_mode,
        &places,
        aimed(context, &places, cursor),
    )
}

/// The cursor, once the values typed at it have had their say.
pub(crate) fn ellipse_aimed_at(context: &SketchContext<'_>, cursor: DVec2) -> DVec2 {
    let (places, _) = places_so_far(context);
    aimed(context, &places, cursor)
}

fn aimed(context: &SketchContext<'_>, places: &[DVec2], cursor: DVec2) -> DVec2 {
    ellipse_aimed(
        context.editor.ellipse_mode,
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
