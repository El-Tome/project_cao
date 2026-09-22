//! An angle between two traits that share no end, drawn as an arc about where
//! their lines cross.

use glam::DVec2;

use super::angle::{Arm, angular, clearance, radius_of};
use super::{AnnotationMetrics, Moved};
use crate::constraints::Toward;
use crate::sketch::{SegmentId, Sketch};

/// Where the two traits meet — an X, a T — the arc stands just off that place,
/// the way a corner's does.
///
/// Where they lie apart, the place their lines cross can be anywhere, often
/// well off the screen, and an arc drawn beside it could be neither seen nor
/// grabbed. The arc is drawn out between the two traits instead, halfway along
/// where they run side by side, each of its ends landing on one of them — and
/// kept there, since the crossing it turns about swings a long way for a small
/// turn. An end that falls past a trait is joined back to it by a thin line, as
/// on a drawing.
pub(super) fn between_traits(
    out: &mut Vec<(DVec2, DVec2)>,
    sketch: &Sketch,
    arms: [(SegmentId, Toward); 2],
    by: Moved,
    metrics: AnnotationMetrics,
) -> Option<(DVec2, DVec2)> {
    let [(first, first_toward), (second, second_toward)] = arms;
    let pivot = sketch.where_lines_cross(first, second)?;
    let (one, other) = (
        sketch.arm(first, first_toward),
        sketch.arm(second, second_toward),
    );
    let apart = sketch.where_traits_meet(first, second).is_none();
    // Held halfway between two traits lying apart whatever was recorded for
    // it: a place kept against their far crossing would follow that crossing
    // off the screen the first time a value typed turned them.
    let by = match apart {
        true => Moved {
            placed: Some(halfway_between(
                sketch,
                pivot,
                [(first, one), (second, other)],
                metrics,
            )),
            nudge: DVec2::ZERO,
        },
        false => by,
    };

    let (text_at, reach) = angular(
        out,
        pivot,
        Arm::Drawn(pivot + one),
        pivot + other,
        by,
        metrics,
    );
    if apart {
        let radius = radius_of(reach, metrics);
        for (segment, arm) in [(first, one), (second, other)] {
            reach_back(out, sketch, segment, pivot + arm.normalize() * radius);
        }
    }
    Some((text_at, reach))
}

/// Where the value of an angle between two traits lying apart goes until it is
/// put down by hand: on the bisector, at the reach that sets the arc halfway
/// along the stretch over which both traits run — or, when one runs out before
/// the other starts, halfway between their middles.
fn halfway_between(
    sketch: &Sketch,
    pivot: DVec2,
    arms: [(SegmentId, DVec2); 2],
    metrics: AnnotationMetrics,
) -> DVec2 {
    // How far out each trait lies from the crossing, whichever way along its
    // line it lies: a trait lying apart from the other sits wholly on one side.
    let extent = |(segment, _): (SegmentId, DVec2)| {
        let (from, to) = sketch.endpoints(segment);
        let (near, far) = (from.distance(pivot), to.distance(pivot));
        (near.min(far), near.max(far))
    };
    let [(one_near, one_far), (other_near, other_far)] = arms.map(extent);
    let (near, far) = (one_near.max(other_near), one_far.min(other_far));
    let radius = match near <= far {
        true => (near + far) * 0.5,
        false => (one_near + one_far + other_near + other_far) * 0.25,
    };
    let bisector = (arms[0].1.normalize() + arms[1].1.normalize()).normalize();
    bisector * (radius + clearance(metrics))
}

/// Joins an arc's end back to its trait with a thin line, when the end falls
/// past one of the trait's own ends.
fn reach_back(out: &mut Vec<(DVec2, DVec2)>, sketch: &Sketch, segment: SegmentId, end: DVec2) {
    let (from, to) = sketch.endpoints(segment);
    let span = to - from;
    let along = (end - from).dot(span) / span.length_squared();
    if along < 0.0 {
        out.push((from, end));
    } else if along > 1.0 {
        out.push((to, end));
    }
}
