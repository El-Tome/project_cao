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
//! surfaces.

use std::collections::BTreeSet;

use glam::DVec3;

use super::curves::{Registry, distance};
use crate::brep::curve::Curve;
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
        let surfaces: BTreeSet<SurfaceId> = surfaces.into_iter().collect();
        let curves: BTreeSet<usize> = curves.into_iter().collect();
        let found_on = on(&surfaces, &curves, registry);
        let here = standing(point, &found_on, registry, self.eps);
        let found = self.corners.iter().position(|corner| {
            let known = on(&corner.surfaces, &corner.curves, registry);
            let there = standing(corner.point, &known, registry, self.eps);
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

/// Under this share of the tolerance, a corner is where its planes meet.
const ROUNDING: f64 = 1e-3;

/// Whether a surface of `one` the other lacks and a surface of `other` the
/// first lacks cross along lines, none of them within `eps` of `point`. Two
/// surfaces touching stand within `eps` of each other far from the line they
/// touch along, and a place on both is not for that on it; nor is a place
/// within `eps` of two cylinders meeting at a grazing angle near the curve
/// they meet along. Two walls decided to cross at a grazing angle are read
/// as two touching are: a corner one of them carries near their crossing,
/// and the node a band puts on the other there, are one corner on both.
fn parted(
    point: DVec3,
    one: &[SurfaceId],
    other: &[SurfaceId],
    registry: &Registry,
    eps: f64,
) -> bool {
    let only = |list: &[SurfaceId], without: &[SurfaceId]| -> Vec<SurfaceId> {
        list.iter()
            .copied()
            .filter(|surface| without.binary_search(surface).is_err())
            .collect()
    };
    let [first, second] = [only(one, other), only(other, one)];
    first.iter().any(|&a| {
        second.iter().any(|&b| {
            if registry.apart.touch(a, b) || registry.apart.graze(a, b) {
                return false;
            }
            let mut shared = registry.list.iter().filter(|known| {
                matches!(known.curve, Curve::Line(_))
                    && known.support.binary_search(&a).is_ok()
                    && known.support.binary_search(&b).is_ok()
            });
            let mut any = false;
            let far = shared.all(|known| {
                any = true;
                distance(&known.curve, point) > eps
            });
            any && far
        })
    })
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
/// are among those the corner lies on, and it stands within the tolerance of
/// the curve. Two surfaces crossing at a grazing angle stand within the
/// tolerance of each other microns from the curve they meet along, and so
/// may a corner on both, or one merged from two corners of a chain: a curve
/// cut there would end that far from its vertex, and two curves cut at one
/// vertex from places that far apart would leave it side by side.
///
/// A curve found on one surface alone — an edge between two faces of one
/// surface an earlier operation left — is not fixed by its support, and a
/// corner lies on it where it stands within the tolerance of it. Nor is the
/// line two surfaces touch along: a corner lies on it only within the
/// tolerance of it and of both surfaces. And no corner lies on a curve one of
/// whose surfaces is decided apart from one of the corner's (decision 5).
pub(in crate::brep) fn lies_on(
    point: DVec3,
    support: &[SurfaceId],
    curve: usize,
    registry: &Registry,
    eps: f64,
) -> bool {
    let own = &registry.list[curve];
    if registry.apart.across(&own.support, support) || distance(&own.curve, point) > eps {
        return false;
    }
    let shared: Vec<SurfaceId> = own
        .support
        .iter()
        .copied()
        .filter(|surface| support.binary_search(surface).is_ok())
        .collect();
    if own.support.len() < 2 {
        return shared.len() == own.support.len();
    }
    shared.iter().enumerate().any(|(index, &one)| {
        shared[index + 1..].iter().any(|&other| {
            !registry.apart.touch(one, other)
                || registry.apart.near(one, point, eps) && registry.apart.near(other, point, eps)
        })
    })
}
