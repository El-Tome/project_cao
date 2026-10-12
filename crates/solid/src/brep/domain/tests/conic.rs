//! Faces bounded by the conics a plane cuts a wall along (#552): seen on
//! their own plane and wall, on a surface a hair beside them, and located.

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::*;
use crate::brep::conic::{Conic, On, Round};
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::tessellation::tests::fixtures;
use crate::brep::topology::tests::conic::{TIP, point_with_a_flat};

const REACH: f64 = 20.0;

fn slanted() -> (Plane, Cylinder, Conic) {
    let (plane, _) = Plane::through(DVec3::new(0.0, 0.0, 10.0), DVec3::new(0.25, 0.0, 1.0));
    let wall = Cylinder::about(DVec3::ZERO, DVec3::Z, REACH);
    let [conic] = Conic::cut(&plane, &Round::Cylinder(wall), Scale::of(REACH))[..] else {
        panic!("a plane at a slant cuts a shaft along one ellipse");
    };
    (plane, wall, conic)
}

/// How far a point stands from a conic, at the foot of the conic's own
/// parameter.
fn off(conic: &Conic, point: DVec3) -> f64 {
    conic.point(conic.parameter(point)).distance(point)
}

#[test]
fn a_conic_is_seen_on_its_own_plane_and_its_own_wall_between_its_parameters() {
    let (plane, wall, conic) = slanted();
    let [from, to] = [-0.3, 1.1];
    for (surface, on) in [
        (Surface::Plane(plane), On::Plane),
        (Surface::Cylinder(wall), On::Wall),
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
        for u in [0.0, 0.4, 1.0] {
            let point = surface.point(trace.at(u)[0]);
            let exact = conic.point(from + (to - from) * u);
            assert!(point.distance(exact) <= 1e-12 * REACH, "{point} at {u}");
        }
    }
}

#[test]
fn a_conic_on_a_wall_a_hair_beside_its_own_is_seen_as_the_conic_that_wall_meets_its_plane_along() {
    let (plane, _, conic) = slanted();
    let eps = Scale::of(REACH).eps();
    let wall = Cylinder::about(DVec3::ZERO, DVec3::Z, REACH + 0.4 * eps);
    let (lifted, _) = Plane::through(
        DVec3::new(0.0, 0.0, 10.0 + 0.4 * eps),
        DVec3::new(0.25, 0.0, 1.0),
    );
    let period = conic.period().expect("an ellipse closes");
    for (surface, on) in [
        (Surface::Cylinder(wall), On::Wall),
        (Surface::Plane(lifted), On::Plane),
    ] {
        for [from, to] in [[-0.3, 1.1], [0.2, 0.2 + period], [1.0, 1.0 - period]] {
            let trace = traced(&Curve::Conic(conic), &surface, from, to, eps);
            let Ok(Trace::Conic {
                conic: seen,
                on: seen_on,
                ..
            }) = trace
            else {
                panic!("a conic beside its wall is the conic of that wall, not {trace:?}");
            };
            assert_eq!(seen_on, on);
            match surface {
                Surface::Cylinder(wall) => {
                    assert_eq!((seen.plane, seen.round), (plane, Round::Cylinder(wall)));
                }
                Surface::Plane(lifted) => assert_eq!(seen.plane, lifted),
                Surface::Cone(_) | Surface::Elliptic(_) => unreachable!(),
            }
            let trace = trace.expect("seen");
            for (u, at) in [(0.0, from), (1.0, to)] {
                let point = surface.point(trace.at(u)[0]);
                assert!(
                    point.distance(conic.point(at)) <= 2.0 * eps,
                    "the trace ends at {point}, its curve at {}",
                    conic.point(at)
                );
            }
            let middle = surface.point(trace.at(0.5)[0]);
            assert!(
                middle.distance(conic.point((from + to) / 2.0)) <= 1e-3 * REACH,
                "the trace runs the way its curve does, through {middle}"
            );
            for u in [0.1, 0.3, 0.7, 0.9] {
                let point = surface.point(trace.at(u)[0]);
                assert!(off(&conic, point) <= eps, "{point} at {u} off the conic");
            }
        }
    }
}

#[test]
fn a_circle_leaning_on_a_plane_that_bulges_off_its_chord_is_seen_as_the_conic_its_wall_meets_it_along()
 {
    let eps = Scale::of(10.0).eps();
    let tilt: f64 = 1.5e-9;
    let (plane, _) = Plane::through(DVec3::ZERO, DVec3::new(tilt.sin(), 0.0, tilt.cos()));
    let circle = Circle::on(&Cylinder::about(DVec3::ZERO, DVec3::Z, 5.0), 0.0);
    for [from, to] in [[0.0, TAU], [0.5, 0.5 + PI], [2.0, 2.0 - 3.0]] {
        let trace = traced(
            &Curve::Circle(circle),
            &Surface::Plane(plane),
            from,
            to,
            eps,
        );
        let Ok(Trace::Conic { conic, on, .. }) = trace else {
            panic!("a rim leaning on a plane by {tilt} is seen as a conic, not {trace:?}");
        };
        assert_eq!((conic.plane, on), (plane, On::Plane));
        let trace = trace.expect("seen");
        for u in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let point = plane.point(trace.at(u)[0]);
            let exact = circle.point(from + (to - from) * u);
            assert!(
                point.distance(exact) <= eps,
                "{point} at {u} stands {} from the rim",
                point.distance(exact)
            );
        }
    }
}

#[test]
fn a_circle_leaning_on_a_plane_well_off_its_own_is_still_declined() {
    let (plane, _) = Plane::through(DVec3::ZERO, DVec3::new(1e-3, 0.0, 1.0));
    let circle = Circle::on(&Cylinder::about(DVec3::ZERO, DVec3::Z, 5.0), 0.0);
    assert_eq!(
        traced(
            &Curve::Circle(circle),
            &Surface::Plane(plane),
            0.0,
            TAU,
            EPS
        ),
        Err(Declined::Unsupported)
    );
}

fn slanted_top(body: &Body) -> FaceId {
    body.face_ids()
        .find(|&face| match body.surface(body.face(face).surface) {
            Surface::Plane(plane) => plane.normal.x != 0.0,
            Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => false,
        })
        .expect("the slanted top")
}

#[test]
fn a_face_of_a_plane_bounded_by_an_ellipse_locates_inside_outside_and_on() {
    let body = fixtures::stock_cut_at_a_slant(0.25);
    let top = slanted_top(&body);
    let Surface::Plane(plane) = *body.surface(body.face(top).surface) else {
        unreachable!()
    };
    let on_top = |x: f64, y: f64| plane.parameters(DVec3::new(x, y, 10.0 - 0.25 * x));
    let rim = |angle: f64| {
        let point = DVec3::new(REACH * angle.cos(), REACH * angle.sin(), 0.0);
        on_top(point.x, point.y)
    };
    let eps = body.scale().eps();
    for (at, expected) in [
        (on_top(0.0, 0.0), Location::Inside),
        (on_top(19.9, 0.0), Location::Inside),
        (on_top(-14.0, 14.0), Location::Inside),
        (on_top(20.1, 0.0), Location::Outside),
        (on_top(0.0, -20.1), Location::Outside),
        (on_top(-30.0, 25.0), Location::Outside),
        (rim(0.0), Location::Boundary),
        (rim(2.0), Location::Boundary),
        (rim(-PI / 2.0), Location::Boundary),
    ] {
        assert_eq!(
            body.locate(top, at, eps),
            Ok(expected),
            "{at} on the slanted top"
        );
    }
    let inside = body
        .point_inside(top)
        .expect("traced")
        .expect("the top has an inside");
    assert_eq!(body.locate(top, inside, eps), Ok(Location::Inside));
}

#[test]
fn a_band_of_a_wall_between_a_rim_and_an_ellipse_locates_a_point_inside() {
    let body = fixtures::stock_cut_at_a_slant(0.25);
    let wall = body
        .face_ids()
        .find(|&face| matches!(body.surface(body.face(face).surface), Surface::Cylinder(_)))
        .expect("the wall");
    let Surface::Cylinder(cylinder) = *body.surface(body.face(wall).surface) else {
        unreachable!()
    };
    let at = |angle: f64, height: f64| {
        let point = DVec3::new(REACH * angle.cos(), REACH * angle.sin(), height);
        cylinder.parameters(point)
    };
    let eps = body.scale().eps();
    for (place, expected) in [
        (at(0.0, 2.0), Location::Inside),
        (at(0.0, 6.0), Location::Outside),
        (at(PI, 14.0), Location::Inside),
        (at(PI, 16.0), Location::Outside),
        (at(PI / 2.0, 9.9), Location::Inside),
        (at(-PI / 2.0, 10.1), Location::Outside),
        (at(0.0, 5.0), Location::Boundary),
        (at(PI / 2.0, 10.0), Location::Boundary),
        (at(1.0, 0.0), Location::Boundary),
        (at(1.0, -0.5), Location::Outside),
    ] {
        assert_eq!(
            body.locate(wall, place, eps),
            Ok(expected),
            "{place} on the wall"
        );
    }
    let inside = body
        .point_inside(wall)
        .expect("traced")
        .expect("the wall has an inside");
    assert_eq!(body.locate(wall, inside, eps), Ok(Location::Inside));
}

#[test]
fn a_flat_bounded_by_a_hyperbola_locates_its_inside_on_the_cone() {
    let cut = point_with_a_flat(4.0);
    let body = &cut.body;
    let eps = body.scale().eps();
    let on = |face: FaceId, point: DVec3| body.surface(body.face(face).surface).parameters(point);
    let slope = |angle: f64, height: f64| {
        let radius = (TIP - height) / 2.0;
        DVec3::new(radius * angle.cos(), radius * angle.sin(), height)
    };
    for (face, point, expected) in [
        (cut.flat, DVec3::new(4.0, 0.0, 5.0), Location::Inside),
        (cut.flat, DVec3::new(4.0, 7.0, 1.0), Location::Inside),
        (cut.flat, DVec3::new(4.0, 0.0, 13.0), Location::Outside),
        (cut.flat, DVec3::new(4.0, 9.5, 1.0), Location::Outside),
        (cut.flat, DVec3::new(4.0, 0.0, -1.0), Location::Outside),
        (cut.flat, DVec3::new(4.0, 0.0, 12.0), Location::Boundary),
        (cut.flat, DVec3::new(4.0, 2.0, 0.0), Location::Boundary),
        (cut.beyond, slope(0.0, 5.0), Location::Inside),
        (cut.beyond, slope(0.3, 2.0), Location::Inside),
        (cut.beyond, slope(0.0, 15.0), Location::Outside),
        (cut.beyond, slope(PI, 5.0), Location::Outside),
        (cut.beyond, slope(0.0, 12.0), Location::Boundary),
        (cut.tip, slope(PI, 5.0), Location::Inside),
        (cut.tip, slope(0.0, 15.0), Location::Inside),
        (cut.tip, slope(0.0, 5.0), Location::Outside),
    ] {
        assert_eq!(
            body.locate(face, on(face, point), eps),
            Ok(expected),
            "{point} on face {face:?}"
        );
    }
    for face in [cut.flat, cut.beyond, cut.tip] {
        let inside = body
            .point_inside(face)
            .expect("traced")
            .expect("each face has an inside");
        assert_eq!(body.locate(face, inside, eps), Ok(Location::Inside));
    }
}

#[test]
fn a_place_outside_the_box_of_a_conic_is_told_far_from_it_before_any_distance() {
    let body = fixtures::stock_cut_at_a_slant(0.25);
    let top = slanted_top(&body);
    let edge = body.face(top).loops[0][0].edge;
    let trace = body
        .trace(edge, body.face(top).surface)
        .expect("the ellipse is seen on its plane");
    let eps = body.scale().eps();
    let on_it = body.point_on(edge, 1.3);
    assert!(!body.far_from(&trace, on_it, eps));
    assert!(!body.far_from(&trace, DVec3::new(0.0, 0.0, 10.0), eps));
    assert!(body.far_from(&trace, DVec3::new(30.0, 0.0, 0.0), eps));
    assert!(body.far_from(&trace, DVec3::new(0.0, 0.0, 30.0), eps));
}

/// The ellipse a plane leaning twenty degrees about X cuts a shaft of radius
/// two along, anchored at the vertex nearest the origin, as the kernel lists
/// it (seed 0 of `a_lone_round_leaf_cut_by_a_slanted_half_space_…`): along
/// its plane its first parameter turns back a quarter of the way round,
/// exactly on a sample, where its derivative rounds to nought.
fn turning_on_a_sample() -> Trace {
    let conic = Conic {
        plane: Plane {
            origin: DVec3::new(-0.0, 0.4537315202591833, -1.2466171064069236),
            normal: DVec3::new(0.0, -0.3420201433256687, 0.9396926207859084),
            u: DVec3::X,
            v: DVec3::new(-0.0, 0.9396926207859084, 0.3420201433256687),
        },
        round: Round::Cylinder(Cylinder {
            origin: DVec3::new(4.0, 8.0, 0.0),
            axis: DVec3::new(-0.0, -0.0, 1.0),
            radius: 2.0000000000000004,
            u: DVec3::X,
            v: DVec3::new(-0.0, 1.0, 0.0),
        }),
        anchor: DVec3::new(4.0, 6.0, 0.7720595314675952),
        tangent: DVec3::new(1.0, 6.123233995736766e-17, 0.0),
        bend: DVec3::new(
            -3.0616169978683824e-17,
            0.49999999999999994,
            0.18198511713310114,
        ),
        k: 0.24999999999999986,
        arm: crate::brep::conic::Arm::Whole,
    };
    Trace::Conic {
        conic,
        on: On::Plane,
        from: 0.0,
        to: conic.period().expect("an ellipse closes"),
    }
}

#[test]
fn an_ellipse_turning_back_exactly_on_a_sample_is_crossed_twice_by_a_line_through_it() {
    let trace = turning_on_a_sample();
    assert_eq!(
        trace.at(0.25)[1].x,
        0.0,
        "the derivative rounds to nought on the sample"
    );
    let heights = crossing::heights(&[vec![trace]], 4.28, false);
    let [low, high] = heights[..] else {
        panic!("a line through an ellipse crosses it twice, not at {heights:?}");
    };
    for (height, expected) in [(low, 5.923), (high, 10.137)] {
        assert!((height - expected).abs() <= 1e-3, "{heights:?}");
    }
}
