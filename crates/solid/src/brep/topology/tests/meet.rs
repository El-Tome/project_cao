//! The extremes of an edge along the curve two walls meet along: where a
//! coordinate turns, not the best of points read along it.
//!
//! A cylinder of radius 5 stands on Z; one of radius 3 lies along X through
//! `(0, 1, 2)`, so the curve passes x = 5 where it crosses the plane y = 0,
//! at `t = acos(1/3)` along the height `y = 1 − 3 cos t`. One edge runs a
//! stretch about that crest, which lies a third, then two thirds, of the way
//! between two of the 64 points the edge is read at: the speed changes sign
//! across that pair of points whichever of the two is the faster. Another
//! ends before the crest, and every coordinate runs one way along it. The
//! pair is drawn at its own size and ten thousand times smaller, where a
//! coordinate's speed is that much smaller too.
//!
//! What no test here tells apart, each in `Meet::turning`: a bracket
//! bisected where the speed keeps its sign (`||` for `&&`) or where one end
//! reads exactly zero (`<=` for `<`), and the sign change read off the
//! quotient of the two speeds rather than their product, which differ only
//! where one is exactly zero. Each bisects one more bracket the edge holds,
//! so the point it adds lies on the curve within the edge, inside the box
//! the edge's own points span, and nothing that reads `extremes` changes.
//!
//! Closes #775.
//! - An edge along a `Meet` curve whose largest x lies between two samples
//!   gives that largest x, to rounding —
//!   `an_edge_along_a_meet_curve_gives_its_largest_x_where_it_falls_between_two_samples`;
//!   that no point is read beyond the edge, by
//!   `an_edge_along_a_meet_curve_ending_before_its_crest_gives_no_point_beyond_its_own`

use crate::brep::curve::{Meet, Wall};
use crate::brep::surface::Cylinder;
use crate::brep::topology::VertexId;

use super::*;

const RADIUS: f64 = 5.0;

const SIZES: [f64; 2] = [1.0, 1e-4];

fn crossing(size: f64) -> Meet {
    Meet {
        first: Wall::Cylinder(Cylinder::about(DVec3::ZERO, DVec3::Z, RADIUS * size)),
        second: Wall::Cylinder(Cylinder::about(
            DVec3::new(0.0, 1.0, 2.0) * size,
            DVec3::X,
            3.0 * size,
        )),
        component: 0,
    }
}

fn crest() -> f64 {
    (1.0_f64 / 3.0).acos()
}

fn extremes_along(meet: Meet, from: f64, to: f64) -> Vec<DVec3> {
    let mut body = Body::empty();
    body.curves.push(Curve::Meet(meet));
    for t in [from, to] {
        body.vertices.push(Vertex {
            point: meet.point(t),
            on: Vec::new(),
        });
    }
    body.extremes(&Edge {
        curve: CurveId(0),
        ends: Some([VertexId(0), VertexId(1)]),
        from,
        to,
    })
}

#[test]
fn an_edge_along_a_meet_curve_gives_its_largest_x_where_it_falls_between_two_samples() {
    for size in SIZES {
        let meet = crossing(size);
        let reach = RADIUS * size;
        assert!(
            (meet.point(crest()).x - reach).abs() <= 1e-12 * reach,
            "the fixture's crest {} lies at x = {reach}",
            meet.point(crest())
        );
        for share in [1.0 / 3.0, 2.0 / 3.0] {
            let step = 0.02;
            let (from, to) = (
                crest() - (31.0 + share) * step,
                crest() + (33.0 - share) * step,
            );
            let largest = extremes_along(meet, from, to)
                .into_iter()
                .map(|point| point.x)
                .fold(f64::NEG_INFINITY, f64::max);
            assert!(
                (largest - reach).abs() <= 1e-12 * reach,
                "at size {size}, the crest {share} of a step past a sample: the edge reaches \
                 x = {reach}, its extremes only {largest}, short by {}",
                reach - largest
            );
        }
    }
}

#[test]
fn an_edge_along_a_meet_curve_ending_before_its_crest_gives_no_point_beyond_its_own() {
    for size in SIZES {
        let meet = crossing(size);
        let reach = RADIUS * size;
        let (from, to) = (crest() - 1.0, crest() - 0.2);
        let [low, high] = (0..=4096)
            .map(|index| meet.point(from + (to - from) * f64::from(index) / 4096.0))
            .fold(
                [DVec3::INFINITY, DVec3::NEG_INFINITY],
                |[low, high], point| [low.min(point), high.max(point)],
            );
        for point in extremes_along(meet, from, to) {
            let past = (low - point).max(point - high).max_element();
            assert!(
                past <= 1e-12 * reach,
                "at size {size}, {point} lies {past} beyond the box {low}..{high} of the edge's \
                 own points"
            );
        }
    }
}
