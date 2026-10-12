//! A curve seen on an elliptic wall, and a place of one located against its
//! face (#571).

use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::conic::{Conic, On, Round};
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::surface::{EllipticCylinder, Plane, Surface};
use crate::profile::{Contour, Frame, Run};

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

/// A slot whose right end is half an ellipse of half-axes 4 along it and 3
/// across, raised 2: its elliptic wall runs from the ruling at its bottom
/// corner to the one at its top corner.
fn slot_with_an_elliptic_end() -> Body {
    let slot = Contour {
        corners: vec![
            DVec2::new(-5.0, -3.0),
            DVec2::new(5.0, -3.0),
            DVec2::new(5.0, 3.0),
            DVec2::new(-5.0, 3.0),
        ],
        runs: vec![
            Run::Straight,
            Run::Oval {
                center: DVec2::new(5.0, 0.0),
                first: DVec2::new(0.0, -3.0),
                second: 4.0,
                turn: PI,
            },
            Run::Straight,
            Run::Straight,
        ],
    };
    let ground = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::raised(&slot, &[], ground, DVec3::Z * 2.0).expect("the slot raises")
}

#[test]
fn a_place_of_an_elliptic_wall_is_located_against_its_face() {
    let body = slot_with_an_elliptic_end();
    let eps = body.scale().eps();
    let (face, wall) = body
        .face_ids()
        .find_map(|face| match body.surface(body.face(face).surface) {
            Surface::Elliptic(wall) => Some((face, *wall)),
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) => None,
        })
        .expect("the slot's end is an elliptic wall");
    let at = |point: DVec3| wall.parameters(point);
    let outward = DVec3::new(9.0, 0.0, 1.0);
    let behind = DVec3::new(1.0, 0.0, 1.0);
    let rim = DVec3::new(9.0, 0.0, 2.0);
    let corner = DVec3::new(5.0, 3.0, 1.0);
    let above = DVec3::new(9.0, 0.0, 3.0);
    let cases = [
        (outward, Location::Inside),
        (behind, Location::Outside),
        (rim, Location::Boundary),
        (corner, Location::Boundary),
        (above, Location::Outside),
    ];
    for (point, expected) in cases {
        assert_eq!(body.locate(face, at(point), eps), Ok(expected), "{point}");
    }
}
