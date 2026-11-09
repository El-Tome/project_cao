//! Two edges of one step, one ending on a face of the other (#792): the
//! bound they set on the one value, so that the round the first lays where
//! it ends on that face stops short of the set-back the other makes on it.

use glam::DVec3;

use super::chain::TANGENT;
use super::{Wedge, corners};
use crate::brep::{self, EdgeId, FaceId, VertexId};

/// The most value at which no edge's round, where it ends on a face of
/// another edge of `edges`, reaches the set-back that other edge makes on
/// that face, by the set-back of a value of one `set_back` says — each edge's
/// wedge in `wedges`, at its rank. Two edges meeting at a corner are left to
/// it.
pub(super) fn bound(
    body: &brep::Body,
    edges: &[EdgeId],
    wedges: &[Wedge],
    set_back: impl Fn(&Wedge) -> f64,
) -> f64 {
    let ranked = || edges.iter().copied().zip(wedges);
    let mut most = f64::INFINITY;
    for (edge, ending) in ranked() {
        let ends: Vec<VertexId> = body.edge(edge).ends.into_iter().flatten().collect();
        for (other_edge, other) in ranked() {
            if other_edge == edge || corners::meet(body, [edge, other_edge]) {
                continue;
            }
            for (side, &face) in other.faces.iter().enumerate() {
                if ending.faces.contains(&face) {
                    continue;
                }
                for &end in ends.iter().filter(|&&end| cornered(body, face, end)) {
                    let at = body.vertex(end).point;
                    if let Some(value) = short_of(ending, at, other, side, &set_back) {
                        most = most.min(value);
                    }
                }
            }
        }
    }
    most
}

/// The most value at which the round of `ending`, at its end `at` on the
/// face of `other` on `side`, stops short of the set-back `other` makes on
/// it: the distance across that face from the other edge to the end, over
/// that set-back at a value of one and the farthest a value of one carries
/// the round's corners on the face towards it — each along one of the ending
/// edge's faces, to where that face meets the one ended on. None where the
/// end stands on the other edge's line or behind it, or the ending edge runs
/// along the face.
fn short_of(
    ending: &Wedge,
    at: DVec3,
    other: &Wedge,
    side: usize,
    set_back: &impl Fn(&Wedge) -> f64,
) -> Option<f64> {
    let (normal, into) = (other.outward[side], other.into[side]);
    let across = into.dot(at - other.point);
    let slant = normal.dot(ending.along);
    if across <= 0.0 || slant.abs() <= TANGENT {
        return None;
    }
    let towards = ending
        .into
        .iter()
        .map(|&way| -into.dot(way - ending.along * (normal.dot(way) / slant)))
        .fold(0.0, f64::max);
    Some(across / (set_back(other) + set_back(ending) * towards))
}

/// Whether a corner of the body is one of a face's.
fn cornered(body: &brep::Body, face: FaceId, corner: VertexId) -> bool {
    body.face(face).loops.iter().flatten().any(|coedge| {
        body.edge(coedge.edge)
            .ends
            .is_some_and(|ends| ends.contains(&corner))
    })
}
