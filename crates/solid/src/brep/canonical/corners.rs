//! Decision 5: corners found within the tolerance of each other are one, in
//! the order they were found, and what was learnt of each — the surfaces it
//! was found on, the curves it was found along — is joined. Once the curves'
//! supports are final, a corner lies on every surface of every curve it was
//! found along; from then on, whether it lies on a curve is read off the
//! supports rather than measured.
//!
//! Two corners are never one where that would put the corner on two surfaces
//! decided apart: the walls of a slit an earlier operation left thinner than
//! this one's tolerance keep their corners each.

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
    /// tolerance and on no surface apart from those `point` was found on, or
    /// a new one.
    pub fn add(
        &mut self,
        point: DVec3,
        surfaces: impl IntoIterator<Item = SurfaceId>,
        curves: impl IntoIterator<Item = usize>,
        registry: &Registry,
    ) -> usize {
        let surfaces: BTreeSet<SurfaceId> = surfaces.into_iter().collect();
        let curves: BTreeSet<usize> = curves.into_iter().collect();
        let found_on = on(&surfaces, &curves, registry);
        let found = self.corners.iter().position(|corner| {
            corner.point.distance(point) <= self.eps
                && !registry
                    .apart
                    .across(&on(&corner.surfaces, &corner.curves, registry), &found_on)
        });
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

    /// Every corner's support: the surfaces it was found on, those of the
    /// curves it was found along, and those of every curve it lies on as
    /// `lies_on` reads it, until none is added — a corner found on two of the
    /// three surfaces a line lies on lies on the third.
    pub fn supports(&self, registry: &Registry) -> Vec<Vec<SurfaceId>> {
        let mut supports: Vec<BTreeSet<SurfaceId>> = self
            .corners
            .iter()
            .map(|corner| {
                let mut support = corner.surfaces.clone();
                for curve in &corner.curves {
                    support.extend(registry.list[*curve].support.iter().copied());
                }
                support
            })
            .collect();
        loop {
            let mut grown = false;
            for (corner, support) in self.corners.iter().zip(&mut supports) {
                for (rank, known) in registry.list.iter().enumerate() {
                    let sorted: Vec<SurfaceId> = support.iter().copied().collect();
                    if known
                        .support
                        .iter()
                        .all(|surface| support.contains(surface))
                        || registry.apart.across(&known.support, &sorted)
                        || !lies_on(corner.point, &sorted, rank, registry, self.eps)
                    {
                        continue;
                    }
                    support.extend(known.support.iter().copied());
                    grown = true;
                }
            }
            if !grown {
                break;
            }
        }
        supports
            .into_iter()
            .map(|support| support.into_iter().collect())
            .collect()
    }
}

/// The surfaces a place was found on and those of the curves it was found
/// along, sorted.
fn on(
    surfaces: &BTreeSet<SurfaceId>,
    curves: &BTreeSet<usize>,
    registry: &Registry,
) -> Vec<SurfaceId> {
    let mut on = surfaces.clone();
    for curve in curves {
        on.extend(registry.list[*curve].support.iter().copied());
    }
    on.into_iter().collect()
}

/// Whether a corner lies on a registered curve: two surfaces the curve lies on
/// are among those the corner lies on, and of the curves lying on both — the
/// two lines a plane cuts a cylinder along, the two loops two cylinders meet
/// along — it stands on the nearest, or within the tolerance of it where two
/// of them cross.
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
    let own = &registry.list[curve];
    let away = distance(&own.curve, point);
    let shared: Vec<SurfaceId> = own
        .support
        .iter()
        .copied()
        .filter(|surface| support.binary_search(surface).is_ok())
        .collect();
    if own.support.len() < 2 {
        return shared.len() == own.support.len() && away <= eps;
    }
    shared.iter().enumerate().any(|(index, &one)| {
        shared[index + 1..].iter().any(|&other| {
            registry
                .list
                .iter()
                .enumerate()
                .filter(|&(rank, known)| {
                    rank != curve
                        && known.support.binary_search(&one).is_ok()
                        && known.support.binary_search(&other).is_ok()
                })
                .all(|(_, known)| away <= distance(&known.curve, point).max(eps))
        })
    })
}
