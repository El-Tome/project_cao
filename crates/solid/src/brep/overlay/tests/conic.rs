//! Arcs of conics leaving a vertex (#552), ordered as circles are.

use glam::DVec3;

use super::*;
use crate::brep::conic::{Conic, On, Round};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane};

/// A whole loop of the ellipse a plane cuts a wall along, from `vertex`
/// round to it, run counterclockwise in the plane's parameters.
fn ellipse(plane: &Plane, wall: Cylinder, vertex: DVec3) -> Arc {
    let [conic] = Conic::cut(plane, &Round::Cylinder(wall), Scale::of(20.0))[..] else {
        panic!("a plane at a slant cuts a wall along one ellipse");
    };
    let period = conic.period().expect("an ellipse closes");
    let from = conic.parameter(vertex);
    let trace = Trace::Conic {
        conic,
        on: On::Plane,
        from,
        to: from + period,
    };
    let [_, speed, bend] = trace.at(0.0);
    let trace = if speed.perp_dot(bend) > 0.0 {
        trace
    } else {
        trace.reversed()
    };
    Arc {
        trace,
        ends: Some([0, 0]),
    }
}

#[test]
fn two_conics_leaving_a_vertex_tangent_are_ordered_by_their_bend() {
    let (plane, _) = Plane::through(DVec3::ZERO, DVec3::new(0.3, -0.2, 1.0));
    let touch =
        |radius: f64| Cylinder::about(DVec3::new(0.0, 10.0 - radius, 0.0), DVec3::Z, radius);
    let foot = DVec3::new(0.0, 10.0, 0.0);
    let vertex = foot - DVec3::Z * (plane.distance(foot) / plane.normal.z);
    let arcs = vec![
        ellipse(&plane, touch(10.0), vertex),
        ellipse(&plane, touch(5.0), vertex),
    ];
    let vertices = vec![plane.parameters(vertex)];
    let cycles = star::cycles(&arcs, &vertices, None).expect("the two ellipses bend apart");
    assert_eq!(
        cycles.list,
        vec![
            vec![(0, true), (1, false)],
            vec![(0, false)],
            vec![(1, true)]
        ]
    );
}
