//! A corner of two planes and a wall where one of the planes stands a hair
//! from tangent to the wall, at a slant (#552): the hairpin it cuts the wall
//! along turns within the box, and the place the line the two planes share
//! crosses the wall is decided on that hairpin.
//!
//! The line runs nearly along the wall there, and the place it crosses it
//! moves along the line by the rounding the line was built with over the
//! sine it crosses at — a few tolerances, a few hundred tolerances off the
//! hairpin, whose plane and wall part by less than that angle again. The
//! hairpin crosses the other plane steeply, and where it does is fixed to
//! rounding, so the corner is read there: in closed form on the hairpin,
//! the same bits whichever of the triple's three curves reaches it. A pair
//! of crossings within the tolerance of each other is one touch, where the
//! hairpin turns back along the plane.
//!
//! The line still decides whether it touches the wall, as every touch is
//! decided, and its crossing stands where the hairpin has none near it to
//! give instead. So it does where the hairpin turns on a radius under a
//! tenth of the tolerance: its vertex stands where the rounding of its
//! plane's offset over the plane's lean puts it — a plane a ten-millionth of
//! a degree off a block's side, a few hundred tolerances along the wall —
//! and its crossings are no better than the line's.

use glam::DVec3;

use super::super::at_a_slant;
use super::Crossing;
use super::lying::STEEP;
use crate::brep::conic::{Arm, Conic, Round};
use crate::brep::scale::Scale;
use crate::brep::surface::{Plane, Surface};

/// The share of the tolerance under which a hairpin turns too sharply to
/// decide where the line crosses the wall.
const SHARP: f64 = 0.1;

/// The crossings the line two planes share makes with a wall, `found` by
/// the line, each crossing it does not only touch re-decided on the
/// hairpin of the triple where the line grazes the wall there and one plane
/// meets the wall more grazingly still — by a tenth of the sine of either
/// other pair. Points, each with whether it is a touch, the others as
/// found; none where no crossing is re-decided.
pub(super) fn grazed(
    planes: [&Plane; 2],
    wall: &Surface,
    found: &[Crossing],
    scale: Scale,
) -> Option<Vec<(DVec3, bool)>> {
    let round = match wall {
        Surface::Cylinder(cylinder) => Round::Cylinder(*cylinder),
        Surface::Cone(cone) => Round::Cone(*cone),
        Surface::Plane(_) => return None,
    };
    let nearest = |point: DVec3| {
        found
            .iter()
            .map(|crossing| crossing.point.distance(point))
            .enumerate()
            .min_by(|one, other| one.1.total_cmp(&other.1))
            .map(|(rank, _)| rank)
    };
    let mut decided = Vec::new();
    let mut redecided = false;
    for (index, crossing) in found.iter().enumerate() {
        let routed = (!crossing.tangent)
            .then(|| route(planes, wall, crossing.point, scale))
            .flatten();
        let on_the_hairpin = routed.and_then(|own| {
            let other = planes[1 - own];
            let hairpins = Conic::cut(planes[own], &round, scale);
            if !turns_wide(&hairpins, scale) {
                return None;
            }
            let roots: Vec<(Conic, f64)> = hairpins
                .iter()
                .flat_map(|hairpin| hairpin.across(other).into_iter().map(|t| (*hairpin, t)))
                .filter(|(hairpin, t)| nearest(hairpin.point(*t)) == Some(index))
                .collect();
            on_the_hairpin(&roots, other, scale)
        });
        match on_the_hairpin {
            Some(points) => {
                redecided = true;
                decided.extend(points);
            }
            None => decided.push((crossing.point, crossing.tangent)),
        }
    }
    redecided.then_some(decided)
}

/// Whether no hairpin turns on a radius under [`SHARP`] of the tolerance: a
/// whole curve's anchor is the vertex nearest the box, where it turns
/// tightest.
fn turns_wide(hairpins: &[Conic], scale: Scale) -> bool {
    hairpins
        .iter()
        .filter(|hairpin| hairpin.arm == Arm::Whole)
        .all(|hairpin| hairpin.bend.length() * SHARP * scale.eps() < 1.0)
}

/// Which plane's hairpin decides a crossing at `point`: the one meeting the
/// wall there by less than a tenth of the sine the planes meet at and the
/// other meets the wall at, where the line they share grazes the wall; none
/// elsewhere, nor where that pair is not at a slant.
fn route(planes: [&Plane; 2], wall: &Surface, point: DVec3, scale: Scale) -> Option<usize> {
    let normal = match wall {
        Surface::Cylinder(cylinder) => cylinder.radial(cylinder.parameters(point).x),
        Surface::Cone(cone) => cone.normal(cone.parameters(point).x),
        Surface::Plane(_) => return None,
    };
    let shared = planes[0].normal.cross(planes[1].normal);
    let between = shared.length();
    if !point.is_finite() || between == 0.0 || shared.dot(normal).abs() >= STEEP * between {
        return None;
    }
    let sines = planes.map(|plane| plane.normal.cross(normal).length());
    (0..2).find(|&own| {
        sines[own] < STEEP * between.min(sines[1 - own])
            && at_a_slant(&Surface::Plane(*planes[own]), wall, scale)
    })
}

/// The corners a hairpin makes with `other` where the line found one: each
/// crossing, or one touch where two stand within the tolerance of each
/// other; none where the hairpin crosses the plane nowhere near.
fn on_the_hairpin(
    roots: &[(Conic, f64)],
    other: &Plane,
    scale: Scale,
) -> Option<Vec<(DVec3, bool)>> {
    match roots {
        [] => None,
        [(hairpin, t)] => Some(vec![(hairpin.point(*t), false)]),
        [(hairpin, t), (second, then), ..] => {
            let (one, two) = (hairpin.point(*t), second.point(*then));
            if one.distance(two) > scale.eps() || hairpin != second {
                return Some(vec![(one, false), (two, false)]);
            }
            let turn = hairpin
                .turning(other.normal, *t, *then)
                .first()
                .copied()
                .unwrap_or((t + then) / 2.0);
            Some(vec![(hairpin.point(turn), true)])
        }
    }
}
