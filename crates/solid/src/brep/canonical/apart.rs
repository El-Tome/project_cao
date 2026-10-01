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
//!
//! Beside them, the pairs decision 2 finds touching: two such surfaces stand
//! within the tolerance of each other over a band far wider than it, so that
//! a place on both is not for that on the line they touch along — and the
//! line, laid on one of them, may stand a hair off the other: a place within
//! the tolerance of the line is on it only where it is within the tolerance
//! of both surfaces too, which are kept here to be measured. And the points
//! where two perpendicular cylinders touch — the node of the curve they meet
//! along, or the one point they share — where a line on one passing through
//! the point only touches the other.

use std::collections::{BTreeMap, BTreeSet};

use glam::DVec3;

use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::SurfaceId;

#[derive(Clone, Debug, Default, PartialEq)]
pub(in crate::brep) struct Apart {
    pairs: BTreeSet<[SurfaceId; 2]>,
    touching: BTreeSet<[SurfaceId; 2]>,
    points: BTreeMap<[SurfaceId; 2], Vec<DVec3>>,
    surfaces: Vec<Surface>,
}

impl Apart {
    pub fn of(surfaces: &[Surface], scale: Scale) -> Apart {
        let mut pairs = BTreeSet::new();
        let mut touching = BTreeSet::new();
        let mut points = BTreeMap::new();
        for (one, first) in surfaces.iter().enumerate() {
            for (other, second) in surfaces.iter().enumerate().skip(one + 1) {
                let pair = [SurfaceId(one as u32), SurfaceId(other as u32)];
                match relation(first, second, scale) {
                    Relation::Same { .. } | Relation::Apart => {
                        pairs.insert(pair);
                    }
                    Relation::Tangent(_) => {
                        touching.insert(pair);
                    }
                    Relation::Meet(meeting) => {
                        let at: Vec<DVec3> = meeting
                            .nodes
                            .iter()
                            .map(|node| node.point)
                            .chain(meeting.contact)
                            .collect();
                        if !at.is_empty() {
                            points.insert(pair, at);
                        }
                    }
                    _ => {}
                }
            }
        }
        Apart {
            pairs,
            touching,
            points,
            surfaces: surfaces.to_vec(),
        }
    }

    pub fn pair(&self, one: SurfaceId, other: SurfaceId) -> bool {
        self.pairs.contains(&[one.min(other), one.max(other)])
    }

    /// Whether two surfaces were decided to touch along a line.
    pub fn touch(&self, one: SurfaceId, other: SurfaceId) -> bool {
        self.touching.contains(&[one.min(other), one.max(other)])
    }

    /// The points where two perpendicular cylinders were decided to touch.
    pub fn points(&self, one: SurfaceId, other: SurfaceId) -> &[DVec3] {
        self.points
            .get(&[one.min(other), one.max(other)])
            .map_or(&[], Vec::as_slice)
    }

    /// Whether a place stands within `eps` of a surface.
    pub fn near(&self, surface: SurfaceId, point: DVec3, eps: f64) -> bool {
        self.surfaces
            .get(surface.0 as usize)
            .is_none_or(|surface| surface.distance(point).abs() <= eps)
    }

    /// Whether a surface of `one` is apart from a surface of `other`.
    pub fn across(&self, one: &[SurfaceId], other: &[SurfaceId]) -> bool {
        one.iter()
            .any(|&first| other.iter().any(|&second| self.pair(first, second)))
    }
}
