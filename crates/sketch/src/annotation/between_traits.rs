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
/// grabbed. Until it is put down by hand, the arc is drawn out between the two
/// traits instead, halfway along where they run side by side, each of its ends
/// landing on one of them; put down, it goes where it was put and moves from
/// there. An end that falls past a trait is joined back to it by a thin line,
/// as on a drawing. A T's stem read past its foot is drawn the same way,
/// prolonged out to the arc.
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
    // Between two traits lying apart, where the value was put down is kept as
    // how far it stands from halfway between them, not from their crossing:
    // that crossing swings a long way for a small turn, and a place kept
    // against it would follow it off the screen the first time a value typed
    // turned the traits.
    let halfway =
        apart.then(|| halfway_between(sketch, pivot, [(first, one), (second, other)], metrics));
    let by = match halfway {
        Some(halfway) => Moved {
            placed: Some(
                by.placed
                    .map_or(halfway, |from_halfway| halfway + from_halfway),
            ),
            ..by
        },
        None => by,
    };

    let (text_at, reach) = angular(
        out,
        pivot,
        Arm::Drawn(pivot + one),
        Arm::Drawn(pivot + other),
        by,
        metrics,
    );
    let radius = radius_of(reach, metrics);
    for (segment, arm) in [(first, one), (second, other)] {
        if apart || runs_past(sketch, segment, pivot, arm) {
            reach_back(out, sketch, segment, pivot + arm.normalize() * radius);
        }
    }
    Some((text_at, reach - halfway.unwrap_or(DVec2::ZERO)))
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

/// Whether an arm runs out from the pivot along the trait's prolongation, the
/// whole trait behind it: a T's stem read past its foot. Through an X the arm
/// runs into its trait, short as it may be, and is drawn as it always was.
fn runs_past(sketch: &Sketch, segment: SegmentId, pivot: DVec2, arm: DVec2) -> bool {
    const BEHIND: f64 = 1e-9;
    let (from, to) = sketch.endpoints(segment);
    let ahead = |end: DVec2| (end - pivot).dot(arm);
    ahead(from).max(ahead(to)) <= BEHIND * arm.length_squared()
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
