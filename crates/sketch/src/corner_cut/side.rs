//! What a side of a cut corner is: a curve ending at the corner.

use serde::{Deserialize, Serialize};

use crate::arc::ArcId;
use crate::ellipse::EllipseId;
use crate::naming::CurveId;
use crate::sketch::SegmentId;

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
}

/// The two traits a cut stands between, when both its sides are traits.
pub(crate) fn both_traits(sides: [CutSide; 2]) -> Option<[SegmentId; 2]> {
    Some([sides[0].as_trait()?, sides[1].as_trait()?])
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
