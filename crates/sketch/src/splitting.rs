//! Dropping a point where two traits cross, and cutting both of them there.

use glam::DVec2;

use crate::edges::off_by;
use crate::sketch::{PointId, SegmentId, Sketch};

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

/// What stands at the place a click fell on, for a division to act upon.
#[derive(Clone, Debug, PartialEq)]
pub enum Crossing {
    /// Traits crossing with no point of the drawing standing there, and the
    /// place they cross at.
    Traits { at: DVec2, segments: Vec<SegmentId> },
    /// A curve runs through the crossing too. A division cuts traits and
    /// nothing else yet, and cutting only the straight halves of a crossing
    /// would leave the drawing saying something nobody asked for.
    Curved,
}

impl Sketch {
    /// What a click at this place offers a division, within the reach given.
    ///
    /// The nearest crossing rather than the one clicked exactly: the cursor is
    /// magnetised onto a crossing before it gets here, but a drawing whose
    /// magnets are off should still be divisible.
    pub fn crossing_at(&self, at: DVec2, reach: f64) -> Option<Crossing> {
        let place = self
            .crossings()
            .into_iter()
            .filter(|place| place.distance(at) <= reach)
            .min_by(|left, right| {
                left.distance_squared(at)
                    .total_cmp(&right.distance_squared(at))
            })?;

        let segments: Vec<SegmentId> = self
            .live_segments()
            .filter(|(id, segment)| !segment.construction && self.runs_through(*id, place))
            .map(|(id, _)| id)
            .collect();

        if self.a_curve_runs_through(place) {
            return Some(Crossing::Curved);
        }
        (segments.len() >= 2).then_some(Crossing::Traits {
            at: place,
            segments,
        })
    }

    /// Whether a circle or an arc of the drawing passes through this place.
    ///
    /// Judged by the sweep's own yardstick rather than a figure of its own.
    /// The sweep merges places nearer than that into one vertex, so a crossing
    /// it reports can sit exactly on whichever pair of curves it happened to
    /// see first and a whole `off_by` away from the circle it swallowed. Asked
    /// any more strictly, the division would cut the traits and leave that
    /// circle round.
    ///
    /// Construction curves are left out, as they are left out of the sweep.
    fn a_curve_runs_through(&self, at: DVec2) -> bool {
        let reach = off_by(at);
        let on_a_circle = self.live_circles().any(|(_, circle)| {
            !circle.construction
                && (at.distance(self.point(circle.center)) - circle.radius).abs() <= reach
        });
        let on_an_arc = self
            .live_arcs()
            .any(|(id, arc)| !arc.construction && self.distance_to_arc(id, at) <= reach);
        on_a_circle || on_an_arc
    }

    /// Drops a point where the named traits cross and cuts each of them in two
    /// there, so that the crossing becomes something to dimension, to
    /// constrain and to drag.
    ///
    /// Built on a copy and kept only once every trait has been cut, each one
    /// asked again of that copy. What held of a trait before the first cut need
    /// not hold after it: a cut carries a point away with a tangency it drops,
    /// and `trim` cuts a trait a previous pass already erased rather than
    /// refusing it.
    pub fn split(&mut self, segments: &[SegmentId], at: DVec2) -> Option<Split> {
        if self.stands_on(at) {
            return None;
        }

        let mut divided = self.clone();
        let point = divided.add_point(at);
        let mut split = Split {
            point,
            pieces: Vec::new(),
            rules_dropped: 0,
            values_dropped: 0,
        };
        for segment in segments {
            if !divided.runs_through(*segment, at) {
                return None;
            }
            let cut = divided.trim(*segment, point, point)?;
            split.pieces.extend(cut.pieces);
            split.rules_dropped += cut.rules_dropped;
            split.values_dropped += cut.values_dropped;
        }

        *self = divided;
        Some(split)
    }

    /// Whether the drawing already owns a point at this place. There is then
    /// nothing for a division to drop, and a second point sitting on the first
    /// is one nobody can tell from it.
    fn stands_on(&self, at: DVec2) -> bool {
        self.live_points()
            .any(|(_, place)| place.distance(at) <= off_by(at))
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
        let near_enough = off_by(at);
        (0.0..=1.0).contains(&along)
            && at.distance(start + span * along) <= near_enough
            && at.distance(start) > near_enough
            && at.distance(end) > near_enough
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
    fn a_division_that_fails_on_its_second_trait_undoes_the_first() {
        let (mut sketch, [across, _], crossing) = two_traits_crossing();
        let before = sketch.clone();

        let refused = sketch.split(&[across, across], crossing);

        assert_eq!(refused, None, "the trait was already cut by the first pass");
        assert!(
            !sketch.is_erased_segment(across),
            "a drawing divided halfway is worse than one not divided at all"
        );
        assert_eq!(sketch.points().len(), before.points().len());
        assert_eq!(sketch.live_segments().count(), 2);
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
    fn the_traits_running_through_a_crossing_are_the_ones_a_division_cuts() {
        let (sketch, [across, up], crossing) = two_traits_crossing();

        let found = sketch.crossing_at(crossing + DVec2::new(0.2, 0.1), 1.0);

        assert_eq!(
            found,
            Some(Crossing::Traits {
                at: crossing,
                segments: vec![across, up],
            }),
            "a click near the crossing names the place and both traits through it"
        );
    }

    #[test]
    fn a_click_nowhere_near_a_crossing_names_none() {
        let (sketch, _, crossing) = two_traits_crossing();

        assert_eq!(
            sketch.crossing_at(crossing + DVec2::new(3.0, 3.0), 1.0),
            None
        );
    }

    #[test]
    fn a_crossing_a_curve_runs_through_is_refused_rather_than_half_divided() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let centre = sketch.add_point(DVec2::new(0.0, 0.0));
        sketch.add_circle(centre, 5.0);
        let west = sketch.add_point(DVec2::new(-10.0, 0.0));
        let east = sketch.add_point(DVec2::new(10.0, 0.0));
        sketch.add_segment(west, east);

        let found = sketch.crossing_at(DVec2::new(5.0, 0.0), 1.0);

        assert_eq!(
            found,
            Some(Crossing::Curved),
            "the trait could be cut there, but the circle it crosses could not"
        );
    }

    #[test]
    fn two_traits_crossing_where_a_curve_also_runs_are_left_alone_together() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let centre = sketch.add_point(DVec2::new(0.0, 0.0));
        sketch.add_circle(centre, 5.0);
        let west = sketch.add_point(DVec2::new(-10.0, 0.0));
        let east = sketch.add_point(DVec2::new(10.0, 0.0));
        sketch.add_segment(west, east);
        let below = sketch.add_point(DVec2::new(5.0, -3.0));
        let above = sketch.add_point(DVec2::new(5.0, 3.0));
        sketch.add_segment(below, above);

        let found = sketch.crossing_at(DVec2::new(5.0, 0.0), 0.5);

        assert_eq!(
            found,
            Some(Crossing::Curved),
            "cutting the two traits and leaving the circle whole is the half-division nobody asked for"
        );
    }

    #[test]
    fn a_drawing_far_from_the_origin_is_divisible_too() {
        let far = 1.0e7;
        let mut sketch = Sketch::new(WorkPlane::XY);
        let west = sketch.add_point(DVec2::new(far - 5.0, 0.0));
        let east = sketch.add_point(DVec2::new(far + 5.0, 0.0));
        let across = sketch.add_segment(west, east);
        let below = sketch.add_point(DVec2::new(far, -5.0));
        let above = sketch.add_point(DVec2::new(far, 5.0));
        let up = sketch.add_segment(below, above);

        let found = sketch.crossing_at(DVec2::new(far, 0.0), 1.0);

        assert_eq!(
            found,
            Some(Crossing::Traits {
                at: DVec2::new(far, 0.0),
                segments: vec![across, up],
            }),
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
