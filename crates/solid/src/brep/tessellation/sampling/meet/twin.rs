//! A third wall crossing two walls leaning a hair off parallel, met by both
//! on the same rays (#706).
//!
//! Its two loops, one on each wall, a hair apart, are two edges, each
//! sampled on its own where the third wall's grid crosses it and where its
//! chord would leave it: at places that, read round either wall's axis and
//! turned onto the other's, stand a hair round from each other. The two
//! walls take rays through both, as they share their rays, and their
//! circles stand on pairs of rays a hair apart. Each loop is sampled instead
//! at the angles round its wall that either took, turned onto the pair's
//! first wall, two within the pair's drift one, the first wall's loop's
//! kept. Where a loop turns back in that angle, the two turn a drift apart,
//! and each takes, for that turn, the angle a drift inside the nearer one:
//! both cross it twice there.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::{DQuat, DVec3};

use super::{Around, alone};
use crate::brep::curve::{Curve, Meet, Wall};
use crate::brep::surface::Cylinder;
use crate::brep::tessellation::sampling::cone::Grid;
use crate::brep::topology::{Body, Edge, SurfaceId};

/// The points of a whole loop along `meet`, `points` as sampled on its own,
/// sampled at the same angles as the loop the same third wall leaves on the
/// wall its wall shares its rays with; `points` when there is none, or the
/// pair's drift is wider than `tolerance` along the wall.
pub(super) fn shared(
    body: &Body,
    edge: &Edge,
    meet: &Meet,
    around: Around,
    grids: &BTreeMap<SurfaceId, Grid>,
    tolerance: f64,
    points: Vec<DVec3>,
) -> Vec<DVec3> {
    let (Some(first), None) = (points.first(), edge.ends) else {
        return points;
    };
    let eps = body.scale().eps();
    for (pair, drift) in around.1.sharing() {
        if drift * pair[0].radius.max(pair[1].radius) > tolerance {
            continue;
        }
        let Some(side) =
            (0..2).find(|side| [meet.first, meet.second].contains(&Wall::Cylinder(pair[*side])))
        else {
            continue;
        };
        let third = if meet.first == Wall::Cylinder(pair[side]) {
            meet.second
        } else {
            meet.first
        };
        let Some((twin_edge, twin)) = twin_of(body, &pair[1 - side], &third, *first, tolerance)
        else {
            continue;
        };
        let theirs = alone(body, twin_edge, &twin, around, grids, tolerance);
        let reading = Reading::of(pair);
        let loops = [(side, meet, &points), (1 - side, &twin, &theirs)];
        let angles = reading.common(loops, drift);
        return reading.sampled(side, meet, edge, &angles, eps);
    }
    points
}

/// The whole loop along a meet of `wall` with `third` standing within
/// `tolerance` of `near`.
fn twin_of<'a>(
    body: &'a Body,
    wall: &Cylinder,
    third: &Wall,
    near: DVec3,
    tolerance: f64,
) -> Option<(&'a Edge, Meet)> {
    body.edge_ids().find_map(|id| {
        let edge = body.edge(id);
        let Curve::Meet(meet) = body.curve(edge.curve) else {
            return None;
        };
        let walls = [meet.first, meet.second];
        let ours = walls.contains(&Wall::Cylinder(*wall)) && walls.contains(third);
        let close = meet.point(meet.parameter(near)).distance(near) <= tolerance;
        (edge.ends.is_none() && ours && close).then_some((edge, *meet))
    })
}

/// Angles round the axis of either wall of a pair, read round the first's.
struct Reading {
    pair: [Cylinder; 2],
    turn: DQuat,
}

impl Reading {
    fn of(pair: [Cylinder; 2]) -> Reading {
        Reading {
            pair,
            turn: DQuat::from_rotation_arc(pair[1].axis, pair[0].axis),
        }
    }

    /// The angle of `point` round the axis of the wall `side`, turned onto
    /// the first's.
    fn read(&self, side: usize, point: DVec3) -> f64 {
        let wall = self.pair[side];
        let from = point - wall.origin;
        let flat = from - wall.axis * wall.axis.dot(from);
        let way = if side == 0 { flat } else { self.turn * flat };
        let frame = self.pair[0];
        way.dot(frame.v).atan2(way.dot(frame.u))
    }

    /// The angle round the wall `side`'s own axis standing for `angle` read
    /// round the first's.
    fn own(&self, side: usize, angle: f64) -> f64 {
        if side == 0 {
            return angle;
        }
        let frame = self.pair[0];
        let way = self.turn.inverse() * frame.radial(angle);
        let wall = self.pair[1];
        way.dot(wall.v).atan2(way.dot(wall.u))
    }

    /// The angles, read round the first wall, both loops are sampled at:
    /// those of the places either took, two within `drift` one, the first
    /// wall's loop's kept; within the span both loops reach, a drift inside
    /// where either turns back.
    fn common(&self, loops: [(usize, &Meet, &Vec<DVec3>); 2], drift: f64) -> Vec<f64> {
        let round = |angle: f64| (angle + PI).rem_euclid(TAU) - PI;
        let base = loops
            .iter()
            .find(|(side, _, _)| *side == 0)
            .map(|(_, _, points)| self.read(0, points[0]))
            .unwrap_or_default();
        let read = |(side, _, points): &(usize, &Meet, &Vec<DVec3>)| -> Vec<f64> {
            points
                .iter()
                .map(|point| round(self.read(*side, *point) - base))
                .collect()
        };
        let mut found: Vec<(usize, f64)> = Vec::new();
        let turning = loops.iter().any(|(side, meet, _)| {
            let on_first = meet.first == Wall::Cylinder(self.pair[*side]);
            !meet.turns(on_first).is_empty()
        });
        let spans = loops.map(|each| {
            let angles = read(&each);
            let low = angles.iter().copied().fold(f64::INFINITY, f64::min);
            let high = angles.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            (each.0, angles, low, high)
        });
        let (low, high) = spans
            .iter()
            .fold((f64::NEG_INFINITY, f64::INFINITY), |(low, high), span| {
                (low.max(span.2), high.min(span.3))
            });
        if turning {
            found.extend([(0, low + drift), (0, high - drift)]);
        }
        let mut each: Vec<(usize, f64)> = spans
            .iter()
            .flat_map(|(side, angles, _, _)| angles.iter().map(move |angle| (*side, *angle)))
            .filter(|(_, angle)| !turning || (low + drift < *angle && *angle < high - drift))
            .collect();
        each.sort_by_key(|(side, _)| *side);
        let mut kept: Vec<f64> = Vec::new();
        for (_, angle) in found.into_iter().chain(each) {
            if kept.iter().all(|other| round(angle - other).abs() > drift) {
                kept.push(angle);
            }
        }
        kept.into_iter().map(|angle| angle + base).collect()
    }

    /// The points of the whole loop `edge` along `meet`, on the wall `side`,
    /// at every place it stands at one of `angles`, in the way it runs.
    fn sampled(
        &self,
        side: usize,
        meet: &Meet,
        edge: &Edge,
        angles: &[f64],
        eps: f64,
    ) -> Vec<DVec3> {
        let Some(period) = meet.period() else {
            return Vec::new();
        };
        let on_first = meet.first == Wall::Cylinder(self.pair[side]);
        let mut places: Vec<f64> = angles
            .iter()
            .flat_map(|angle| meet.at_angle(on_first, self.own(side, *angle)))
            .map(|t| edge.from + (t - edge.from).rem_euclid(period))
            .collect();
        places.sort_by(f64::total_cmp);
        let mut points: Vec<DVec3> = Vec::with_capacity(places.len());
        for t in places {
            let point = meet.point(t);
            if points.last().is_none_or(|last| last.distance(point) > eps) {
                points.push(point);
            }
        }
        if points.len() > 1 && points[0].distance(points[points.len() - 1]) <= eps {
            points.pop();
        }
        if edge.to < edge.from {
            points.reverse();
        }
        points
    }
}
