//! How many times an operand wraps each side of a region of a surface.

use glam::DVec3;

use crate::brep::Declined;
use crate::brep::combine::Operands;
use crate::brep::domain::Location;
use crate::brep::topology::SurfaceId;

/// An operand's winding just on the side a surface's own normal points to,
/// and just on the other; `covered` when a face of the operand lies there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::brep) struct Wrapped {
    pub covered: bool,
    pub above: i32,
    pub below: i32,
}

/// `point` stands inside a region of `surface`, well away from its boundary,
/// so from every face of the operand on that surface too: one on the
/// boundary of such a face is a tie the kernel does not settle.
pub(in crate::brep) fn wrapped(
    operands: &Operands,
    operand: usize,
    surface: SurfaceId,
    point: DVec3,
) -> Result<Wrapped, Declined> {
    let located = operands.located(operand, surface, point)?;
    if located
        .iter()
        .any(|(_, location)| *location == Location::Boundary)
    {
        return Err(Declined::Tie);
    }
    let mut covering = located
        .iter()
        .filter(|(_, location)| *location == Location::Inside);
    match (covering.next(), covering.next()) {
        (Some(&(face, _)), None) => {
            let flipped = operands.flipped(operand, face);
            Ok(Wrapped {
                covered: true,
                above: i32::from(flipped),
                below: i32::from(!flipped),
            })
        }
        (None, _) => {
            let winding = operands.bodies[operand].winding(point, operands.eps())?;
            Ok(Wrapped {
                covered: false,
                above: winding,
                below: winding,
            })
        }
        (Some(_), Some(_)) => Err(Declined::Tie),
    }
}
