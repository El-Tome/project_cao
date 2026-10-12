use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use super::*;
use crate::profile::{Contour, Frame, Run};

fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

#[test]
fn a_point_of_an_edge_is_given_the_parameter_its_edge_runs_through() {
    let slot = Contour {
        corners: vec![
            DVec2::new(-10.0, -5.0),
            DVec2::new(10.0, -5.0),
            DVec2::new(10.0, 5.0),
            DVec2::new(-10.0, 5.0),
        ],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::new(10.0, 0.0),
                turn: PI,
            },
            Run::Straight,
            Run::Round {
                center: DVec2::new(-10.0, 0.0),
                turn: PI,
            },
        ],
    };
    let disc = Contour::circle(DVec2::new(3.0, 1.0), 4.0);
    for outline in [slot, disc] {
        let body = Body::raised(&outline, &[], ground(), DVec3::Z * 10.0).expect("it raises");
        for edge in body.edge_ids() {
            let stretch = body.edge(edge);
            for share in [0.0, 1e-9, 0.25, 0.5, 0.75, 1.0 - 1e-9, 1.0] {
                let t = stretch.from + (stretch.to - stretch.from) * share;
                let found = body.parameter_on(edge, body.point_on(edge, t));
                let turn = body.curve(stretch.curve).period().unwrap_or(f64::INFINITY);
                let whole = stretch.ends.is_none();
                let same = (found - t).abs() <= 1e-12 * (1.0 + t.abs())
                    || (whole && ((found - t).abs() - turn).abs() <= 1e-12);
                assert!(
                    same,
                    "edge {edge:?} over {}..{} gives {found} for {t}",
                    stretch.from, stretch.to
                );
            }
        }
    }
}

#[test]
fn a_body_bounded_by_an_ellipse_reaches_as_far_as_its_ellipse_does() {
    use crate::brep::tessellation::tests::fixtures;
    let lean = 0.25;
    let body = fixtures::stock_cut_at_a_slant(lean);
    let (low, high) = body.bounds().expect("a body has a box");
    let radius = fixtures::STOCK_RADIUS;
    let reached = [
        DVec3::new(-radius, -radius, 0.0),
        DVec3::new(radius, radius, fixtures::HEIGHT + lean * radius),
    ];
    for (found, exact) in [low, high].into_iter().zip(reached) {
        assert!(
            (found - exact).abs().max_element() <= 1e-12 * radius,
            "the box reaches {found} where the body reaches {exact}"
        );
    }
    for edge in body.edge_ids() {
        let stretch = body.edge(edge);
        for share in [0.0, 0.3, 0.7] {
            let t = stretch.from + (stretch.to - stretch.from) * share;
            let found = body.parameter_on(edge, body.point_on(edge, t));
            let turn = body.curve(stretch.curve).period().unwrap_or(f64::INFINITY);
            let apart = (found - t).abs();
            assert!(
                apart <= 1e-12 * (1.0 + t.abs()) || (apart - turn).abs() <= 1e-12 * turn,
                "{found} for {t}"
            );
        }
    }
}

#[test]
fn the_corners_of_a_face_bounded_by_an_ellipse_hold_points_along_it_that_move_with_it() {
    use crate::brep::tessellation::tests::fixtures;
    let corners_at = |lean: f64| {
        let body = fixtures::stock_cut_at_a_slant(lean);
        let top = body
            .face_ids()
            .find(|&face| match body.surface(body.face(face).surface) {
                Surface::Plane(plane) => plane.normal.x != 0.0,
                Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => false,
            })
            .expect("the slanted top");
        let Curve::Conic(conic) = *body.curve(body.edge(body.face(top).loops[0][0].edge).curve)
        else {
            panic!("the top is bounded by its ellipse");
        };
        (conic, body.corners_of(top))
    };
    let (conic, corners) = corners_at(0.25);
    let period = conic.period().expect("an ellipse closes");
    let along: Vec<DVec3> = (0..48)
        .map(|step| conic.point(period * f64::from(step) / 48.0))
        .collect();
    for point in &along {
        assert!(
            corners
                .iter()
                .any(|corner| corner.distance(*point) <= 1e-12),
            "{point} along the ellipse is one of the top's corners"
        );
    }
    let (_, nudged) = corners_at(0.25 + 1e-7);
    for point in &along {
        let nearest = nudged
            .iter()
            .map(|corner| corner.distance(*point))
            .fold(f64::INFINITY, f64::min);
        assert!(
            nearest <= 1e-5,
            "{point} moves by {nearest} as the plane leans a hair further"
        );
    }
}

pub(in crate::brep) mod conic;
mod rotated;
