//! The two clicks of the rectangle tool, and what a freshly-drawn rectangle
//! earns on its own: three right angles held by construction, and whichever
//! of its two sizes the user typed while dragging it.

use cao_part::Operation;
use cao_sketch::{DimensionTarget, SegmentId, ToolState};
use glam::DVec2;

use super::super::values::lay_values;
use super::point_ref_at;
use crate::screens::SketchContext;

/// One click of the rectangle tool: the first remembers a corner, the second
/// draws it opposite.
pub(crate) fn two_click_shape(
    context: &mut SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    snap: f64,
    pixel: f64,
) -> bool {
    let ToolState::Rectangle { start } = context.editor.tool_state else {
        context.editor.tool_state = ToolState::Rectangle { start: cursor };
        context.editor.live.open();
        return false;
    };
    // A shape with no extent is a stray click, not a drawing.
    if start.distance(cursor) < 1e-6 {
        return false;
    }
    context.editor.tool_state = ToolState::None;

    // Corners reuse a point already drawn when one is under the cursor, so
    // shapes hang together instead of stacking points on top of each other.
    // Nothing forces the user to place those points first.
    let corner = point_ref_at(context, index, start, snap);
    let opposite = point_ref_at(context, index, cursor, snap);
    context.document.apply(Operation::AddRectangle {
        sketch: index,
        corner,
        opposite,
        construction: context.editor.construction,
    });
    dimension_the_rectangle(context, index, pixel);
    context.editor.live.clear();
    true
}

pub(crate) fn rectangle_corner(context: &SketchContext<'_>, cursor: DVec2) -> DVec2 {
    let ToolState::Rectangle { start } = context.editor.tool_state else {
        return cursor;
    };
    cao_sketch::rectangle_corner(
        start,
        cursor,
        context.editor.live.locked(),
        context.document.scale(),
    )
}

/// Places on a fresh rectangle what makes it a rectangle, and whichever of its
/// two sizes the user typed while dragging it.
///
/// Drawing one and then having to say four times that its corners are square is
/// busywork: that is what a rectangle *is*. Three right angles are enough — the
/// fourth follows — held as constraints rather than dimensions, since a right
/// angle is a relationship the rectangle holds by construction, not a
/// measurement someone could retype. A side left untyped stays at whatever
/// length the cursor gave it, undimensioned.
fn dimension_the_rectangle(context: &mut SketchContext<'_>, index: usize, pixel: f64) {
    let count = context.document.sketches()[index].segments().len();
    let Some(first) = count.checked_sub(4) else {
        return;
    };
    let sides: [SegmentId; 4] = std::array::from_fn(|offset| SegmentId(first + offset));

    let scale = context.document.scale();
    let locked = context.editor.live.locked();
    let (corners, wanted) =
        cao_sketch::rectangle_dimensions(&context.document.sketches()[index], sides, locked, scale);

    for constraint in corners {
        context.document.apply(Operation::Constrain {
            sketch: index,
            constraint,
        });
    }
    let sizes = [0, 1].map(|rank| context.editor.live.typed_as_written(rank));
    let typed = |target| {
        sides[..2]
            .iter()
            .zip(&sizes)
            .find(|(side, _)| DimensionTarget::Length(**side) == target)
            .and_then(|(_, size)| size.clone())
    };
    lay_values(context, index, wanted, typed, pixel);
}
