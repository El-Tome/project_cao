//! Decision 1: each surface of the second operand is compared with the
//! first's, never a body's surface with its own, and one within the
//! tolerance is the first's.

use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, SurfaceId};

/// The surfaces both operands stand on, the first's first under their own
/// ranks, and where each operand's own surfaces went.
pub(in crate::brep) struct Surfaces {
    pub list: Vec<Surface>,
    /// For each operand, each of its surfaces as one of `list`, and whether
    /// their own normals point the same way.
    pub mapped: [Vec<(SurfaceId, bool)>; 2],
}

impl Surfaces {
    pub fn of(first: &Body, second: &Body, scale: Scale) -> Surfaces {
        let mut list = first.surfaces.clone();
        let own: Vec<(SurfaceId, bool)> = (0..list.len() as u32)
            .map(|rank| (SurfaceId(rank), true))
            .collect();
        let shared = list.len();
        let mut other = Vec::with_capacity(second.surfaces.len());
        for surface in &second.surfaces {
            let same = list[..shared].iter().enumerate().find_map(|(rank, known)| {
                match relation(known, surface, scale) {
                    Relation::Same { agree } => Some((SurfaceId(rank as u32), agree)),
                    _ => None,
                }
            });
            other.push(same.unwrap_or_else(|| {
                list.push(*surface);
                (SurfaceId(list.len() as u32 - 1), true)
            }));
        }
        Surfaces {
            list,
            mapped: [own, other],
        }
    }
}
