//! How a cone meets another surface (#536): not decided yet. Every pair
//! holding a cone is unsupported, which declines a boolean unless the two
//! faces stand clear of each other.

use super::Relation;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Surface};

pub(super) fn relation(_cone: &Cone, _other: &Surface, _scale: Scale) -> Relation {
    Relation::Unsupported
}
