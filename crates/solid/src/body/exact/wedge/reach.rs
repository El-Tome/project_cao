//! How far a face runs from an edge of it, square to the edge: the most a
//! chamfer or a fillet of the edge can set back along that face — on the
//! whole face, or, for what stands on the face, only where the edge's blend
//! runs on it (#829): up to the nearest corner of its foot there, or point
//! where its foot enters.

use std::f64::consts::PI;

use glam::DVec3;

use super::Wedge;
use super::extent::Within;
use crate::brep::{self, Curve, Edge, FaceId};

/// How far a face runs along `into` from the edge's line through `point`:
/// to the nearest corner of its boundary off that line, or to the nearest
/// point where a round or a conic of its boundary turns back along `into`.
/// A whole closed edge counts where its turn starts as a corner, so that a
/// turning point standing there is not lost between the two ends of the
/// turn. None where its boundary holds a curve two walls meet along, which
/// turns back where no closed form says.
pub(super) fn reach(body: &brep::Body, face: FaceId, into: DVec3, point: DVec3) -> Option<f64> {
    reach_within(body, face, (into, point), &[])
}

/// How far a face runs along `into` from the edge's line through `point`, as
/// [`reach`] reads it, but for the feet of what stands on the face — each
/// straight edge of its boundary meeting the face beyond inwards — read only
/// where the edge's blend runs, within the half-spaces `held`: at its ends
/// they hold, within the tolerance, and where it crosses the plane of one of
/// them inside the others, as [`entering`] finds it. Its depth is linear
/// between, so the least of those is where it first stands in the sweep; a
/// corner of it standing past where the blend stops limits nothing. The
/// face's own outline, every edge meeting the face beyond outwards, curved or
/// between faces that are not both flat, is read whole, as [`reach`] reads
/// it: where the face ends, the laying decides what a blend running past does.
/// Declined as [`reach`] declines.
pub(super) fn reach_within(
    body: &brep::Body,
    face: FaceId,
    (into, point): (DVec3, DVec3),
    held: &[Within],
) -> Option<f64> {
    let eps = body.scale().eps();
    let mut far = Vec::new();
    for coedge in body.face(face).loops.iter().flatten() {
        let edge = body.edge(coedge.edge);
        let standing = matches!(body.curve(edge.curve), Curve::Line(_))
            && Wedge::faced(body, coedge.edge).is_ok_and(|wedge| wedge.inwards);
        match (standing, edge.ends) {
            (true, Some(ends)) => {
                far.extend(entering(held, ends.map(|end| body.vertex(end).point), eps));
            }
            (_, Some(ends)) => far.extend(ends.map(|end| body.vertex(end).point)),
            (_, None) => far.push(body.point_on(coedge.edge, edge.from)),
        }
        far.extend(turning_back(body, edge, into)?);
    }
    Some(
        far.into_iter()
            .map(|at| into.dot(at - point))
            .filter(|&off| off > eps)
            .fold(f64::INFINITY, f64::min),
    )
}

/// The points of the straight edge from `ends[0]` to `ends[1]` where it
/// stands inside every half-space of `held`, within `eps`, at its least and
/// most: each end they hold, and each point where it crosses the plane of
/// one of them — read on that plane exactly, so that an edge leaving a corner
/// the plane passes through is met there and not a tolerance past it — that
/// the others hold.
fn entering(held: &[Within], ends: [DVec3; 2], eps: f64) -> Vec<DVec3> {
    let forms: Vec<[f64; 2]> = (held.iter())
        .map(|&(normal, offset)| ends.map(|at| normal.dot(at) - offset))
        .collect();
    let inside = |share: f64| (forms.iter()).all(|&[from, to]| from + (to - from) * share <= eps);
    let crossings = forms.iter().filter_map(|&[from, to]| {
        let level = -from / (to - from);
        ((to - from).abs() > eps && (0.0..=1.0).contains(&level)).then_some(level)
    });
    ([0.0, 1.0].into_iter().chain(crossings))
        .filter(|&share| inside(share))
        .map(|share| ends[0] + (ends[1] - ends[0]) * share)
        .collect()
}

/// The points strictly inside an edge where its curve turns back along
/// `into`: none on a line, every half turn from the phase of `into` on a
/// circle, the conic's own turning points.
fn turning_back(body: &brep::Body, edge: &Edge, into: DVec3) -> Option<Vec<DVec3>> {
    let curve = body.curve(edge.curve);
    let (low, high) = (edge.from.min(edge.to), edge.from.max(edge.to));
    let turns = match curve {
        Curve::Line(_) => Vec::new(),
        Curve::Circle(circle) => {
            let phase = into.dot(circle.v).atan2(into.dot(circle.u));
            let first = ((low - phase) / PI).ceil() as i64;
            let last = ((high - phase) / PI).floor() as i64;
            (first..=last)
                .map(|turn| phase + PI * turn as f64)
                .filter(|&at| at > low && at < high)
                .collect()
        }
        Curve::Conic(conic) => conic.turning(into, low, high),
        Curve::Meet(_) => return None,
    };
    Some(turns.into_iter().map(|at| curve.point(at)).collect())
}
