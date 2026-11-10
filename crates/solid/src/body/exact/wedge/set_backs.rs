//! Two edges of one step whose set-backs would reach one point of an edge of
//! the body (#792): the edge where the face one ends on meets a face of the
//! other, or the edge between two that are not parallel on a face they share.
//! The bound they set on the one value, so that they do not.

use std::collections::BTreeMap;

use super::{Wedge, corners};
use crate::brep::{self, Curve, EdgeId};

/// The values at which an edge's set-back along one of its faces reaches
/// either end of an edge of the body bounding that face: the distance from
/// the edge, square to it along the face, over its set-back of one.
type Depths = [f64; 2];

/// The least value at which the set-backs of two of `edges` that do not meet
/// at a corner — of their wedges in `wedges`, at their ranks, by the set-back
/// of a value of one `set_back` says — reach one point of a straight edge of
/// the body bounding a face of each. An edge of the list laid before the
/// later of the two takes that point away first, and bounds nothing for them.
/// Where a straight edge joins two parallel edges facing each other across a
/// face, it is [`super::pairs::bound`]'s; across a face whose sides between
/// them are curved, only that one sees them.
pub(super) fn bound(
    body: &brep::Body,
    edges: &[EdgeId],
    wedges: &[Wedge],
    set_back: impl Fn(&Wedge) -> f64,
) -> f64 {
    let mut bounding: BTreeMap<EdgeId, Vec<(usize, usize)>> = BTreeMap::new();
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
        let listed = edges.iter().position(|&listed| listed == edge);
        let stretch = body.edge(edge);
        let (Curve::Line(_), Some(ends)) = (body.curve(stretch.curve), stretch.ends) else {
            continue;
        };
        let ends = ends.map(|end| body.vertex(end).point);
        let depths = |&(rank, side): &(usize, usize)| {
            let wedge = &wedges[rank];
            let per = set_back(wedge);
            (
                ends.map(|at| wedge.into[side].dot(at - wedge.point) / per),
                per,
            )
        };
        for (index, one) in sides.iter().enumerate() {
            for other in &sides[index + 1..] {
                let taken_first = listed.is_some_and(|rank| rank != one.0 && rank < other.0);
                if one.0 == other.0
                    || taken_first
                    || corners::meet(body, [edges[one.0], edges[other.0]])
                {
                    continue;
                }
                if let Some(value) = reached([depths(one), depths(other)], eps) {
                    most = most.min(value);
                }
            }
        }
    }
    most
}

/// The least value at which two set-backs reach one point of a straight
/// edge, each reaching its ends at the values its [`Depths`] say, with its
/// set-back of one, and the points between in proportion: at an end, or
/// where both reach it at one value. A point behind either's edge, by more
/// than `eps`, is reached by none.
fn reached(lines: [(Depths, f64); 2], eps: f64) -> Option<f64> {
    let [(one, _), (other, _)] = lines;
    let closing = (one[1] - one[0]) - (other[1] - other[0]);
    let level = (other[0] - one[0]) / closing;
    [0.0, 1.0, level]
        .into_iter()
        .filter(|share| (0.0..=1.0).contains(share))
        .filter_map(|share| {
            let [one, other] = lines.map(|(depths, per)| {
                let depth = depths[0] + (depths[1] - depths[0]) * share;
                (depth * per >= -eps).then_some(depth.max(0.0))
            });
            Some(one?.max(other?))
        })
        .reduce(f64::min)
}
