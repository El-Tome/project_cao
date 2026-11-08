//! The extremes of an edge along the curve two walls meet along: where a
//! coordinate turns, not the best of points read along it.
//!
//! A cylinder of radius 5 stands on Z; one of radius 3 lies along X through
//! `(0, 1, 2)`, so the curve passes x = 5 where it crosses the plane y = 0,
//! at `t = acos(1/3)` along the height `y = 1 − 3 cos t`. The edge runs a
//! stretch about that crest, which lies a third of the way between two of
//! the 64 points the edge is read at today.
//!
//! Closes #775.
//! - An edge along a `Meet` curve whose largest x lies between two samples
//!   gives that largest x, to rounding —
//!   `an_edge_along_a_meet_curve_gives_its_largest_x_where_it_falls_between_two_samples`

use crate::brep::curve::{Meet, Wall};
use crate::brep::surface::Cylinder;
use crate::brep::topology::VertexId;

use super::*;

const RADIUS: f64 = 5.0;

#[test]
fn an_edge_along_a_meet_curve_gives_its_largest_x_where_it_falls_between_two_samples() {
    let meet = Meet {
        first: Wall::Cylinder(Cylinder::about(DVec3::ZERO, DVec3::Z, RADIUS)),
        second: Wall::Cylinder(Cylinder::about(DVec3::new(0.0, 1.0, 2.0), DVec3::X, 3.0)),
        component: 0,
    };
    let crest = (1.0_f64 / 3.0).acos();
    assert!(
        (meet.point(crest).x - RADIUS).abs() <= 1e-12 * RADIUS,
        "the fixture's crest {} lies at x = {RADIUS}",
        meet.point(crest)
    );
    let step = 0.02;
    let (from, to) = (
        crest - (31.0 + 1.0 / 3.0) * step,
        crest + (32.0 + 2.0 / 3.0) * step,
    );

    let mut body = Body::empty();
    body.curves.push(Curve::Meet(meet));
    for t in [from, to] {
        body.vertices.push(Vertex {
            point: meet.point(t),
            on: Vec::new(),
        });
    }
    let edge = Edge {
        curve: CurveId(0),
        ends: Some([VertexId(0), VertexId(1)]),
        from,
        to,
    };

    let largest = body
        .extremes(&edge)
        .into_iter()
        .map(|point| point.x)
        .fold(f64::NEG_INFINITY, f64::max);
    assert!(
        (largest - RADIUS).abs() <= 1e-12 * RADIUS,
        "the edge reaches x = {RADIUS}, its extremes only {largest}, short by {}",
        RADIUS - largest
    );
}
