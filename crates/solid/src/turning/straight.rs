//! A profile of straight runs laid square to its axis: every run parallel to
//! the axis or square to it, exactly, ready for the exact kernel to turn.

use glam::DVec2;

use super::Turn;
use crate::profile::{Contour, Frame};

/// A profile whose every run is parallel or square to its axis, read along
/// the axis and away from it.
#[derive(Clone, Debug, PartialEq)]
pub struct Straight {
    /// `+1.0` or `-1.0`: a corner's distance from the axis, signed as
    /// [`super::Axis::side`] reads it, is `side` times its distance away.
    pub side: f64,
    /// The outline, then each hole in the profile's order, as the corners
    /// they run through; a run laid to no length is left out.
    pub contours: Vec<Vec<Corner>>,
    /// How many runs the profile has, the outline's first and then each
    /// hole's: what its faces are numbered by.
    pub runs: u32,
    /// The highest number of a run not lying on the axis.
    pub last_off_the_axis: Option<u32>,
}

/// A corner of a profile laid square to its axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Corner {
    /// How far along the axis, and how far away from it, never below nought.
    pub at: DVec2,
    /// The number of the run leaving the corner.
    pub run: u32,
}

impl Straight {
    /// The profile laid square to the axis of `turn`, or `None` when a run is
    /// round, or leans at the drawing's resolution: the flats turn it then.
    /// Not written yet (#533): every profile goes to the flats.
    pub fn of(
        _outline: &Contour,
        _holes: &[Contour],
        _frame: Frame,
        _turn: &Turn,
        _part_reach: f64,
    ) -> Option<Straight> {
        None
    }

    /// How many numbers a turn of the profile names: one per run, and the
    /// two ends of a partial turn. A whole turn counts up to its last run off
    /// the axis, as the flats always have, so that the faces of the steps
    /// after it keep the numbers a part saved before turns were exact gave
    /// them.
    pub fn numbers(&self, whole: bool) -> u32 {
        if whole {
            self.last_off_the_axis.map_or(0, |run| run + 1)
        } else {
            self.runs + 2
        }
    }
}
