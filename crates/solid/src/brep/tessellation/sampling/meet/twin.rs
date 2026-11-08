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
//! kept — or within what the tolerance spans round the wall, where the
//! drift is wider: two places that far apart are both needed. Where a loop
//! turns back in that angle, the two turn up to a drift apart, and each
//! takes, for that turn, the angle a hair inside the nearer one: both cross
//! it twice there, and the further one's tip, beyond, stands off the chord
//! by about as far as the two walls stand apart there.

use std::collections::BTreeMap;

use glam::{DQuat, DVec3};

use super::{Around, alone, turned};
use crate::brep::curve::{Curve, Meet, Wall};
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;
use crate::brep::tessellation::contact;
use crate::brep::tessellation::sampling::cone::Grid;
use crate::brep::topology::{Body, Edge, EdgeId, SurfaceId};

/// The points of a whole loop along `meet`, `points` as sampled on its own,
/// sampled at the same angles as the loop the same third wall leaves on the
/// wall its wall shares its rays with; `points` when there is none.
pub(super) fn shared(
    body: &Body,
    id: EdgeId,
    meet: &Meet,
    around: Around,
    grids: &BTreeMap<SurfaceId, Grid>,
    tolerance: f64,
    points: Vec<DVec3>,
) -> Vec<DVec3> {
    let edge = body.edge(id);
    let (Some(first), None) = (points.first(), edge.ends) else {
        return points;
    };
    let held = contact::held(body, id, meet);
    let eps = body.scale().eps();
    for (pair, drift) in around.1.sharing() {
        let radius = pair[0].radius.min(pair[1].radius);
        let apart = drift.min(tolerance / radius);
        let hair = Scale::HAIR * eps / radius;
        let Some(side) = (0..2).find(|side| held.contains(&Wall::Cylinder(pair[*side]))) else {
            continue;
        };
        let on_first = held[0] == Wall::Cylinder(pair[side]);
        let third = if on_first { held[1] } else { held[0] };
        let reach = 2.0 * drift * pair[0].radius.max(pair[1].radius) + tolerance;
        let Some((twin_edge, twin, twin_on_first)) =
            twin_of(body, &pair[1 - side], &third, *first, reach)
        else {
            continue;
        };
        let theirs = alone(body, twin_edge, &twin, around, grids, tolerance);
        if theirs.is_empty() {
            continue;
        }
        let reading = Reading::of(pair);
        let loops = [
            (side, (meet, on_first), &points),
            (1 - side, (&twin, twin_on_first), &theirs),
        ];
        let angles = reading.common(loops, apart, hair);
        return reading.sampled(side, (meet, on_first), edge, &angles, eps);
    }
    points
}

/// The whole loop along a meet of `wall` with `third`, as the body holds
/// them, standing within `reach` of `near`, and whether `wall` is its
/// first: the two loops stand about as far apart as the two walls do, up to
/// the pair's drift along the wall, further along a third wall crossing them
/// at a slant.
fn twin_of(
    body: &Body,
    wall: &Cylinder,
    third: &Wall,
    near: DVec3,
    reach: f64,
) -> Option<(EdgeId, Meet, bool)> {
    body.edge_ids().find_map(|id| {
        let edge = body.edge(id);
        let Curve::Meet(meet) = body.curve(edge.curve) else {
            return None;
        };
        if edge.ends.is_some() {
            return None;
        }
        let held = contact::held(body, id, meet);
        let ours = held.contains(&Wall::Cylinder(*wall)) && held.contains(third);
        let close = meet.point(meet.parameter(near)).distance(near) <= reach;
        (ours && close).then_some((id, *meet, held[0] == Wall::Cylinder(*wall)))
    })
}

/// A meet, and whether the wall of the pair it runs on is its first.
type Along<'a> = (&'a Meet, bool);

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
    /// those of the places either took, two within `apart` one, the first
    /// wall's loop's kept; within the span both loops reach, a `hair` inside
    /// where either turns back.
    fn common(&self, loops: [(usize, Along, &Vec<DVec3>); 2], apart: f64, hair: f64) -> Vec<f64> {
        let mut ordered = loops;
        ordered.sort_by_key(|(side, _, _)| *side);
        let base = ordered
            .iter()
            .find_map(|(side, _, points)| points.first().map(|point| self.read(*side, *point)))
            .unwrap_or_default();
        let read = |(side, _, points): &(usize, Along, &Vec<DVec3>)| -> Vec<f64> {
            points
                .iter()
                .map(|point| turned(self.read(*side, *point) - base))
                .collect()
        };
        let mut found: Vec<(usize, f64)> = Vec::new();
        let turning = loops
            .iter()
            .any(|(_, (meet, on_first), _)| !meet.turns(*on_first).is_empty());
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
            found.extend([(0, low + hair), (0, high - hair)]);
        }
        let mut each: Vec<(usize, f64)> = spans
            .iter()
            .flat_map(|(side, angles, _, _)| angles.iter().map(move |angle| (*side, *angle)))
            .filter(|(_, angle)| !turning || (low + hair < *angle && *angle < high - hair))
            .collect();
        each.sort_by_key(|(side, _)| *side);
        let mut kept: Vec<f64> = Vec::new();
        for (_, angle) in found.into_iter().chain(each) {
            if kept.iter().all(|other| turned(angle - other).abs() > apart) {
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
        (meet, on_first): Along,
        edge: &Edge,
        angles: &[f64],
        eps: f64,
    ) -> Vec<DVec3> {
        let Some(period) = meet.period() else {
            return Vec::new();
        };
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
