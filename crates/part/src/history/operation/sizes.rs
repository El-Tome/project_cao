//! The sizes a tool was asked for, as the user wrote them. Each is a formula,
//! worked out against the part's variables every time the part is rebuilt —
//! which is what lets a chamfer or a pattern follow a variable.

use cao_sketch::{Chamfer, Repeats};
use serde::{Deserialize, Serialize};

use super::Operation;
use crate::formula::Formula;

impl Operation {
    /// Every size a step or a tool was written with, gestures opened up.
    ///
    /// A value set on a drawing is not one of them: it carries what it was
    /// written from on the drawing itself, where a cut can hand it on to
    /// another trait — so the drawing is the one to ask.
    pub(crate) fn sizes(&self) -> Vec<&Formula> {
        match self {
            Self::Gesture(done) => done.iter().flat_map(Self::sizes).collect(),
            Self::Extrude { distance, .. } => vec![distance],
            Self::Revolve { angle, .. } => vec![angle],
            Self::Fillet { radius, .. } => vec![radius],
            Self::Chamfer { mode, .. } => mode.formulas(),
            Self::CircularPattern { degrees, count, .. } => vec![degrees, count],
            Self::RectangularPattern { along, across, .. } => {
                vec![&along.step, &along.count, &across.step, &across.count]
            }
            _ => Vec::new(),
        }
    }

    /// What a part with no scale yet would learn from this operation's sizes,
    /// when it carries one that says anything about size at all.
    ///
    /// A count is not a size, and an angle says nothing about how big anything
    /// is: a revolution and a circular pattern answer nothing.
    pub(crate) fn first_value(&self) -> Option<FirstValue> {
        match self {
            Self::Gesture(done) => done.iter().find_map(Self::first_value),
            Self::Extrude { .. } | Self::RectangularPattern { .. } => {
                Some(FirstValue::OffTheDrawing)
            }
            Self::Fillet { .. } | Self::Chamfer { .. } => Some(FirstValue::AgainstTheDrawing),
            Self::SetDimension { target, .. } => {
                (!target.is_angle()).then_some(FirstValue::AgainstTheDrawing)
            }
            _ => None,
        }
    }
}

/// What the first value a part is given teaches it about its scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FirstValue {
    /// A length the drawing already measures. What a unit is worth comes out of
    /// comparing the two, and nothing moves.
    AgainstTheDrawing,
    /// A length with nothing drawn to read it against — a step of matter's
    /// depth, a pattern's step. A unit is a millimetre.
    OffTheDrawing,
}

/// How much of each side of a corner a chamfer takes, as written.
///
/// Written into a part file the way [`Chamfer`] is, so that a chamfer typed as
/// plain numbers reads as it always did.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ChamferAsked {
    Equal(Formula),
    Angled { along: Formula, degrees: Formula },
    Sided { first: Formula, second: Formula },
}

impl ChamferAsked {
    /// The chamfer these come to, or nothing when one of them comes to no
    /// number at all.
    pub fn worked_out(&self, values: &[f64]) -> Option<Chamfer> {
        Some(match self {
            Self::Equal(reach) => Chamfer::Equal(reach.value(values)?),
            Self::Angled { along, degrees } => Chamfer::Angled {
                along: along.value(values)?,
                degrees: degrees.value(values)?,
            },
            Self::Sided { first, second } => Chamfer::Sided {
                first: first.value(values)?,
                second: second.value(values)?,
            },
        })
    }

    /// Each value as written, in the order `Chamfered::typed` lays them down
    /// as dimensions: the reach twice for an even cut, otherwise the two
    /// values in the order they are asked.
    pub(crate) fn as_laid(&self) -> [&Formula; 2] {
        match self {
            Self::Equal(reach) => [reach, reach],
            Self::Angled { along, degrees } => [along, degrees],
            Self::Sided { first, second } => [first, second],
        }
    }

    /// Every formula it is written with.
    pub fn formulas(&self) -> Vec<&Formula> {
        match self {
            Self::Equal(reach) => vec![reach],
            Self::Angled { along, degrees } => vec![along, degrees],
            Self::Sided { first, second } => vec![first, second],
        }
    }
}

impl From<Chamfer> for ChamferAsked {
    fn from(chamfer: Chamfer) -> Self {
        match chamfer {
            Chamfer::Equal(reach) => Self::Equal(Formula::Number(reach)),
            Chamfer::Angled { along, degrees } => Self::Angled {
                along: Formula::Number(along),
                degrees: Formula::Number(degrees),
            },
            Chamfer::Sided { first, second } => Self::Sided {
                first: Formula::Number(first),
                second: Formula::Number(second),
            },
        }
    }
}

/// One direction of a rectangular pattern as written: how far apart the copies
/// stand, and how many there are with the original among them. Written into a
/// part file the way [`Repeats`] is.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepeatsAsked {
    pub step: Formula,
    pub count: Formula,
}

impl RepeatsAsked {
    /// The run these come to, or nothing when the step comes to no number or
    /// the count to no whole one.
    pub fn worked_out(&self, values: &[f64]) -> Option<Repeats> {
        Some(Repeats {
            step: self.step.value(values)?,
            count: self.count.whole(values)?,
        })
    }
}

impl From<Repeats> for RepeatsAsked {
    fn from(run: Repeats) -> Self {
        Self {
            step: Formula::Number(run.step),
            count: Formula::Number(run.count as f64),
        }
    }
}
