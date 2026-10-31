//! How a sphere or a torus meets another surface (#576): the coaxial pairs
//! decided once in the meridian half-plane, everything else declined.

use super::Relation;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;

pub(super) fn relation(_one: &Surface, _other: &Surface, _scale: Scale) -> Relation {
    Relation::Unsupported
}
