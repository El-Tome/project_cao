//! Decision 4: where an edge of one operand passes through a surface the
//! other operand has a face on. A curve can only leave a face through the
//! other operand's edge, and two crossings can only meet where the other
//! operand's faces do, so with the operands' own corners and the relations'
//! special points these are all the corners.
//!
//! A crossing counts where it lies on the edge and on a face of the other
//! operand, its boundary included; a curve lying along the surface lies on
//! it from then on — when the pair of a surface it lies on with that one
//! shares it, as decision 3 reads it. A curve running within the tolerance of
//! a surface it was decided off, on a surface decided apart from it or a hair
//! beside the curve they share, never meets it.

use glam::DVec3;

use super::held::Held;
use super::operands::Operands;
use crate::brep::Declined;
use crate::brep::canonical::{Registry, same};
use crate::brep::curve::Curve;
use crate::brep::relation::{Crossings, crossings, relation};
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
                    if along(operands, registry, curve, surface) {
                        registry.join(curve, surface);
                    }
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

/// Whether a registered curve lies on `surface`: none of its surfaces is
/// apart from it, and one of them meets it along that very curve — one no
/// other curve, lying on a surface apart from this one's, was taken for.
fn along(operands: &Operands, registry: &Registry, curve: usize, surface: SurfaceId) -> bool {
    let list = &operands.surfaces.list;
    let known = &registry.list[curve];
    let elsewhere = |shared: &Curve| {
        registry.list.iter().enumerate().any(|(rank, other)| {
            rank != curve
                && other.support.contains(&surface)
                && registry.apart.across(&other.support, &known.support)
                && same(shared, &other.curve, operands.scale)
        })
    };
    registry.admits(curve, surface)
        && known.support.iter().any(|own| {
            relation(
                &list[own.0 as usize],
                &list[surface.0 as usize],
                operands.scale,
            )
            .curves()
            .iter()
            .any(|shared| same(shared, &known.curve, operands.scale) && !elsewhere(shared))
        })
}
