//! A profile laid square to its axis, turned about it into a body (#533).

use super::Declined;
use super::topology::Body;
use crate::profile::Frame;
use crate::turning::{Straight, Turn};

impl Body {
    /// The profile turned about the axis of `turn` by its angle, the profile
    /// standing in `frame`. Not written yet (#533): every turn is declined
    /// as unfinished.
    pub fn turned(_straight: &Straight, _frame: Frame, _turn: &Turn) -> Result<Body, Declined> {
        Err(Declined::Unfinished)
    }
}
