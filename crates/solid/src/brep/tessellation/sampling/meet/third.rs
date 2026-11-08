//! Where a meet passes within the tolerance of a third wall, one it does
//! not lie on and does not meet at either end (#786).
//!
//! The meet's chords sag towards its own walls' axes, and a third wall
//! standing that close is crossed by them between two of its samples: a
//! post whose wall passes through the place a cone and a bore meet, the
//! chord from there along the meet running inside the post's triangles.
//! The two part as the square of the distance from there, as a chord sags,
//! so no sampling of the meet alone keeps it outside. Sampled where it
//! crosses the angles round the third wall the wall's own samples stand
//! at, the meet stands on the rays the wall's triangles are cut between,
//! and the two keep their order between them, as two walls in contact do.
//!
//! Read once every wall's rays are settled, and not handed back: the bore
//! would take a ray through such a place, the post a sample there through
//! the curve the two meet along, and the meet another ray of the post.
//! Not beside an end where the wall bounds a face, which the meet leaves
//! from a corner of them both rather than grazes, nor on a wall parallel
//! to one of the meet's own, which shares its rays in contact, or sharing
//! them leaning a hair off parallel, which [`super::twin`] reads.

use std::f64::consts::{PI, TAU};

use std::collections::BTreeMap;

use glam::DVec3;

use super::super::cone::Grid;
use super::super::{divisions, taken};
use super::{Around, alone, twin};
use crate::brep::Scale;
use crate::brep::curve::{Curve, Meet, Wall};
use crate::brep::tessellation::contact::{self, Contact};
use crate::brep::topology::{Body, EdgeId, SurfaceId, VertexId};

/// How many equal stretches the meet's span is read in for the angle round
/// a third wall, each bisected wherever a ray of that wall crosses it.
const STRETCHES: usize = 256;

/// How many times a stretch is halved to find where a ray crosses it: past
/// this, the parameter's rounding.
const HALVINGS: u32 = 60;

/// The points of every edge, `meets` as last sampled, each sampled once
/// more where it grazes a wall it does not lie on, at the angles round that
/// wall of the samples `meets` put on it; as they were elsewhere.
pub(in super::super) fn settled(
    body: &Body,
    around: Around,
    grids: &BTreeMap<SurfaceId, Grid>,
    tolerance: f64,
    meets: Vec<Vec<DVec3>>,
) -> Vec<Vec<DVec3>> {
    let seen = seen(body, around.0, around.2, &meets, tolerance);
    let mut settled = meets;
    for id in body.edge_ids() {
        let Curve::Meet(meet) = body.curve(body.edge(id).curve) else {
            continue;
        };
        let places = places(body, (id, meet), around, &seen, tolerance);
        if !places.is_empty() {
            let points = alone(body, id, meet, around, grids, tolerance, &places);
            settled[id.0 as usize] = twin::shared(body, id, meet, around, grids, tolerance, points);
        }
    }
    settled
}

/// For each wall of the body, the angles round it its triangles are cut
/// at: its grid, the rays its circles take in `contacts`, its vertices,
/// and the samples `meets` put on it.
fn seen(
    body: &Body,
    walls: &[contact::Wall],
    contacts: &BTreeMap<SurfaceId, Contact>,
    meets: &[Vec<DVec3>],
    tolerance: f64,
) -> BTreeMap<SurfaceId, Vec<f64>> {
    let on: Vec<(Vec<Wall>, &Vec<DVec3>)> = body
        .edge_ids()
        .filter_map(|id| match body.curve(body.edge(id).curve) {
            Curve::Meet(meet) => Some((
                contact::held(body, id, meet).to_vec(),
                &meets[id.0 as usize],
            )),
            Curve::Line(_) | Curve::Circle(_) | Curve::Conic(_) => None,
        })
        .collect();
    walls
        .iter()
        .filter(|(id, _)| (id.0 as usize) < body.surfaces.len())
        .map(|(id, cylinder)| {
            let held = Wall::Cylinder(*cylinder);
            let steps = divisions(cylinder.radius, tolerance);
            let vertices = body
                .vertex_ids()
                .map(|vertex| body.vertex(vertex))
                .filter(|vertex| vertex.on.contains(id))
                .map(|vertex| vertex.point);
            let sampled = on
                .iter()
                .filter(|(walls, _)| walls.contains(&held))
                .flat_map(|(_, points)| points.iter().copied());
            let angles = (0..steps)
                .map(|step| TAU * step as f64 / steps as f64)
                .chain(taken(&held, cylinder, walls, contacts))
                .chain(
                    vertices
                        .chain(sampled)
                        .map(|point| cylinder.parameters(point).x),
                )
                .collect();
            (*id, angles)
        })
        .collect()
}

/// The parameters of the edge `id` along `meet` where it crosses an angle
/// round a wall it grazes, as [`seen`] gives them: a wall of the body other
/// than its own two, standing within the tolerance of it.
fn places(
    body: &Body,
    (id, meet): (EdgeId, &Meet),
    (walls, zones, _): Around,
    seen: &BTreeMap<SurfaceId, Vec<f64>>,
    tolerance: f64,
) -> Vec<f64> {
    let edge = body.edge(id);
    let Some(period) = meet.period() else {
        return Vec::new();
    };
    let (low, high) = match edge.ends {
        None => (edge.from, edge.from + period),
        Some(_) => (edge.from.min(edge.to), edge.from.max(edge.to)),
    };
    let own = contact::held(body, id, meet);
    let ends = match edge.ends {
        Some([first, last]) if edge.from <= edge.to => {
            [cornered(body, first), cornered(body, last)]
        }
        Some([first, last]) => [cornered(body, last), cornered(body, first)],
        None => [Vec::new(), Vec::new()],
    };
    let at = |rank: usize| low + (high - low) * rank as f64 / STRETCHES as f64;
    let mut found = Vec::new();
    for (wall, cylinder) in walls {
        let held = Wall::Cylinder(*cylinder);
        let parallel = own.iter().any(|one| {
            matches!(one, Wall::Cylinder(one) if one.axis.cross(cylinder.axis).length() <= Scale::RELATIVE)
        });
        let twinned = zones.sharing().any(|(pair, _)| {
            pair.contains(cylinder) && pair.iter().any(|one| own.contains(&Wall::Cylinder(*one)))
        });
        let Some(rays) = seen.get(wall) else {
            continue;
        };
        if own.contains(&held) || parallel || twinned {
            continue;
        }
        let near = |t: f64| cylinder.distance(meet.point(t)).abs() <= tolerance;
        let angle = |t: f64| cylinder.parameters(meet.point(t)).x;
        let close: Vec<bool> = (0..=STRETCHES).map(|rank| near(at(rank))).collect();
        let mut kept: Vec<bool> = close.windows(2).map(|pair| pair[0] || pair[1]).collect();
        for (end, corner) in ends.iter().enumerate() {
            if !corner.contains(wall) {
                continue;
            }
            let run: Vec<usize> = if end == 0 {
                (0..STRETCHES).collect()
            } else {
                (0..STRETCHES).rev().collect()
            };
            for rank in run {
                if !kept[rank] {
                    break;
                }
                kept[rank] = false;
            }
        }
        for rank in (0..STRETCHES).filter(|rank| kept[*rank]) {
            let (from, to) = (at(rank), at(rank + 1));
            let start = angle(from);
            let end = start + turned(angle(to) - start);
            let middle = (start + end) / 2.0;
            for ray in rays {
                let ray = ray + TAU * ((middle - ray) / TAU).round();
                if (ray - start) * (ray - end) >= 0.0 {
                    continue;
                }
                let past = |t: f64| (turned(angle(t) - ray) > 0.0) == (end > start);
                let (mut before, mut after) = (from, to);
                for _ in 0..HALVINGS {
                    let half = (before + after) / 2.0;
                    if past(half) {
                        after = half;
                    } else {
                        before = half;
                    }
                }
                let t = (before + after) / 2.0;
                if near(t) {
                    found.push(t);
                }
            }
        }
    }
    found
}

/// The surfaces of every face bounded by an edge ending at `vertex`: the
/// walls of that corner.
fn cornered(body: &Body, vertex: VertexId) -> Vec<SurfaceId> {
    let mut surfaces: Vec<SurfaceId> = body
        .edge_ids()
        .filter(|id| body.edge(*id).ends.is_some_and(|at| at.contains(&vertex)))
        .flat_map(|id| body.uses(id))
        .map(|(face, _)| body.face(face).surface)
        .collect();
    surfaces.sort_unstable();
    surfaces.dedup();
    surfaces
}

/// An angle brought within half a turn of nought.
fn turned(angle: f64) -> f64 {
    (angle + PI).rem_euclid(TAU) - PI
}
