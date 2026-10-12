//! The angle at a corner, where two traits share an end.
//!
//! The two traits and their prolongations past the corner cut the plane into
//! four quarters, and which way each arm runs out from the corner — along its
//! trait, or along its prolongation — says which quarter a dimension measures.
//! Where it is put down is what chooses it.

use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::constraints::DimensionTarget;
use crate::sketch::{SegmentId, Sketch};

/// Below this sine of the angle between them, two traits run the same way and
/// cut no quarters at their corner.
const NO_QUARTERS: f64 = 1e-9;

/// Which way one arm of a corner's angle runs out from the corner.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Along {
    /// Along the trait itself, towards its far end.
    Trait,
    /// Along its prolongation, past the corner.
    Prolongation,
}

impl Along {
    fn way(self) -> f64 {
        match self {
            Self::Trait => 1.0,
            Self::Prolongation => -1.0,
        }
    }

    /// The arm on the side `place` lies, of the line the other trait runs on:
    /// the trait's own when both stand on the same side of it.
    fn facing(trait_side: f64, place_side: f64) -> Self {
        match trait_side * place_side >= 0.0 {
            true => Self::Trait,
            false => Self::Prolongation,
        }
    }
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

impl Sketch {
    /// The angle at the corner two traits share: the one the drawing already
    /// carries there, or else the one between the traits themselves.
    pub fn corner_angle(&self, first: SegmentId, second: SegmentId) -> DimensionTarget {
        self.angle_already_at(first, second)
            .unwrap_or(DimensionTarget::corner(first, second))
    }

    /// The angle a dimension already reads at the corner of these two traits,
    /// whichever way round they were clicked and whichever quarter it measures.
    ///
    /// A corner carries one angle at most: the other quarters are that one or
    /// what it leaves of a half turn.
    pub(crate) fn angle_already_at(
        &self,
        first: SegmentId,
        second: SegmentId,
    ) -> Option<DimensionTarget> {
        self.dimensions()
            .iter()
            .map(|dimension| dimension.target)
            .find(|target| {
                matches!(*target, DimensionTarget::Angle { first: one, second: other, .. }
                    if (one, other) == (first, second) || (one, other) == (second, first))
            })
    }

    /// The same angle, turned to face `placed`: the quarter it is put down in.
    ///
    /// A place lies in the quarter between two arms when it stands on each
    /// arm's side of the line the other trait runs on. Two traits running the
    /// same way cut no quarters, and the angle comes back as it was.
    pub(crate) fn facing_the_corner(
        &self,
        target: DimensionTarget,
        placed: DVec2,
    ) -> DimensionTarget {
        let DimensionTarget::Angle { first, second, .. } = target else {
            return target;
        };
        if let Some(laid) = self.angle_already_at(first, second) {
            return laid;
        }
        let Some((pivot, far_first, far_second)) = self.corner_points(first, second) else {
            return target;
        };
        let (one, other, place) = (far_first - pivot, far_second - pivot, placed - pivot);
        if one.perp_dot(other).abs() < one.length() * other.length() * NO_QUARTERS {
            return target;
        }
        DimensionTarget::Angle {
            first,
            first_along: Along::facing(other.perp_dot(one), other.perp_dot(place)),
            second,
            second_along: Along::facing(one.perp_dot(other), one.perp_dot(place)),
        }
    }

    /// Where a corner's angle is read, and its two arms out from there: the
    /// corner, and each trait's own way or its prolongation's, as long as the
    /// trait.
    pub(crate) fn corner_arms(&self, target: DimensionTarget) -> Option<(DVec2, DVec2, DVec2)> {
        let DimensionTarget::Angle {
            first,
            first_along,
            second,
            second_along,
        } = target
        else {
            return None;
        };
        self.segments().get(first.0)?;
        self.segments().get(second.0)?;
        let (pivot, far_first, far_second) = self.corner_points(first, second)?;
        Some((
            pivot,
            (far_first - pivot) * first_along.way(),
            (far_second - pivot) * second_along.way(),
        ))
    }
}

#[cfg(test)]
mod tests;
