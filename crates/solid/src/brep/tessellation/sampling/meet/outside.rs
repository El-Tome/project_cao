//! A meet's chords kept out of a cylinder an end of its edge lies on.
//!
//! Where three walls meet at a vertex, all three within the tolerance of one
//! another — a bore grazing two cones joined along a meet — the meet leaves
//! the third wall's cylinder at the vertex and stays outside it, but a
//! chord of it near there can pass inside, deeper than the third wall's own
//! triangles sag, and through them. The chord bounds the two faces either
//! side of it, so no cut of either face undoes that (354000000011719); it is
//! halved instead until it stays outside wherever the curve does. Only a
//! chord reaching further than the tolerance from the wall is: a meet
//! staying within it of the wall — on a wall a hair from it — is left
//! alone: halved there, a cone across a shaft and a bore a hair off it is
//! cut crossed. And only a wall the chord passes inside deeper than the
//! wall's own triangles sag, where it holds a face, is one the chord can
//! cross ([`pierced`]): a wall bored away there, or sagging further, is
//! crossed by nothing, and the samples halving adds only cut the faces
//! either side into slivers fanned from far corners, which do cross
//! (354000000007974, 353000000000880).

use std::f64::consts::TAU;

use glam::DVec3;

use super::super::chords::{PROBES, halved_until};
use super::super::{divisions_in, taken};
use super::Around;
use crate::brep::curve::{Meet, Wall};
use crate::brep::domain::Location;
use crate::brep::surface::Cylinder;
use crate::brep::tessellation::contact;
use crate::brep::topology::{Body, EdgeId};

/// The walls of `around` other than the meet's own, an end of the edge `id`
/// lying on each, that a chord between two of its `points` — its ends either
/// side — passes inside deeper than the wall's own triangles sag there, where
/// the wall holds a face: those sag as the chord of the wall's grid and the
/// rays it takes, either side of that place, does.
pub(super) fn pierced(
    body: &Body,
    id: EdgeId,
    meet: &Meet,
    (walls, _, contacts): Around,
    (tolerance, span): (f64, f64),
    points: &[DVec3],
) -> Vec<contact::Wall> {
    let edge = body.edge(id);
    let Some(ends) = edge.ends else {
        return Vec::new();
    };
    let eps = body.scale().eps();
    let ends = ends.map(|end| body.vertex(end).point);
    let mut run = vec![ends[0]];
    run.extend_from_slice(points);
    run.push(ends[1]);
    let deep = |(surface, wall): &&contact::Wall| {
        let steps = divisions_in(wall.radius, tolerance, span);
        let mut angles: Vec<f64> = (0..steps)
            .map(|step| TAU * step as f64 / steps as f64)
            .chain(
                taken(&Wall::Cylinder(*wall), wall, walls, contacts)
                    .into_iter()
                    .map(|angle| angle.rem_euclid(TAU)),
            )
            .collect();
        angles.sort_by(f64::total_cmp);
        let sag = |angle: f64| {
            let next = angles.partition_point(|each| *each <= angle);
            let low = angles[next.saturating_sub(1)];
            let high = angles.get(next).copied().unwrap_or(angles[0] + TAU);
            let half = (high - low) / 2.0;
            wall.radius * (1.0 - half.cos() / (angle - (low + high) / 2.0).cos())
        };
        let faces: Vec<_> = body
            .face_ids()
            .filter(|face| body.face(*face).surface == *surface)
            .collect();
        run.windows(2).any(|pair| {
            (1..PROBES).any(|part| {
                let chord = pair[0].lerp(pair[1], part as f64 / PROBES as f64);
                let at = wall.parameters(chord);
                let depth = -wall.distance(chord);
                depth > eps
                    && depth > sag(at.x.rem_euclid(TAU))
                    && faces
                        .iter()
                        .any(|face| !matches!(body.locate(*face, at, eps), Ok(Location::Outside)))
            })
        })
    };
    walls
        .iter()
        .filter(|(_, wall)| {
            ![meet.first, meet.second].contains(&Wall::Cylinder(*wall))
                && ends.iter().any(|end| wall.distance(*end).abs() <= eps)
        })
        .filter(deep)
        .copied()
        .collect()
}

/// The points of the edge `id` along `meet` between its ends, `points` with
/// places added along the curve wherever a chord between two of them, one
/// standing further than `tolerance` outside a wall of `walls` an end of the
/// edge lies on, passes inside it where the curve stands outside, either by
/// more than the kernel's tolerance. `points` stand as they were, bit for
/// bit.
pub(super) fn kept_outside(
    body: &Body,
    id: EdgeId,
    meet: &Meet,
    walls: &[contact::Wall],
    tolerance: f64,
    points: Vec<DVec3>,
) -> Vec<DVec3> {
    let edge = body.edge(id);
    let Some(ends) = edge.ends else {
        return points;
    };
    let eps = body.scale().eps();
    let ends = ends.map(|end| body.vertex(end).point);
    let beside: Vec<Cylinder> = walls
        .iter()
        .map(|(_, cylinder)| *cylinder)
        .filter(|cylinder| {
            ![meet.first, meet.second].contains(&Wall::Cylinder(*cylinder))
                && ends.iter().any(|end| cylinder.distance(*end).abs() <= eps)
        })
        .collect();
    if beside.is_empty() {
        return points;
    }
    let low = edge.from.min(edge.to);
    let along = |point: DVec3| {
        let t = meet.parameter(point);
        let t = meet
            .period()
            .map_or(t, |period| low + (t - low).rem_euclid(period));
        (meet.point(t).distance(point) <= eps).then_some(t)
    };
    let inside = |wall: &Cylinder, from: f64, to: f64| {
        let (start, end) = (meet.point(from), meet.point(to));
        (1..PROBES).any(|part| {
            let share = part as f64 / PROBES as f64;
            wall.distance(start.lerp(end, share)) < -eps
                && wall.distance(meet.point(from + (to - from) * share)) > eps
        })
    };
    let run: Vec<Option<f64>> = std::iter::once(Some(edge.from))
        .chain(points.iter().map(|point| along(*point)))
        .chain(std::iter::once(Some(edge.to)))
        .collect();
    let mut kept = Vec::with_capacity(points.len());
    for (rank, pair) in run.windows(2).enumerate() {
        if let [Some(from), Some(to)] = *pair {
            let clear = |wall: &&Cylinder| {
                [from, to]
                    .into_iter()
                    .any(|t| wall.distance(meet.point(t)) > tolerance)
            };
            let left: Vec<&Cylinder> = beside.iter().filter(clear).collect();
            let mut places = Vec::new();
            let done = |from, to| !left.iter().any(|wall| inside(wall, from, to));
            halved_until(&done, from, to, &mut places);
            kept.extend(places.into_iter().map(|t| meet.point(t)));
        }
        kept.extend(points.get(rank).copied());
    }
    kept
}

#[cfg(test)]
mod tests;
