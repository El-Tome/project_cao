//! A face stood apart from a cone's: inside the hollow the cone's face leaves
//! about its axis, or beyond a plane touching the cone along a ruling.

use glam::{DVec2, DVec3};

use super::{farthest, radii, reach};
use crate::brep::curve::Curve;
use crate::brep::surface::Cone;
use crate::brep::topology::{Body, FaceId};

/// Whether a face stands nearer a cone's axis everywhere than the face on
/// the cone does anywhere: a pocket within a chamfered end.
pub(super) fn in_the_hollow(
    cone: &Cone,
    nappe: (&Body, FaceId),
    face: (&Body, FaceId),
    room: f64,
) -> bool {
    match (radii(cone, nappe), farthest(face, cone.origin, cone.axis)) {
        (Some([inner, _]), Some(farthest)) => farthest + room < inner,
        _ => false,
    }
}

/// Whether a face stands beyond a plane touching the cone along one of its
/// rulings, on the side away from the solid the nappe bounds: a block beside
/// a point's slope, which no direction the two surfaces offer tells apart
/// from the ring the slope makes. The planes tried touch the cone where the
/// face's corners stand about the axis, and where their mean does.
pub(super) fn beyond(cone: &Cone, face: (&Body, FaceId), room: f64) -> bool {
    let corners = corners(face);
    let mean = corners.iter().sum::<DVec3>() / corners.len().max(1) as f64;
    corners.iter().chain([&mean]).any(|&corner| {
        let theta = cone.parameters(corner).x;
        let outward = cone.normal(theta) * cone.ruling.x.signum();
        let touching = outward.dot(cone.point(DVec2::new(theta, 0.0)));
        reach(face, outward).is_some_and(|[low, _]| low > touching + room)
    })
}

/// The ends of a face's edges, and the centres of its circles.
fn corners((body, face): (&Body, FaceId)) -> Vec<DVec3> {
    body.face(face)
        .loops
        .iter()
        .flatten()
        .flat_map(|coedge| {
            let edge = body.edge(coedge.edge);
            let curve = body.curve(edge.curve);
            let centre = match curve {
                Curve::Circle(circle) => Some(circle.center),
                Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => None,
            };
            [curve.point(edge.from), curve.point(edge.to)]
                .into_iter()
                .chain(centre)
        })
        .collect()
}
