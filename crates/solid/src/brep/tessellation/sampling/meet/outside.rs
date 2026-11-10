//! A meet's chords kept out of a cylinder an end of its edge lies on.
//!
//! Where three walls meet at a vertex, all three within the tolerance of one
//! another — a bore grazing two cones joined along a meet — the meet leaves
//! the third wall's cylinder at the vertex and stays outside it, but a
//! chord of it near there can pass inside, deeper than the third wall's own
//! triangles sag, and through them. The chord bounds the two faces either
//! side of it, so no cut of either face undoes that (354000000011719); it is
//! halved instead, by one rule ([`dips`]), until it dips no deeper than the
//! wall's triangles sag where it passes, or dips only where no face of the
//! wall holds — the sag read at the wall's grid and at the rays it takes,
//! without which it is too wide and the chord still crosses. Halved any
//! finer, or where the wall is bored away, the faces either side are cut
//! into slivers fanned from far corners, which do cross (354000000007974,
//! 353000000000880). [`pierced`] finds the walls the rule halves against.
//! Only a chord reaching further than the tolerance from the wall is
//! halved: a meet staying within it of the wall — on a wall a hair from it —
//! is left alone, as halved there a cone across a shaft and a bore a hair
//! off it is cut crossed.

use std::f64::consts::TAU;

use glam::DVec3;

use super::super::chords::{PROBES, halved_until};
use super::super::{divisions_in, taken};
use super::Around;
use crate::brep::curve::{Meet, Wall};
use crate::brep::domain::Location;
use crate::brep::surface::Cylinder;
use crate::brep::tessellation::contact;
use crate::brep::topology::{Body, EdgeId, FaceId, SurfaceId};

/// The walls of `around` other than the meet's own, an end of the edge `id`
/// lying on each, that a chord between two of its `points` — its ends either
/// side — dips into ([`dips`]).
pub(super) fn pierced(
    body: &Body,
    id: EdgeId,
    meet: &Meet,
    around: Around,
    fineness: (f64, f64),
    points: &[DVec3],
) -> Vec<contact::Wall> {
    let Some((ends, beside)) = beside(body, id, meet, around.0, around, fineness) else {
        return Vec::new();
    };
    let eps = body.scale().eps();
    let mut run = vec![ends[0]];
    run.extend_from_slice(points);
    run.push(ends[1]);
    beside
        .into_iter()
        .filter(|seen| {
            run.windows(2)
                .any(|pair| dips(body, seen, eps, pair[0], pair[1]).next().is_some())
        })
        .map(|seen| (seen.surface, seen.wall))
        .collect()
}

/// The wall on `surface` as its triangles stand: sampled round its axis at
/// the ordered `angles`, and holding `faces`.
struct Seen {
    surface: SurfaceId,
    wall: Cylinder,
    angles: Vec<f64>,
    faces: Vec<FaceId>,
}

/// The edge `id`'s ends, and the walls of `walls` other than `meet`'s own
/// that one of them lies on, each seen at its grid cut for `fineness` and
/// the rays it takes in `around`: none for an edge without ends.
fn beside(
    body: &Body,
    id: EdgeId,
    meet: &Meet,
    walls: &[contact::Wall],
    (all, _, contacts): Around,
    (tolerance, span): (f64, f64),
) -> Option<([DVec3; 2], Vec<Seen>)> {
    let eps = body.scale().eps();
    let ends = body.edge(id).ends?.map(|end| body.vertex(end).point);
    let beside = walls
        .iter()
        .filter(|(_, wall)| {
            ![meet.first, meet.second].contains(&Wall::Cylinder(*wall))
                && ends.iter().any(|end| wall.distance(*end).abs() <= eps)
        })
        .map(|&(surface, wall)| {
            let steps = divisions_in(wall.radius, tolerance, span);
            Seen {
                surface,
                wall,
                angles: sampled_angles(steps, taken(&Wall::Cylinder(wall), &wall, all, contacts)),
                faces: body
                    .face_ids()
                    .filter(|face| body.face(*face).surface == surface)
                    .collect(),
            }
        })
        .collect();
    Some((ends, beside))
}

/// The places along the chord from `start` to `end`, as shares of it, where
/// it dips into the wall `seen` deeper than `eps` and deeper than the wall's
/// triangles sag there, at a place one of its faces holds.
fn dips<'a>(
    body: &'a Body,
    seen: &'a Seen,
    eps: f64,
    start: DVec3,
    end: DVec3,
) -> impl Iterator<Item = f64> + 'a {
    let wall = &seen.wall;
    (1..PROBES).filter_map(move |part| {
        let share = part as f64 / PROBES as f64;
        let chord = start.lerp(end, share);
        let at = wall.parameters(chord);
        let depth = -wall.distance(chord);
        (depth > eps
            && depth > sag_at(&seen.angles, wall.radius, at.x.rem_euclid(TAU))
            && seen
                .faces
                .iter()
                .any(|face| !matches!(body.locate(*face, at, eps), Ok(Location::Outside))))
        .then_some(share)
    })
}

/// The angles a wall of a grid of `steps` is sampled at round its axis, and
/// the `rays` it takes, all brought within a turn, in order.
fn sampled_angles(steps: usize, rays: Vec<f64>) -> Vec<f64> {
    let mut angles: Vec<f64> = (0..steps)
        .map(|step| TAU * step as f64 / steps as f64)
        .chain(rays.into_iter().map(|angle| angle.rem_euclid(TAU)))
        .collect();
    angles.sort_by(f64::total_cmp);
    angles
}

/// How far inside a wall of `radius` sampled at the ordered `angles` its
/// triangles stand at `angle`, within a turn: the sag of the chord between
/// the two angles either side of it, the first a turn on past the last.
fn sag_at(angles: &[f64], radius: f64, angle: f64) -> f64 {
    let next = angles.partition_point(|each| *each <= angle);
    let low = angles[next.saturating_sub(1)];
    let high = angles.get(next).copied().unwrap_or(angles[0] + TAU);
    let half = (high - low) / 2.0;
    radius * (1.0 - half.cos() / (angle - (low + high) / 2.0).cos())
}

/// The points of the edge `id` along `meet` between its ends, `points` with
/// places added along the curve wherever a chord between two of them, one
/// standing further than the tolerance of `fineness` outside a wall of
/// `walls` an end of the edge lies on, dips into it ([`dips`]) where the
/// curve stands outside it by more than the kernel's tolerance: halved until
/// it no longer does. `points` stand as they were, bit for bit.
pub(super) fn kept_outside(
    body: &Body,
    id: EdgeId,
    meet: &Meet,
    walls: &[contact::Wall],
    fineness: (f64, f64),
    points: Vec<DVec3>,
    around: Around,
) -> Vec<DVec3> {
    let Some((_, beside)) = beside(body, id, meet, walls, around, fineness) else {
        return points;
    };
    if beside.is_empty() {
        return points;
    }
    let edge = body.edge(id);
    let eps = body.scale().eps();
    let low = edge.from.min(edge.to);
    let along = |point: DVec3| {
        let t = meet.parameter(point);
        let t = meet
            .period()
            .map_or(t, |period| low + (t - low).rem_euclid(period));
        (meet.point(t).distance(point) <= eps).then_some(t)
    };
    let dipping = |left: &[&Seen], from: f64, to: f64| {
        let (start, end) = (meet.point(from), meet.point(to));
        left.iter().any(|seen| {
            dips(body, seen, eps, start, end)
                .any(|share| seen.wall.distance(meet.point(from + (to - from) * share)) > eps)
        })
    };
    let run: Vec<Option<f64>> = std::iter::once(Some(edge.from))
        .chain(points.iter().map(|point| along(*point)))
        .chain(std::iter::once(Some(edge.to)))
        .collect();
    let mut kept = Vec::with_capacity(points.len());
    for (rank, pair) in run.windows(2).enumerate() {
        if let [Some(from), Some(to)] = *pair {
            let left: Vec<&Seen> = beside
                .iter()
                .filter(|seen| {
                    [from, to]
                        .into_iter()
                        .any(|t| seen.wall.distance(meet.point(t)) > fineness.0)
                })
                .collect();
            let mut places = Vec::new();
            halved_until(&|from, to| !dipping(&left, from, to), from, to, &mut places);
            kept.extend(places.into_iter().map(|t| meet.point(t)));
        }
        kept.extend(points.get(rank).copied());
    }
    kept
}

#[cfg(test)]
mod tests;
