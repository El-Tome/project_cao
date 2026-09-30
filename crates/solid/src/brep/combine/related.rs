//! Decision 2, once per pair of surfaces whose faces, one of each operand,
//! stand in boxes that meet: the curves the pair shares, registered as lying
//! on both, and the points where such a curve crosses itself or where two
//! cylinders only touch.

use glam::DVec3;

use super::operands::Operands;
use crate::brep::Declined;
use crate::brep::canonical::Registry;
use crate::brep::relation::{Relation, relation};
use crate::brep::topology::{FaceId, SurfaceId};

/// A point a relation says must be a corner, on both surfaces of its pair
/// and on the registered curves passing through it.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Special {
    pub point: DVec3,
    pub surfaces: [SurfaceId; 2],
    pub curves: Vec<usize>,
}

pub(super) fn related(
    operands: &Operands,
    registry: &mut Registry,
) -> Result<Vec<Special>, Declined> {
    let list = &operands.surfaces.list;
    let mut special = Vec::new();
    for one in 0..list.len() {
        for other in one + 1..list.len() {
            let pair = [SurfaceId(one as u32), SurfaceId(other as u32)];
            if !facing(operands, pair) {
                continue;
            }
            let found = relation(&list[one], &list[other], operands.scale);
            if found == Relation::Unsupported {
                return Err(Declined::Unsupported);
            }
            let ranks: Vec<usize> = found
                .curves()
                .into_iter()
                .map(|curve| registry.register(curve, &pair))
                .collect();
            if let Relation::Meet(meeting) = &found {
                for node in &meeting.nodes {
                    special.push(Special {
                        point: node.point,
                        surfaces: pair,
                        curves: node
                            .on
                            .iter()
                            .map(|(component, _)| ranks[*component as usize])
                            .collect(),
                    });
                }
                if let Some(point) = meeting.contact {
                    special.push(Special {
                        point,
                        surfaces: pair,
                        curves: Vec::new(),
                    });
                }
            }
        }
    }
    Ok(special)
}

/// Whether a face of one operand on one surface of the pair and a face of
/// the other on the other surface stand in boxes that meet.
fn facing(operands: &Operands, [one, other]: [SurfaceId; 2]) -> bool {
    let on = |operand: usize, surface: SurfaceId| -> &[FaceId] {
        &operands.lying[operand][surface.0 as usize]
    };
    let near = |firsts: &[FaceId], seconds: &[FaceId]| {
        firsts
            .iter()
            .any(|first| seconds.iter().any(|second| operands.near(*first, *second)))
    };
    near(on(0, one), on(1, other)) || near(on(0, other), on(1, one))
}
