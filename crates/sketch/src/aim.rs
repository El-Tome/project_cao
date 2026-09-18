//! Where a trait being drawn actually ends: what the cursor asks for, once
//! everything the user has already decided has had its say.

use glam::DVec2;

use crate::sketch::{PointId, SegmentId, Sketch};

/// What the two live fields hold, in millimetres, when the user has typed into
/// them. A field left alone is only a readout and decides nothing.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LockedInput {
    pub first: Option<f64>,
    pub second: Option<f64>,
}

/// Where the trait being drawn starts from.
///
/// The first click of a chain has no point in the drawing yet — a lone point
/// would put a step in the history that draws nothing — so the place is held
/// until the second click turns the pair into one segment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChainAnchor {
    Pending(DVec2),
    Point(PointId),
}

/// Where the trait being drawn ends, and what that implies.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aim {
    pub position: DVec2,
    /// The trait this one has just been squared up against.
    pub square_with: Option<SegmentId>,
}

/// Half the width of the band, in degrees, inside which a corner is taken as
/// square. Wide enough to be easy to hit, narrow enough that an angle really
/// meant to be 80 degrees is not stolen.
const SQUARE_TOLERANCE_DEGREES: f64 = 4.0;

/// The far corner of a rectangle being drawn, once the sizes typed have had
/// their say. A size left alone follows the cursor; one typed only fixes that
/// side, so the other can still be dragged out.
pub fn rectangle_corner(start: DVec2, cursor: DVec2, locked: LockedInput, scale: f64) -> DVec2 {
    let scale = scale.max(1e-9);
    let span = cursor - start;
    // The sign follows the cursor: 40 typed means 40 the way the user is
    // dragging, not 40 the other way.
    let side = |locked: Option<f64>, current: f64| match locked {
        Some(millimeters) => (millimeters / scale).copysign(current),
        None => current,
    };
    start + DVec2::new(side(locked.first, span.x), side(locked.second, span.y))
}

impl Sketch {
    /// Applies to the cursor everything the user has already decided.
    ///
    /// A locked angle leaves the trait free to lengthen along that direction; a
    /// locked length leaves it free to turn at that distance; both leave nothing
    /// to the cursor at all. That is the point of locking one and not the other.
    ///
    /// `scale` is how many millimetres a unit of the drawing is worth, since the
    /// two locked values are typed in millimetres.
    pub fn aim(
        &self,
        anchor: ChainAnchor,
        previous: Option<SegmentId>,
        cursor: DVec2,
        locked: LockedInput,
        scale: f64,
    ) -> Aim {
        let nowhere = Aim {
            position: cursor,
            square_with: None,
        };
        let Some(from) = self.anchor_position(anchor) else {
            return nowhere;
        };

        let scale = scale.max(1e-9);
        let span = cursor - from;
        let mut direction = span.normalize_or(DVec2::X);
        let mut square_with = None;

        if let Some(degrees) = locked.second {
            // The sign follows the cursor: 30 degrees typed means the 30 the
            // user is pointing at, not the one below the axis they are not.
            let wanted = DVec2::from_angle(degrees.to_radians());
            direction = if wanted.dot(direction) >= 0.0 {
                wanted
            } else {
                -wanted
            };
        } else if let Some(squared) = previous.and_then(|id| self.right_angle(id, from, span)) {
            direction = squared;
            square_with = previous;
        }

        let length = match locked.first {
            Some(millimeters) => millimeters / scale,
            None => span.dot(direction).max(0.0),
        };

        Aim {
            position: from + direction * length.max(1e-6),
            square_with,
        }
    }

    /// The two ends of a trait growing equally in both directions from
    /// `middle`, as the symmetric line tool draws one. A length typed reads
    /// the same way the plain line tool reads it: the distance from the
    /// anchor — here, the middle — to the edge being aimed at.
    pub fn symmetric_ends(
        &self,
        middle: DVec2,
        cursor: DVec2,
        locked: LockedInput,
        scale: f64,
    ) -> (DVec2, DVec2) {
        let end = self
            .aim(ChainAnchor::Pending(middle), None, cursor, locked, scale)
            .position;
        (middle * 2.0 - end, end)
    }

    /// Where a chain being drawn starts from, when that place still exists.
    pub fn anchor_position(&self, anchor: ChainAnchor) -> Option<DVec2> {
        match anchor {
            ChainAnchor::Pending(position) => Some(position),
            ChainAnchor::Point(id) => (id.0 < self.points().len()).then(|| self.point(id)),
        }
    }

    /// The direction that squares this trait up against the one before it, when
    /// the cursor is close enough to it.
    fn right_angle(&self, previous: SegmentId, from: DVec2, span: DVec2) -> Option<DVec2> {
        if previous.0 >= self.segments().len() || self.is_erased_segment(previous) {
            return None;
        }
        let (start, end) = self.endpoints(previous);
        // The arm runs from the shared corner outwards, whichever way it was
        // drawn.
        let arm = if start.distance(from) < end.distance(from) {
            end - start
        } else {
            start - end
        }
        .normalize_or_zero();
        let direction = span.normalize_or_zero();
        if arm == DVec2::ZERO || direction == DVec2::ZERO {
            return None;
        }

        let off_square = direction.dot(arm).abs().asin().to_degrees();
        if off_square > SQUARE_TOLERANCE_DEGREES {
            return None;
        }
        let square = DVec2::new(-arm.y, arm.x);
        Some(if square.dot(direction) >= 0.0 {
            square
        } else {
            -square
        })
    }
}

#[cfg(test)]
mod tests;
