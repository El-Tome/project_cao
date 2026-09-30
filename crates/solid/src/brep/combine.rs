//! Joining and cutting: the boolean of `docs/exact-kernel.md`.

use super::Declined;
use super::topology::Body;

impl Body {
    /// Everything in either body.
    pub fn joined(&self, _other: &Body) -> Result<Body, Declined> {
        Err(Declined::Unfinished)
    }

    /// Everything in this body and not in the tool.
    pub fn cut_by(&self, _tool: &Body) -> Result<Body, Declined> {
        Err(Declined::Unfinished)
    }
}
