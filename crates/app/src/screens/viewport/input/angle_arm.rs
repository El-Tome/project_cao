//! The arm a typed angle is read against, laid beside the trait just drawn.

use cao_part::formula::Operator;
use cao_part::history::PointRef;
use cao_part::{Formula, Operation};
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
    // The angle typed for the trait — or, for an ellipse, for its first axis,
    // typed before the fields opened again for the second.
    let typed = context
        .editor
        .live
        .typed_as_written(1)
        .or_else(|| context.editor.live.carried(1));
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
        value: as_opened(typed, opened),
        placement: annotation_position(context, index, target, pixel)
            .map(|placement| placement.offset),
    });
}

/// The angle typed, written so that it comes to the angle the drawing opened.
///
/// The drawing reads a trait against an arm that always runs east, so a trait
/// typed at 30° and drawn the other way opens 150°, and one typed at -30°
/// opens 30°: the formula is turned the same way. An angle opened otherwise
/// is the number the drawing reads, as a plain number typed always was.
pub(crate) fn as_opened(typed: Option<(Formula, f64)>, opened: f64) -> Formula {
    let Some((written, value)) = typed.filter(|(written, _)| written.as_number().is_none()) else {
        return Formula::Number(opened);
    };
    let near = |candidate: f64| (candidate - opened).abs() < 1e-6;
    let from_half_a_turn = |operator| {
        Formula::Combined(
            operator,
            Box::new(Formula::Number(180.0)),
            Box::new(written.clone()),
        )
    };
    match () {
        _ if near(value) => written,
        _ if near(-value) => Formula::Negative(Box::new(written)),
        _ if near(180.0 - value) => from_half_a_turn(Operator::Subtract),
        _ if near(180.0 + value) => from_half_a_turn(Operator::Add),
        _ => Formula::Number(opened),
    }
}

#[cfg(test)]
mod tests;
