//! What a freshly-drawn rectangle earns on its own: three right angles held
//! by construction, and whichever of its two sizes the user typed while
//! dragging it.

use cao_part::Operation;
use cao_sketch::SegmentId;

use super::annotation_position;
use crate::screens::viewport::SketchContext;

/// Places on a fresh rectangle what makes it a rectangle, and whichever of its
/// two sizes the user typed while dragging it.
///
/// Drawing one and then having to say four times that its corners are square is
/// busywork: that is what a rectangle *is*. Three right angles are enough — the
/// fourth follows — held as constraints rather than dimensions, since a right
/// angle is a relationship the rectangle holds by construction, not a
/// measurement someone could retype. A side left untyped stays at whatever
/// length the cursor gave it, undimensioned.
pub(crate) fn dimension_the_rectangle(context: &mut SketchContext<'_>, index: usize, pixel: f64) {
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
    for (target, value) in wanted {
        // Pinned down where it is drawn, in sketch units: left to stand off by
        // a distance in pixels, an annotation slides back over the drawing as
        // soon as one zooms out.
        context.document.apply(Operation::SetDimension {
            sketch: index,
            target,
            value,
            placement: annotation_position(context, index, target, pixel)
                .map(|placement| placement.offset),
        });
    }
}
