//! The matter a shell encloses, measured from its entities by the divergence
//! theorem: a third of the flux of `x - c` out of every face.
//!
//! On a face over the parameters `(u, v)`, the flux is the integral over the
//! trimmed domain of `f = (S - c)·(S_u × S_v)`. Green's theorem turns it into
//! the integral of `-H du` round the face's loops, `H(u, v)` being `f`
//! integrated in `v` from a base `v0`. The loops run with the face on their
//! left seen from its normal, so a face whose normal is its surface's has them
//! counter-clockwise in `(u, v)`, and the other kind clockwise — and the same
//! integral gives the flux out of the face either way. `H` is periodic in
//! every `u` that turns, so `du` is all that needs unwrapping. The base sits
//! where the surface folds — a cone's apex, a sphere's pole inside the face —
//! so `H` vanishes there; a torus's base is a circle of `v` no loop crosses.
//! The same integral of `|S_u × S_v|` is the face's area, and its sign is how
//! a base is told right from wrong: the region it describes must have the
//! face's own sense.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::geometry::Surface;
use super::{Bound, Edge, Face};

const GAUSS: [(f64, f64); 8] = [
    (-0.960_289_856_497_536_3, 0.101_228_536_290_376_3),
    (-0.796_666_477_413_626_7, 0.222_381_034_453_374_5),
    (-0.525_532_409_916_329, 0.313_706_645_877_887_3),
    (-0.183_434_642_495_649_8, 0.362_683_783_378_362),
    (0.183_434_642_495_649_8, 0.362_683_783_378_362),
    (0.525_532_409_916_329, 0.313_706_645_877_887_3),
    (0.796_666_477_413_626_7, 0.222_381_034_453_374_5),
    (0.960_289_856_497_536_3, 0.101_228_536_290_376_3),
];

/// Where `H` starts, and for a torus the circle of `v` the parameter is cut at.
struct Base {
    v0: f64,
    cut: Option<f64>,
}

/// The volume the faces enclose and their area.
pub fn enclosed(faces: &[Face], edges: &BTreeMap<usize, Edge>) -> Result<(f64, f64), String> {
    let corners: Vec<DVec3> = edges
        .values()
        .flat_map(|edge| [edge.start.point, edge.end.point])
        .collect();
    let center = corners.iter().copied().sum::<DVec3>() / corners.len().max(1) as f64;
    let mut volume = 0.0;
    let mut area = 0.0;
    for face in faces {
        let sense = if face.same_sense { 1.0 } else { -1.0 };
        let measured = bases(face, edges)
            .iter()
            .map(|base| integrate(face, edges, center, base, sense))
            .find(|(_, area)| sense * area > 0.0);
        let Some((flux, face_area)) = measured else {
            return Err(format!(
                "#{}: its loops bound no region of the sense it claims",
                face.id
            ));
        };
        volume += flux / 3.0;
        area += sense * face_area;
    }
    Ok((volume, area))
}

fn bases(face: &Face, edges: &BTreeMap<usize, Edge>) -> Vec<Base> {
    let at = |v0| Base { v0, cut: None };
    match &face.surface {
        Surface::Cone { radius, angle, .. } => vec![at(-radius / angle.tan())],
        Surface::Sphere { .. } | Surface::Torus { outer: Some(_), .. } => {
            face.surface.poles().into_iter().map(at).collect()
        }
        Surface::Torus { .. } => {
            let mut turns: Vec<f64> = samples(face, edges)
                .map(|point| face.surface.parameters(point).1)
                .collect();
            turns.sort_by(f64::total_cmp);
            let mut gaps: Vec<(f64, f64)> = turns
                .windows(2)
                .map(|pair| (pair[1] - pair[0], (pair[0] + pair[1]) / 2.0))
                .collect();
            if let (Some(first), Some(last)) = (turns.first(), turns.last()) {
                gaps.push((first + TAU - last, (first + TAU + last) / 2.0));
            }
            if turns.is_empty() {
                return vec![Base {
                    v0: -PI,
                    cut: Some(-PI),
                }];
            }
            gaps.retain(|(gap, _)| *gap > 1e-3);
            gaps.sort_by(|one, other| other.0.total_cmp(&one.0));
            gaps.into_iter()
                .map(|(_, cut)| Base {
                    v0: cut,
                    cut: Some(cut),
                })
                .collect()
        }
        _ => vec![at(0.0)],
    }
}

fn samples<'a>(
    face: &'a Face,
    edges: &'a BTreeMap<usize, Edge>,
) -> impl Iterator<Item = DVec3> + 'a {
    face.bounds.iter().flat_map(move |bound| {
        let points: Vec<DVec3> = match bound {
            Bound::Vertex(point) => vec![*point],
            Bound::Edges(uses) => uses
                .iter()
                .flat_map(|using| {
                    let edge = &edges[&using.edge];
                    (0..=256).map(move |index| {
                        edge.curve
                            .point(edge.from + (edge.to - edge.from) * index as f64 / 256.0)
                    })
                })
                .collect(),
        };
        points
    })
}

/// The flux of `x - center` out of the face and the area its loops bound,
/// both signed by the sense the loops run in `(u, v)`.
fn integrate(
    face: &Face,
    edges: &BTreeMap<usize, Edge>,
    center: DVec3,
    base: &Base,
    sense: f64,
) -> (f64, f64) {
    let surface = &face.surface;
    let parameters = |point: DVec3| {
        let (u, v) = surface.parameters(point);
        match base.cut {
            Some(cut) => (u, cut + (v - cut).rem_euclid(TAU)),
            None => (u, v),
        }
    };
    let step = |difference: f64| {
        if surface.turns() {
            difference - TAU * (difference / TAU).round()
        } else {
            difference
        }
    };
    let inner = |u: f64, v: f64| {
        let pieces = ((v - base.v0).abs() / surface.span()).ceil().max(1.0);
        let half = (v - base.v0) / pieces / 2.0;
        let mut sums = (0.0, 0.0);
        for piece in 0..pieces as usize {
            let middle = base.v0 + half * (2 * piece + 1) as f64;
            for (node, weight) in GAUSS {
                let local = surface.at(u, middle + half * node);
                let normal = local.along_u.cross(local.along_v);
                sums.0 += (local.point - center).dot(normal) * weight * half;
                sums.1 += normal.length() * weight * half;
            }
        }
        sums
    };
    let mut flux = 0.0;
    let mut area = 0.0;
    let mut add = |u: f64, v: f64, du: f64| {
        let (of_flux, of_area) = inner(u, v);
        flux -= of_flux * du;
        area -= of_area * du;
    };
    if !face
        .bounds
        .iter()
        .any(|bound| matches!(bound, Bound::Edges(_)))
    {
        let poles = surface.poles();
        let end = match (surface, poles.as_slice()) {
            (_, [south, north]) if base.v0 == *south => *north,
            (Surface::Torus { outer: None, .. }, _) => base.v0 + TAU,
            _ => return (0.0, 0.0),
        };
        for piece in 0..16 {
            let half = -sense * TAU / 32.0;
            for (node, weight) in GAUSS {
                add(
                    half * (2 * piece + 1) as f64 + half * node,
                    end,
                    weight * half,
                );
            }
        }
        return (flux, area);
    }
    for bound in &face.bounds {
        let Bound::Edges(uses) = bound else {
            continue;
        };
        let runs: Vec<(&Edge, f64, f64)> = uses
            .iter()
            .map(|using| {
                let edge = &edges[&using.edge];
                if using.along {
                    (edge, edge.from, edge.to)
                } else {
                    (edge, edge.to, edge.from)
                }
            })
            .collect();
        for (edge, from, to) in &runs {
            for pair in edge.curve.pieces(*from, *to).windows(2) {
                let half = (pair[1] - pair[0]) / 2.0;
                let middle = (pair[0] + pair[1]) / 2.0;
                let nudge = 1e-2 * half.abs();
                for (node, weight) in GAUSS {
                    let t = middle + half * node;
                    let (u, v) = parameters(edge.curve.point(t));
                    let moved = |by: f64| step(parameters(edge.curve.point(t + by * nudge)).0 - u);
                    let du = (8.0 * (moved(1.0) - moved(-1.0)) - moved(2.0) + moved(-2.0))
                        / (12.0 * nudge);
                    add(u, v, du * weight * half);
                }
            }
        }
        for (index, (edge, from, to)) in runs.iter().enumerate() {
            let (next, next_from, next_to) = runs[(index + 1) % runs.len()];
            let corner = edge.curve.point(*to);
            for pole in surface.poles() {
                if corner.distance(surface.pole_point(pole)) > 1e-6 * corner.length().max(1.0) {
                    continue;
                }
                let arriving = parameters(edge.curve.point(to - 1e-7 * (to - from))).0;
                let leaving =
                    parameters(next.curve.point(next_from + 1e-7 * (next_to - next_from))).0;
                let forward = sense * if pole > 0.0 { -1.0 } else { 1.0 } > 0.0;
                let sweep = if forward {
                    (leaving - arriving).rem_euclid(TAU)
                } else {
                    -(arriving - leaving).rem_euclid(TAU)
                };
                for piece in 0..8 {
                    let half = sweep / 16.0;
                    let middle = arriving + half * (2 * piece + 1) as f64;
                    for (node, weight) in GAUSS {
                        add(middle + half * node, pole, weight * half);
                    }
                }
            }
        }
    }
    (flux, area)
}
