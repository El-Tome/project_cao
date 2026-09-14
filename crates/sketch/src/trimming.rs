//! Taking a stretch out of a trait, between two of the points sitting on it.

use glam::DVec2;

use crate::constraints::Constraint;
use crate::erased::Erased;
use crate::sketch::{Element, PointId, SegmentId, Sketch};

/// Below this, the two ends of a piece are the same place and the piece is no
/// trait at all. Far under anything a drawing tells apart: a gap this small
/// only ever comes of a cut landing on a point already sitting at an end.
const NO_LENGTH: f64 = 1e-9;

impl Sketch {
    /// The points sitting on a trait, each with how far along it they sit, in
    /// order from its start to its end.
    ///
    /// Its own two ends are the first and the last of them; anything held on
    /// its body — a rule, a crossing given a point, a place that simply landed
    /// there — falls between the two.
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
    ///
    /// Nothing when the cut cannot be made: a trait or a point the drawing does
    /// not have, a point that does not fall between the trait's own ends, or a
    /// cut that would take nothing away and hand back the whole trait.
    pub fn trim(
        &mut self,
        segment: SegmentId,
        from: PointId,
        to: PointId,
    ) -> Option<Vec<SegmentId>> {
        let cut = self.segments().get(segment.0).copied()?;
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let reach = span.length_squared();
        if reach == 0.0 {
            self.erase(Element::Segment(segment));
            return Some(Vec::new());
        }
        let along = |place: DVec2| (place - start).dot(span) / reach;

        let from_at = along(self.points().get(from.0).copied()?);
        let to_at = along(self.points().get(to.0).copied()?);
        if !(0.0..=1.0).contains(&from_at) || !(0.0..=1.0).contains(&to_at) {
            return None;
        }
        let (low, high) = match from_at <= to_at {
            true => (from, to),
            false => (to, from),
        };
        let (opens, closes) = (from_at.min(to_at), from_at.max(to_at));

        let keeps_below = self.is_a_piece(cut.start, low);
        let keeps_above = self.is_a_piece(high, cut.end);
        let hands_the_trait_back =
            (keeps_below && !keeps_above && self.point(low).distance(end) <= NO_LENGTH)
                || (keeps_above && !keeps_below && self.point(high).distance(start) <= NO_LENGTH);
        if hands_the_trait_back {
            return None;
        }

        let held: Vec<(f64, PointId)> = self
            .constraints()
            .iter()
            .filter_map(|rule| match rule {
                Constraint::OnSegment { point, segment: on } if *on == segment => {
                    Some((along(self.points().get(point.0).copied()?), *point))
                }
                _ => None,
            })
            .collect();

        // A tangency carries its contact point away when it goes, since the
        // point would be left in mid-air. Stopping a cut on that contact is
        // the case where it would not: it is an end of a trait now, and that
        // is what holds it.
        let standing: Vec<PointId> = [cut.start, low, high, cut.end]
            .into_iter()
            .filter(|point| !self.is_erased_point(*point))
            .collect();

        self.erase(Element::Segment(segment));

        for point in standing {
            Erased::unmark(&mut self.erased.points, point.0);
        }

        let below = keeps_below.then(|| self.piece(cut.start, low, cut.construction));
        let above = keeps_above.then(|| self.piece(high, cut.end, cut.construction));

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

        Some(below.into_iter().chain(above).collect())
    }

    /// Whether what runs between these two points is a trait at all.
    fn is_a_piece(&self, from: PointId, to: PointId) -> bool {
        from != to && self.point(from).distance(self.point(to)) > NO_LENGTH
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
mod tests;
