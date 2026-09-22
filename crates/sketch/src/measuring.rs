//! What one click of the dimension tool does.
//!
//! The smart dimension tool works out what is under the cursor and measures
//! it, unless a mode is forcing one kind. Two-step measurements — point to
//! point, angle — collect their first half and wait; everything else is
//! settled in a single click.

use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
use crate::dimensioning::axis_under;
use crate::sketch::{PointId, SegmentId, Sketch};
use crate::trimming::ON_THE_TRAIT;

/// What the smart dimension tool is allowed to measure.
///
/// `Auto` takes whatever is under the cursor, which covers most of the work.
/// The others force one kind, for when two things overlap and the wrong one
/// keeps winning.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DimensionMode {
    #[default]
    Auto,
    /// Between two points, joined or not.
    PointToPoint,
    /// The length of a segment, by clicking the segment itself.
    Length,
    /// Between two segments, or a segment and a sketch axis.
    Angle,
    /// The radius of a circle.
    Radius,
}

impl DimensionMode {
    /// Whether this mode may pick a point.
    pub fn takes_points(self) -> bool {
        matches!(self, Self::Auto | Self::PointToPoint)
    }
}

/// What the dimension tool has picked so far, on the way to a target it can
/// place: a point waiting for its partner, or a trait waiting for the second
/// half of an angle.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DimensionPicks {
    pub first_point: Option<PointId>,
    pub first_angle_segment: Option<SegmentId>,
    pub first_axis: Option<SketchAxis>,
}

/// What one click of the dimension tool does, before a target is placed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DimensionPick {
    /// Enough is known to place this.
    Target(DimensionTarget),
    /// A point taken as the first half of a distance.
    WaitingForSecondPoint,
    /// A trait taken as the first half of an angle.
    WaitingForSecondTraitOrAxis,
    /// An axis taken first, waiting for the trait to measure it against.
    WaitingForTraitAfterAxis(SketchAxis),
    /// Two traits picked for an angle, but they run the same way: their lines
    /// never cross, so there is no angle between them to read.
    TraitsAreParallel,
    /// A point picked for a distance to a trait it already lies on: there is
    /// no distance to measure, and a zero laid there could only ever be
    /// read, never typed.
    PointAlreadyOnTheTrait,
    /// The same for a point on the trait's line but beyond its ends: the
    /// distance square to the line is nothing there too, and calling the point
    /// "on the trait" would not be true.
    PointInLineWithTheTrait,
    /// Nothing under the cursor answers to what this mode measures.
    Nothing,
    /// The click changes nothing already under way.
    Unchanged,
}

/// One click of the dimension tool. What is already under way — a point
/// half-picked, a trait waiting for its angle's second half — is `picks`,
/// carried in and handed back changed.
pub fn measure_pick(
    sketch: &Sketch,
    mode: DimensionMode,
    picks: DimensionPicks,
    cursor: DVec2,
    snap: f64,
) -> (DimensionPicks, DimensionPick) {
    // A point wins over a segment under the same cursor: it is the smaller
    // target, so aiming at it is the deliberate act.
    if mode.takes_points()
        && let Some(point) = sketch.nearest_point(cursor, snap * 0.8)
    {
        return match picks.first_point {
            None => (
                DimensionPicks {
                    first_point: Some(point),
                    ..picks
                },
                DimensionPick::WaitingForSecondPoint,
            ),
            Some(first) if first == point => (picks, DimensionPick::Unchanged),
            Some(first) => (
                DimensionPicks {
                    first_point: None,
                    ..picks
                },
                DimensionPick::Target(DimensionTarget::Distance {
                    from: first,
                    to: point,
                }),
            ),
        };
    }
    // A point already picked, and now a segment: the distance from that point
    // to the line, taken square to it.
    if mode != DimensionMode::PointToPoint
        && let Some(point) = picks.first_point
        && let Some(segment) = sketch.nearest_segment(cursor, snap)
    {
        let cleared = DimensionPicks {
            first_point: None,
            ..picks
        };
        let pick = no_distance_to(sketch, point, segment).unwrap_or(DimensionPick::Target(
            DimensionTarget::PointToSegment { point, segment },
        ));
        return (cleared, pick);
    }
    if mode == DimensionMode::PointToPoint {
        return (picks, DimensionPick::Unchanged);
    }

    // Second half of an angle already in hand: another segment, or one of the
    // sketch axes.
    if matches!(mode, DimensionMode::Auto | DimensionMode::Angle)
        && let Some(first) = picks.first_angle_segment
    {
        // A segment lying along an axis is found before the axis itself, so
        // clicking the same segment twice is read as "and now the axis it
        // sits on" rather than ignored — which is what made a rectangle drawn
        // on the axes impossible to pin down.
        if let Some(second) = sketch.nearest_segment(cursor, snap)
            && second != first
        {
            let cleared = DimensionPicks {
                first_angle_segment: None,
                ..picks
            };
            if sketch.angle_between(first, second).is_some() {
                return (
                    cleared,
                    DimensionPick::Target(DimensionTarget::Angle { first, second }),
                );
            }
            // No shared end: crossing, one ending on the other, or lying apart.
            // Any two that do not run the same way make an angle, read at first
            // between the traits as drawn; where the dimension is put down can
            // turn it to face another side.
            if let Some(laid) = sketch.angle_already_between(first, second) {
                return (cleared, DimensionPick::Target(laid));
            }
            return match sketch.run_the_same_way(first, second) {
                true => (cleared, DimensionPick::TraitsAreParallel),
                false => (
                    cleared,
                    DimensionPick::Target(sketch.angle_between_traits(first, second)),
                ),
            };
        }
        // An axis rather than a second segment gives the drawing a fixed
        // direction to lean on — the only way to stop it turning about its
        // origin.
        return match axis_under(cursor, snap) {
            Some(axis) => (
                DimensionPicks {
                    first_angle_segment: None,
                    ..picks
                },
                DimensionPick::Target(DimensionTarget::AxisAngle {
                    segment: first,
                    axis,
                }),
            ),
            None => (picks, DimensionPick::Unchanged),
        };
    }

    // An axis picked first waits for the segment to measure against it.
    if matches!(mode, DimensionMode::Auto | DimensionMode::Angle)
        && picks.first_angle_segment.is_none()
        && let Some(axis) = axis_under(cursor, snap)
        && sketch.nearest_segment(cursor, snap).is_none()
    {
        return (
            DimensionPicks {
                first_axis: Some(axis),
                ..picks
            },
            DimensionPick::WaitingForTraitAfterAxis(axis),
        );
    }

    if mode != DimensionMode::Radius
        && let Some(segment) = sketch.nearest_segment(cursor, snap)
    {
        if let Some(axis) = picks.first_axis {
            return (
                DimensionPicks {
                    first_axis: None,
                    ..picks
                },
                DimensionPick::Target(DimensionTarget::AxisAngle { segment, axis }),
            );
        }
        if mode == DimensionMode::Angle {
            return (
                DimensionPicks {
                    first_angle_segment: Some(segment),
                    ..picks
                },
                DimensionPick::WaitingForSecondTraitOrAxis,
            );
        }
        return (
            picks,
            DimensionPick::Target(DimensionTarget::Length(segment)),
        );
    }

    if mode != DimensionMode::Length
        && let Some(circle) = sketch.nearest_circle(cursor, snap)
    {
        return (
            picks,
            DimensionPick::Target(DimensionTarget::Diameter(circle)),
        );
    }

    if mode != DimensionMode::Length
        && let Some(arc) = sketch.nearest_arc(cursor, snap)
    {
        return (
            picks,
            DimensionPick::Target(DimensionTarget::ArcRadius(arc)),
        );
    }

    (picks, DimensionPick::Nothing)
}

/// Why a point has no distance to a trait, when it has none: it lies on the
/// trait's line — held there by a rule, or genuinely on it — either between the
/// trait's two ends, its own ends included, or beyond them.
///
/// Read off the drawing, never off the view: the tolerance is the one a cut
/// uses for the same question, far under anything the eye tells apart.
pub(crate) fn no_distance_to(
    sketch: &Sketch,
    point: PointId,
    segment: SegmentId,
) -> Option<DimensionPick> {
    sketch.segments().get(segment.0)?;
    let (start, end) = sketch.endpoints(segment);
    let span = end - start;
    let reach = span.length_squared();
    if reach == 0.0 {
        return None;
    }
    let place = sketch.point(point);
    let along = (place - start).dot(span) / reach;
    let held = sketch
        .constraints()
        .contains(&Constraint::OnSegment { point, segment });
    if !held && place.distance(start + span * along) > ON_THE_TRAIT {
        return None;
    }
    Some(match (0.0..=1.0).contains(&along) {
        true => DimensionPick::PointAlreadyOnTheTrait,
        false => DimensionPick::PointInLineWithTheTrait,
    })
}

#[cfg(test)]
mod tests;
