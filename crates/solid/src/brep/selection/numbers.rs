//! The names a kept region takes: those of every face of either operand
//! lying on it, the first operand's and the second's, coincident faces
//! answering to both. A region is kept only where an operand's face lies on
//! it — nowhere else does the operation say something different on its two
//! sides — so a kept region always has a name.

use super::wrapped::Wrapped;
use crate::brep::combine::Operands;
use crate::brep::topology::ascending;

/// The numbers of the faces each operand covers a region with, ascending and
/// each once.
pub(super) fn numbers(operands: &Operands, wrapped: [Wrapped; 2]) -> Vec<u32> {
    ascending(
        wrapped
            .iter()
            .zip(operands.bodies)
            .filter_map(|(wrapped, body)| wrapped.face.map(|face| body.numbers(face)))
            .flatten()
            .copied(),
    )
}
