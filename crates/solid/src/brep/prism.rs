//! A profile raised along its plane's normal into a body.

use glam::DVec3;

use super::Declined;
use super::topology::Body;
use crate::profile::{Contour, Frame};

impl Body {
    /// A profile pushed along `travel`, which must stand square to its plane.
    pub fn raised(
        _outline: &Contour,
        _holes: &[Contour],
        _frame: Frame,
        _travel: DVec3,
    ) -> Result<Body, Declined> {
        Err(Declined::Unfinished)
    }
}
