//! Taking a stretch out of a trait, between two of the points sitting on it.

use glam::DVec2;

use crate::constraints::Constraint;
use crate::sketch::{Element, PointId, SegmentId, Sketch};

impl Sketch {
    /// The points sitting on a trait, in order from its start to its end.
    ///
    /// Its own two ends are the first and the last of them; anything held on
    /// its body — a rule, a crossing given a point, a place that simply landed
    /// there — falls between the two.
    pub fn points_along(&self, segment: SegmentId, tolerance: f64) -> Vec<PointId> {
        self.sitting_along(segment, tolerance)
            .into_iter()
            .map(|(_, id)| id)
            .collect()
    }

    /// The same, each point with how far along the trait it sits.
    fn sitting_along(&self, segment: SegmentId, tolerance: f64) -> Vec<(f64, PointId)> {
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let reach = span.length_squared();
        if reach == 0.0 {
            return Vec::new();
        }

        let mut sitting: Vec<(f64, PointId)> = self
            .live_points()
            .filter_map(|(id, place)| {
                let fraction = (place - start).dot(span) / reach;
                let foot = start + span * fraction;
                ((0.0..=1.0).contains(&fraction) && place.distance(foot) <= tolerance)
                    .then_some((fraction, id))
            })
            .collect();
        sitting.sort_by(|first, second| first.0.total_cmp(&second.0));
        sitting
    }

    /// Takes the stretch between `from` and `to` out of a trait, and leaves
    /// what is left of it as traits of their own.
    ///
    /// The two named points survive, as the ends the pieces now stop at. Named
    /// the same point twice, nothing is taken away and the trait is merely cut
    /// in two there — which is what a fillet, a chamfer and a split each need
    /// of a trait before they can do their own work.
    pub fn trim(&mut self, segment: SegmentId, from: PointId, to: PointId) -> Vec<SegmentId> {
        let Some(cut) = self.segments().get(segment.0).copied() else {
            return Vec::new();
        };
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let reach = span.length_squared();
        if reach == 0.0 {
            self.erase(Element::Segment(segment));
            return Vec::new();
        }
        let along = |place: DVec2| (place - start).dot(span) / reach;

        let (low, high) = match along(self.point(from)) <= along(self.point(to)) {
            true => (from, to),
            false => (to, from),
        };
        let (opens, closes) = (along(self.point(low)), along(self.point(high)));
        let held: Vec<(f64, PointId)> = self
            .constraints()
            .iter()
            .filter_map(|rule| match rule {
                Constraint::OnSegment { point, segment: on } if *on == segment => {
                    Some((along(self.point(*point)), *point))
                }
                _ => None,
            })
            .collect();

        self.erase(Element::Segment(segment));

        let below = (cut.start != low).then(|| self.piece(cut.start, low, cut.construction));
        let above = (high != cut.end).then(|| self.piece(high, cut.end, cut.construction));

        for (fraction, point) in held {
            let piece = match fraction {
                _ if fraction < opens => below,
                _ if fraction > closes => above,
                _ => None,
            };
            if let Some(segment) = piece {
                self.add_constraint(Constraint::OnSegment { point, segment });
            }
        }

        below.into_iter().chain(above).collect()
    }

    fn piece(&mut self, from: PointId, to: PointId, construction: bool) -> SegmentId {
        match construction {
            true => self.add_construction_segment(from, to),
            false => self.add_segment(from, to),
        }
    }

    /// The stretch of a trait a click falls in: the two points it runs
    /// between.
    ///
    /// Nothing when the trait carries no stretch at all — a trait with no
    /// length, which is the one case where there is nothing to run between.
    pub fn stretch_at(
        &self,
        segment: SegmentId,
        at: DVec2,
        tolerance: f64,
    ) -> Option<(PointId, PointId)> {
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let reach = span.length_squared();
        if reach == 0.0 {
            return None;
        }

        // Clamped, so that a click read as just past an end still asks for the
        // stretch that ends there rather than for nothing at all.
        let fraction = ((at - start).dot(span) / reach).clamp(0.0, 1.0);
        self.sitting_along(segment, tolerance)
            .windows(2)
            .find(|stretch| fraction <= stretch[1].0)
            .map(|stretch| (stretch[0].1, stretch[1].1))
    }
}

#[cfg(test)]
mod tests {
    use crate::plane::WorkPlane;

    use super::*;

    const NEAR: f64 = 1e-6;

    #[test]
    fn the_points_sitting_on_a_trait_come_in_order_from_its_start() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(0.0, 1.0));
        let end = sketch.add_point(DVec2::new(10.0, 1.0));
        let segment = sketch.add_segment(start, end);

        let far = sketch.add_point(DVec2::new(7.0, 1.0));
        let near = sketch.add_point(DVec2::new(3.0, 1.0));
        sketch.add_point(DVec2::new(5.0, 4.0));
        sketch.add_point(DVec2::new(14.0, 1.0));

        assert_eq!(
            sketch.points_along(segment, NEAR),
            vec![start, near, far, end],
        );
    }

    fn a_trait_with_two_points_on_it() -> (Sketch, SegmentId, [PointId; 4]) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(0.0, 1.0));
        let end = sketch.add_point(DVec2::new(10.0, 1.0));
        let segment = sketch.add_segment(start, end);
        let near = sketch.add_point(DVec2::new(3.0, 1.0));
        let far = sketch.add_point(DVec2::new(7.0, 1.0));
        (sketch, segment, [start, near, far, end])
    }

    #[test]
    fn a_click_names_the_stretch_it_fell_in() {
        let (sketch, segment, [start, near, far, end]) = a_trait_with_two_points_on_it();

        for (at, expected) in [
            (DVec2::new(1.0, 1.0), (start, near)),
            (DVec2::new(5.0, 1.2), (near, far)),
            (DVec2::new(9.0, 1.0), (far, end)),
        ] {
            assert_eq!(
                sketch.stretch_at(segment, at, NEAR),
                Some(expected),
                "at {at}"
            );
        }
    }

    #[test]
    fn trimming_a_stretch_leaves_the_rest_of_the_trait_standing() {
        let (mut sketch, segment, [start, near, far, end]) = a_trait_with_two_points_on_it();

        let kept = sketch.trim(segment, near, far);

        assert!(
            sketch.is_erased_segment(segment),
            "the trait it was cut from is gone"
        );
        assert_eq!(
            kept.len(),
            2,
            "an end on either side of the stretch taken out"
        );
        let ends: Vec<(PointId, PointId)> = kept
            .iter()
            .map(|piece| {
                let piece = sketch.segments()[piece.0];
                (piece.start, piece.end)
            })
            .collect();
        assert_eq!(ends, vec![(start, near), (far, end)]);
    }

    #[test]
    fn trimming_a_trait_nothing_sits_on_takes_the_whole_trait() {
        let (mut sketch, segment, [start, _, _, end]) = a_trait_with_two_points_on_it();

        assert!(sketch.trim(segment, start, end).is_empty());
        assert!(sketch.is_erased_segment(segment));
    }

    #[test]
    fn naming_one_point_twice_cuts_the_trait_there_and_takes_nothing() {
        let (mut sketch, segment, [start, near, _, end]) = a_trait_with_two_points_on_it();

        let pieces = sketch.trim(segment, near, near);

        let ends: Vec<(PointId, PointId)> = pieces
            .iter()
            .map(|piece| {
                let piece = sketch.segments()[piece.0];
                (piece.start, piece.end)
            })
            .collect();
        assert_eq!(ends, vec![(start, near), (near, end)]);
    }

    #[test]
    fn a_trait_that_only_helps_build_the_drawing_still_only_helps_once_trimmed() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(0.0, 1.0));
        let end = sketch.add_point(DVec2::new(10.0, 1.0));
        let segment = sketch.add_construction_segment(start, end);
        let middle = sketch.add_point(DVec2::new(5.0, 1.0));

        let pieces = sketch.trim(segment, middle, end);

        assert_eq!(pieces.len(), 1);
        assert!(sketch.segments()[pieces[0].0].construction);
    }

    #[test]
    fn a_point_held_on_a_trait_is_held_on_the_piece_it_lands_on() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(0.0, 1.0));
        let end = sketch.add_point(DVec2::new(10.0, 1.0));
        let segment = sketch.add_segment(start, end);
        let held = sketch.add_point(DVec2::new(2.0, 1.0));
        sketch.add_constraint(Constraint::OnSegment {
            point: held,
            segment,
        });
        let from = sketch.add_point(DVec2::new(5.0, 1.0));
        let to = sketch.add_point(DVec2::new(8.0, 1.0));

        let pieces = sketch.trim(segment, from, to);

        assert!(
            sketch.constraints().contains(&Constraint::OnSegment {
                point: held,
                segment: pieces[0],
            }),
            "held on {:?}, and the rules standing are {:?}",
            pieces[0],
            sketch.constraints(),
        );
    }
}
