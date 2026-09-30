//! Decision 4: where an edge of one operand passes through a surface the
//! other operand has a face on. A curve can only leave a face through the
//! other operand's edge, and two crossings can only meet where the other
//! operand's faces do, so with the operands' own corners and the relations'
//! special points these are all the corners.
//!
//! A crossing counts where it lies on the edge and on a face of the other
//! operand, its boundary included; a curve lying along the surface lies on
//! it from then on.

use glam::DVec3;

use super::held::Held;
use super::operands::Operands;
use crate::brep::Declined;
use crate::brep::canonical::Registry;
use crate::brep::relation::{Crossings, crossings};
use crate::brep::topology::SurfaceId;

/// A corner found where a registered curve passes through a surface.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Found {
    pub point: DVec3,
    pub surface: SurfaceId,
    pub curve: usize,
}

pub(super) fn crossed(
    operands: &Operands,
    registry: &mut Registry,
    held: &[Held],
) -> Result<Vec<Found>, Declined> {
    let eps = operands.eps();
    let mut found = Vec::new();
    for curve in 0..registry.list.len() {
        let on_it: Vec<&Held> = held.iter().filter(|held| held.curve == curve).collect();
        for rank in 0..operands.surfaces.list.len() {
            let surface = SurfaceId(rank as u32);
            let across: Vec<&Held> = on_it
                .iter()
                .copied()
                .filter(|held| operands.carries(1 - held.operand, surface))
                .collect();
            if across.is_empty() || registry.list[curve].support.contains(&surface) {
                continue;
            }
            let geometry = registry.list[curve].curve;
            let list = match crossings(&geometry, &operands.surfaces.list[rank], operands.scale) {
                Crossings::Along => {
                    registry.join(curve, surface);
                    continue;
                }
                Crossings::Unsupported => continue,
                Crossings::At(list) => list,
            };
            for crossing in list {
                let slack = eps / geometry.derivative(crossing.parameter).length();
                let mut counts = false;
                for held in &across {
                    if held.covers(crossing.parameter, geometry.period(), slack)
                        && operands.touched(1 - held.operand, surface, crossing.point)?
                    {
                        counts = true;
                        break;
                    }
                }
                if counts {
                    found.push(Found {
                        point: crossing.point,
                        surface,
                        curve,
                    });
                }
            }
        }
    }
    Ok(found)
}
