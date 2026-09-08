//! Which reading of a trait a cursor is asking for.
//!
//! A trait that leans has three measurements, not one: its length, its width
//! and its height. Where the cursor falls when the dimension is put down is
//! what says which of them the user meant, and that is a rule about the
//! drawing rather than about the interface drawing it.

use glam::DVec2;

use crate::constraints::{DimensionTarget, SketchAxis};
use crate::sketch::{PointId, SegmentId, Sketch};

/// How far off an axis a trait has to be before its width and its height are
/// worth offering.
///
/// Only a trait sitting square on an axis is left out: its width *is* its
/// length, and two names for one measurement is one too many. Everything else,
/// however slightly leaning, gets the choice.
const SLANT_DEGREES: f64 = 0.5;

impl Sketch {
    /// Which of the three readings of a slanted trait the cursor is asking for.
    ///
    /// The two ends box off the plane: above or below that box the cursor asks
    /// for the width, left or right of it the height, and inside it — the
    /// triangle the trait closes — or out past a corner, the length itself.
    pub fn oriented(&self, target: DimensionTarget, cursor: DVec2) -> DimensionTarget {
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
mod tests {
    use super::*;
    use crate::WorkPlane;

    fn trait_from(start: DVec2, end: DVec2) -> (Sketch, DimensionTarget, PointId, PointId) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let from = sketch.add_point(start);
        let to = sketch.add_point(end);
        let segment = sketch.add_segment(from, to);
        (sketch, DimensionTarget::Length(segment), from, to)
    }

    fn leaning(degrees: f64) -> (Sketch, DimensionTarget, PointId, PointId) {
        let start = DVec2::new(10.0, 10.0);
        let span = DVec2::from_angle(degrees.to_radians()) * 40.0;
        trait_from(start, start + span)
    }

    #[test]
    fn a_trait_square_on_an_axis_offers_only_its_length() {
        let (sketch, length, ..) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(50.0, 10.0));

        assert_eq!(sketch.oriented(length, DVec2::new(30.0, 60.0)), length);
        assert_eq!(sketch.oriented(length, DVec2::new(70.0, 10.0)), length);
    }

    #[test]
    fn a_trait_square_on_the_other_axis_offers_only_its_length_too() {
        let (sketch, length, ..) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(10.0, 50.0));

        assert!(!sketch.is_slanted(length));
        assert_eq!(sketch.oriented(length, DVec2::new(60.0, 30.0)), length);
    }

    #[test]
    fn a_trait_drawn_right_to_left_is_no_more_slanted_than_the_same_one_drawn_the_other_way() {
        let (sketch, length, ..) = trait_from(DVec2::new(50.0, 10.0), DVec2::new(10.0, 10.0));

        assert!(!sketch.is_slanted(length));
    }

    #[test]
    fn a_trait_a_hair_off_an_axis_is_not_yet_worth_two_more_readings() {
        let (sketch, length, ..) = leaning(0.4);

        assert!(!sketch.is_slanted(length));
        assert_eq!(sketch.oriented(length, DVec2::new(30.0, 60.0)), length);
    }

    #[test]
    fn a_trait_just_past_the_threshold_offers_its_width_and_its_height() {
        let (sketch, length, ..) = leaning(0.6);

        assert!(sketch.is_slanted(length));
    }

    #[test]
    fn a_cursor_above_a_slanted_trait_asks_for_its_width() {
        let (sketch, length, from, to) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(50.0, 40.0));

        assert_eq!(
            sketch.oriented(length, DVec2::new(30.0, 60.0)),
            DimensionTarget::Projected {
                from,
                to,
                axis: SketchAxis::U,
            },
        );
    }

    #[test]
    fn a_cursor_beside_a_slanted_trait_asks_for_its_height() {
        let (sketch, length, from, to) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(50.0, 40.0));

        assert_eq!(
            sketch.oriented(length, DVec2::new(70.0, 25.0)),
            DimensionTarget::Projected {
                from,
                to,
                axis: SketchAxis::V,
            },
        );
    }

    #[test]
    fn a_cursor_inside_the_box_of_the_ends_asks_for_the_length() {
        let (sketch, length, ..) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(50.0, 40.0));

        assert_eq!(sketch.oriented(length, DVec2::new(30.0, 25.0)), length);
        assert_eq!(sketch.oriented(length, DVec2::new(70.0, 60.0)), length);
    }

    #[test]
    fn a_width_is_named_the_same_way_round_whichever_end_was_drawn_first() {
        let (start, end) = (DVec2::new(10.0, 10.0), DVec2::new(50.0, 40.0));
        let cursor = DVec2::new(30.0, 60.0);
        let (drawn_up, up, ..) = trait_from(start, end);

        let mut drawn_down = Sketch::new(WorkPlane::XY);
        let first = drawn_down.add_point(start);
        let second = drawn_down.add_point(end);
        let down = DimensionTarget::Length(drawn_down.add_segment(second, first));

        assert_eq!(
            drawn_up.oriented(up, cursor),
            drawn_down.oriented(down, cursor),
        );
    }

    #[test]
    fn a_reading_already_asked_for_is_not_asked_for_a_second_time() {
        let (sketch, _, from, to) = trait_from(DVec2::new(10.0, 10.0), DVec2::new(50.0, 40.0));
        let width = DimensionTarget::Projected {
            from,
            to,
            axis: SketchAxis::U,
        };

        assert_eq!(sketch.oriented(width, DVec2::new(30.0, 60.0)), width);
    }

    #[test]
    fn a_segment_touches_its_own_ends_and_nothing_else() {
        let (mut sketch, length, from, to) =
            trait_from(DVec2::new(10.0, 10.0), DVec2::new(50.0, 40.0));
        let DimensionTarget::Length(segment) = length else {
            unreachable!()
        };
        let apart = sketch.add_point(DVec2::new(0.0, 80.0));

        assert!(sketch.segment_touches(segment, from));
        assert!(sketch.segment_touches(segment, to));
        assert!(!sketch.segment_touches(segment, apart));
    }

    #[test]
    fn the_horizontal_axis_is_under_a_cursor_on_it() {
        assert_eq!(axis_under(DVec2::new(30.0, 0.2), 0.5), Some(SketchAxis::U));
    }

    #[test]
    fn the_vertical_axis_is_under_a_cursor_on_it() {
        assert_eq!(axis_under(DVec2::new(0.2, 30.0), 0.5), Some(SketchAxis::V));
    }

    #[test]
    fn a_cursor_off_both_axes_is_on_neither() {
        assert_eq!(axis_under(DVec2::new(30.0, 30.0), 0.5), None);
    }
}
