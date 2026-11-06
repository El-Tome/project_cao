//! Nothing sampled in the wedge beside a line two cylinders a hair off
//! parallel cross along (#706).
//!
//! Sampled at its ends alone, the line stands between their two angles on
//! each wall, and each wall takes both as rays, and the other's turned onto
//! its axis: a curve leaving one end would take a sample a hair from it, at
//! the angle of the other end read from either axis. Two faces bounding the
//! sliver beside the line then rest its chord on samples a hair apart, and
//! are not cut alike ([`super::super::folds`]). A curve leaving an end keeps
//! no sample standing, read from either axis as the two walls share their
//! rays, from past that end's angle to one of those of the two ends, and
//! a hair from it: a curve running on from there within those angles, a
//! ruling or a rim coming back round, keeps its samples.

use std::f64::consts::{PI, TAU};

use glam::{DQuat, DVec3};

use super::Samples;
use crate::brep::curve::{Curve, Wall};
use crate::brep::relation::BAND;
use crate::brep::surface::Cylinder;
use crate::brep::topology::Body;

/// How far round either wall, in radians, the ends of a line two walls
/// cross along may stand from each other for the line to stand in a wedge:
/// far under a step of the finest grid, `2π/1024`.
const WEDGE: f64 = 1e-3;

/// Leaves out of every curve leaving an end of such a line its samples in
/// the wedge between the angles of the line's two ends, read from either
/// wall's axis and turned onto the first's, told apart from them by `eps`
/// along the wall.
pub(super) fn cleared(body: &Body, samples: &mut Samples, eps: f64) {
    for id in body.edge_ids() {
        let edge = body.edge(id);
        let Curve::Meet(meet) = body.curve(edge.curve) else {
            continue;
        };
        let (Wall::Cylinder(one), Wall::Cylinder(other)) = (meet.first, meet.second) else {
            continue;
        };
        let (Some(vertices), 2) = (edge.ends, samples.edges[id.0 as usize].len()) else {
            continue;
        };
        if one.axis.cross(other.axis).length() >= BAND {
            continue;
        }
        let walls = [one, other];
        let turn = DQuat::from_rotation_arc(other.axis, one.axis);
        let read = |side: usize, point: DVec3| {
            let wall = walls[side];
            let from = point - wall.origin;
            let flat = from - wall.axis * wall.axis.dot(from);
            let way = if side == 0 { flat } else { turn * flat };
            way.dot(one.v).atan2(way.dot(one.u))
        };
        let round = |angle: f64| (angle + PI).rem_euclid(TAU) - PI;
        let ends = vertices.map(|end| body.vertex(end).point);
        let wide =
            (0..2).any(|side| round(read(side, ends[0]) - read(side, ends[1])).abs() > WEDGE);
        if wide || !crossing(&one, &other, ends) {
            continue;
        }
        let radius = one.radius.max(other.radius);
        let hair = eps / one.radius.min(other.radius);
        for (vertex, far) in [(vertices[0], vertices[1]), (vertices[1], vertices[0])] {
            let [from, to] = [vertex, far].map(|end| body.vertex(end).point);
            let within = |point: DVec3| {
                let near = |pin: f64| point.distance(from) <= 2.0 * pin.abs() * radius + eps;
                (0..2).any(|side| {
                    let base = read(side, from);
                    let off = round(read(side, point) - base);
                    [read(0, to), read(1, to), read(1 - side, from)]
                        .map(|pin| round(pin - base))
                        .iter()
                        .any(|pin| {
                            pin * off > 0.0
                                && off.abs() > hair
                                && off.abs() <= pin.abs() + hair
                                && near(*pin)
                        })
                })
            };
            for curve in body.edge_ids().filter(|curve| *curve != id) {
                let Some(ends) = body.edge(curve).ends else {
                    continue;
                };
                if !ends.contains(&vertex) {
                    continue;
                }
                let run = &samples.edges[curve.0 as usize];
                let kept: Vec<usize> = run
                    .iter()
                    .enumerate()
                    .filter(|(at, sample)| {
                        *at == 0 || *at == run.len() - 1 || !within(samples.points[**sample])
                    })
                    .map(|(_, sample)| *sample)
                    .collect();
                samples.edges[curve.0 as usize] = kept;
            }
        }
    }
}

/// Whether the axes of two walls cross, or all but, over the heights between
/// `ends`: they stand nearest each other there no further apart than that
/// changes. Two walls whose axes stand apart read a point from either at
/// angles wholly apart, and share no rays (`contact/leaning.rs`).
fn crossing(one: &Cylinder, other: &Cylinder, ends: [DVec3; 2]) -> bool {
    let flat = |vector: DVec3| vector - other.axis * other.axis.dot(vector);
    let (rate, at) = (flat(one.axis), flat(one.origin - other.origin));
    let [low, high] = ends.map(|end| (end - one.origin).dot(one.axis));
    let (low, high) = (low.min(high), low.max(high));
    let between = |height: f64| (at + rate * height).length();
    let nearest = if rate.length_squared() > 0.0 {
        (-at.dot(rate) / rate.length_squared()).clamp(low, high)
    } else {
        low
    };
    let [nearest, farthest] = [between(nearest), between(low).max(between(high))];
    nearest <= farthest - nearest
}
