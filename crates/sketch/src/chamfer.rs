//! The three ways of saying how much of each side a chamfer takes. What a
//! chamfer draws is the corner's business — see `corner_cut`.

use serde::{Deserialize, Serialize};

/// Which of the three ways of saying a chamfer the tool is asking for. The
/// values themselves arrive separately, as they are typed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChamferMode {
    /// One distance, taken the same way along both sides.
    #[default]
    Equal,
    /// A distance along the first side, and the angle the cut leaves it at.
    Angled,
    /// A distance of its own along each side.
    Sided,
}

impl ChamferMode {
    /// How many values the mode needs typed.
    pub fn wants(self) -> usize {
        match self {
            Self::Equal => 1,
            Self::Angled | Self::Sided => 2,
        }
    }

    /// The chamfer the mode makes of the values typed for it.
    pub fn of(self, first: f64, second: f64) -> Chamfer {
        match self {
            Self::Equal => Chamfer::Equal(first),
            Self::Angled => Chamfer::Angled {
                along: first,
                degrees: second,
            },
            Self::Sided => Chamfer::Sided { first, second },
        }
    }
}

/// How much of each side of a corner a chamfer takes.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Chamfer {
    /// One distance, taken the same way along both sides.
    Equal(f64),
    /// A distance along the first side, and the angle the cut leaves it at.
    Angled { along: f64, degrees: f64 },
    /// A distance of its own along each side.
    Sided { first: f64, second: f64 },
}
