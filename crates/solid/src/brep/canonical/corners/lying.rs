//! What a corner lies on: the surfaces of the curves it was found along,
//! but a cone it stands off the own nappe of; whether it lies on a
//! registered curve; and what keeps two corners apart.

use std::collections::BTreeSet;

use glam::DVec3;

use super::super::curves::{Registry, distance};
use super::ROUNDING;
use crate::brep::curve::Curve;
use crate::brep::topology::SurfaceId;

/// The surfaces of a curve a corner on it lies on: all of them, but a cone
/// the corner stands off the own nappe of — a line through the cone's apex
/// runs on past it, and a corner there lies on the curve's other surfaces
/// only.
pub(super) fn on_the_nappe(
    surfaces: &[SurfaceId],
    point: DVec3,
    registry: &Registry,
    eps: f64,
) -> impl Iterator<Item = SurfaceId> {
    surfaces
        .iter()
        .copied()
        .filter(move |surface| !registry.apart.off_the_nappe(*surface, point, eps))
}

/// Whether a surface of `one` the other lacks and a surface of `other` the
/// first lacks cross along lines or a conic, none of them within `eps` of
/// `point`. Two surfaces touching stand within `eps` of each other far from
/// the line they touch along, and a place on both is not for that on it; nor
/// is a place within `eps` of two cylinders meeting at a grazing angle near
/// the curve they meet along, nor of a plane and a wall cutting a conic at a
/// grazing angle near it. Two walls decided to cross at a grazing angle are
/// read as two touching are: a corner one of them carries near their
/// crossing, and the node a band puts on the other there, are one corner on
/// both. Nor are two planes decided to graze: within the tolerance of each
/// other wherever their faces may meet, far from the line they cross along
/// too, a place on one within it of the other is on both (5520121880). Nor
/// two too near parallel to fix that line where the place stands within
/// rounding of both: the line is fixed only to that rounding over the sine
/// of their angle, and the place is on it however far from it the line was
/// computed (5542123080).
pub(super) fn parted(
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
            if registry.apart.touch(a, b)
                || registry.apart.graze(a, b)
                || registry.planes.graze(a, b)
                || registry.planes.meet_at(a, b, point, eps * ROUNDING)
            {
                return false;
            }
            let mut shared = registry.list.iter().filter(|known| {
                matches!(known.curve, Curve::Line(_) | Curve::Conic(_))
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
/// along it stands on, sorted: not a cone it stands off the own nappe of,
/// past the apex of a ruling it was found along ([`on_the_nappe`]).
pub(super) fn on(
    surfaces: &BTreeSet<SurfaceId>,
    curves: &BTreeSet<usize>,
    point: DVec3,
    registry: &Registry,
    eps: f64,
) -> Vec<SurfaceId> {
    let mut on = surfaces.clone();
    for curve in curves {
        on.extend(on_the_nappe(
            &registry.list[*curve].support,
            point,
            registry,
            eps,
        ));
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
/// whose surfaces is decided apart from one of the corner's (decision 5),
/// but a cone the corner stands off the own nappe of: a line through its
/// apex — the ruling a plane touches it along, which a flat face holding
/// the axis carries too — runs on past the apex, where a surface apart from
/// the cone may well cross it.
pub(in crate::brep) fn lies_on(
    point: DVec3,
    support: &[SurfaceId],
    curve: usize,
    registry: &Registry,
    eps: f64,
) -> bool {
    let own = &registry.list[curve];
    let standing: Vec<SurfaceId> = on_the_nappe(&own.support, point, registry, eps).collect();
    if registry.apart.across(&standing, support) || distance(&own.curve, point) > eps {
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
