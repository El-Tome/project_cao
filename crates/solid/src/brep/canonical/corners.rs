//! Decision 5: corners found within the tolerance of each other are one, in
//! the order they were found, and what was learnt of each — the surfaces it
//! was found on, the curves it was found along — is joined. Once the curves'
//! supports are final, a corner lies on every surface of every curve it was
//! found along; from then on, whether it lies on a curve is read off the
//! supports rather than measured.
//!
//! Two corners are never one where that would put the corner on two surfaces
//! decided apart: the walls of a slit an earlier operation left thinner than
//! this one's tolerance keep their corners each. Nor where it would put the
//! corner on two surfaces, one from each, whose curves all stand further
//! than the tolerance from it: each within the tolerance of the other, the
//! two corners would still not be within it of one place on all their
//! surfaces. A corner found along a ruling through a cone's apex is on the
//! cone only where it stands on its own nappe: past the apex, the ruling is
//! the line of the curve's other surfaces alone.

mod lying;

use std::collections::BTreeSet;

use glam::DVec3;

use super::curves::Registry;
use crate::brep::topology::SurfaceId;
pub(in crate::brep) use lying::lies_on;
use lying::{on, on_the_nappe, parted};

#[derive(Clone, Debug, PartialEq)]
pub(in crate::brep) struct Corner {
    pub point: DVec3,
    pub surfaces: BTreeSet<SurfaceId>,
    pub curves: BTreeSet<usize>,
    /// For an operand's own corner, the surfaces its operand built it on:
    /// on those planes alone, it stands where that operand decided, not
    /// where they meet.
    pub built: Option<Vec<SurfaceId>>,
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
    /// tolerance — or found on three planes whose normals span space that
    /// `point` was found on too — and on no surface apart from those `point`
    /// was found on, or a new one. Each is measured where it stands
    /// ([`standing`]): an operand's corner on three planes, each taken for
    /// another a hair off, stands where they meet, as much as √3 hairs from
    /// where the operand put it, and a curve crossing a fourth surface there
    /// crosses it at that place, not at the operand's.
    pub fn add(
        &mut self,
        point: DVec3,
        surfaces: impl IntoIterator<Item = SurfaceId>,
        curves: impl IntoIterator<Item = usize>,
        registry: &Registry,
    ) -> usize {
        self.added(point, surfaces, curves, registry, false)
    }

    /// As `add`, for an operand's own corner on the surfaces its operand
    /// built it on, none taken for another nor moved: on no plane but those,
    /// it stands where its operand put it. That operand decided it, at its
    /// own tolerance, on every surface it then lay on: a cut a hair from a
    /// block's edge left the edge's corner on the cut's plane within the
    /// tolerance, though the cut, the floor and a side meet twice the
    /// tolerance beyond it, and the end face it was decided on is gone. Moved
    /// to where those three meet, it left the edge it ends, laid where the
    /// operand built it, that far off (5520107844).
    pub fn add_built(
        &mut self,
        point: DVec3,
        surfaces: impl IntoIterator<Item = SurfaceId>,
        curves: impl IntoIterator<Item = usize>,
        registry: &Registry,
    ) -> usize {
        self.added(point, surfaces, curves, registry, true)
    }

    fn added(
        &mut self,
        point: DVec3,
        surfaces: impl IntoIterator<Item = SurfaceId>,
        curves: impl IntoIterator<Item = usize>,
        registry: &Registry,
        built: bool,
    ) -> usize {
        let surfaces: BTreeSet<SurfaceId> = surfaces.into_iter().collect();
        let curves: BTreeSet<usize> = curves.into_iter().collect();
        let found_on = on(&surfaces, &curves, point, registry, self.eps);
        let built = built.then(|| surfaces.iter().copied().collect::<Vec<_>>());
        let here = placed(point, built.as_deref(), &found_on, registry, self.eps);
        let found = self.corners.iter().position(|corner| {
            let known = on(
                &corner.surfaces,
                &corner.curves,
                corner.point,
                registry,
                self.eps,
            );
            let there = corner.standing(&known, registry, self.eps);
            (there.distance(here) <= self.eps
                && !parted(there, &known, &found_on, registry, self.eps)
                || registry.planes.fix_a_place(&known, &found_on))
                && !registry.apart.across(&known, &found_on)
        });
        let rank = found.unwrap_or_else(|| {
            self.corners.push(Corner {
                point,
                surfaces: BTreeSet::new(),
                curves: BTreeSet::new(),
                built,
            });
            self.corners.len() - 1
        });
        let corner = &mut self.corners[rank];
        corner.surfaces.extend(surfaces);
        corner.curves.extend(curves);
        rank
    }

    /// Every corner's support: the surfaces it was found on, those of the
    /// curves it was found along, those of every curve it lies on as
    /// `lies_on` reads it, and those decided to touch one of them at a point
    /// it stands on, until none is added — a corner found on two of the
    /// three surfaces a line lies on lies on the third (5544320693).
    pub fn supports(&self, registry: &Registry) -> Vec<Vec<SurfaceId>> {
        let mut supports: Vec<BTreeSet<SurfaceId>> = self
            .corners
            .iter()
            .map(|corner| {
                let mut support = corner.surfaces.clone();
                for curve in &corner.curves {
                    support.extend(on_the_nappe(
                        &registry.list[*curve].support,
                        corner.point,
                        registry,
                        self.eps,
                    ));
                }
                support
            })
            .collect();
        loop {
            let mut grown = false;
            for (corner, support) in self.corners.iter().zip(&mut supports) {
                let touched = registry.apart.touching(support, corner.point, self.eps);
                grown |= !touched.is_empty();
                support.extend(touched);
                for (rank, known) in registry.list.iter().enumerate() {
                    let sorted: Vec<SurfaceId> = support.iter().copied().collect();
                    let standing: Vec<SurfaceId> =
                        on_the_nappe(&known.support, corner.point, registry, self.eps).collect();
                    if standing.iter().all(|surface| support.contains(surface))
                        || registry.apart.across(&known.support, &sorted)
                        || !lies_on(corner.point, &sorted, rank, registry, self.eps)
                    {
                        continue;
                    }
                    support.extend(standing);
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

impl Corner {
    /// Where the corner stands, given what it lies on ([`standing`]): where
    /// its operand built it, for an operand's own.
    pub fn standing(&self, support: &[SurfaceId], registry: &Registry, eps: f64) -> DVec3 {
        placed(self.point, self.built.as_deref(), support, registry, eps)
    }
}

/// Where a corner stands: where three planes it lies on meet, when their
/// normals span space and it was found off that place by more than
/// rounding — each plane taken for another within the tolerance, the place
/// they fix moved — and where it was found otherwise. A cylinder the corner
/// lies on too holds the planes' place when it was decided to touch one of
/// them: the line they touch along is fixed by the planes the corner's
/// line lies on, and corners found along it, some on the planes, some on
/// the touch, would lean an edge between them a hair across both.
///
/// A corner on two planes across each other and no third stands on the
/// line they share, at its foot there: a wall touching one of the planes a
/// hair from the other's line puts corners along that line where it
/// touches, and the planes' own corners stand on it, so an edge between
/// them would lean a hair across both planes, and the wall's strip beside
/// it be drawn through one (8010303).
pub(in crate::brep) fn standing(
    point: DVec3,
    support: &[SurfaceId],
    registry: &Registry,
    eps: f64,
) -> DVec3 {
    let touching = |cylinder: SurfaceId| {
        support
            .iter()
            .any(|&plane| registry.planes.is_plane(plane) && registry.apart.touch(cylinder, plane))
    };
    let foot =
        |(origin, direction): (DVec3, DVec3)| origin + direction * direction.dot(point - origin);
    let fixed = registry
        .planes
        .place(support, touching)
        .or_else(|| registry.planes.line(support).map(foot));
    match fixed {
        Some(place) if place.distance(point) > eps * ROUNDING => place,
        _ => point,
    }
}

/// Where a corner stands ([`standing`]), or, for an operand's own corner
/// lying on no plane but those its operand `built` it on, on the line the
/// two of them most across each other share, at its foot there, never moved
/// to where a third crosses that line: its operand decided where. A plane
/// of the other operand through the corner fixes its place anew, as it
/// does for any corner it crosses: the place the boolean finds its other
/// crossings at (5523006542).
fn placed(
    point: DVec3,
    built: Option<&[SurfaceId]>,
    support: &[SurfaceId],
    registry: &Registry,
    eps: f64,
) -> DVec3 {
    let own = |surface: &SurfaceId| {
        !registry.planes.is_plane(*surface) || built.is_some_and(|built| built.contains(surface))
    };
    if built.is_none() || !support.iter().all(own) {
        return standing(point, support, registry, eps);
    }
    let foot =
        |(origin, direction): (DVec3, DVec3)| origin + direction * direction.dot(point - origin);
    match registry.planes.line_across(support).map(foot) {
        Some(place) if place.distance(point) > eps * ROUNDING => place,
        _ => point,
    }
}

/// Under this share of the tolerance, a corner is where its planes meet.
const ROUNDING: f64 = 1e-3;
