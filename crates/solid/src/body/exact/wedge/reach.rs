//! How far a face runs from an edge of it, square to the edge: the most a
//! chamfer or a fillet of the edge can set back along that face — on the
//! whole face, or only where the edge's blend runs on it (#829).

use std::f64::consts::PI;

use glam::DVec3;

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
/// [`reach`] reads it, counting only the corners and turning points the
/// half-spaces `held` hold within the tolerance, where the edge's blend runs:
/// a corner of the face standing past where the blend stops limits nothing,
/// and a blend may run into a face across one of its edges, as it does at an
/// end it is trimmed on. Declined as [`reach`] declines.
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
        match edge.ends {
            Some(ends) => far.extend(ends.map(|end| body.vertex(end).point)),
            None => far.push(body.point_on(coedge.edge, edge.from)),
        }
        far.extend(turning_back(body, edge, into)?);
    }
    let within =
        |at: &DVec3| (held.iter()).all(|&(normal, offset)| normal.dot(*at) - offset <= eps);
    Some(
        far.into_iter()
            .filter(within)
            .map(|at| into.dot(at - point))
            .filter(|&off| off > eps)
            .fold(f64::INFINITY, f64::min),
    )
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
