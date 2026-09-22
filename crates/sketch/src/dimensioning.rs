//! Which reading of a trait a cursor is asking for.
//!
//! A trait that leans has three measurements, not one: its length, its width
//! and its height. Where the cursor falls when the dimension is put down is
//! what says which of them the user meant, and that is a rule about the
//! drawing rather than about the interface drawing it.

use glam::DVec2;

use crate::constraints::{DimensionTarget, SketchAxis, Toward};
use crate::measuring::no_distance_to;
use crate::sketch::{PointId, SegmentId, Sketch};

/// How far off an axis a trait has to be before its width and its height are
/// worth offering.
///
/// Only a trait sitting square on an axis is left out: its width *is* its
/// length, and two names for one measurement is one too many. Everything else,
/// however slightly leaning, gets the choice.
const SLANT_DEGREES: f64 = 0.5;

impl Sketch {
    /// Which reading of a dimension the cursor is asking for, where there is
    /// more than one: the side an angle between two crossing traits opens
    /// towards, or one of the three readings of a slanted trait.
    ///
    /// For a slanted trait, the two ends box off the plane: above or below that
    /// box the cursor asks for the width, left or right of it the height, and
    /// inside it — the triangle the trait closes — or out past a corner, the
    /// length itself.
    pub fn oriented(&self, target: DimensionTarget, cursor: DVec2) -> DimensionTarget {
        if let DimensionTarget::AngleBetween { .. } = target {
            return self.opening_toward(target, cursor);
        }
        let Some((from, to)) = self.ends_of(target).filter(|_| self.is_slanted(target)) else {
            return target;
        };
        let (Some(a), Some(b)) = (
            self.points().get(from.0).copied(),
            self.points().get(to.0).copied(),
        ) else {
            return target;
        };
        let (low, high) = (a.min(b), a.max(b));
        let within_x = (low.x..=high.x).contains(&cursor.x);
        let within_y = (low.y..=high.y).contains(&cursor.y);
        let axis = match (within_x, within_y) {
            (true, false) => SketchAxis::U,
            (false, true) => SketchAxis::V,
            _ => return target,
        };
        DimensionTarget::Projected { from, to, axis }.normalised()
    }

    /// Whether a point is one of a segment's own ends — measuring a segment to
    /// its own corner would be a distance of nothing.
    pub fn segment_touches(&self, segment: SegmentId, point: PointId) -> bool {
        match self.segments().get(segment.0) {
            Some(segment) => segment.start == point || segment.end == point,
            None => false,
        }
    }

    /// Whether a trait leans far enough off both axes for its width and its
    /// height to be worth offering beside its length.
    pub fn is_slanted(&self, target: DimensionTarget) -> bool {
        let Some((from, to)) = self.ends_of(target) else {
            return false;
        };
        let (Some(start), Some(end)) = (self.points().get(from.0), self.points().get(to.0)) else {
            return false;
        };
        let span = *end - *start;
        let slant = span.y.atan2(span.x).to_degrees().abs();
        slant.min((slant - 90.0).abs()).min((slant - 180.0).abs()) >= SLANT_DEGREES
    }

    /// The two ends of what a linear dimension measures, when it has two.
    fn ends_of(&self, target: DimensionTarget) -> Option<(PointId, PointId)> {
        match target {
            DimensionTarget::Length(segment) => {
                let segment = self.segments().get(segment.0)?;
                Some((segment.start, segment.end))
            }
            DimensionTarget::Distance { from, to }
            | DimensionTarget::Projected { from, to, .. } => Some((from, to)),
            _ => None,
        }
    }

    /// The second half a dimension in hand can still take: another segment
    /// makes it an angle, a point makes it a distance to a line, an axis a
    /// direction.
    ///
    /// Without this, clicking a segment could only ever mean its length, and
    /// an angle between two traits had to be asked for through the dimension
    /// mode picker — which is exactly what one expects the smart dimension to
    /// do on its own.
    pub fn refine(
        &self,
        target: DimensionTarget,
        cursor: DVec2,
        snap: f64,
    ) -> Option<DimensionTarget> {
        // A diameter taken back to the centre is a radius: it is the one
        // thing the centre can add to a circle already picked.
        if let DimensionTarget::Diameter(circle) = target {
            let center = self.circles().get(circle.0)?.center;
            return self
                .nearest_point(cursor, snap * 0.8)
                .filter(|point| *point == center)
                .map(|_| DimensionTarget::Radius(circle));
        }

        // A radius taken on to one of the arc's own ends asks for the sweep
        // instead: the one other thing there is to know about an arc.
        if let DimensionTarget::ArcRadius(arc) = target {
            let drawn = *self.arcs().get(arc.0)?;
            return self
                .nearest_point(cursor, snap * 0.8)
                .filter(|point| *point == drawn.start || *point == drawn.end)
                .map(|_| DimensionTarget::ArcSweep(arc));
        }

        let DimensionTarget::Length(first) = target else {
            return None;
        };

        if let Some(second) = self.nearest_segment(cursor, snap)
            && second != first
        {
            if self.angle_between(first, second).is_some() {
                return Some(DimensionTarget::Angle { first, second }.normalised());
            }
            // No shared end, but they may still meet — crossing, or one ending
            // on the other. Which of their angles is meant is left to where the
            // dimension is put down.
            if let Some(laid) = self.angle_already_between(first, second) {
                return Some(laid);
            }
            if self.where_traits_meet(first, second).is_some() {
                return Some(DimensionTarget::AngleBetween {
                    first,
                    first_toward: Toward::End,
                    second,
                    second_toward: Toward::End,
                });
            }
        }
        // A point on the trait's own line — one of its ends, one held on it,
        // one on its prolongation — is no distance from it: the length already
        // in hand is put down there instead of a zero nobody could type.
        if let Some(point) = self.nearest_point(cursor, snap * 0.8)
            && no_distance_to(self, point, first).is_none()
        {
            return Some(DimensionTarget::PointToSegment {
                point,
                segment: first,
            });
        }
        if self.nearest_segment(cursor, snap).is_none()
            && let Some(axis) = axis_under(cursor, snap)
        {
            return Some(DimensionTarget::AxisAngle {
                segment: first,
                axis,
            });
        }
        None
    }
}

/// Which sketch axis the cursor is on, if either. The axes are drawn as lines
/// through the origin, so they are picked the same way a segment is.
pub fn axis_under(cursor: DVec2, tolerance: f64) -> Option<SketchAxis> {
    if cursor.y.abs() <= tolerance {
        return Some(SketchAxis::U);
    }
    if cursor.x.abs() <= tolerance {
        return Some(SketchAxis::V);
    }
    None
}

#[cfg(test)]
mod tests;
