//! The pairs of surfaces no place lies on both of, read off decision 2: two
//! the relation of their pair finds apart, and two it finds one within the
//! tolerance though the boolean kept them two.
//!
//! Those are surfaces an operand decided apart when it was made — two walls
//! of a slit a hair wide — at a tolerance finer than this operation's, which
//! the larger reach of the other operand may have grown past the hair. The
//! body need not carry the pairs it decided apart: two surfaces it keeps
//! distinct were decided apart, whatever tolerance a later operation brings,
//! and that operation reads it here rather than deciding again.

use std::collections::BTreeSet;

use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::SurfaceId;

#[derive(Clone, Debug, Default, PartialEq)]
pub(in crate::brep) struct Apart {
    pairs: BTreeSet<[SurfaceId; 2]>,
}

impl Apart {
    pub fn of(surfaces: &[Surface], scale: Scale) -> Apart {
        let mut pairs = BTreeSet::new();
        for (one, first) in surfaces.iter().enumerate() {
            for (other, second) in surfaces.iter().enumerate().skip(one + 1) {
                if matches!(
                    relation(first, second, scale),
                    Relation::Same { .. } | Relation::Apart
                ) {
                    pairs.insert([SurfaceId(one as u32), SurfaceId(other as u32)]);
                }
            }
        }
        Apart { pairs }
    }

    pub fn pair(&self, one: SurfaceId, other: SurfaceId) -> bool {
        self.pairs.contains(&[one.min(other), one.max(other)])
    }

    /// Whether a surface of `one` is apart from a surface of `other`.
    pub fn across(&self, one: &[SurfaceId], other: &[SurfaceId]) -> bool {
        one.iter()
            .any(|&first| other.iter().any(|&second| self.pair(first, second)))
    }
}
