//! Two walls nearly parallel, past decision 11 and in #552's band, that the
//! kernel declines where their faces may meet (#577): each gives back the
//! decline `main` gave rather than a wrong answer.
//!
//! Leaning off parallel by a sine `s`, their sections one or touching,
//! inside or outside, they stand within the tolerance of each other along
//! `2·eps/s` where they part fastest — one size, their axes crossing — and
//! much longer where the second's axis drifts across the line they touch
//! along, the gap growing as the square of the height. Along a hundredth of
//! the reach or more they are as two parallel walls within the tolerance
//! all round though not one: at 1e-5° the kernel built them in bands no
//! corner tells apart.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use crate::brep::curve::Wall;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::tessellation::divisions;

/// The share of the reach along which two walls standing within the
/// tolerance of each other are declined.
const STRETCH: f64 = 1e-2;

/// The sine of a lean off parallel past which two walls are not read for
/// a stretch: #552's band.
const BAND: f64 = 1e-3;

/// The sine of a lean off parallel under which two walls whose sections
/// meet where their faces may, touching or crossing, are unsupported: the
/// lean that keeps one size, axes crossing, within the tolerance along the
/// `STRETCH`. Crossing at 1e-5°, the corners where caps cut them left a
/// body the kernel could not close (134820673287757).
const SLIVER: f64 = 2.0 * Scale::RELATIVE / STRETCH;

/// The coarsest tolerance a consumer is expected to draw a body's triangles
/// at, as a share of the reach the boolean reads, chosen against those that
/// exist: the tests' campaigns draw at it, the view five times finer of the
/// body's own reach. A stopgap with no exact value — an STL exported coarse
/// from a part under a hundred units, or a part a later step makes wider,
/// samples coarser and may still cross — until #706 lays such pairs on one
/// shared order and this goes.
const COARSEST_DRAWN: f64 = 1e-3;

/// How many heights across a box two walls are compared at for a sag.
const HEIGHTS: usize = 9;

/// Whether two walls not parallel, leaning off it by less than a
/// thousandth, are declined in one of the boxes `within` where their faces
/// may meet: leaning under `SLIVER`, their sections meet there at all; or
/// they stand within the tolerance of each other along a stretch of a
/// hundredth of the reach or more; or closer than a chord's sag. Square to
/// the first wall's axis, the second's stands at an offset affine in the
/// height; the walls are one, or touch inside or outside, within the
/// tolerance where that offset's length is within it of the difference or
/// the sum of their reaches along it, a quadratic in the height solved for
/// the stretch.
pub(in crate::brep) fn too_near_to_build(
    one: &Surface,
    other: &Surface,
    within: &[[DVec3; 2]],
    scale: Scale,
) -> bool {
    let (Some(first), Some(second)) = (Wall::of(one), Wall::of(other)) else {
        return false;
    };
    let (axis, along) = (first.axis(), second.axis());
    let sine = axis.cross(along).length();
    if sine >= BAND {
        return false;
    }
    let eps = scale.eps();
    let reach = |wall: &Wall, way: DVec3| match wall {
        Wall::Cylinder(cylinder) => cylinder.radius,
        Wall::Elliptic(oval) => oval.support(way),
    };
    let between = second.origin() - first.origin();
    let rate = along / along.dot(axis) - axis;
    let at_nought = between - along * (between.dot(axis) / along.dot(axis));
    let offset = |height: f64| at_nought + rate * height;
    let squared = rate.length_squared();
    let base = first.origin().dot(axis);
    within.iter().any(|[low, high]| {
        let (from, to) = (0..8)
            .map(|corner| {
                let pick =
                    |bit: usize, at: usize| if corner & bit == 0 { low[at] } else { high[at] };
                DVec3::new(pick(1, 0), pick(2, 1), pick(4, 2)).dot(axis) - base
            })
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(from, to), height| {
                (from.min(height), to.max(height))
            });
        let nearest = if squared > 0.0 {
            -at_nought.dot(rate) / squared
        } else {
            (from + to) / 2.0
        };
        let closest = offset(nearest).length_squared();
        let way = offset(nearest.clamp(from, to)).normalize_or(axis.any_orthonormal_vector());
        let (near, far) = (reach(&first, way), reach(&second, way));
        let within_of = |bound: f64| {
            if bound * bound < closest {
                return None;
            }
            let half = if squared > 0.0 {
                ((bound * bound - closest) / squared).sqrt()
            } else {
                f64::INFINITY
            };
            Some([(nearest - half).max(from), (nearest + half).min(to)])
        };
        let stretch = |target: f64| {
            let Some([start, end]) = within_of(target + eps) else {
                return 0.0;
            };
            match (target > eps).then(|| within_of(target - eps)).flatten() {
                Some([inner, outer]) => (inner.min(end) - start).max(end - outer.max(start)),
                None => end - start,
            }
            .max(0.0)
        };
        let longest = stretch((near - far).abs()).max(stretch(near + far));
        let farthest = offset(from).length().max(offset(to).length());
        let meet = closest.sqrt() <= near + far + eps && farthest >= (near - far).abs() - eps;
        longest >= STRETCH * scale.reach() || sine < SLIVER && meet
    }) || closer_than_a_sag(&first, &second, within, scale)
}

/// Whether two walls in the band stand closer to each other than a chord's
/// sag over a step of a grid, where their faces may meet (#577): each wall
/// cut into `divisions(size, COARSEST_DRAWN · reach)` steps, as its
/// triangles would be at that tolerance, its sag
/// `size · (1 − cos(π / steps))`; at `HEIGHTS` heights evenly across each
/// box `within`, along its axis, two consecutive steps of its grid both
/// stand within that sag of the other wall. The triangles of two
/// such walls, each sampled on its own grid a hair turned from the other's,
/// cross where the sweep orders their samples differently.
fn closer_than_a_sag(first: &Wall, second: &Wall, within: &[[DVec3; 2]], scale: Scale) -> bool {
    let tolerance = COARSEST_DRAWN * scale.reach();
    [(first, second), (second, first)]
        .into_iter()
        .any(|(wall, other)| {
            let size = match wall {
                Wall::Cylinder(cylinder) => cylinder.radius,
                Wall::Elliptic(oval) => oval.major,
            };
            let steps = divisions(size, tolerance);
            let sag = size * (1.0 - (PI / steps as f64).cos());
            let (origin, axis) = (wall.origin(), wall.axis());
            within.iter().any(|[low, high]| {
                let (from, to) = heights(*low, *high, origin, axis);
                (0..HEIGHTS).any(|rank| {
                    let height = from + (to - from) * rank as f64 / (HEIGHTS - 1) as f64;
                    let near = |step: usize| {
                        let angle = TAU * (step % steps) as f64 / steps as f64;
                        let point = match wall {
                            Wall::Cylinder(cylinder) => cylinder.point(DVec2::new(angle, height)),
                            Wall::Elliptic(oval) => oval.point(DVec2::new(angle, height)),
                        };
                        gap(other, point) <= sag
                    };
                    (0..steps).any(|step| near(step) && near(step + 1))
                })
            })
        })
}

/// How far along `axis`, from `origin`, a box spans.
fn heights(low: DVec3, high: DVec3, origin: DVec3, axis: DVec3) -> (f64, f64) {
    (0..8)
        .map(|corner| {
            let pick = |bit: usize, at: usize| if corner & bit == 0 { low[at] } else { high[at] };
            (DVec3::new(pick(1, 0), pick(2, 1), pick(4, 2)) - origin).dot(axis)
        })
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(from, to), height| {
            (from.min(height), to.max(height))
        })
}

/// How far a point stands from a wall: exactly from a cylinder, to first
/// order from an elliptic wall.
fn gap(wall: &Wall, point: DVec3) -> f64 {
    match wall {
        Wall::Cylinder(cylinder) => cylinder.distance(point).abs(),
        Wall::Elliptic(oval) => {
            let from = point - oval.origin;
            let [x, y] = [from.dot(oval.u), from.dot(oval.v)];
            let [a, b] = [oval.major * oval.major, oval.minor * oval.minor];
            let level = x * x / a + y * y / b - 1.0;
            let slope = 2.0 * DVec2::new(x / a, y / b).length();
            if slope > 0.0 {
                level.abs() / slope
            } else {
                f64::INFINITY
            }
        }
    }
}
