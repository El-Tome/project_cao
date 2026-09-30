//! Decision 5: corners found within the tolerance of each other are one, in
//! the order they were found, and what was learnt of each — the surfaces it
//! was found on, the curves it was found along — is joined. Once the curves'
//! supports are final, a corner lies on every surface of every curve it was
//! found along; from then on, whether it lies on a curve is read off the
//! supports rather than measured.

use std::collections::BTreeSet;

use glam::DVec3;

use super::curves::{Registry, distance};
use crate::brep::topology::SurfaceId;

#[derive(Clone, Debug, PartialEq)]
pub(in crate::brep) struct Corner {
    pub point: DVec3,
    pub surfaces: BTreeSet<SurfaceId>,
    pub curves: BTreeSet<usize>,
}

pub(in crate::brep) struct Pool {
    pub corners: Vec<Corner>,
    eps: f64,
}

impl Pool {
    pub fn new(eps: f64) -> Pool {
        Pool {
            corners: Vec::new(),
            eps,
        }
    }

    /// The rank of the corner at `point`: the first already there within the
    /// tolerance, or a new one.
    pub fn add(
        &mut self,
        point: DVec3,
        surfaces: impl IntoIterator<Item = SurfaceId>,
        curves: impl IntoIterator<Item = usize>,
    ) -> usize {
        let found = self
            .corners
            .iter()
            .position(|corner| corner.point.distance(point) <= self.eps);
        let rank = found.unwrap_or_else(|| {
            self.corners.push(Corner {
                point,
                surfaces: BTreeSet::new(),
                curves: BTreeSet::new(),
            });
            self.corners.len() - 1
        });
        let corner = &mut self.corners[rank];
        corner.surfaces.extend(surfaces);
        corner.curves.extend(curves);
        rank
    }

    /// Every corner's support: the surfaces it was found on, and those of the
    /// curves it was found along.
    pub fn supports(&self, registry: &Registry) -> Vec<Vec<SurfaceId>> {
        self.corners
            .iter()
            .map(|corner| {
                let mut support = corner.surfaces.clone();
                for curve in &corner.curves {
                    support.extend(registry.list[*curve].support.iter().copied());
                }
                support.into_iter().collect()
            })
            .collect()
    }
}

/// Whether a corner lies on a registered curve: every surface the curve lies
/// on is one the corner lies on, and of the curves lying on two of those
/// surfaces too — the two lines a plane cuts a cylinder along, the two loops
/// two cylinders meet along — it stands on the nearest, or within the
/// tolerance of it where two of them cross.
///
/// A curve found on one surface alone — an edge between two faces of one
/// surface an earlier operation left — is not fixed by its support, and a
/// corner lies on it where it stands within the tolerance of it.
pub(in crate::brep) fn lies_on(
    point: DVec3,
    support: &[SurfaceId],
    curve: usize,
    registry: &Registry,
    eps: f64,
) -> bool {
    let within = |rank: usize| {
        registry.list[rank]
            .support
            .iter()
            .all(|surface| support.binary_search(surface).is_ok())
    };
    if !within(curve) {
        return false;
    }
    let own = &registry.list[curve];
    let away = distance(&own.curve, point);
    if own.support.len() < 2 {
        return away <= eps;
    }
    registry
        .list
        .iter()
        .enumerate()
        .filter(|&(rank, other)| {
            rank != curve
                && other
                    .support
                    .iter()
                    .filter(|surface| own.support.binary_search(surface).is_ok())
                    .count()
                    >= 2
                && within(rank)
        })
        .all(|(_, other)| away <= distance(&other.curve, point).max(eps))
}
