//! Two edges of one step whose set-backs would reach one point of an edge of
//! the body (#792): the edge where the face one ends on meets a face of the
//! other, or the edge between two that are not parallel on a face they share.
//! The bound they set on the one value, so that they do not. A set-back
//! reaches only where its blend runs (#829): between the faces its edge ends
//! on, or the mitres an edge laid before cuts it to, or that two edges beside
//! one of the other sense run on into whatever their order. Two fills on the
//! lower block's top round a standing block's chamfered uprights, whose lines
//! would cross far out on that top, never reach its far edge.

use std::collections::BTreeMap;

use glam::DVec3;

use super::chain::TANGENT;
use super::corners::Beside;
use super::end::end_face;
use super::flush::stops;
use super::{Wedge, corners};
use crate::brep::{self, Curve, EdgeId, Surface};

/// The values at which an edge's set-back along one of its faces reaches
/// either end of an edge of the body bounding that face: the distance from
/// the edge, square to it along the face, over its set-back of one.
type Depths = [f64; 2];

/// A listed edge by its rank, and the side of its wedge a face is on.
type Ranked = (usize, usize);

/// A half-space, `normal · p ≤ offset`: one side of where a set-back ends.
type Within = (DVec3, f64);

/// The least value at which the set-backs of two of `edges` that do not meet
/// at a corner — of their wedges in `wedges`, at their ranks, by the set-back
/// of a value of one `set_back` says — reach one point of a straight edge of
/// the body bounding a face of each, within where each set-back runs on that
/// face, as [`runs`] holds it. Where that edge is itself one of the
/// list, neither of the two, and laid before the later of them, its own cut
/// takes the point away first: the two meet only once its own set-back closes
/// between theirs, which bounds them instead, as [`closing`] reads it.
/// Where a straight edge joins two parallel edges facing each other across a
/// face, it is [`super::pairs::bound`]'s; across a face whose sides between
/// them are curved, only that one sees them.
pub(super) fn bound(
    body: &brep::Body,
    edges: &[EdgeId],
    wedges: &[Wedge],
    beside: &[Beside],
    set_back: impl Fn(&Wedge) -> f64,
) -> f64 {
    let mut bounding: BTreeMap<EdgeId, Vec<Ranked>> = BTreeMap::new();
    for (rank, wedge) in wedges.iter().enumerate() {
        for (side, &face) in wedge.faces.iter().enumerate() {
            for coedge in body.face(face).loops.iter().flatten() {
                bounding.entry(coedge.edge).or_default().push((rank, side));
            }
        }
    }
    let eps = body.scale().eps();
    let mut most = f64::INFINITY;
    for (edge, sides) in bounding {
        let listed = edges.iter().position(|&named| named == edge);
        let stretch = body.edge(edge);
        let (Curve::Line(_), Some(ends)) = (body.curve(stretch.curve), stretch.ends) else {
            continue;
        };
        let ends = ends.map(|end| body.vertex(end).point);
        let depths = |&(rank, side): &Ranked| {
            let wedge = &wedges[rank];
            let per = set_back(wedge);
            (
                ends.map(|at| wedge.into[side].dot(at - wedge.point) / per),
                per,
            )
        };
        for (index, one) in sides.iter().enumerate() {
            for other in &sides[index + 1..] {
                let (first, second) = (one.0, other.0);
                if first == second || corners::meet(body, [edges[first], edges[second]]) {
                    continue;
                }
                let between = listed
                    .filter(|&rank| rank != first && rank != second && rank < first.max(second));
                let value = match between {
                    Some(rank) => closing(wedges, [*one, *other], rank, &set_back),
                    None => {
                        let later = first.max(second);
                        let held = [one, other].map(|&ranked| {
                            runs(body, (edges, wedges), beside, ranked, later, &set_back)
                                .into_iter()
                                .map(|(normal, offset)| ends.map(|at| normal.dot(at) - offset))
                        });
                        let within = (held.into_iter().flatten())
                            .try_fold([0.0, 1.0], |shares, past| narrowed(shares, past, eps));
                        within.and_then(|within| reached([depths(one), depths(other)], within, eps))
                    }
                };
                if let Some(value) = value {
                    most = most.min(value);
                }
            }
        }
    }
    most
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

/// The shares of the way along a straight edge `shares` holds, less those
/// where a half-space's form, `past` at the edge's two ends and in proportion
/// between, stands above `eps`. None where none is left.
fn narrowed([low, high]: [f64; 2], past: [f64; 2], eps: f64) -> Option<[f64; 2]> {
    let rising = past[1] - past[0];
    let level = (eps - past[0]) / rising;
    let [low, high] = if rising > 0.0 {
        [low, high.min(level)]
    } else if rising < 0.0 {
        [low.max(level), high]
    } else if past[0] <= eps {
        [low, high]
    } else {
        return None;
    };
    (low <= high).then_some([low, high])
}

/// Where the set-back of the listed edge `ranked` runs on its face on that
/// side once the edge of rank `later` is laid, as the half-spaces that hold
/// it, each on the side the middle of the edge stands on, beyond `eps` of
/// it. At an end one other edge of the list meets, sharing that face, where
/// that edge's set-back crosses its own at every value, the plane through the
/// corner the two mitre on: where the other is laid before that one, or
/// where the two meet beside an edge of the other sense, as `beside` holds
/// them, and the edge stops flush there, as [`stops`] reads it — whichever
/// is laid later then takes their overlap and carries this one on past the
/// face it stops on into the mitre, in any order. At any other end, the
/// plane of the flat face it ends on there, which trims its blend until an
/// edge laid later mitres it, back from that plane at a convex corner.
/// Nothing holds it at an end where the face it ends on is curved or not one
/// face, several edges of the list meet it, or the one meeting it does not
/// share the face.
fn runs(
    body: &brep::Body,
    (edges, wedges): (&[EdgeId], &[Wedge]),
    beside: &[Beside],
    (rank, side): Ranked,
    later: usize,
    set_back: &impl Fn(&Wedge) -> f64,
) -> Vec<Within> {
    let wedge = &wedges[rank];
    let eps = body.scale().eps();
    let stopping = stops(body, edges[rank], wedge);
    let Some(ends) = body.edge(edges[rank]).ends else {
        return Vec::new();
    };
    let middle = (body.vertex(ends[0]).point + body.vertex(ends[1]).point) / 2.0;
    let depth = |wedge: &Wedge, side: usize| {
        let per = set_back(wedge);
        (
            wedge.into[side] / per,
            wedge.into[side].dot(wedge.point) / per,
        )
    };
    let mut held = Vec::new();
    for end in ends {
        let meeting: Vec<usize> = (0..edges.len())
            .filter(|&other| other != rank)
            .filter(|&other| (body.edge(edges[other]).ends).is_some_and(|at| at.contains(&end)))
            .collect();
        let trimmed = || {
            let face = end_face(body, end, wedge.faces)?;
            let Surface::Plane(plane) = body.surface(body.face(face).surface) else {
                return None;
            };
            Some((plane.normal, plane.normal.dot(body.vertex(end).point)))
        };
        let carried = |neighbour: usize| {
            beside.contains(&(end, [rank.min(neighbour), rank.max(neighbour)]))
                && stopping.iter().any(|stop| stop.end == end)
        };
        let half = match meeting[..] {
            [neighbour] if neighbour < later || carried(neighbour) => {
                let theirs = &wedges[neighbour];
                let on = theirs
                    .faces
                    .iter()
                    .position(|&face| face == wedge.faces[side]);
                on.map(|on| {
                    let [(mine, own), (their, other)] = [depth(wedge, side), depth(theirs, on)];
                    (their - mine, other - own)
                })
            }
            [] | [_] => trimmed(),
            _ => None,
        };
        if let Some((normal, offset)) = half {
            let inside = normal.dot(middle) - offset;
            if inside.abs() > eps {
                let toward = -inside.signum();
                held.push((normal * toward, offset * toward));
            }
        }
    }
    held
}

/// The value at which the set-back of the listed edge `between`, laid before
/// the later of a pair whose set-backs cross on it, closes between theirs on
/// the face all three share: the point where the pair's set-backs cross runs
/// from where their lines meet, behind `between`, in proportion to the value,
/// and once past its own set-back the two meet beyond its cut. None where the
/// pair are parallel — left to [`super::pairs::bound`] where they face each
/// other, to the laying's checks otherwise —, do not share that face with it,
/// or their crossing never outruns its set-back, left to the laying's checks.
fn closing(
    wedges: &[Wedge],
    [(one, one_side), (other, other_side)]: [Ranked; 2],
    between: usize,
    set_back: &impl Fn(&Wedge) -> f64,
) -> Option<f64> {
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
    let behind = into.dot(met - third.point);
    let outrunning = into.dot(running) - per_between;
    (behind < 0.0 && outrunning > 0.0).then(|| -behind / outrunning)
}
