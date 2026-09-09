//! What one click of the dimension tool does.
//!
//! The smart dimension tool works out what is under the cursor and measures
//! it, unless a mode is forcing one kind. Two-step measurements — point to
//! point, angle — collect their first half and wait; everything else is
//! settled in a single click.

use glam::DVec2;

use crate::constraints::{DimensionTarget, SketchAxis};
use crate::dimensioning::axis_under;
use crate::sketch::{PointId, SegmentId, Sketch};

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
    /// Two traits picked for an angle, but they never meet.
    TraitsDoNotTouch,
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
        return (
            DimensionPicks {
                first_point: None,
                ..picks
            },
            DimensionPick::Target(DimensionTarget::PointToSegment { point, segment }),
        );
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
            return match sketch.angle_between(first, second) {
                None => (cleared, DimensionPick::TraitsDoNotTouch),
                Some(_) => (
                    cleared,
                    DimensionPick::Target(DimensionTarget::Angle { first, second }),
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

    (picks, DimensionPick::Nothing)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plane::WorkPlane;

    #[test]
    fn a_point_to_point_click_waits_for_the_second_point_then_measures_between_them() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(10.0, 10.0));
        let b = sketch.add_point(DVec2::new(30.0, 10.0));

        let (picks, first_pick) = measure_pick(
            &sketch,
            DimensionMode::PointToPoint,
            DimensionPicks::default(),
            sketch.point(a),
            1.0,
        );
        assert_eq!(first_pick, DimensionPick::WaitingForSecondPoint);

        let (_, second_pick) = measure_pick(
            &sketch,
            DimensionMode::PointToPoint,
            picks,
            sketch.point(b),
            1.0,
        );
        assert_eq!(
            second_pick,
            DimensionPick::Target(DimensionTarget::Distance { from: a, to: b }),
        );
    }

    #[test]
    fn an_angle_between_two_traits_that_do_not_meet_is_told_apart_from_one_that_does() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(0.0, 0.0));
        let b = sketch.add_point(DVec2::new(40.0, 0.0));
        let first = sketch.add_segment(a, b);
        let apart_from = sketch.add_point(DVec2::new(0.0, 40.0));
        let apart_to = sketch.add_point(DVec2::new(40.0, 40.0));
        let _stray = sketch.add_segment(apart_from, apart_to);

        let picks = DimensionPicks {
            first_angle_segment: Some(first),
            ..DimensionPicks::default()
        };
        let (_, outcome) = measure_pick(
            &sketch,
            DimensionMode::Angle,
            picks,
            DVec2::new(20.0, 40.0),
            1.0,
        );

        assert_eq!(outcome, DimensionPick::TraitsDoNotTouch);
    }

    #[test]
    fn nothing_under_the_cursor_leaves_what_was_already_picked_untouched() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let point = sketch.add_point(DVec2::new(0.0, 0.0));
        let picks = DimensionPicks {
            first_point: Some(point),
            ..DimensionPicks::default()
        };

        let (kept, outcome) = measure_pick(
            &sketch,
            DimensionMode::Auto,
            picks,
            DVec2::new(500.0, 500.0),
            1.0,
        );

        assert_eq!(outcome, DimensionPick::Nothing);
        assert_eq!(
            kept, picks,
            "a click on empty ground says nothing to measure, but does not forget the point already picked",
        );
    }
}
