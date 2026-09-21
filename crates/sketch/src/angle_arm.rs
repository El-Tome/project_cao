//! What a trait drawn at a typed angle has that angle read against.
//!
//! #301 drew the first arm of the reading as a line the drawing did not hold,
//! which is what a screenshot needs and not what a drawing does. A real trait
//! is geometry like any other: it can be grabbed, measured and leaned on.

use glam::DVec2;

use crate::constraints::SketchAxis;
use crate::sketch::{PointId, SegmentId, Sketch};

/// How far off an axis a place may sit and still be *on* it. A fact about the
/// drawing rather than about the view: a point put on the axis lands on it to
/// within rounding, and one merely placed nearby is somewhere else.
const ON_THE_AXIS: f64 = 1e-9;

/// What a trait drawn at a typed angle has its angle read against.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AngleArm {
    /// A construction arm springing from the trait's own start and running
    /// east along the horizontal, as far as this place.
    Arm(DVec2),
    /// The axis of the drawing the trait already lies along, starting on it: a
    /// rule says so with one line fewer on screen.
    Axis(SketchAxis),
}

/// What holds the angle of a trait just drawn at a typed angle.
///
/// `from` is where the arm springs from — the trait's start, or a symmetric
/// line's middle. `least` is the shortest arm worth laying, in sketch units:
/// a trait standing straight up has no width at all, and an arm as wide as
/// that is one nobody can see, let alone take hold of.
///
/// Nothing when the trait names nothing the drawing has.
pub fn angle_arm(
    sketch: &Sketch,
    segment: SegmentId,
    from: PointId,
    least: f64,
) -> Option<AngleArm> {
    let (start, end) = sketch.endpoints(segment);
    let springs_from = sketch.points().get(from.0).copied()?;
    let along = (end - start).normalize_or_zero();
    if along == DVec2::ZERO {
        return None;
    }

    for (axis, off_the_axis, across) in [
        (SketchAxis::U, springs_from.y, along.y),
        (SketchAxis::V, springs_from.x, along.x),
    ] {
        if across.abs() < ON_THE_AXIS && off_the_axis.abs() < ON_THE_AXIS {
            return Some(AngleArm::Axis(axis));
        }
    }

    let width = (end.x - start.x).abs().max(least);
    Some(AngleArm::Arm(springs_from + DVec2::X * width))
}
