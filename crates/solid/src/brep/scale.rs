//! The one tolerance of the kernel, and what it is taken against.

use serde::{Deserialize, Serialize};

/// How far a body reaches from the origin, never less than one, and the
/// distance under which two places are one.
///
/// A billionth of the reach: ten times the harness's `NEAR`, so what the kernel
/// keeps apart the rules see apart, and a thousand times below the tolerance
/// of a line of measure, so a merge never shows as matter gained or lost. The
/// result of an operation carries the larger reach of its operands, so a
/// later operation never decides with a finer tolerance than an earlier one.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scale {
    reach: f64,
}

impl Scale {
    pub const RELATIVE: f64 = 1e-9;

    pub fn of(reach: f64) -> Scale {
        Scale {
            reach: reach.abs().max(1.0),
        }
    }

    pub fn reach(self) -> f64 {
        self.reach
    }

    /// Two places closer than this are one.
    pub fn eps(self) -> f64 {
        Self::RELATIVE * self.reach
    }

    pub fn joined(self, other: Scale) -> Scale {
        Scale {
            reach: self.reach.max(other.reach),
        }
    }
}
