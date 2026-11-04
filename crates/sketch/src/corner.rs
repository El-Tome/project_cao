//! How a corner is named, and what a corner is: two sides ending at one point,
//! each a trait or a curve. The fillet and the chamfer both stand on one — see
//! `corner_cut` for what they draw there.

use std::f64::consts::TAU;

use serde::{Deserialize, Serialize};

use crate::corner_cut::CutSide;
use crate::sketch::{PointId, Sketch};

/// How a corner was named, as the history records it.
///
/// A point is enough wherever two sides meet. The two sides are recorded
/// only where a point cannot say which corner is meant, and for the chamfer
/// modes that measure from the side named first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Corner {
    /// The point two sides meet at. Which two is read from the drawing at the
    /// moment the cut is made.
    At(PointId),
    /// The two sides themselves, the one named first before the other — for a
    /// point too crowded to name a corner on its own, and for the chamfer
    /// modes that measure from the side named first.
    ///
    /// A side divided or trimmed since is followed through what it became,
    /// which is the part layer's business — [`Sketch::sides_of`] answers only
    /// for a drawing nothing has divided since.
    Between(CutSide, CutSide),
    /// These two sides at this point, the one named first before the other —
    /// for two sides sharing both their ends, a D or a lens, whose sides alone
    /// name two corners and whose point may be crowded by a third side.
    BetweenAt(PointId, CutSide, CutSide),
}

impl Sketch {
    /// The point a corner stands at and the two sides it stands between, as
    /// the drawing has them now.
    ///
    /// Nothing when the corner is no longer there to cut: a side erased by an
    /// earlier cut, a point that stopped being a corner, or two sides that end
    /// together at both their ends — a D, a lens — which make two corners and
    /// name neither until a point says which.
    pub fn sides_of(&self, corner: Corner) -> Option<(PointId, CutSide, CutSide)> {
        match corner {
            Corner::At(point) => {
                let centred = self.live_circles().any(|(_, round)| round.center == point)
                    || self.live_arcs().any(|(_, arc)| arc.center == point);
                match self.sides_ending_at(point).as_slice() {
                    [first, second] if !centred => Some((point, *first, *second)),
                    _ => None,
                }
            }
            Corner::Between(first, second) => match self.corners_between(first, second)[..] {
                [point] => Some((point, first, second)),
                _ => None,
            },
            Corner::BetweenAt(point, first, second) => {
                let ends_there = |side: CutSide| self.far_end_of(side, point).is_some();
                (first != second && ends_there(first) && ends_there(second))
                    .then_some((point, first, second))
            }
        }
    }

    /// The point a corner stands at, whichever way it was named. What a tool
    /// shows as taken.
    pub fn corner_point(&self, corner: Corner) -> Option<PointId> {
        match corner {
            Corner::At(point) | Corner::BetweenAt(point, ..) => Some(point),
            Corner::Between(..) => self.sides_of(corner).map(|(point, _, _)| point),
        }
    }

    /// The ends two sides share: none for two sides that do not meet, one for
    /// a corner, two for a D or a lens.
    pub fn corners_between(&self, first: CutSide, second: CutSide) -> Vec<PointId> {
        let (Some(ours), Some(theirs)) = (self.side_ends(first), self.side_ends(second)) else {
            return Vec::new();
        };
        let mut shared: Vec<PointId> = ours
            .into_iter()
            .filter(|end| theirs.contains(end))
            .collect();
        shared.dedup();
        shared
    }

    /// Every side drawn as the shape that ends at a point — its traits, arcs
    /// and arcs of ellipse, in the order the drawing holds them. How many
    /// there are is what tells a tool whether the point names a corner on its
    /// own, or whether it has to ask which two sides are meant.
    ///
    /// Only the sides the shape is drawn with count: a corner of construction
    /// traits is still named one trait at a time.
    pub fn sides_ending_at(&self, point: PointId) -> Vec<CutSide> {
        let traits = self
            .live_segments()
            .filter(|(_, side)| !side.construction && (side.start == point || side.end == point))
            .map(|(id, _)| CutSide::Trait(id));
        let arcs = self
            .live_arcs()
            .filter(|(_, arc)| !arc.construction && (arc.start == point || arc.end == point))
            .map(|(id, _)| CutSide::Arc(id));
        let arcs_of_ellipse = self
            .live_ellipses()
            .filter(|(_, ellipse)| !ellipse.construction)
            .filter(|(_, ellipse)| {
                ellipse
                    .drawn
                    .is_some_and(|(from, to)| from == point || to == point)
            })
            .map(|(id, _)| CutSide::ArcOfEllipse(id));
        traits.chain(arcs).chain(arcs_of_ellipse).collect()
    }

    /// The other side of the corner a side was named at, when the click that
    /// should have named it landed on the corner itself.
    ///
    /// Both sides pass through that point, so the nearest is whichever the
    /// drawing holds first — the same one every time, and the corner never
    /// completes. The side already named is the one thing that settles it:
    /// with exactly two sides meeting there, the other is the only choice
    /// left, which is not a guess.
    pub fn other_side_at(&self, point: PointId, named: CutSide) -> Option<CutSide> {
        let (_, first, second) = self.sides_of(Corner::At(point))?;
        match (first == named, second == named) {
            (true, false) => Some(second),
            (false, true) => Some(first),
            _ => None,
        }
    }

    /// The far end of each of two sides ending at `corner`, when it is a
    /// corner a tool may cut back.
    ///
    /// Not where either is an axis of an ellipse: the axes are what the curve
    /// stands on, and a side pulled back would leave it standing on nothing.
    pub(crate) fn corner_to_cut(
        &self,
        corner: PointId,
        first: CutSide,
        second: CutSide,
    ) -> Option<(PointId, PointId)> {
        let an_axis = |side: CutSide| {
            side.as_trait()
                .is_some_and(|segment| self.ellipse_of_axis(segment).is_some())
        };
        if an_axis(first) || an_axis(second) || first == second {
            return None;
        }
        Some((
            self.far_end_of(first, corner)?,
            self.far_end_of(second, corner)?,
        ))
    }

    /// How wide a corner stands open, the shorter way round.
    pub(crate) fn opening_at(
        &self,
        pivot: PointId,
        far_first: PointId,
        far_second: PointId,
    ) -> f64 {
        let at = self.point(pivot);
        let turn = ((self.point(far_second) - at).to_angle()
            - (self.point(far_first) - at).to_angle())
        .rem_euclid(TAU);
        turn.min(TAU - turn)
    }
}

#[cfg(test)]
mod tests;
