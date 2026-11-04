//! What a side of a cut corner is: a curve ending at the corner.

use serde::{Deserialize, Serialize};

use super::CutRefused;
use crate::arc::ArcId;
use crate::ellipse::EllipseId;
use crate::naming::CurveId;
use crate::sketch::{PointId, SegmentId, Sketch};

/// One of the two sides a cut stands between.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CutSide {
    Trait(SegmentId),
    Arc(ArcId),
    /// An ellipse drawn as an arc: a whole one has no end, so no corner.
    ArcOfEllipse(EllipseId),
}

impl CutSide {
    /// The trait this side is, when it is one.
    pub fn as_trait(self) -> Option<SegmentId> {
        match self {
            Self::Trait(segment) => Some(segment),
            Self::Arc(_) | Self::ArcOfEllipse(_) => None,
        }
    }

    /// Why a chamfer may not stand beside this side, when it may not: a
    /// chamfer cuts only a corner between two traits.
    pub fn refuses_a_chamfer(self) -> Option<CutRefused> {
        match self {
            Self::Trait(_) => None,
            Self::Arc(_) | Self::ArcOfEllipse(_) => Some(CutRefused::ChamferOnACurve),
        }
    }
}

/// The two traits a cut stands between, when both its sides are traits.
pub(crate) fn both_traits(sides: [CutSide; 2]) -> Option<[SegmentId; 2]> {
    Some([sides[0].as_trait()?, sides[1].as_trait()?])
}

impl Sketch {
    /// The two ends of a side, start then end: none for a whole ellipse,
    /// which has no end.
    pub(crate) fn side_ends(&self, side: CutSide) -> Option<[PointId; 2]> {
        match side {
            CutSide::Trait(segment) => {
                let segment = self.segments()[segment.0];
                Some([segment.start, segment.end])
            }
            CutSide::Arc(arc) => {
                let arc = self.arc(arc);
                Some([arc.start, arc.end])
            }
            CutSide::ArcOfEllipse(ellipse) => {
                self.ellipse_ends(ellipse).map(|(from, to)| [from, to])
            }
        }
    }

    /// The end of a side away from the corner, when the side ends there.
    pub(crate) fn far_end_of(&self, side: CutSide, corner: PointId) -> Option<PointId> {
        match self.side_ends(side)? {
            [start, end] if start == corner => Some(end),
            [start, end] if end == corner => Some(start),
            _ => None,
        }
    }

    pub(crate) fn is_erased_side(&self, side: CutSide) -> bool {
        match side {
            CutSide::Trait(segment) => self.is_erased_segment(segment),
            CutSide::Arc(arc) => self.is_erased_arc(arc),
            CutSide::ArcOfEllipse(ellipse) => self.is_erased_ellipse(ellipse),
        }
    }
}

impl From<SegmentId> for CutSide {
    fn from(segment: SegmentId) -> Self {
        Self::Trait(segment)
    }
}

impl From<ArcId> for CutSide {
    fn from(arc: ArcId) -> Self {
        Self::Arc(arc)
    }
}

impl From<EllipseId> for CutSide {
    fn from(ellipse: EllipseId) -> Self {
        Self::ArcOfEllipse(ellipse)
    }
}

impl From<CutSide> for CurveId {
    fn from(side: CutSide) -> Self {
        match side {
            CutSide::Trait(segment) => Self::Segment(segment),
            CutSide::Arc(arc) => Self::Arc(arc),
            CutSide::ArcOfEllipse(ellipse) => Self::Ellipse(ellipse),
        }
    }
}
