//! The point of a shape that stays where it is while another is pulled.

use crate::element::Element;
use crate::sketch::{PointId, SegmentId, Sketch};

impl Sketch {
    /// The point of a shape that stays where it is while `point` is pulled,
    /// when nothing else holds the shape: the one farthest from the hand.
    ///
    /// Taken among the points lying on the traits a rule of direction ties —
    /// their ends, and what is held on them — which is what makes it a
    /// rectangle's opposite corner whatever free tail hangs off it; from the
    /// whole shape only when no trait is tied. Farthest counted in joins, then
    /// in distance from where the point stood when the press landed. A point
    /// that is only a curve's centre, an ellipse's axis end and the touch of a
    /// tangency are never it: they are how a curve is held, not where a shape
    /// stands.
    pub(crate) fn stay_point(&self, point: PointId, shape: &[PointId]) -> Option<PointId> {
        // Counted along what is drawn first, when what is drawn with the point
        // hangs from nothing: a trait standing on a point held on the shape —
        // one drawn square off a side — only hangs from it, and its far end is
        // not where the shape stands. What hangs from something is held by
        // it, and stays by it.
        let drawn = self.drawn_pairs();
        let hangs = self
            .steps_from(point, &drawn)
            .iter()
            .enumerate()
            .any(|(each, step)| step.is_some() && !self.holds_on(PointId(each)).is_empty());
        match hangs {
            false => self
                .stay_along(point, shape, &drawn)
                .or_else(|| self.stay_along(point, shape, &self.joined_pairs())),
            true => self.stay_along(point, shape, &self.joined_pairs()),
        }
    }

    fn stay_along(
        &self,
        point: PointId,
        shape: &[PointId],
        pairs: &[(PointId, PointId)],
    ) -> Option<PointId> {
        let aside = self.held_aside();
        let steps = self.steps_from(point, pairs);
        let eligible = |each: &PointId| {
            *each != point
                && !self.is_erased_point(*each)
                && !self.only_a_centre(*each)
                && !aside.contains(each)
        };
        let ranked = |among: &[PointId]| -> Vec<(PointId, usize)> {
            among
                .iter()
                .filter(|each| eligible(each))
                .filter_map(|each| steps.get(each.0).copied().flatten().map(|far| (*each, far)))
                .collect()
        };
        let tied = self.tied_by_direction(shape);
        let profile: Vec<SegmentId> = tied
            .iter()
            .copied()
            .filter(|segment| !self.is_construction(Element::Segment(*segment)))
            .collect();
        let drawn: Vec<PointId> = shape
            .iter()
            .copied()
            .filter(|each| self.stands_in_the_profile(*each))
            .collect();
        let pool = [
            ranked(&self.points_on(&profile)),
            ranked(&self.points_on(&tied)),
            ranked(&drawn),
            ranked(shape),
        ]
        .into_iter()
        .find(|pool| !pool.is_empty())
        .unwrap_or_default();

        let from = self.point(point);
        let tie = self.drawing_size() * 1e-9;
        let mut best: Option<(PointId, usize, f64)> = None;
        for (each, joins) in pool {
            let far = self.point(each).distance(from);
            let wins = match best {
                None => true,
                Some((_, best_joins, best_far)) => {
                    joins > best_joins || (joins == best_joins && far > best_far + tie)
                }
            };
            if wins {
                best = Some((each, joins, far));
            }
        }
        best.map(|(each, _, _)| each)
    }

    /// Whether some piece of the profile stands on a point: construction
    /// gives before the profile, so a point only scaffolding stands on is
    /// never where the profile stays.
    fn stands_in_the_profile(&self, point: PointId) -> bool {
        let profile = |element: Element| !self.is_construction(element);
        self.live_segments().any(|(id, line)| {
            (line.start == point || line.end == point) && profile(Element::Segment(id))
        }) || self.live_arcs().any(|(id, arc)| {
            [arc.center, arc.start, arc.end].contains(&point) && profile(Element::Arc(id))
        }) || self
            .live_circles()
            .any(|(id, round)| round.center == point && profile(Element::Circle(id)))
            || self.live_ellipses().any(|(id, _)| {
                self.ellipse_stands_on(id).contains(&point) && profile(Element::Ellipse(id))
            })
    }
}
