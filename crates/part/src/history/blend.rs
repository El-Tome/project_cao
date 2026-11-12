//! An edge of the part chamfered or rounded: the kernel's `Body::chamfer` and
//! `Body::fillet`, as a step of the history.

use serde::{Deserialize, Serialize};

/// What an edge of the part is turned into: a flat set back along its two
/// faces, or a round tangent to both. The size the step asks for is the
/// set-back of the one and the radius of the other, in millimetres.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlendShape {
    Chamfer,
    Fillet,
}
