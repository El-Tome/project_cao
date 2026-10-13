//! Whether the kernel reads a step unstably (#552): a plane of one operand
//! leaning off square to, or along, the axis of a cylinder, a cone or an
//! elliptic wall (#571) of the other, their faces near enough to meet, by
//! more than decision 11 lays exactly and less than `UNSTABLE`. The conic
//! such a plane cuts divides by the lean, so the rounding of a step is
//! magnified by about one over it: the step is built, and said to the
//! user, who is never blocked.
//!
//! Read after decision 11, on the operands as the boolean lays them, so a
//! lean it turned exact is not noted, and only pairs across the two
//! operands are: a later step never notes an earlier one's lean again.

use super::clear::clear;
use super::operands::Operands;
use super::related::facing;
use crate::brep::canonical::{laid_exactly, leaning};
use crate::brep::surface::Surface;
use crate::brep::topology::SurfaceId;
use crate::brep::{Lean, Unstable};

/// The sine past which a lean no longer magnifies rounding up to the
/// tolerance: a thousandth of a radian, where two planes stop fixing the
/// line they share (`canonical/planes.rs`) and a surface stops parting a
/// band across (`combine/band.rs`). A lean of a hundredth of a degree is
/// noted; one of a tenth is not.
const UNSTABLE: f64 = 1e-3;

/// The least lean in the band of a plane and a wall, one of each operand,
/// whose faces may meet; none when no pair stands in it.
pub(super) fn unstable(operands: &Operands) -> Option<Unstable> {
    let exactly = laid_exactly(operands.bodies[1], operands.scale)?;
    let list = &operands.surfaces.list;
    let mut least: Option<(Lean, f64)> = None;
    for one in 0..list.len() {
        for other in one + 1..list.len() {
            let Some((lean, sine)) = lean_of(&list[one], &list[other]) else {
                continue;
            };
            if exactly(sine) || sine >= UNSTABLE || least.is_some_and(|(_, most)| most <= sine) {
                continue;
            }
            let pair = [SurfaceId(one as u32), SurfaceId(other as u32)];
            if facing(operands, pair) && !clear(operands, pair) {
                least = Some((lean, sine));
            }
        }
    }
    least.map(|(lean, sine)| Unstable {
        lean,
        degrees: sine.asin().to_degrees(),
    })
}

/// How a plane leans off a wall's axis, and by what sine; none for any
/// other pair.
fn lean_of(one: &Surface, other: &Surface) -> Option<(Lean, f64)> {
    let axis = |wall: &Surface| match wall {
        Surface::Cylinder(cylinder) => Some(cylinder.axis),
        Surface::Cone(cone) => Some(cone.axis),
        Surface::Elliptic(wall) => Some(wall.axis),
        Surface::Plane(_) => None,
    };
    match (one, other) {
        (Surface::Plane(plane), wall) | (wall, Surface::Plane(plane)) => {
            Some(leaning(plane.normal, axis(wall)?))
        }
        _ => None,
    }
}
