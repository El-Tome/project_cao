//! What hangs on a cut, through the changes the cut goes through.
//!
//! A point hung on a cut's point, or held on its curve, and a line brushing a
//! fillet, are held by a rule that names the cut. The cut can be taken off,
//! given another shape, have its sides swapped, or be cut again in place, and
//! each of these says what becomes of those rules: what still has somewhere to
//! stand follows, and what has lost its place is let go where it is. Nothing
//! hung on a cut is ever erased with it.

use glam::DVec2;

use super::points::CutPoint;
use super::{CornerCutId, CutCurve, CutShape};
use crate::constraints::Constraint;
use crate::edges::off_by;
use crate::holding::Support;
use crate::laid_from::LaidFrom;
use crate::sketch::{PointId, Sketch};
use crate::snap::onto_rim;

impl Sketch {
    /// Whether a cut still stands and still offers this point: a chamfer
    /// has no centre. Read off the cut as it was laid, never off where its
    /// corner stands this instant, so that a corner dragged through straight
    /// does not let go of what hangs on it.
    pub(crate) fn cut_offers(&self, cut: CornerCutId, at: CutPoint) -> bool {
        let Some(laid) = self.live_cut(cut) else {
            return false;
        };
        at != CutPoint::Centre || matches!(laid.shape, CutShape::Fillet { .. })
    }

    /// Whether a cut still stands, as a rule holding a point on its curve
    /// reads it.
    pub(crate) fn cut_holds(&self, cut: CornerCutId) -> bool {
        self.live_cut(cut).is_some()
    }

    /// A cut something may hang on: one still standing, between two traits.
    /// Nothing hangs on a cut beside a curve.
    fn live_cut(&self, cut: CornerCutId) -> Option<super::CornerCut> {
        let laid = *self.corner_cuts().get(cut.0)?;
        let between_traits = super::both_traits(laid.sides).is_some();
        (!self.is_erased_cut(cut) && self.stands(&laid) && between_traits).then_some(laid)
    }

    /// The points a cut is worked out from: its corner, then the far end of
    /// each side. A point that is one of them is not hung on the cut — it
    /// is what the cut stands on.
    pub(crate) fn cut_stands_on(&self, cut: CornerCutId) -> Vec<PointId> {
        let Some(laid) = self.corner_cuts().get(cut.0).copied() else {
            return Vec::new();
        };
        let fars = laid
            .sides
            .iter()
            .filter_map(|side| side.as_trait())
            .map(|side| self.far_end(side, laid.corner));
        std::iter::once(laid.corner).chain(fars).collect()
    }

    /// What a point laid at this place would hang on, among the cuts: one of
    /// their points, which holds it entirely, or else the curve it lies on.
    pub(crate) fn cut_supports_at(&self, place: DVec2) -> Vec<Support> {
        let near_enough = off_by(place);
        if let Some((cut, at)) = self.nearest_cut_point(place, near_enough) {
            return vec![Support::CutPoint(cut, at)];
        }
        self.drawn_cuts()
            .into_iter()
            .filter(|drawn| super::picking::distance_to(drawn.curve, place) <= near_enough)
            .map(|drawn| Support::Cut(drawn.id))
            .collect()
    }

    /// Where a place lands, pulled onto the whole curve a cut draws: the
    /// circle a fillet is a piece of, or the line a chamfer's cut lies on.
    pub(crate) fn onto_cut(&self, cut: CornerCutId, place: DVec2) -> Option<DVec2> {
        match self.drawn_cut(cut)?.curve {
            CutCurve::Arc(arc) => onto_rim(place, arc.centre, arc.centre.distance(arc.start)),
            CutCurve::Straight { from, to } => {
                let along = (to - from).try_normalize()?;
                Some(from + along * (place - from).dot(along))
            }
        }
    }

    /// Whether anything hangs on a cut of the drawing: what a change to the
    /// cuts has to settle, the places it hangs on having moved with them.
    pub fn anything_hangs_on_a_cut(&self) -> bool {
        self.constraints()
            .iter()
            .any(|rule| rule.hung_on_a_cut().is_some())
    }

    /// Every rule hanging something on one of these cuts.
    pub(crate) fn rules_hung_on(&self, cuts: &[CornerCutId]) -> Vec<Constraint> {
        self.constraints()
            .iter()
            .copied()
            .filter(|rule| rule.hung_on_a_cut().is_some_and(|cut| cuts.contains(&cut)))
            .collect()
    }

    /// Lays again the rules a change took away with their cut, once it stands
    /// again: a trim erases the trait a cut stands on before it hands the cut
    /// to the piece left at the corner.
    pub(crate) fn hang_again(&mut self, rules: &[Constraint]) {
        for rule in rules {
            self.add_constraint(*rule);
        }
    }

    /// What a cut taken off leaves of what hung on it: each point where it
    /// is, an end of the cut still held on its side, the rest let go.
    pub(crate) fn let_go_of_the_cut(&mut self, cut: CornerCutId) {
        let sides = self.corner_cuts()[cut.0].sides;
        for rule in self.rules_hung_on(&[cut]) {
            self.erase_constraint(rule);
            let Constraint::AtCutPoint { point, at, .. } = rule else {
                continue;
            };
            let side = match at {
                CutPoint::First => sides[0],
                CutPoint::Second => sides[1],
                CutPoint::Centre => continue,
            };
            let Some(segment) = side.as_trait() else {
                continue;
            };
            self.add_constraint(Constraint::OnSegment {
                point,
                segment,
                from: LaidFrom::Nowhere,
            });
        }
    }

    /// What a cut given another shape keeps: everything, but what leans on a
    /// fillet's circle once it is a chamfer, which has none — a point at its
    /// centre, a line brushing it — each left where it was. A point on the
    /// curve goes onto the new one.
    pub(crate) fn keep_what_the_shape_offers(&mut self, cut: CornerCutId) {
        for rule in self.rules_hung_on(&[cut]) {
            let needs = match rule {
                Constraint::AtCutPoint { at, .. } => at,
                Constraint::CutTangent { .. } => CutPoint::Centre,
                _ => continue,
            };
            if !self.cut_offers(cut, needs) {
                self.erase_constraint(rule);
            }
        }
    }

    /// The points hung on a cut's ends follow their side when the sides
    /// trade ranks.
    pub(crate) fn follow_the_sides_swapped(&mut self, cut: CornerCutId) {
        self.hand_hung_over(cut, cut, true);
    }

    /// Hands what hung on a cut to the one laid in its place, on the same
    /// corner: an end follows its side, whichever rank the new cut gives it,
    /// and what the new shape does not offer is let go.
    pub(crate) fn hand_hung_over(&mut self, from: CornerCutId, to: CornerCutId, swapped: bool) {
        for rule in self.rules_hung_on(&[from]) {
            self.erase_constraint(rule);
            let handed = match rule {
                Constraint::AtCutPoint { point, at, .. } => Constraint::AtCutPoint {
                    point,
                    cut: to,
                    at: match swapped {
                        true => at.swapped(),
                        false => at,
                    },
                },
                Constraint::OnCut { point, .. } => Constraint::OnCut { point, cut: to },
                Constraint::CutTangent { segment, from, .. } => Constraint::CutTangent {
                    cut: to,
                    segment,
                    from,
                },
                other => other,
            };
            self.add_constraint(handed);
        }
    }
}

impl Constraint {
    /// The cut a rule hangs something on, when it is one of those: a point,
    /// or a line brushing a fillet. Such a rule stands only once its cut is
    /// laid.
    pub fn hung_on_a_cut(self) -> Option<CornerCutId> {
        match self {
            Self::AtCutPoint { cut, .. }
            | Self::OnCut { cut, .. }
            | Self::CutTangent { cut, .. } => Some(cut),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests;
