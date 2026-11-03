//! The points a cut offers to hang something on: the centre of a fillet, and
//! the two places a cut meets its sides.
//!
//! None of them is a point of the drawing until something hangs on it. They
//! are shown and aimed at all the same, read off the cut as it is drawn; once
//! a line starts there, or a dimension measures from there, a point is laid
//! and a rule holds it to the place — see [`Constraint::AtCutPoint`]. The rule
//! works the place out again from the corner, its sides and the value, with
//! how it moves with each of them: that is what lets pulling what hangs move
//! the corner.
//!
//! [`Constraint::AtCutPoint`]: crate::constraints::Constraint::AtCutPoint

use glam::DVec2;
use serde::{Deserialize, Serialize};

use super::drawn::NOTHING;
use super::{CornerCutId, CutCurve, CutShape, DrawnCut};
use crate::chamfer::Chamfer;
use crate::jet::{Jet, JetPlace};
use crate::sketch::Sketch;

/// How many coordinates a cut's places are worked out over: the corner and
/// the far end of each side, then the point a rule holds — or the two ends of
/// a line brushing a fillet.
pub(crate) const COORDINATES: usize = 10;

/// Where the corner's coordinates sit among them, then each far end's.
pub(crate) const CORNER: usize = 0;
pub(crate) const FAR_FIRST: usize = 2;
pub(crate) const FAR_SECOND: usize = 4;
/// Where the point the rule holds sits, or the first end of the line.
pub(crate) const OWN: usize = 6;
/// Where the other end of a line brushing a fillet sits.
pub(crate) const OWN_END: usize = 8;

/// One of the points a cut offers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CutPoint {
    /// The centre of a fillet's curve. A chamfer has none.
    Centre,
    /// Where the cut meets the side named first.
    First,
    /// Where it meets the other side.
    Second,
}

impl CutPoint {
    /// The same place once the two sides have traded ranks.
    pub(crate) fn swapped(self) -> Self {
        match self {
            Self::Centre => Self::Centre,
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}

impl DrawnCut {
    /// The points this cut offers, where it is drawn now: a fillet's centre,
    /// then where it meets each side.
    pub fn points(&self) -> impl Iterator<Item = (CutPoint, DVec2)> + use<> {
        let centre = match self.curve {
            CutCurve::Arc(arc) => Some((CutPoint::Centre, arc.centre)),
            CutCurve::Straight { .. } => None,
        };
        centre.into_iter().chain([
            (CutPoint::First, self.touches[0]),
            (CutPoint::Second, self.touches[1]),
        ])
    }
}

/// A cut's places as the solver reads them: each with how it moves with the
/// corner and the far ends of the sides.
pub(crate) struct CutPlaces {
    /// Which way the first side leaves the corner: the frame a place is read
    /// in, so that a shape turned whole reads the same.
    pub along: JetPlace<COORDINATES>,
    pub touches: [JetPlace<COORDINATES>; 2],
    /// The centre of a fillet's curve, and its radius.
    pub round: Option<(JetPlace<COORDINATES>, f64)>,
}

impl CutPlaces {
    pub(crate) fn at(&self, point: CutPoint) -> Option<JetPlace<COORDINATES>> {
        match point {
            CutPoint::Centre => self.round.map(|(centre, _)| centre),
            CutPoint::First => Some(self.touches[0]),
            CutPoint::Second => Some(self.touches[1]),
        }
    }
}

impl Sketch {
    /// Where one point of a cut stands now, as it is drawn: nothing for a cut
    /// standing nowhere, or for the centre of a chamfer.
    pub fn cut_point(&self, cut: CornerCutId, point: CutPoint) -> Option<DVec2> {
        let drawn = self.drawn_cut(cut)?;
        match point {
            CutPoint::First => Some(drawn.touches[0]),
            CutPoint::Second => Some(drawn.touches[1]),
            CutPoint::Centre => match drawn.curve {
                CutCurve::Arc(arc) => Some(arc.centre),
                CutCurve::Straight { .. } => None,
            },
        }
    }

    /// Every point the cuts drawn offer, where it stands: what the canvas
    /// shows as points, whether something hangs on them yet or not.
    pub fn cut_points(&self) -> Vec<(CornerCutId, CutPoint, DVec2)> {
        self.drawn_cuts()
            .into_iter()
            .flat_map(|drawn| drawn.points().map(move |(at, place)| (drawn.id, at, place)))
            .collect()
    }

    /// The point of a cut nearest `position`, within `reach`.
    pub fn nearest_cut_point(
        &self,
        position: DVec2,
        reach: f64,
    ) -> Option<(CornerCutId, CutPoint)> {
        self.cut_points()
            .into_iter()
            .map(|(cut, point, at)| (cut, point, at.distance(position)))
            .filter(|(_, _, distance)| *distance <= reach)
            .min_by(|(_, _, one), (_, _, other)| one.total_cmp(other))
            .map(|(cut, point, _)| (cut, point))
    }

    /// A cut's places worked out again from its corner and its sides, with
    /// how each moves with them: nothing for a cut that does not stand, or
    /// whose corner gives it no place — which is when it is not drawn either.
    pub(crate) fn cut_places(&self, cut: CornerCutId) -> Option<CutPlaces> {
        let laid = *self.corner_cuts().get(cut.0)?;
        if self.is_erased_cut(cut) || !self.stands(&laid) {
            return None;
        }
        let corner = JetPlace::free(self.point(laid.corner), CORNER);
        let far = |side: usize, rank: usize| {
            JetPlace::free(
                self.point(self.far_end(laid.sides[side], laid.corner)),
                rank,
            )
        };
        places(corner, [far(0, FAR_FIRST), far(1, FAR_SECOND)], laid.shape)
    }
}

/// A cut's places, from its corner, the far end of each side and its shape:
/// the formula `drawn` reads in plain numbers, here carried with its slopes.
fn places(
    corner: JetPlace<COORDINATES>,
    fars: [JetPlace<COORDINATES>; 2],
    shape: CutShape,
) -> Option<CutPlaces> {
    let along = fars.map(|far| (far - corner).normalized());
    let cos = along[0].dot(along[1]);
    let sin = along[0].perp_dot(along[1]).abs();
    let opening = sin.atan2(cos);
    let placed = opening.value > NOTHING && opening.value < std::f64::consts::PI - NOTHING;
    if !placed {
        return None;
    }
    let constant = Jet::constant;
    let (backs, round) = match shape {
        CutShape::Fillet { radius } => {
            // The tangent points stand radius / tan(half the opening) back
            // from the corner, and the centre radius / sin(half of it) along
            // the line halving it — both read here off the sine and cosine of
            // the whole opening, which the unit sides give without an angle.
            let back = constant(radius) * (cos + 1.0) / sin;
            let centre = corner + (along[0] + along[1]).scaled(constant(radius) / sin);
            ([back, back], Some((centre, radius)))
        }
        CutShape::Chamfer(Chamfer::Equal(reach)) => ([constant(reach), constant(reach)], None),
        CutShape::Chamfer(Chamfer::Sided { first, second }) => {
            ([constant(first), constant(second)], None)
        }
        CutShape::Chamfer(Chamfer::Angled {
            along: reach,
            degrees,
        }) => {
            let leaves = degrees.to_radians();
            let second = constant(reach * leaves.sin()) / (opening + leaves).sin();
            ([constant(reach), second], None)
        }
    };
    let finite = backs
        .iter()
        .all(|back| back.value.is_finite() && back.value > NOTHING);
    finite.then(|| CutPlaces {
        along: along[0],
        touches: [
            corner + along[0].scaled(backs[0]),
            corner + along[1].scaled(backs[1]),
        ],
        round,
    })
}

#[cfg(test)]
mod tests;
