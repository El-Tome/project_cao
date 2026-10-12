//! A curve seen on an elliptic wall (#571).

use glam::DVec3;

use super::*;
use crate::brep::conic::{Conic, On, Round};
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::surface::{EllipticCylinder, Plane, Surface};

const REACH: f64 = 20.0;

fn oval() -> EllipticCylinder {
    EllipticCylinder::about(DVec3::new(1.0, -2.0, 0.0), DVec3::Z, DVec3::Y * 7.0, 3.0)
}

#[test]
fn the_conic_a_plane_cuts_an_elliptic_wall_along_is_seen_on_both_between_its_parameters() {
    let wall = oval();
    let (plane, _) = Plane::through(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.2, -0.3, 1.0));
    let [conic] = Conic::cut(&plane, &Round::Elliptic(wall), Scale::of(REACH))[..] else {
        panic!("a slant cuts an elliptic wall along one ellipse");
    };
    let [from, to] = [-2.0, 7.5];
    for (surface, on) in [
        (Surface::Plane(plane), On::Plane),
        (Surface::Elliptic(wall), On::Wall),
    ] {
        let trace = traced(
            &Curve::Conic(conic),
            &surface,
            from,
            to,
            Scale::of(REACH).eps(),
        );
        assert_eq!(
            trace,
            Ok(Trace::Conic {
                conic,
                on,
                from,
                to
            })
        );
        let trace = trace.expect("seen");
        for u in [0.0, 0.3, 0.8, 1.0] {
            let point = surface.point(trace.at(u)[0]);
            let exact = conic.point(from + (to - from) * u);
            assert!(point.distance(exact) <= 1e-12 * REACH, "{point} at {u}");
        }
    }
}
