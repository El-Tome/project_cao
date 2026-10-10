//! The angle at a corner, where two traits share an end.
//!
//! The two traits and their prolongations past the corner cut the plane into
//! four quarters, and which way each arm runs out from the corner — along its
//! trait, or along its prolongation — says which quarter a dimension measures.

use serde::{Deserialize, Serialize};

use crate::constraints::DimensionTarget;
use crate::sketch::SegmentId;

/// Which way one arm of a corner's angle runs out from the corner.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Along {
    /// Along the trait itself, towards its far end.
    Trait,
    /// Along its prolongation, past the corner.
    Prolongation,
}

impl DimensionTarget {
    /// The angle between two traits sharing an end, both arms along the traits
    /// themselves: what a corner's angle reads unless it was put down
    /// elsewhere.
    pub fn corner(first: SegmentId, second: SegmentId) -> Self {
        Self::Angle {
            first,
            first_along: Along::Trait,
            second,
            second_along: Along::Trait,
        }
    }
}
