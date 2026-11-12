//! Two edges of one step whose set-backs would reach one point of an edge of
//! the body (#792): the edge where the face one ends on meets a face of the
//! other, or the edge between two that are not parallel on a face they share.
//! The bound they set on the one value, so that they do not. A set-back
//! reaches only where its blend runs (#829), as [`super::extent`] holds it:
//! two fills on the lower block's top round a standing block's chamfered
//! uprights, whose lines would cross far out on that top, never reach its
//! far edge. On an arc of a circle bounding a face of each (#823) — the
//! curved end of a plate whose sides converge on it — they meet where the
//! larger of their two depths is least along the arc, which no longer moves
//! in proportion along it as on a straight edge.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::chain::TANGENT;
use super::extent::{Ranked, Within, runs, span};
use super::{Wedge, corners};
use crate::brep::{self, Circle, Curve, EdgeId};

/// The values at which an edge's set-back along one of its faces reaches
/// either end of an edge of the body bounding that face: the distance from
/// the edge, square to it along the face, over its set-back of one.
type Depths = [f64; 2];

/// The least value at which the set-backs of two of `edges` that do not meet
/// at a corner — of their wedges in `wedges`, at their ranks, by the set-back
/// of a value of one `set_back` says — reach one point of a straight edge of
/// the body bounding a face of each, within where each set-back runs on that
/// face, as [`runs`] holds it and [`span`] reads it on that edge — or of an
/// arc of a circle bounding a face of each, as [`reached_on_arc`] reads it
/// there. Where that
/// edge is itself one of the list, neither of the two, its own cut takes the
/// point away once every edge is laid, whichever is laid first (#822): the
/// two meet only once its own set-back closes between theirs, which bounds
/// them instead, as [`closing`] reads it.
/// Two parallel edges facing each other across a face meet across it no
/// later than on any edge joining them, as [`super::pairs::bound`] reads
/// them. An arc of a conic, or any other curve, is not read.
pub(super) fn bound(
    body: &brep::Body,
    edges: &[EdgeId],
    wedges: &[Wedge],
    set_back: impl Fn(&Wedge) -> f64,
) -> f64 {
    let eps = body.scale().eps();
    let mut most = f64::INFINITY;
    for (along, [one, other], between) in across(body, edges, wedges) {
        let depth = |&(rank, side): &Ranked| {
            let wedge = &wedges[rank];
            let per = set_back(wedge);
            let into = wedge.into[side];
            (into / per, into.dot(wedge.point) / per, per)
        };
        let held = || -> Vec<Within> {
            [one, other]
                .into_iter()
                .flat_map(|ranked| runs(body, (edges, wedges), ranked, &set_back))
                .map(|(within, _)| within)
                .collect()
        };
        let value = match (along, between) {
            (Along::Straight(_), Some(rank)) => closing(wedges, [one, other], rank, &set_back),
            (Along::Straight(ends), None) => {
                let depths = |ranked: &Ranked| {
                    let (normal, offset, per) = depth(ranked);
                    (ends.map(|at| normal.dot(at) - offset), per)
                };
                let within = span(&held(), ends, eps);
                within.and_then(|within| reached([depths(&one), depths(&other)], within, eps))
            }
            (Along::Arc(circle, turn), None) => {
                reached_on_arc(&circle, turn, [depth(&one), depth(&other)], &held(), eps)
            }
            (Along::Arc(..), Some(_)) => None,
        };
        if let Some(value) = value {
            most = most.min(value);
        }
    }
    most
}

/// An edge of the body two set-backs may meet on: straight, between its two
/// ends, or an arc of a circle, over the angles it runs from the lower — a
/// whole turn from its start for a whole circle.
#[derive(Clone, Copy)]
enum Along {
    Straight([DVec3; 2]),
    Arc(Circle, [f64; 2]),
}

/// Every two sides of two of `edges` that do not meet at a corner, their
/// wedges in `wedges`, whose faces a straight edge or an arc of a circle of
/// the body bounds: that edge, the two sides, and the edge's rank where it is
/// one of the list but neither of the two.
fn across(
    body: &brep::Body,
    edges: &[EdgeId],
    wedges: &[Wedge],
) -> Vec<(Along, [Ranked; 2], Option<usize>)> {
    let mut across = Vec::new();
    for (edge, sides) in bounding(body, wedges) {
        let listed = edges.iter().position(|&named| named == edge);
        let stretch = body.edge(edge);
        let along = match (body.curve(stretch.curve), stretch.ends) {
            (Curve::Line(_), Some(ends)) => Along::Straight(ends.map(|end| body.vertex(end).point)),
            (Curve::Circle(circle), Some(_)) => Along::Arc(
                *circle,
                [stretch.from.min(stretch.to), stretch.from.max(stretch.to)],
            ),
            (Curve::Circle(circle), None) => {
                Along::Arc(*circle, [stretch.from, stretch.from + TAU])
            }
            _ => continue,
        };
        for (index, &one) in sides.iter().enumerate() {
            for &other in &sides[index + 1..] {
                let (first, second) = (one.0, other.0);
                if first == second || corners::meet(body, [edges[first], edges[second]]) {
                    continue;
                }
                let between = listed.filter(|&rank| rank != first && rank != second);
                across.push((along, [one, other], between));
            }
        }
    }
    across
}

/// The edges of the body bounding a face of the wedges `wedges`, each with
/// the ranks of the wedges and the sides of the faces it bounds.
fn bounding(body: &brep::Body, wedges: &[Wedge]) -> BTreeMap<EdgeId, Vec<Ranked>> {
    let mut bounding: BTreeMap<EdgeId, Vec<Ranked>> = BTreeMap::new();
    for (rank, wedge) in wedges.iter().enumerate() {
        for (side, &face) in wedge.faces.iter().enumerate() {
            for coedge in body.face(face).loops.iter().flatten() {
                bounding.entry(coedge.edge).or_default().push((rank, side));
            }
        }
    }
    bounding
}

/// The listed edges two others' set-backs cross on, as [`bound`] reads them
/// on `edges` and `wedges` with `set_back`: where their crossing stands behind
/// the edge at a value of nought and comes over it onto the face all three
/// share as the value grows, as [`crossing`] runs it — the point the edge's
/// own cut takes away once it is laid, there until then. Each by its rank,
/// with the two's ranks, the lower first.
pub(super) fn crossing_on(
    body: &brep::Body,
    edges: &[EdgeId],
    wedges: &[Wedge],
    set_back: impl Fn(&Wedge) -> f64,
) -> Vec<(usize, [usize; 2])> {
    (across(body, edges, wedges).into_iter())
        .filter_map(|(_, pair, between)| {
            let between = between?;
            let [behind, speed, _] = crossing(wedges, pair, between, &set_back)?;
            let [first, second] = pair.map(|(rank, _)| rank);
            let ranks = [first.min(second), first.max(second)];
            (behind < 0.0 && speed > 0.0).then_some((between, ranks))
        })
        .collect()
}

/// The least value at which two set-backs reach one point of a straight
/// edge, each reaching its ends at the values its [`Depths`] say, with its
/// set-back of one, and the points between in proportion, among the points
/// whose share of the way along it from its first end `within` holds: at an
/// end of those, or where both reach it at one value. A point behind
/// either's edge, by more than `eps`, is reached by none.
fn reached(lines: [(Depths, f64); 2], within: [f64; 2], eps: f64) -> Option<f64> {
    let [(one, _), (other, _)] = lines;
    let closing = (one[1] - one[0]) - (other[1] - other[0]);
    let level = (other[0] - one[0]) / closing;
    [within[0], within[1], level]
        .into_iter()
        .filter(|share| (within[0]..=within[1]).contains(share))
        .filter_map(|share| {
            let [one, other] = lines.map(|(depths, per)| {
                let depth = depths[0] + (depths[1] - depths[0]) * share;
                (depth * per >= -eps).then_some(depth.max(0.0))
            });
            Some(one?.max(other?))
        })
        .reduce(f64::min)
}

/// A set-back's depth along its face, as `normal · p − offset` at a point
/// `p`: the value at which it reaches there, behind its edge where negative;
/// and its set-back of one.
type Depth = (DVec3, f64, f64);

/// The least value at which two set-backs, each of the [`Depth`] it has,
/// reach one point of the arc of `circle` over the angles `turn`, among the
/// points every half-space of `held` holds, within `eps`: the least, along
/// the arc, of the larger of their two depths. On a circle each depth, and
/// each half-space's form, runs as `a + b·cos θ + c·sin θ`, so that least
/// stands at an end of the arc, where a half-space's form crosses nought,
/// where the two depths are equal, or where one is at its lowest. A point
/// behind either's edge, by more than `eps`, is reached by none.
fn reached_on_arc(
    circle: &Circle,
    [low, high]: [f64; 2],
    lines: [Depth; 2],
    held: &[Within],
    eps: f64,
) -> Option<f64> {
    let form = |normal: DVec3, offset: f64| {
        let radius = circle.radius;
        [
            normal.dot(circle.center) - offset,
            radius * normal.dot(circle.u),
            radius * normal.dot(circle.v),
        ]
    };
    let roots = |[a, b, c]: [f64; 3]| {
        let size = b.hypot(c);
        let ratio = -a / size;
        let spread = ratio.acos();
        let at = c.atan2(b);
        (ratio.abs() <= 1.0).then_some([at - spread, at + spread])
    };
    let [one, other] = lines.map(|(normal, offset, _)| form(normal, offset));
    let mut angles = vec![low, high];
    angles.extend(
        roots([0, 1, 2].map(|at| one[at] - other[at]))
            .into_iter()
            .flatten(),
    );
    angles.extend([one, other].map(|[_, b, c]| c.atan2(b) + PI));
    for &(normal, offset) in held {
        angles.extend(roots(form(normal, offset)).into_iter().flatten());
    }
    (angles.into_iter())
        .map(|angle| angle + ((low - angle) / TAU).ceil() * TAU)
        .filter(|&angle| angle <= high)
        .filter_map(|angle| {
            let point = circle.point(angle);
            if held
                .iter()
                .any(|&(normal, offset)| normal.dot(point) - offset > eps)
            {
                return None;
            }
            let [one, other] = lines.map(|(normal, offset, per)| {
                let depth = normal.dot(point) - offset;
                (depth * per >= -eps).then_some(depth.max(0.0))
            });
            Some(one?.max(other?))
        })
        .reduce(f64::min)
}

/// The value at which the set-back of the listed edge `between` closes
/// between those of a pair whose set-backs cross on it, on the face all three
/// share: the point where the pair's set-backs cross runs from where their
/// lines meet, behind `between`, in proportion to the value, as [`crossing`]
/// runs it, and once past its own set-back the two meet beyond its cut. None
/// where the pair are parallel — left to [`super::pairs::bound`] where they
/// face each other, to the laying's checks otherwise —, do not share that face
/// with it, or their crossing never outruns its set-back, left to the
/// laying's checks.
fn closing(
    wedges: &[Wedge],
    pair: [Ranked; 2],
    between: usize,
    set_back: &impl Fn(&Wedge) -> f64,
) -> Option<f64> {
    let [behind, speed, per] = crossing(wedges, pair, between, set_back)?;
    let outrunning = speed - per;
    (behind < 0.0 && outrunning > 0.0).then(|| -behind / outrunning)
}

/// How the point where the set-backs of a pair cross runs on the face all
/// three share, square to the listed edge `between`, into that face: where it
/// stands at a value of nought — where the pair's lines meet, behind the edge
/// where negative —, how far it runs for a value of one, and the edge's own
/// set-back of one. None where the pair are parallel, or do not share that
/// face with it.
fn crossing(
    wedges: &[Wedge],
    [(one, one_side), (other, other_side)]: [Ranked; 2],
    between: usize,
    set_back: &impl Fn(&Wedge) -> f64,
) -> Option<[f64; 3]> {
    let [first, second, third] = [one, other, between].map(|rank| &wedges[rank]);
    let face = first.faces[one_side];
    let side = third.faces.iter().position(|&theirs| theirs == face)?;
    if second.faces[other_side] != face || first.along.cross(second.along).length() <= TANGENT {
        return None;
    }
    let [a, b] = [first.into[one_side], second.into[other_side]];
    let [per_first, per_second, per_between] = [first, second, third].map(set_back);
    let met = first.point + first.along * (b.dot(second.point - first.point) / b.dot(first.along));
    let cosine = a.dot(b);
    let square = 1.0 - cosine * cosine;
    let running = a * ((per_first - cosine * per_second) / square)
        + b * ((per_second - cosine * per_first) / square);
    let into = third.into[side];
    Some([into.dot(met - third.point), into.dot(running), per_between])
}
