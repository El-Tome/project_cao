//! How a corner is named, and what a corner is: two sides ending at one point,
//! each a trait or a curve. The fillet and the chamfer both stand on one — see
//! `corner_cut` for what they draw there.

use std::f64::consts::TAU;

use serde::{Deserialize, Serialize};

use crate::corner_cut::CutSide;
use crate::sketch::{PointId, SegmentId, Sketch};

/// How a corner was named, as the history records it.
///
/// A point is enough wherever two traits meet. The two traits are recorded
/// only where a point cannot say which corner is meant, and for the chamfer
/// modes that measure from the side named first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Corner {
    /// The point two traits meet at. Which two is read from the drawing at the
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
}

impl Sketch {
    /// The two sides a corner stands between, as the drawing has them now.
    ///
    /// Nothing when the corner is no longer there to cut: a trait erased by an
    /// earlier cut, or a point that stopped being a corner.
    pub fn sides_of(&self, corner: Corner) -> Option<(CutSide, CutSide)> {
        match corner {
            Corner::At(point) => self.sides_at(point),
            Corner::Between(first, second) => {
                self.point_between(first, second).map(|_| (first, second))
            }
        }
    }

    /// The point two traits meet at, for naming the corner they make.
    pub fn shared_point(&self, first: SegmentId, second: SegmentId) -> Option<PointId> {
        let (pivot, _, _) = self.shared_corner(first, second)?;
        Some(pivot)
    }

    /// The point a corner stands at, whichever way it was named. What a tool
    /// shows as taken, and what the other trait of a half-named corner is
    /// looked up from.
    pub fn corner_point(&self, corner: Corner) -> Option<PointId> {
        match corner {
            Corner::At(point) => Some(point),
            Corner::Between(first, second) => self.point_between(first, second),
        }
    }

    /// The point two sides meet at: the first end of `first` that is an end
    /// of `second` too.
    fn point_between(&self, first: CutSide, second: CutSide) -> Option<PointId> {
        let theirs = self.side_ends(second)?;
        self.side_ends(first)?
            .into_iter()
            .find(|end| theirs.contains(end))
    }

    /// The two sides a point is the corner of: the traits, the arcs and the
    /// arcs of ellipse drawn as the shape that end there, when exactly two do
    /// and no curve is centred on it.
    fn sides_at(&self, point: PointId) -> Option<(CutSide, CutSide)> {
        let centred = self.live_circles().any(|(_, round)| round.center == point)
            || self.live_arcs().any(|(_, arc)| arc.center == point);
        if centred {
            return None;
        }
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
        let sides: Vec<CutSide> = self
            .traits_at(point)
            .into_iter()
            .map(CutSide::Trait)
            .chain(arcs)
            .chain(arcs_of_ellipse)
            .collect();
        match sides.as_slice() {
            [first, second] => Some((*first, *second)),
            _ => None,
        }
    }

    /// The other trait of the corner a side was named at, when the click that
    /// should have named it landed on the corner itself.
    ///
    /// Both traits pass through that point, so the nearest is whichever the
    /// drawing holds first — the same one every time, and the corner never
    /// completes. The side already named is the one thing that settles it:
    /// with exactly two traits meeting there, the other is the only choice
    /// left, which is not a guess.
    pub fn other_side_at(&self, point: PointId, named: SegmentId) -> Option<SegmentId> {
        let (first, second) = self.corner_at(point)?;
        match (first == named, second == named) {
            (true, false) => Some(second),
            (false, true) => Some(first),
            _ => None,
        }
    }

    /// The corner two sides share, when it is one a tool may cut back.
    ///
    /// Not where either is an axis of an ellipse: the axes are what the curve
    /// stands on, and a side pulled back would leave it standing on nothing.
    pub(crate) fn corner_to_cut(
        &self,
        first: CutSide,
        second: CutSide,
    ) -> Option<(PointId, PointId, PointId)> {
        let an_axis = |side: CutSide| {
            side.as_trait()
                .is_some_and(|segment| self.ellipse_of_axis(segment).is_some())
        };
        if an_axis(first) || an_axis(second) {
            return None;
        }
        let corner = self.point_between(first, second)?;
        Some((
            corner,
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

    /// The corner a point makes: the two traits that meet there, when exactly
    /// two do and nothing else leans on it.
    ///
    /// This is what lets a corner be named by one click instead of two. Three
    /// traits leave no saying which two the cut is meant for, and a curve
    /// running in is neither one of them nor nothing — a corner is still two
    /// straight traits. Both cases are refused here rather than guessed at, so
    /// the tool can ask for the two traits by name.
    ///
    /// Only the traits the shape is drawn with count: a corner of construction
    /// traits is still named one trait at a time.
    pub fn corner_at(&self, point: PointId) -> Option<(SegmentId, SegmentId)> {
        if !self.arcs_leaning_on(point).is_empty()
            || self.live_circles().any(|(_, round)| round.center == point)
        {
            return None;
        }
        match self.traits_at(point).as_slice() {
            [first, second] => Some((*first, *second)),
            _ => None,
        }
    }

    /// Every trait that runs into a point, in the order the drawing holds
    /// them. How many there are is what tells a tool whether the point names a
    /// corner on its own, or whether it has to ask which two traits are meant.
    pub fn traits_at(&self, point: PointId) -> Vec<SegmentId> {
        self.live_segments()
            .filter(|(_, side)| !side.construction)
            .filter(|(_, side)| side.start == point || side.end == point)
            .map(|(id, _)| id)
            .collect()
    }
}

#[cfg(test)]
mod tests;
