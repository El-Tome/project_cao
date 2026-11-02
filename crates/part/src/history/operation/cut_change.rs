//! A fillet or a chamfer changed in place after it was laid. Each is one step
//! of the history, taken back by one undo, as any value retyped is.

use cao_sketch::CornerCutId;
use serde::{Deserialize, Serialize};

use super::ChamferAsked;
use crate::formula::Formula;

/// What is changed of a cut already laid.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CutChange {
    /// Every cut laid with `cut` takes these values: the corners a gesture
    /// rounded together change together.
    Reshape {
        sketch: usize,
        cut: CornerCutId,
        value: CutAsked,
    },
    /// The cuts taken off, their corners sharp again.
    TakeOff {
        sketch: usize,
        cuts: Vec<CornerCutId>,
    },
    /// A chamfer measures from its other side.
    SwapSides { sketch: usize, cut: CornerCutId },
    /// One cut drawn as construction, or as the outline.
    Construction {
        sketch: usize,
        cut: CornerCutId,
        construction: bool,
    },
}

impl CutChange {
    /// The sketch whose cut is changed.
    pub fn sketch(&self) -> usize {
        match self {
            Self::Reshape { sketch, .. }
            | Self::TakeOff { sketch, .. }
            | Self::SwapSides { sketch, .. }
            | Self::Construction { sketch, .. } => *sketch,
        }
    }
}

/// The values a cut is given again, as written: millimetres and degrees,
/// worked out against the part's variables on every rebuild.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CutAsked {
    Fillet(Formula),
    Chamfer(ChamferAsked),
}

impl CutAsked {
    /// Each value as the field it is typed in holds it, as stored: the
    /// first, then the second where the mode asks for two.
    pub fn written(&self) -> [Option<String>; 2] {
        let formulas = self.formulas();
        [0, 1].map(|rank| formulas.get(rank).map(|formula| formula.stored()))
    }

    /// Every formula the values are written with.
    pub fn formulas(&self) -> Vec<&Formula> {
        match self {
            Self::Fillet(radius) => vec![radius],
            Self::Chamfer(mode) => mode.formulas(),
        }
    }
}
