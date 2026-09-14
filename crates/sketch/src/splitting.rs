//! Dropping a point where two traits cross, and cutting both of them there.

use glam::DVec2;

use crate::sketch::{PointId, SegmentId, Sketch};
use crate::trimming::{NO_LENGTH, ON_THE_TRAIT};

/// What a division left behind: the point it dropped at the crossing, the
/// pieces the traits it cut were left as, and what the cut cost.
///
/// A division takes no stretch away, but it does take the traits themselves:
/// a rule or a value that spoke of a whole trait and of neither of its pieces
/// goes with it, exactly as under a cut.
#[derive(Clone, Debug, PartialEq)]
pub struct Split {
    pub point: PointId,
    pub pieces: Vec<SegmentId>,
    pub rules_dropped: usize,
    pub values_dropped: usize,
}

impl Sketch {
    /// Drops a point where the named traits cross and cuts each of them in two
    /// there, so that the crossing becomes something to dimension, to
    /// constrain and to drag.
    pub fn split(&mut self, segments: &[SegmentId], at: DVec2) -> Option<Split> {
        if self.stands_on(at) {
            return None;
        }
        if !segments
            .iter()
            .all(|segment| self.runs_through(*segment, at))
        {
            return None;
        }
        let point = self.add_point(at);
        let mut split = Split {
            point,
            pieces: Vec::new(),
            rules_dropped: 0,
            values_dropped: 0,
        };
        for segment in segments {
            let cut = self.trim(*segment, point, point)?;
            split.pieces.extend(cut.pieces);
            split.rules_dropped += cut.rules_dropped;
            split.values_dropped += cut.values_dropped;
        }
        Some(split)
    }

    /// Whether the drawing already owns a point at this place. There is then
    /// nothing for a division to drop, and a second point sitting on the first
    /// is one nobody can tell from it.
    fn stands_on(&self, at: DVec2) -> bool {
        self.live_points()
            .any(|(_, place)| place.distance(at) <= ON_THE_TRAIT)
    }

    /// Whether a trait passes through this place, short of either of its own
    /// ends.
    ///
    /// A place level with the trait but past an end is not on it, and one an
    /// end already occupies has nothing to be cut off it.
    fn runs_through(&self, segment: SegmentId, at: DVec2) -> bool {
        if self.is_erased_segment(segment) || segment.0 >= self.segments().len() {
            return false;
        }
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let reach = span.length_squared();
        if reach == 0.0 {
            return false;
        }
        let along = (at - start).dot(span) / reach;
        (0.0..=1.0).contains(&along)
            && at.distance(start + span * along) <= ON_THE_TRAIT
            && at.distance(start) > NO_LENGTH
            && at.distance(end) > NO_LENGTH
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constraints::DimensionTarget;
    use crate::plane::WorkPlane;

    fn two_traits_crossing() -> (Sketch, [SegmentId; 2], DVec2) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let west = sketch.add_point(DVec2::new(0.0, 0.0));
        let east = sketch.add_point(DVec2::new(10.0, 0.0));
        let south = sketch.add_point(DVec2::new(5.0, -5.0));
        let north = sketch.add_point(DVec2::new(5.0, 5.0));
        let across = sketch.add_segment(west, east);
        let up = sketch.add_segment(south, north);
        (sketch, [across, up], DVec2::new(5.0, 0.0))
    }

    #[test]
    fn splitting_a_crossing_leaves_four_pieces_meeting_at_one_point() {
        let (mut sketch, [across, up], crossing) = two_traits_crossing();

        let split = sketch
            .split(&[across, up], crossing)
            .expect("a crossing that can be split");

        assert!(
            sketch.is_erased_segment(across) && sketch.is_erased_segment(up),
            "both traits the click fell on are gone, replaced by their pieces"
        );
        assert_eq!(split.pieces.len(), 4, "two pieces out of each trait");
        for piece in &split.pieces {
            let piece = sketch.segments()[piece.0];
            assert!(
                piece.start == split.point || piece.end == split.point,
                "every piece stops at the point dropped at the crossing"
            );
        }
    }

    #[test]
    fn a_division_that_cannot_be_made_leaves_the_drawing_as_it_was() {
        let (mut sketch, [across, up], _) = two_traits_crossing();
        let before = sketch.clone();

        let refused = sketch.split(&[across, up], DVec2::new(5.0, 3.0));

        assert_eq!(
            refused, None,
            "the place named is on one trait, not on both"
        );
        assert_eq!(
            sketch.points().len(),
            before.points().len(),
            "a refused division drops no point"
        );
        assert!(
            !sketch.is_erased_segment(across) && !sketch.is_erased_segment(up),
            "a refused division cuts neither trait"
        );
    }

    #[test]
    fn a_crossing_a_point_already_stands_on_is_not_divided_again() {
        let (mut sketch, [across, up], crossing) = two_traits_crossing();
        sketch.add_point(crossing);
        let points = sketch.points().len();

        let refused = sketch.split(&[across, up], crossing);

        assert_eq!(refused, None, "there is nothing left to drop there");
        assert_eq!(
            sketch.points().len(),
            points,
            "a second point on the same place would be one the user cannot tell from the first"
        );
    }

    #[test]
    fn a_length_measured_over_the_whole_trait_does_not_survive_its_division() {
        let (mut sketch, [across, up], crossing) = two_traits_crossing();
        sketch.set_dimension(DimensionTarget::Length(across), 10.0, false);

        sketch
            .split(&[across, up], crossing)
            .expect("a crossing that can be split");

        let measured: Vec<DimensionTarget> = sketch
            .dimensions()
            .iter()
            .map(|value| value.target)
            .collect();
        assert!(
            !measured.contains(&DimensionTarget::Length(across)),
            "the trait it measured is gone, and it measured neither piece: {measured:?}"
        );
    }

    #[test]
    fn a_division_says_what_it_cost() {
        let (mut sketch, [across, up], crossing) = two_traits_crossing();
        sketch.set_dimension(DimensionTarget::Length(across), 10.0, false);
        sketch.set_dimension(DimensionTarget::Length(up), 10.0, false);

        let split = sketch
            .split(&[across, up], crossing)
            .expect("a crossing that can be split");

        assert_eq!(
            split.values_dropped, 2,
            "one length measured over each of the two traits divided"
        );
    }
}
