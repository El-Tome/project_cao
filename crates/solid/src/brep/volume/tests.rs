use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use super::*;
use crate::profile::{Contour, Frame};

fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

fn assert_relative(found: f64, expected: f64) {
    assert!(
        (found - expected).abs() <= 1e-12 * expected.abs(),
        "{found} against {expected}, off by {}",
        (found - expected) / expected
    );
}

#[test]
fn a_block_holds_its_base_times_its_height() {
    let block = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let body = Body::raised(&block, &[], ground(), DVec3::Z * 10.0).expect("a block raises");
    assert_relative(body.volume(), 40.0 * 40.0 * 10.0);
}

/// A frame standing off the origin, its plane leaning on all three axes.
fn leaning() -> Frame {
    Frame {
        origin: DVec3::new(3.0, -2.0, 5.0),
        u: DVec3::new(1.0, 1.0, 0.0) / 2f64.sqrt(),
        v: DVec3::new(-1.0, 1.0, 2f64.sqrt()) / 2.0,
    }
}

fn slot() -> Contour {
    use crate::profile::Run;
    Contour {
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
    }
}

#[test]
fn a_raised_profile_holds_its_area_times_its_height_in_any_frame() {
    let block = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let disc = Contour::circle(DVec2::new(8.0, -3.0), 5.0);
    let hole = Contour::circle(DVec2::new(8.0, 0.0), 5.0);
    let square_hole = Contour::rectangle(DVec2::new(-15.0, -15.0), DVec2::new(-5.0, -5.0));
    let cases: [(&Contour, &[Contour], f64); 5] = [
        (&block, &[], 1600.0),
        (&disc, &[], PI * 25.0),
        (&slot(), &[], 200.0 + PI * 25.0),
        (&block, std::slice::from_ref(&hole), 1600.0 - PI * 25.0),
        (&block, &[hole.clone(), square_hole], 1500.0 - PI * 25.0),
    ];
    for frame in [ground(), leaning()] {
        for height in [10.0, -7.0] {
            for (outline, holes, area) in &cases {
                let travel = frame.normal() * height;
                let body = Body::raised(outline, holes, frame, travel).expect("a profile raises");
                assert_relative(body.volume(), area * height.abs());
            }
        }
    }
}

#[test]
fn a_body_with_every_face_turned_inside_out_holds_the_negative() {
    let block = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let hole = Contour::circle(DVec2::new(8.0, 0.0), 5.0);
    let body = Body::raised(&block, &[hole], leaning(), leaning().normal() * 10.0)
        .expect("a plate raises");
    let mut inside_out = body.clone();
    for face in &mut inside_out.faces {
        face.flipped = !face.flipped;
        for lap in &mut face.loops {
            lap.reverse();
            for coedge in lap.iter_mut() {
                coedge.forward = !coedge.forward;
            }
        }
    }
    assert_relative(inside_out.volume(), -body.volume());
}

#[test]
fn the_quadrature_along_a_circle_agrees_with_its_closed_form() {
    use crate::brep::curve::Circle;
    use crate::brep::surface::Cylinder;
    let frame = leaning();
    let cylinder = Cylinder::about(frame.at(DVec2::new(8.0, 1.0)), frame.normal(), 5.0);
    let circle = Circle::on(&cylinder, 4.0);
    let (plane, _) = Plane::through(circle.center, frame.normal());
    let [from, to] = [-2.5, 1.75];
    let on_cylinder = Surface::Cylinder(cylinder);
    let on_plane = Surface::Plane(plane);
    let trace = traced(&Curve::Circle(circle), &on_plane, from, to).expect("a round");
    let exact = [
        wall(&cylinder, from, to, 4.0, 4.0).expect("a level circle"),
        plane.offset() * swept(&trace).expect("a round has an area"),
    ];
    let numeric =
        [on_cylinder, on_plane].map(|surface| along(&Curve::Circle(circle), from, to, &surface));
    for (exact, numeric) in exact.into_iter().zip(numeric) {
        assert_relative(numeric, exact);
    }
}

#[test]
fn an_outline_drawn_clockwise_still_holds_a_positive_volume() {
    let mut block = Contour::rectangle(DVec2::ZERO, DVec2::new(4.0, 3.0));
    block.corners.reverse();
    let body = Body::raised(&block, &[], ground(), DVec3::Z * 2.0).expect("a block raises");
    assert_relative(body.volume(), 24.0);
}
