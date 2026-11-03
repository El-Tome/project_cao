//! What a rule says of one of the sketch's own axes.

use super::{Constraint, SketchAxis};
use crate::sketch::SegmentId;

impl Constraint {
    /// The trait a rule lays the way of one of the sketch's axes, and the axis
    /// it then runs along: a trait square to one axis runs along the other.
    pub(crate) fn along_an_axis(self) -> Option<(SegmentId, SketchAxis)> {
        match self {
            Self::AxisCollinear { segment, axis } | Self::AxisParallel { segment, axis } => {
                Some((segment, axis))
            }
            Self::AxisPerpendicular { segment, axis } => Some((segment, axis.across())),
            _ => None,
        }
    }
}
