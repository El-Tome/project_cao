//! A fillet or a chamfer, as a property of the sharp corner it stands on.
//!
//! The corner stays two whole traits meeting at a point: nothing is trimmed,
//! no curve is added, no rule is laid. The cut only says what is drawn there
//! instead of the corner, and what is drawn is worked out from the corner, its
//! two traits and the values each time it is read — see `drawn`. Every gesture
//! on the corner is a gesture on a sharp corner, and the cut follows it.
//!
//! The values are in the drawing's own units, as its points are. The history
//! records millimetres, and converts on the way in.

use serde::{Deserialize, Serialize};

use crate::chamfer::Chamfer;
use crate::erased::Erased;
use crate::sketch::{PointId, SegmentId, Sketch};

mod changing;
mod curved;
mod drawn;
mod hanging;
mod picking;
pub(crate) mod points;
mod showing;
mod side;

pub use drawn::{CutCurve, DrawnCut, PulledEnds};
pub use picking::Grip;
pub use points::CutPoint;
pub use showing::Stretches;
pub use side::CutSide;
pub(crate) use side::both_traits;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CornerCutId(pub usize);

/// What is drawn where the corner was.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CutShape {
    /// A curve of this radius, tangent to both sides.
    Fillet { radius: f64 },
    /// A straight cut, as much of each side taken as the chamfer says.
    Chamfer(Chamfer),
}

/// A fillet or a chamfer standing on a corner.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CornerCut {
    /// The point the two sides meet at.
    pub corner: PointId,
    /// The two sides, the one named first before the other: a chamfer in two
    /// distances, or in a distance and an angle, measures from the first.
    pub sides: [CutSide; 2],
    pub shape: CutShape,
    /// Whether the cut is drawn as construction, decided when it was laid
    /// whatever its sides are.
    pub construction: bool,
    /// The cuts laid by one gesture share one, and one value: they are one
    /// rounding of several corners, and change together.
    pub group: usize,
}

/// Why a corner was not cut.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CutRefused {
    /// The two sides do not end at one point, one of them is an axis of an
    /// ellipse, which the curve stands on, or the cut is a chamfer and a side
    /// is not a trait.
    NoCorner,
}

impl Sketch {
    /// Cuts the corner two sides share. A corner already cut is cut again:
    /// what it carried is taken off and the new cut stands in its place, in
    /// the group it is laid with — a corner re-cut leaves the group it was
    /// laid in.
    ///
    /// Every pair of traits meeting at a point is a corner of its own: a cut
    /// between two other traits of the same point stands beside this one.
    ///
    /// A value too big for the corner is still laid: the cut is drawn as one
    /// that does not fit, and the sharp corner stands in for it until the
    /// drawing makes room. Holding a value back before it is laid is the
    /// tool's business, through [`Sketch::fitting_share`].
    pub fn cut_corner(
        &mut self,
        first: impl Into<CutSide>,
        second: impl Into<CutSide>,
        shape: CutShape,
        construction: bool,
        group: usize,
    ) -> Result<CornerCutId, CutRefused> {
        let (first, second) = (first.into(), second.into());
        if matches!(shape, CutShape::Chamfer(_)) && both_traits([first, second]).is_none() {
            return Err(CutRefused::NoCorner);
        }
        let (corner, _, _) = self
            .corner_to_cut(first, second)
            .ok_or(CutRefused::NoCorner)?;
        let standing = self.cut_standing_between(first, second);
        if let Some(standing) = standing {
            Erased::mark(&mut self.erased.cuts, standing.0);
        }
        self.corner_cuts.push(CornerCut {
            corner,
            sides: [first, second],
            shape,
            construction,
            group,
        });
        let laid = CornerCutId(self.corner_cuts.len() - 1);
        if let Some(standing) = standing {
            let swapped = self.corner_cuts[standing.0].sides[0] != first;
            self.hand_hung_over(standing, laid, swapped);
        }
        Ok(laid)
    }

    /// Every cut ever laid, by rank, erased ones included.
    pub fn corner_cuts(&self) -> &[CornerCut] {
        &self.corner_cuts
    }

    pub fn is_erased_cut(&self, cut: CornerCutId) -> bool {
        Erased::holds(&self.erased.cuts, cut.0)
    }

    /// The cuts still standing, with their rank: not erased, their corner
    /// and both sides still drawn, the sides still running into the corner.
    pub fn live_corner_cuts(&self) -> impl Iterator<Item = (CornerCutId, CornerCut)> + '_ {
        self.corner_cuts
            .iter()
            .enumerate()
            .map(|(rank, cut)| (CornerCutId(rank), *cut))
            .filter(|(id, cut)| !self.is_erased_cut(*id) && self.stands(cut))
    }

    /// Whether what a cut stands on is still drawn as it was laid.
    fn stands(&self, cut: &CornerCut) -> bool {
        !self.is_erased_point(cut.corner)
            && cut.sides.iter().all(|side| {
                !self.is_erased_side(*side) && self.far_end_of(*side, cut.corner).is_some()
            })
    }

    /// Every cut standing on a point, one for each pair of its traits cut.
    pub fn cuts_standing_at(&self, corner: PointId) -> Vec<CornerCutId> {
        self.live_corner_cuts()
            .filter(|(_, cut)| cut.corner == corner)
            .map(|(id, _)| id)
            .collect()
    }

    /// The cut standing between two sides, whichever was named first.
    pub fn cut_standing_between(&self, first: CutSide, second: CutSide) -> Option<CornerCutId> {
        self.live_corner_cuts()
            .find(|(_, cut)| cut.sides == [first, second] || cut.sides == [second, first])
            .map(|(id, _)| id)
    }

    /// The cuts laid with this one, itself included.
    pub fn cuts_grouped_with(&self, cut: CornerCutId) -> Vec<CornerCutId> {
        let Some(group) = self.corner_cuts.get(cut.0).map(|cut| cut.group) else {
            return Vec::new();
        };
        self.live_corner_cuts()
            .filter(|(_, other)| other.group == group)
            .map(|(id, _)| id)
            .collect()
    }

    /// A group no cut of the drawing uses yet.
    pub fn fresh_cut_group(&self) -> usize {
        self.corner_cuts
            .iter()
            .map(|cut| cut.group + 1)
            .max()
            .unwrap_or(0)
    }

    /// Takes away every cut whose corner or either side was erased. A cut
    /// taken away stays taken away: a trait drawn again from its corner later
    /// is a corner nobody has cut.
    pub(crate) fn cuts_lose_what_is_erased(&mut self) {
        for rank in 0..self.corner_cuts.len() {
            let cut = self.corner_cuts[rank];
            let lost = self.is_erased_point(cut.corner)
                || cut.sides.iter().any(|side| self.is_erased_side(*side));
            if lost {
                Erased::mark(&mut self.erased.cuts, rank);
            }
        }
    }

    /// The cuts standing on a side, read before a trim erases it.
    pub(crate) fn cuts_standing_on(&self, side: CutSide) -> Vec<CornerCutId> {
        self.live_corner_cuts()
            .filter(|(_, cut)| cut.sides.contains(&side))
            .map(|(id, _)| id)
            .collect()
    }

    /// Hands each of `standing` — the cuts that stood on a trait now cut
    /// apart — to the piece still running into its corner, whatever rank that
    /// piece got. A trim erases the trait before it lays the pieces, so the
    /// cuts it took away are given back here; one whose corner no piece runs
    /// into stays gone with the trait. A cut taken off or replaced before the
    /// trim is none of these, and stays as it was.
    pub(crate) fn cuts_follow_pieces(
        &mut self,
        standing: &[CornerCutId],
        was: SegmentId,
        pieces: &[SegmentId],
    ) {
        for id in standing {
            let cut = self.corner_cuts[id.0];
            let Some(at) = cut
                .sides
                .iter()
                .position(|side| *side == CutSide::Trait(was))
            else {
                continue;
            };
            let touching = pieces.iter().copied().find(|piece| {
                let segment = self.segments()[piece.0];
                segment.start == cut.corner || segment.end == cut.corner
            });
            let Some(piece) = touching else {
                continue;
            };
            self.corner_cuts[id.0].sides[at] = piece.into();
            if self.stands(&self.corner_cuts[id.0]) {
                Erased::unmark(&mut self.erased.cuts, id.0);
            }
        }
    }
}

#[cfg(test)]
mod tests;
