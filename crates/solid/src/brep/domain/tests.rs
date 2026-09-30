use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::profile::{Contour, Frame};

const CLOSE: f64 = 1e-12;

fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

fn close(one: DVec2, other: DVec2) -> bool {
    one.distance(other) <= CLOSE
}

#[test]
fn a_line_on_a_plane_is_a_segment_between_its_ends_parameters() {
    let (plane, _) = Plane::through(DVec3::new(0.0, 0.0, 10.0), DVec3::Z);
    let line = Line::through(DVec3::new(1.0, 2.0, 10.0), DVec3::new(3.0, -1.0, 0.0));
    let [from, to] = [-2.0, 5.0];
    let trace = traced(&Curve::Line(line), &Surface::Plane(plane), from, to);
    let Ok(Trace::Segment {
        from: start,
        to: end,
    }) = trace
    else {
        panic!("a segment, not {trace:?}");
    };
    assert!(close(start, plane.parameters(line.point(from))));
    assert!(close(end, plane.parameters(line.point(to))));
}

#[test]
fn a_circle_square_to_a_plane_is_a_round_turning_the_way_its_axis_says() {
    let (plane, _) = Plane::through(DVec3::new(0.0, 0.0, 10.0), DVec3::Z);
    let cylinder = Cylinder::about(DVec3::new(8.0, 0.0, 0.0), DVec3::Z, 5.0);
    let circle = Circle::on(&cylinder, 10.0);
    let backwards = Circle {
        axis: -circle.axis,
        v: -circle.v,
        ..circle
    };
    for circle in [circle, backwards] {
        let trace = traced(&Curve::Circle(circle), &Surface::Plane(plane), 0.5, 2.0)
            .expect("a circle square to a plane is a round");
        for u in [0.0, 0.3, 1.0] {
            let expected = plane.parameters(circle.point(0.5 + 1.5 * u));
            assert!(close(trace.at(u)[0], expected), "{trace:?} at {u}");
        }
    }
}

#[test]
fn a_circle_leaning_on_a_plane_is_declined() {
    let (plane, _) = Plane::through(DVec3::ZERO, DVec3::Z);
    let cylinder = Cylinder::about(DVec3::ZERO, DVec3::new(0.0, 1.0, 1.0), 5.0);
    let circle = Curve::Circle(Circle::on(&cylinder, 0.0));
    assert_eq!(
        traced(&circle, &Surface::Plane(plane), 0.0, 1.0),
        Err(Declined::Unsupported)
    );
}

#[test]
fn a_ruling_of_a_cylinder_is_a_vertical_segment() {
    let cylinder = Cylinder::about(DVec3::new(8.0, 0.0, 0.0), DVec3::Z, 5.0);
    let foot = cylinder.point(DVec2::new(2.5, 0.0));
    let line = Line::through(foot, -DVec3::Z);
    let [from, to] = [line.parameter(foot), line.parameter(foot + DVec3::Z * 10.0)];
    let trace = traced(&Curve::Line(line), &Surface::Cylinder(cylinder), from, to);
    let Ok(Trace::Segment {
        from: start,
        to: end,
    }) = trace
    else {
        panic!("a segment, not {trace:?}");
    };
    assert_eq!(start.x, end.x);
    assert!(close(start, DVec2::new(2.5, 0.0)));
    assert!(close(end, DVec2::new(2.5, 10.0)));
}

#[test]
fn a_circle_of_a_cylinder_is_a_level_segment_with_its_angle_unwrapped() {
    let cylinder = Cylinder::about(DVec3::new(8.0, 0.0, 0.0), DVec3::Z, 5.0);
    let circle = Circle::on(&cylinder, 10.0);
    let trace = traced(
        &Curve::Circle(circle),
        &Surface::Cylinder(cylinder),
        3.0,
        3.0 + PI,
    );
    assert_eq!(
        trace,
        Ok(Trace::Segment {
            from: DVec2::new(3.0, 10.0),
            to: DVec2::new(3.0 + PI, 10.0),
        })
    );
    let whole = traced(
        &Curve::Circle(circle),
        &Surface::Cylinder(cylinder),
        0.0,
        TAU,
    );
    assert_eq!(
        whole,
        Ok(Trace::Segment {
            from: DVec2::new(0.0, 10.0),
            to: DVec2::new(TAU, 10.0),
        })
    );
}

#[test]
fn the_loops_of_a_face_are_traces_that_follow_each_other_modulo_a_turn() {
    let plate = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let hole = Contour::circle(DVec2::new(8.0, 0.0), 5.0);
    let body = Body::raised(&plate, &[hole], ground(), DVec3::Z * 10.0).expect("a plate raises");
    for face in body.face_ids() {
        let period = body.surface(body.face(face).surface).period();
        for lap in body.traces(face).expect("a raised body is traced") {
            for (index, trace) in lap.iter().enumerate() {
                let next = lap[(index + 1) % lap.len()];
                let mut gap = next.start() - trace.end();
                if let Some(period) = period {
                    gap.x -= (gap.x / period).round() * period;
                }
                assert!(gap.length() <= CLOSE, "face {face:?} jumps by {gap}");
            }
        }
    }
}
