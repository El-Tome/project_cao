//! What a cut draws, worked out from its corner every time it is read.
//!
//! Nothing here is solved. The solver holds the corner and its two whole
//! sides; the curve or the cut, where it meets each side, and what is left of
//! the sides to draw follow from them and from the values.
//!
//! A cut fits when each of its sides has room for what it takes, together
//! with what the cuts drawn as the outline at the side's other end take. Two
//! cuts on the same end of a side, between it and two other traits of the
//! point, overlap rather than add up: the larger is what that end gives. One
//! that does not still has a place, where it would be, so it can be shown
//! there; the outline and the area take the sharp corner instead until it fits
//! again. A cut in construction takes nothing from the outline, so it holds no
//! other cut back.
//!
//! Every reading here is one pass over the cuts, each side's share indexed
//! once: the canvas asks for them several times a frame, on drawings a
//! pattern can fill with hundreds of corners.

use std::collections::HashMap;

use glam::DVec2;

use super::{CornerCut, CornerCutId, CutShape, CutSide, both_traits};
use crate::arcing::ArcDraft;
use crate::chamfer::Chamfer;
use crate::sketch::{PointId, SegmentId, Sketch};

/// Under this a cut takes nothing off a side, or the corner stands too open
/// or too closed for a cut to have a place.
pub(super) const NOTHING: f64 = 1e-12;

/// The curve a cut draws.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CutCurve {
    /// A fillet: the arc from one side to the other, counter-clockwise.
    Arc(ArcDraft),
    /// A chamfer: the straight cut from the first side to the second.
    Straight { from: DVec2, to: DVec2 },
}

/// A cut as it is drawn now.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawnCut {
    pub id: CornerCutId,
    /// The two sides, the one named first first.
    pub sides: [CutSide; 2],
    /// The point the corner stands at, and where it stands.
    pub corner_point: PointId,
    pub corner: DVec2,
    /// Where the cut meets each side, in the order the sides were named.
    pub touches: [DVec2; 2],
    pub curve: CutCurve,
    /// Whether each side has room for it.
    pub fits: bool,
    pub construction: bool,
}

/// A cut worked out on its own, before what the other cuts on its sides take
/// is counted: where it meets each side, the curve it draws, how much of each
/// side it takes from the corner and how much room the side has for it — both
/// in the side's own measure, a length along a trait, an angle round an arc,
/// a turn round an ellipse. `alone` is false for one that fits nowhere.
#[derive(Clone, Copy, Debug)]
pub(super) struct Worked {
    pub(super) touches: [DVec2; 2],
    pub(super) curve: CutCurve,
    pub(super) takes: [f64; 2],
    pub(super) room: [f64; 2],
    pub(super) alone: bool,
}

/// Where the cuts that fit and are drawn as the outline pull the ends of each
/// side back to, read once for the whole drawing: start then end, nothing
/// for an end no cut pulls.
#[derive(Clone, Debug, Default)]
pub struct PulledEnds(HashMap<SegmentId, [Option<DVec2>; 2]>);

impl Sketch {
    /// Every cut standing, as it is drawn now. A cut on a corner standing
    /// exactly straight, or folded exactly shut, has no place anywhere and is
    /// left out: there is nothing to draw for it, and its corner stays sharp.
    pub fn drawn_cuts(&self) -> Vec<DrawnCut> {
        let tolerance = crate::precision::SAME * crate::precision::drawing_size(self);
        let worked = self.worked_cuts(tolerance);
        let taken = self.taken_from_the_outline(&worked);
        worked
            .iter()
            .filter_map(|(id, cut, worked)| {
                let worked = (*worked)?;
                let fits = worked.alone
                    && (0..2).all(|rank| {
                        let side = cut.sides[rank];
                        let other = taken.get(&side).copied().unwrap_or_default()
                            [1 - self.end_of(side, cut.corner)];
                        worked.takes[rank] + other <= worked.room[rank]
                    });
                Some(DrawnCut {
                    id: *id,
                    sides: cut.sides,
                    corner_point: cut.corner,
                    corner: self.point(cut.corner),
                    touches: worked.touches,
                    curve: worked.curve,
                    fits,
                    construction: cut.construction,
                })
            })
            .collect()
    }

    /// One cut as it is drawn now.
    pub fn drawn_cut(&self, id: CornerCutId) -> Option<DrawnCut> {
        self.drawn_cuts().into_iter().find(|drawn| drawn.id == id)
    }

    /// Where the cuts among `drawn` pull the ends of each side back to. Of two
    /// cuts on one end, the side shows up to the one meeting it nearer the
    /// corner, and the other joins it further along.
    pub fn pulled_ends(&self, drawn: &[DrawnCut]) -> PulledEnds {
        let mut pulled: HashMap<SegmentId, [Option<DVec2>; 2]> = HashMap::new();
        for cut in drawn.iter().filter(|cut| cut.fits && !cut.construction) {
            for (rank, side) in cut.sides.iter().enumerate() {
                let Some(side) = side.as_trait() else {
                    continue;
                };
                let touch = cut.touches[rank];
                let end = &mut pulled.entry(side).or_default()
                    [self.end_of(side.into(), cut.corner_point)];
                let nearer = end
                    .is_none_or(|before| touch.distance(cut.corner) < before.distance(cut.corner));
                if nearer {
                    *end = Some(touch);
                }
            }
        }
        PulledEnds(pulled)
    }

    /// What is left to draw of a side once the cuts on its ends have taken
    /// their part, from its start towards its end. The whole trait when no cut
    /// that fits stands on it, and nothing when two cuts meeting exactly have
    /// taken all of it.
    ///
    /// Only a cut drawn as the outline takes its part: one in construction
    /// leaves the side whole.
    pub fn visible_part(&self, segment: SegmentId) -> Option<(DVec2, DVec2)> {
        let side = self.segments()[segment.0];
        let pulled = self.pulled_ends(&self.drawn_cuts());
        let [from, to] = self.visible_ends(segment, &pulled)?;
        Some((
            from.unwrap_or(self.point(side.start)),
            to.unwrap_or(self.point(side.end)),
        ))
    }

    /// Where the cuts pull each end of a side back to, start then end —
    /// nothing for an end no cut pulls — or nothing at all when they leave no
    /// straight piece.
    pub fn visible_ends(
        &self,
        segment: SegmentId,
        pulled: &PulledEnds,
    ) -> Option<[Option<DVec2>; 2]> {
        let side = self.segments()[segment.0];
        let ends = pulled.0.get(&segment).copied().unwrap_or_default();
        let (start, end) = (self.point(side.start), self.point(side.end));
        let (from, to) = (ends[0].unwrap_or(start), ends[1].unwrap_or(end));
        let tolerance = crate::precision::SAME * start.distance(end);
        match (to - from).dot((end - start).normalize_or_zero()) > tolerance {
            true => Some(ends),
            false => None,
        }
    }

    /// How much of the values given a gesture can lay on these corners at
    /// most, as a share of them: 1 when they fit as given, less when the
    /// tightest side holds them back, nothing when a corner gives a cut no
    /// place at all.
    ///
    /// Every length the shape takes off a side grows with its values, so the
    /// share is read side by side: what is left of each once the cuts already
    /// drawn as the outline have taken theirs, over what the new cuts would
    /// take.
    /// A corner already cut is cut again in place, so what its cut takes now
    /// is not counted against it.
    ///
    /// On each end of a side the new cuts and the standing ones overlap, the
    /// larger giving what the end takes; what a new cut on one end has to
    /// leave room for is what the other end takes.
    ///
    /// A fillet asked beside a curve takes nothing in proportion to its
    /// radius, and the share is searched for instead.
    pub fn fitting_share(&self, corners: &[(CutSide, CutSide)], shape: CutShape) -> f64 {
        if let CutShape::Fillet { radius } = shape
            && corners
                .iter()
                .any(|(first, second)| both_traits([*first, *second]).is_none())
        {
            return self.share_by_halving(corners, radius);
        }
        let replaced: Vec<CornerCutId> = corners
            .iter()
            .filter_map(|(first, second)| self.cut_standing_between(*first, *second))
            .collect();
        let tolerance = crate::precision::SAME * crate::precision::drawing_size(self);
        let standing: Vec<_> = self
            .worked_cuts(tolerance)
            .into_iter()
            .filter(|(id, _, _)| !replaced.contains(id))
            .collect();
        let taken = self.taken_from_the_outline(&standing);
        let mut share = f64::INFINITY;
        let mut asked: HashMap<SegmentId, [f64; 2]> = HashMap::new();
        for (first, second) in corners {
            let Some((corner, far_first, far_second)) = self.corner_to_cut(*first, *second) else {
                continue;
            };
            let Some(sides) = both_traits([*first, *second]) else {
                continue;
            };
            let Some(back) = self.backs(corner, [far_first, far_second], shape) else {
                return 0.0;
            };
            for (side, own) in sides.into_iter().zip(back) {
                let end = &mut asked.entry(side).or_default()[self.end_of(side.into(), corner)];
                *end = end.max(own);
            }
        }
        for (side, wanted) in asked {
            let gone = taken.get(&side.into()).copied().unwrap_or_default();
            let (start, end) = self.endpoints(side);
            let room = start.distance(end) + tolerance;
            for at in [0, 1] {
                if wanted[at] > 0.0 {
                    share = share.min((room - gone[1 - at]) / wanted[at]);
                }
            }
            if wanted[0] > 0.0 && wanted[1] > 0.0 {
                share = share.min(room / (wanted[0] + wanted[1]));
            }
        }
        share.max(0.0)
    }

    /// Every cut standing, each worked out on its own — nothing for a corner
    /// that gives it no place.
    fn worked_cuts(&self, tolerance: f64) -> Vec<(CornerCutId, CornerCut, Option<Worked>)> {
        self.live_corner_cuts()
            .map(|(id, cut)| {
                let worked = match both_traits(cut.sides) {
                    Some(sides) => self.worked_between_traits(&cut, sides, tolerance),
                    None => self.worked_beside_a_curve(&cut, tolerance),
                };
                (id, cut, worked)
            })
            .collect()
    }

    /// How far back from the corner a cut takes each side, in the order the
    /// sides are given — nothing when the corner stands too open or too closed
    /// for it, or the values take nothing.
    fn backs(&self, corner: PointId, fars: [PointId; 2], shape: CutShape) -> Option<[f64; 2]> {
        let opening = self.opening_at(corner, fars[0], fars[1]);
        let backs = match shape {
            CutShape::Fillet { radius } => {
                let back = radius / (opening * 0.5).tan();
                [back, back]
            }
            CutShape::Chamfer(Chamfer::Equal(reach)) => [reach, reach],
            CutShape::Chamfer(Chamfer::Sided { first, second }) => [first, second],
            CutShape::Chamfer(Chamfer::Angled { along, degrees }) => {
                // The angle is read at the place the cut meets the first side,
                // inside the triangle the cut closes: the corner's own angle
                // and what is left of half a turn are the other two, and the
                // sine rule gives the second distance.
                let leaves = degrees.to_radians();
                let closing = (opening + leaves).sin();
                [along, along * leaves.sin() / closing]
            }
        };
        let placed = opening > NOTHING && opening < std::f64::consts::PI - NOTHING;
        match placed && backs.iter().all(|back| back.is_finite() && *back > NOTHING) {
            true => Some(backs),
            false => None,
        }
    }

    /// Which end of a side a corner stands on: 0 for its start, 1 for its end.
    fn end_of(&self, side: CutSide, corner: PointId) -> usize {
        self.side_ends(side)
            .map_or(0, |[start, _]| usize::from(start != corner))
    }

    /// How much of each end of a side the cuts drawn as the outline take,
    /// start then end, in the side's own measure: the largest of the cuts
    /// standing on that end.
    fn taken_from_the_outline(
        &self,
        worked: &[(CornerCutId, CornerCut, Option<Worked>)],
    ) -> HashMap<CutSide, [f64; 2]> {
        let mut taken: HashMap<CutSide, [f64; 2]> = HashMap::new();
        for (_, cut, worked) in worked.iter().filter(|(_, cut, _)| !cut.construction) {
            let Some(worked) = worked else {
                continue;
            };
            for (side, own) in cut.sides.iter().zip(worked.takes) {
                let end = &mut taken.entry(*side).or_default()[self.end_of(*side, cut.corner)];
                *end = end.max(own);
            }
        }
        taken
    }

    /// The end of a side away from the corner.
    pub(super) fn far_end(&self, side: SegmentId, corner: PointId) -> PointId {
        let segment = self.segments()[side.0];
        match segment.start == corner {
            true => segment.end,
            false => segment.start,
        }
    }

    /// A cut between two traits worked out on its own.
    fn worked_between_traits(
        &self,
        cut: &CornerCut,
        sides: [SegmentId; 2],
        tolerance: f64,
    ) -> Option<Worked> {
        let fars = sides.map(|side| self.far_end(side, cut.corner));
        let back = self.backs(cut.corner, fars, cut.shape)?;
        let corner = self.point(cut.corner);
        let away = sides
            .map(|side| (self.point(self.far_end(side, cut.corner)) - corner).normalize_or_zero());
        let touches = [corner + away[0] * back[0], corner + away[1] * back[1]];
        let curve = match cut.shape {
            CutShape::Fillet { radius } => {
                let inwards = (away[0] + away[1]).try_normalize()?;
                let opening = away[0].angle_to(away[1]).abs();
                let centre = corner + inwards * (radius / (opening * 0.5).sin());
                CutCurve::Arc(short_way_round(centre, touches))
            }
            CutShape::Chamfer(_) => CutCurve::Straight {
                from: touches[0],
                to: touches[1],
            },
        };
        let room = sides.map(|side| {
            let (start, end) = self.endpoints(side);
            start.distance(end) + tolerance
        });
        Some(Worked {
            touches,
            curve,
            takes: back,
            room,
            alone: true,
        })
    }
}

/// The arc of a fillet between the two places it touches its sides, the
/// short way round: the side facing the corner.
pub(super) fn short_way_round(centre: DVec2, touches: [DVec2; 2]) -> ArcDraft {
    let (start, end) = match (touches[0] - centre).perp_dot(touches[1] - centre) > 0.0 {
        true => (touches[0], touches[1]),
        false => (touches[1], touches[0]),
    };
    ArcDraft { centre, start, end }
}

#[cfg(test)]
mod tests;
