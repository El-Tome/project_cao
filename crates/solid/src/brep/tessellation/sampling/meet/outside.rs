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
//! cut crossed.

use glam::DVec3;

use super::super::chords::{PROBES, halved_until};
use crate::brep::curve::{Meet, Wall};
use crate::brep::surface::Cylinder;
use crate::brep::tessellation::contact;
use crate::brep::topology::{Body, EdgeId};

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
