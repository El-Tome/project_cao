//! Drawings made at random with every tool of the sketch mode, the rules their
//! areas must keep, and everything a failing one needs to be read and run
//! again by a human.
//!
//! A drawing is a list of gestures, in the words a printed case is written in,
//! so that a drawing found by a campaign pastes into a named test as it is.
//! A gesture acts on what lies at a place rather than on an index, the way a
//! click does: dropped from a drawing while it shrinks, the gestures after it
//! still mean what they meant, and one left with nothing under its place does
//! nothing.

pub mod areas;
mod campaign;
pub mod checking;
mod drawing;
pub mod enclosing;
mod laying;
mod printing;
mod random;
mod rules;
mod smaller;

pub use campaign::{Check, Report, answer, apart, campaign, shrink};
pub use checking::{check, holds, played};
pub use drawing::drawn;
pub use laying::laid;
pub use printing::Drawing;
pub use random::Random;
pub use rules::{
    Area, Flaw, Place, Relaying, Rule, Silence, holding, laid_alike, nothing_extra,
    nothing_missing, surface, tint_is_measure,
};
pub use smaller::smaller;

/// One gesture of the sketch mode.
#[derive(Clone, Debug, PartialEq)]
pub enum Gesture {
    /// Traits from place to place, each starting where the last one ended,
    /// and one more back to the first place when `closed`.
    Chain {
        through: Vec<[f64; 2]>,
        closed: bool,
        construction: bool,
    },
    /// Two opposite corners, the sides square to the axes.
    Rectangle {
        corner: [f64; 2],
        opposite: [f64; 2],
        construction: bool,
    },
    Circle {
        centre: [f64; 2],
        radius: f64,
        construction: bool,
    },
    /// A piece of circle, counter-clockwise about `centre` from `start`, over
    /// so many degrees.
    Arc {
        centre: [f64; 2],
        start: [f64; 2],
        degrees: f64,
        construction: bool,
    },
    /// An ellipse from its centre, the end of its first axis, and how far its
    /// second reaches either side.
    Ellipse {
        centre: [f64; 2],
        reach: [f64; 2],
        across: f64,
        construction: bool,
    },
    /// Half an ellipse across two ends, rising to the left of the way from
    /// one to the other by `rise`, to its right when `rise` is negative.
    HalfEllipse {
        from: [f64; 2],
        to: [f64; 2],
        rise: f64,
        construction: bool,
    },
    /// A point on its own.
    Point { at: [f64; 2] },
    /// The corner standing at a place, rounded.
    Fillet { at: [f64; 2], radius: f64 },
    /// The corner standing at a place, cut back the same length along both
    /// sides.
    Chamfer { at: [f64; 2], length: f64 },
    /// What lies under the places, copied across an axis.
    Mirror { of: Vec<[f64; 2]>, axis: Axis },
    /// What lies under the places, copied round the point standing at
    /// `centre`: `count` of them, the original among them, `degrees` apart.
    PatternAround {
        of: Vec<[f64; 2]>,
        centre: [f64; 2],
        degrees: f64,
        count: usize,
    },
    /// What lies under the places, copied along an axis and across it: for
    /// each way, how far apart and how many, the original among them.
    PatternAlong {
        of: Vec<[f64; 2]>,
        axis: Axis,
        along: (f64, usize),
        across: (f64, usize),
    },
    /// The traits and arcs crossing at a place, each cut in two there.
    Divide { at: [f64; 2] },
    /// The stretch of curve under a place, between the points on either side
    /// of it, taken away.
    Trim { at: [f64; 2] },
    /// What lies under a place, erased.
    Erase { at: [f64; 2] },
}

/// What a copy is laid along or across.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Axis {
    /// The sketch's horizontal axis.
    U,
    /// The sketch's vertical axis.
    V,
    /// The trait lying under a place.
    Trait([f64; 2]),
}

impl Gesture {
    /// Whether the gesture draws something new rather than changing what is
    /// there.
    pub fn draws(&self) -> bool {
        matches!(
            self,
            Gesture::Chain { .. }
                | Gesture::Rectangle { .. }
                | Gesture::Circle { .. }
                | Gesture::Arc { .. }
                | Gesture::Ellipse { .. }
                | Gesture::HalfEllipse { .. }
                | Gesture::Point { .. }
        )
    }
}
