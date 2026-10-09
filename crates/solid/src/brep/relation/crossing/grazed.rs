//! A corner of two planes and a wall where one of the planes stands a hair
//! from tangent to the wall, at a slant (#552): the hairpin it cuts the wall
//! along turns within the box, and the corner is decided on that hairpin.
//!
//! The line the two planes share runs nearly along the wall there, and the
//! place it crosses it moves along the line by the rounding the line was
//! built with over the sine it crosses at — a few tolerances, a few
//! hundred tolerances off the hairpin, whose plane and wall part by less
//! than that angle again. The hairpin crosses the other plane steeply, and
//! where it does is fixed to rounding, so the corner is read there: in
//! closed form on the hairpin, the same bits whichever of the triple's
//! three curves reaches it. A plane a few tolerances past the hairpin's
//! vertex crosses it twice, though the line only touches the wall within
//! rounding; a pair of crossings within the tolerance of each other is one
//! touch, where the hairpin turns back along the plane, and a crossing or a
//! touch the line found where the hairpin does not reach the plane is none.

use glam::DVec3;

use super::super::at_a_slant;
use super::Crossing;
use super::lying::STEEP;
use crate::brep::conic::{Conic, Round};
use crate::brep::scale::Scale;
use crate::brep::surface::{Plane, Surface};

/// The crossings the line two planes share makes with a wall, `found` by
/// the line, each re-decided on the hairpin of the triple where the line
/// grazes the wall there and one plane meets the wall more grazingly still:
/// by a tenth of the sine of either other pair. Points, each with whether it
/// is a touch, the others as found; none where no crossing is re-decided.
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
    let routes: Vec<Option<usize>> = found
        .iter()
        .map(|crossing| route(planes, wall, crossing.point, scale))
        .collect();
    if routes.iter().all(Option::is_none) {
        return None;
    }
    let nearest = |point: DVec3| {
        found
            .iter()
            .map(|crossing| crossing.point.distance(point))
            .enumerate()
            .min_by(|one, other| one.1.total_cmp(&other.1))
            .map(|(rank, _)| rank)
    };
    let mut decided = Vec::new();
    for (index, crossing) in found.iter().enumerate() {
        let Some(own) = routes[index] else {
            decided.push((crossing.point, crossing.tangent));
            continue;
        };
        let other = planes[1 - own];
        let hairpins = Conic::cut(planes[own], &round, scale);
        let roots: Vec<(Conic, f64)> = hairpins
            .iter()
            .flat_map(|hairpin| hairpin.across(other).into_iter().map(|t| (*hairpin, t)))
            .filter(|(hairpin, t)| nearest(hairpin.point(*t)) == Some(index))
            .collect();
        decided.extend(on_the_hairpin(&roots, other, scale));
    }
    Some(decided)
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
/// other; none where the hairpin does not reach the plane there.
fn on_the_hairpin(roots: &[(Conic, f64)], other: &Plane, scale: Scale) -> Vec<(DVec3, bool)> {
    match roots {
        [] => Vec::new(),
        [(hairpin, t)] => vec![(hairpin.point(*t), false)],
        [(hairpin, t), (second, then), ..] => {
            let (one, two) = (hairpin.point(*t), second.point(*then));
            if one.distance(two) > scale.eps() || hairpin != second {
                return vec![(one, false), (two, false)];
            }
            let turn = hairpin
                .turning(other.normal, *t, *then)
                .first()
                .copied()
                .unwrap_or((t + then) / 2.0);
            vec![(hairpin.point(turn), true)]
        }
    }
}
