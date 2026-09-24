//! The arm a typed angle is read against, laid beside the trait just drawn.

use cao_part::Operation;
use cao_part::history::PointRef;
use cao_sketch::{
    AngleArm, Constraint, DimensionTarget, PointId, SegmentId, SketchAxis, angle_arm,
};

use super::annotation_position;
use crate::screens::SketchContext;

/// How short an arm may get before the zoom says how long it is rather than
/// the trait's own width: a trait standing straight up is no wider than
/// nothing, and an arm that short is one nobody can take hold of.
const LEAST_PIXELS: f64 = 40.0;

/// Lays what holds the angle of a trait drawn at a typed angle: a construction
/// arm running east along the horizontal, with the angle read between the two
/// — or, for a trait square to an axis it starts on, the rule saying it lies
/// on that axis, which says the same with one line fewer on screen.
pub(crate) fn lean_on_an_arm(
    context: &mut SketchContext<'_>,
    index: usize,
    drawn: SegmentId,
    from: PointId,
    pixel: f64,
) {
    let Some(sketch) = context.document.sketches().get(index) else {
        return;
    };
    let Some(leaning) = angle_arm(sketch, drawn, from, pixel * LEAST_PIXELS) else {
        return;
    };
    let arm = SegmentId(sketch.segments().len());

    let reaches = match leaning {
        AngleArm::Axis(axis) => {
            context.document.apply(Operation::Constrain {
                sketch: index,
                constraint: Constraint::AxisCollinear {
                    segment: drawn,
                    axis,
                },
            });
            return;
        }
        AngleArm::Arm(reaches) => reaches,
    };

    context.document.apply(Operation::AddSegment {
        sketch: index,
        start: PointRef::Existing(from),
        end: PointRef::New(reaches),
        construction: true,
    });
    context.document.apply(Operation::Constrain {
        sketch: index,
        constraint: Constraint::AxisParallel {
            segment: arm,
            axis: SketchAxis::U,
        },
    });

    // Read off the drawing rather than repeated from what was typed: the
    // direction follows the cursor, so a trait typed at 30° and drawn the
    // other way opens 150° against an arm that always runs east.
    //
    // A corner names its one angle; an arm springing from a symmetric line's
    // middle shares no end with it, and is read the way two traits that meet
    // are.
    let sketch = &context.document.sketches()[index];
    let Some((target, opened)) = sketch
        .angle_between(arm, drawn)
        .map(|opened| {
            (
                DimensionTarget::Angle {
                    first: arm,
                    second: drawn,
                },
                opened,
            )
        })
        .or_else(|| {
            let target = sketch.angle_between_traits(arm, drawn);
            Some((target, sketch.opening(target)?))
        })
    else {
        return;
    };
    context.document.apply(Operation::SetDimension {
        sketch: index,
        target,
        value: opened,
        placement: annotation_position(context, index, target, pixel)
            .map(|placement| placement.offset),
    });
}

#[cfg(test)]
mod tests;
