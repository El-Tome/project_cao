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
mod tests {
    use super::*;
    use crate::plane::WorkPlane;

    const TOLERANCE: f64 = 1e-9;

    fn chain_of_one(sketch: &mut Sketch) -> (SegmentId, DVec2) {
        let start = sketch.add_point(DVec2::new(0.0, 0.0));
        let corner = sketch.add_point(DVec2::new(40.0, 0.0));
        (sketch.add_segment(start, corner), sketch.point(corner))
    }

    fn cursor_off_square(from: DVec2, degrees: f64) -> DVec2 {
        from + DVec2::from_angle((90.0 - degrees).to_radians()) * 100.0
    }

    #[test]
    fn a_cursor_within_four_degrees_of_square_snaps_to_the_right_angle() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let (previous, corner) = chain_of_one(&mut sketch);

        let aimed = sketch.aim(
            ChainAnchor::Pending(corner),
            Some(previous),
            cursor_off_square(corner, 3.0),
            LockedInput::default(),
            1.0,
        );

        assert!(
            (aimed.position.x - corner.x).abs() < TOLERANCE,
            "three degrees off square is square: expected x {}, got {}",
            corner.x,
            aimed.position.x,
        );
        assert_eq!(
            aimed.square_with,
            Some(previous),
            "the trait it was squared against is named, so a rule can be written"
        );
    }

    #[test]
    fn a_cursor_eight_degrees_off_square_is_left_alone() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let (previous, corner) = chain_of_one(&mut sketch);
        let cursor = cursor_off_square(corner, 8.0);

        let aimed = sketch.aim(
            ChainAnchor::Pending(corner),
            Some(previous),
            cursor,
            LockedInput::default(),
            1.0,
        );

        assert!(
            aimed.position.distance(cursor) < TOLERANCE,
            "eight degrees off square is an angle meant as it was drawn: expected {cursor}, got {}",
            aimed.position,
        );
        assert_eq!(
            aimed.square_with, None,
            "nothing was squared up, so nothing is named"
        );
    }

    #[test]
    fn a_locked_length_leaves_the_line_free_to_turn() {
        let sketch = Sketch::new(WorkPlane::XY);
        let from = DVec2::new(5.0, 5.0);
        let locked = LockedInput {
            first: Some(50.0),
            second: None,
        };

        let east = sketch.aim(
            ChainAnchor::Pending(from),
            None,
            from + DVec2::new(200.0, 0.0),
            locked,
            1.0,
        );
        let north = sketch.aim(
            ChainAnchor::Pending(from),
            None,
            from + DVec2::new(0.0, 200.0),
            locked,
            1.0,
        );

        for (aimed, way) in [(east, "east"), (north, "north")] {
            let reached = aimed.position.distance(from);
            assert!(
                (reached - 50.0).abs() < TOLERANCE,
                "a length typed holds however the cursor turns: {way} reached {reached}",
            );
        }
        assert!(
            east.position.distance(north.position) > 1.0,
            "the two still point different ways: {} and {}",
            east.position,
            north.position,
        );
    }

    #[test]
    fn a_locked_angle_follows_the_side_the_cursor_is_on() {
        let sketch = Sketch::new(WorkPlane::XY);
        let from = DVec2::ZERO;
        let locked = LockedInput {
            first: None,
            second: Some(30.0),
        };
        let thirty = DVec2::from_angle(30.0_f64.to_radians());

        let pointing_up = sketch.aim(
            ChainAnchor::Pending(from),
            None,
            thirty * 100.0,
            locked,
            1.0,
        );
        let pointing_down = sketch.aim(
            ChainAnchor::Pending(from),
            None,
            thirty * -100.0,
            locked,
            1.0,
        );

        assert!(
            pointing_up.position.normalize().distance(thirty) < TOLERANCE,
            "pointing up the line, thirty degrees is the one above: {}",
            pointing_up.position,
        );
        assert!(
            pointing_down.position.normalize().distance(-thirty) < TOLERANCE,
            "pointing the other way, it is the thirty degrees the cursor is on: {}",
            pointing_down.position,
        );
    }

    #[test]
    fn a_side_typed_fixes_only_that_side_of_the_rectangle() {
        let start = DVec2::ZERO;
        let locked = LockedInput {
            first: Some(40.0),
            second: None,
        };

        let dragged_right = rectangle_corner(start, DVec2::new(120.0, -80.0), locked, 1.0);
        let dragged_left = rectangle_corner(start, DVec2::new(-120.0, -80.0), locked, 1.0);

        assert!(
            (dragged_right.x - 40.0).abs() < TOLERANCE
                && (dragged_right.y + 80.0).abs() < TOLERANCE,
            "the width typed holds and the height still follows the cursor: {dragged_right}",
        );
        assert!(
            (dragged_left.x + 40.0).abs() < TOLERANCE,
            "forty typed means forty the way the user is dragging: {dragged_left}",
        );
    }
}
