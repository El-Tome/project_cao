use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::curve::{Circle, Curve, Line, Meet};
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
    let trace = traced(&Curve::Line(line), &Surface::Plane(plane), from, to, EPS);
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
        let trace = traced(
            &Curve::Circle(circle),
            &Surface::Plane(plane),
            0.5,
            2.0,
            EPS,
        )
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
        traced(&circle, &Surface::Plane(plane), 0.0, 1.0, EPS),
        Err(Declined::Unsupported)
    );
}

#[test]
fn a_ruling_of_a_cylinder_is_a_vertical_segment() {
    let cylinder = Cylinder::about(DVec3::new(8.0, 0.0, 0.0), DVec3::Z, 5.0);
    let foot = cylinder.point(DVec2::new(2.5, 0.0));
    let line = Line::through(foot, -DVec3::Z);
    let [from, to] = [line.parameter(foot), line.parameter(foot + DVec3::Z * 10.0)];
    let trace = traced(
        &Curve::Line(line),
        &Surface::Cylinder(cylinder),
        from,
        to,
        EPS,
    );
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

/// Seed 290 of the campaign: the pair decided a touch and moved the second
/// cylinder onto it by less than the tolerance, which the curve carries. The
/// curve still lies on the cylinder the body stands on, and is seen there as
/// it is seen on its own.
#[test]
fn a_meet_whose_second_cylinder_was_moved_onto_a_touch_is_seen_on_the_cylinder_it_was_moved_from() {
    let first = Cylinder::about(DVec3::new(7.0, 7.0, 0.0), DVec3::Z, 4.5);
    let standing = Cylinder::about(DVec3::new(3.499_999_99, 0.0, 2.0), DVec3::Y, 1.0);
    let moved = Cylinder::about(DVec3::new(3.5, 0.0, 2.0), DVec3::Y, 1.0);
    let meet = Meet {
        first,
        second: moved,
        component: 0,
    };
    let tolerance = 2e-8;
    for (surface, on_first) in [(standing, false), (first, true)] {
        let trace = traced(
            &Curve::Meet(meet),
            &Surface::Cylinder(surface),
            0.5,
            2.0,
            tolerance,
        );
        assert_eq!(
            trace,
            Ok(Trace::Graph {
                meet,
                on_first,
                beside: None,
                from: 0.5,
                to: 2.0,
            })
        );
    }
}

/// The arc two crossing walls meet along, taken for the arc of a wall of
/// one radius a hair off one of them (decision 6), is seen in that wall's
/// own angles: read in the angles of the wall it runs round, it stood the
/// angle the two axes part by off its corners there, and a loop it closed
/// with the wall's own arcs could not be placed (seed 8501866).
#[test]
fn a_meet_taken_for_an_arc_of_a_wall_a_hair_off_its_own_is_seen_in_that_wall_s_angles() {
    let own = Cylinder::about(DVec3::new(0.0, 1.0, 3.0), DVec3::X, 3.0);
    let beside = Cylinder::about(DVec3::new(0.0, 1.000_01, 3.0), DVec3::X, 3.0);
    let across = Cylinder::about(DVec3::new(7.5, 2.5, 0.0), DVec3::Z, 1.5);
    let meet = Meet {
        first: across,
        second: own,
        component: 0,
    };
    let [from, to] = [3.0, 3.1];
    let trace = traced(
        &Curve::Meet(meet),
        &Surface::Cylinder(beside),
        from,
        to,
        EPS,
    )
    .expect("a trace");
    for share in [0.0, 0.5, 1.0] {
        let seen = trace.at(share)[0];
        let point = meet.point(from + (to - from) * share);
        let wanted = beside.parameters(point);
        assert!(
            (seen.x - wanted.x).abs() <= CLOSE && (seen.y - wanted.y).abs() <= CLOSE,
            "at {share}: seen at {seen:?}, standing at {wanted:?}"
        );
    }
    let [at, speed, _] = trace.at(0.5);
    let step = 1e-6;
    let ahead = trace.at(0.5 + step)[0];
    assert!(
        ((ahead - at) / step - speed).length() <= 1e-4 * speed.length(),
        "the speed {speed:?} against the step {:?}",
        (ahead - at) / step
    );
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
        EPS,
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
        EPS,
    );
    assert_eq!(
        whole,
        Ok(Trace::Segment {
            from: DVec2::new(0.0, 10.0),
            to: DVec2::new(TAU, 10.0),
        })
    );
}

const EPS: f64 = 1e-9;

/// Seed 1019570 of the campaign: the arc of a cap's circle from where it
/// touches a side to a corner a hair along it is one with the side's edge,
/// and the line kept for both lies on the circle's cylinder only within the
/// tolerance: it is seen there between its ends' parameters, as the arc was.
#[test]
fn a_line_lying_on_a_cylinder_across_its_axis_is_a_segment_between_its_ends_parameters() {
    let cylinder = Cylinder::about(DVec3::new(0.0, 4.499_999_98, 10.5), DVec3::X, 2.5);
    let side = Line::through(DVec3::new(4.0, 0.0, 8.0), DVec3::Y);
    let [from, to] = [4.499_999_98, 4.5];
    let trace = traced(
        &Curve::Line(side),
        &Surface::Cylinder(cylinder),
        from,
        to,
        EPS,
    );
    let arc = Circle::on(&cylinder, 4.0);
    let expected = [from, to].map(|y| cylinder.parameters(DVec3::new(4.0, y, 8.0)));
    assert_eq!(
        trace,
        Ok(Trace::Segment {
            from: expected[0],
            to: expected[1],
        })
    );
    let [start, end] = [from, to].map(|y| arc.parameter(DVec3::new(4.0, y, 8.0)));
    assert!((expected[1].x - expected[0].x - (end - start)).abs() < 1e-15);
}

/// Seed 1040082 of the campaign: two circles of one radius a tenth of a
/// micron apart, each on its own cylinder, run within the tolerance of each
/// other near where they cross, and the arc kept for both lies on the other
/// cylinder there: it is seen between the angles its ends stand at on that
/// cylinder, not at its own angles, which are the other's turned by the
/// offset over the radius.
#[test]
fn a_circle_off_a_cylinder_s_axis_is_seen_between_its_ends_angles_on_that_cylinder() {
    let own = Cylinder::about(DVec3::ZERO, DVec3::Z, 2.5);
    let other = Cylinder::about(DVec3::X * 1e-7, DVec3::Z, 2.5);
    let circle = Circle::on(&own, 3.0);
    for (from, to) in [(1.5, 1.6), (1.6, 1.5 + TAU), (-3.0, -3.1 + TAU)] {
        let trace = traced(
            &Curve::Circle(circle),
            &Surface::Cylinder(other),
            from,
            to,
            EPS,
        )
        .expect("a circle square to the axis");
        for (u, at) in [(0.0, from), (1.0, to)] {
            let expected = other.parameters(circle.point(at));
            let [point] = [trace.at(u)[0]];
            let turns = ((point.x - expected.x) / TAU).round();
            assert!(
                close(point - DVec2::new(turns * TAU, 0.0), expected),
                "{trace:?}"
            );
        }
        let Trace::Segment {
            from: start,
            to: end,
        } = trace
        else {
            panic!("a level segment: {trace:?}");
        };
        assert!(((end.x - start.x) - (to - from)).abs() < 1e-6, "{trace:?}");
    }
}

/// Seed 1014146 of the campaign: a stretch of a circle four tenths of a
/// micron long, taken for the arc of a post square to its axis between the
/// same two corners, is seen on the post between where its ends stand; a
/// quarter of the same circle, far from any chord, is declined.
#[test]
fn a_short_stretch_of_a_circle_square_to_a_cylinder_is_seen_along_its_chord() {
    let groove = Cylinder::about(DVec3::new(0.0, 255.0, 165.0), DVec3::X, 45.0);
    let circle = Circle::on(&groove, 60.0);
    let post = Cylinder::about(DVec3::new(3e-7, 300.0, 0.0), DVec3::Z, 75.0);
    let [from, to] = [-PI / 2.0 - 8.9e-9, -PI / 2.0];
    let trace = traced(
        &Curve::Circle(circle),
        &Surface::Cylinder(post),
        from,
        to,
        EPS * 400.0,
    );
    let [start, end] = [from, to].map(|at| post.parameters(circle.point(at)));
    assert_eq!(
        trace,
        Ok(Trace::Segment {
            from: start,
            to: end
        })
    );
    assert_eq!(
        traced(
            &Curve::Circle(circle),
            &Surface::Cylinder(post),
            0.0,
            PI / 2.0,
            EPS * 400.0
        ),
        Err(Declined::Unsupported)
    );
}

/// Seed 8550915 of the campaign: a ring's inner rim, on the plane touching a
/// disc's wall along a ruling, passes that ruling a hundred-thousandth
/// inside and crosses it twice. Taken for the arc of the
/// curve the ring's wall meets the disc's along between those two crossings,
/// the rim lies on the disc's wall, where it bulges off its chord by the
/// whole hair: the wall sees it as that curve, between where its ends stand.
#[test]
fn a_rim_bulging_off_its_chord_on_a_square_wall_is_seen_as_the_curve_its_own_wall_meets_that_one_along()
 {
    let ring = Cylinder::about(DVec3::new(10.0, 0.0, 3.0), DVec3::Y, 4.0);
    let rim = Circle::on(&ring, 6.0);
    let disc = Cylinder::about(DVec3::new(6.000_01, 3.0, 0.0), DVec3::Z, 3.0);
    let crossings = (DVec3::new(6.000_01, 6.0, 3.0) - rim.center).normalize();
    let across = (3.999_99_f64 / 4.0).acos();
    let towards = rim.parameter(rim.center + crossings);
    let [from, to] = [towards - across, towards + across];
    let trace = traced(&Curve::Circle(rim), &Surface::Cylinder(disc), from, to, EPS)
        .expect("the rim is seen on the disc's wall");
    assert!(matches!(
        trace,
        Trace::Graph {
            on_first: false,
            ..
        }
    ));
    for (at, end) in [(0.0, from), (1.0, to)] {
        let seen = disc.point(trace.at(at)[0]);
        assert!(seen.distance(rim.point(end)) <= EPS, "an end stands off");
    }
    let middle = disc.point(trace.at(0.5)[0]);
    let on_the_rim = rim.point(rim.parameter(middle));
    assert!(
        middle.distance(on_the_rim) <= EPS,
        "the middle stands {} off the rim",
        middle.distance(on_the_rim)
    );
    assert!(
        middle.x < 6.000_005,
        "the middle stands on the ruling, at {middle}"
    );
}

/// Seed 8552226 of the campaign: two discs of one radius a
/// hundred-thousandth apart, and a hole square to them whose rim grazes the
/// ruling their caps' plane touches them along. The curve the hole meets the
/// first disc along is taken for the one it meets the second along, and lies
/// on the second's wall: the wall sees it as its own curve with the hole,
/// not in the first's angles, which stand a hair round from its own.
#[test]
fn a_meet_lying_on_a_parallel_wall_a_hair_off_its_own_is_seen_as_the_curve_that_wall_meets_the_other_along()
 {
    let first = Cylinder::about(DVec3::new(2.0, 0.0, 4.5), DVec3::Y, 3.0);
    let hole = Cylinder::about(DVec3::new(0.0, 9.0, 0.0), DVec3::Z, 2.0);
    let second = Cylinder::about(DVec3::new(1.999_99, 0.0, 4.5), DVec3::Y, 3.0);
    let meet = Meet {
        first,
        second: hole,
        component: 0,
    };
    let [from, to] = [3.139_010_664, 3.144_174_643];
    let trace = traced(
        &Curve::Meet(meet),
        &Surface::Cylinder(second),
        from,
        to,
        EPS,
    )
    .expect("the curve is seen on the second wall");
    for (at, end) in [(0.0, from), (1.0, to)] {
        let seen = second.point(trace.at(at)[0]);
        assert!(
            seen.distance(meet.point(end)) <= EPS,
            "an end stands {} off",
            seen.distance(meet.point(end))
        );
    }
    let middle = second.point(trace.at(0.5)[0]);
    let on_the_curve = meet.point(meet.parameter(middle));
    assert!(
        middle.distance(on_the_curve) <= EPS,
        "the middle stands {} off the curve",
        middle.distance(on_the_curve)
    );
}

fn top_of(body: &Body) -> FaceId {
    body.face_ids()
        .find(|face| {
            let face = body.face(*face);
            matches!(body.surface(face.surface), Surface::Plane(plane) if plane.normal.z > 0.5 && plane.offset() > 0.0)
        })
        .expect("a raised body has a top")
}

fn assert_located(body: &Body, face: FaceId, cases: &[(DVec2, Location)]) {
    for (at, expected) in cases {
        let found = body
            .locate(face, *at, EPS)
            .expect("a raised face is traced");
        assert_eq!(found, *expected, "{at} on face {face:?}");
    }
}

#[test]
fn a_point_of_a_square_with_a_square_hole_is_inside_outside_or_on_it() {
    let plate = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let hole = Contour::rectangle(DVec2::new(-5.0, -5.0), DVec2::new(5.0, 5.0));
    let body = Body::raised(&plate, &[hole], ground(), DVec3::Z * 10.0).expect("a plate raises");
    let top = top_of(&body);
    let near = 2.0 * EPS;
    assert_located(
        &body,
        top,
        &[
            (DVec2::new(10.0, 3.0), Location::Inside),
            (DVec2::new(0.0, 0.0), Location::Outside),
            (DVec2::new(25.0, 0.0), Location::Outside),
            (DVec2::new(0.0, -12.0), Location::Inside),
            (DVec2::new(5.0, 5.0), Location::Boundary),
            (DVec2::new(5.0, 0.5 * EPS), Location::Boundary),
            (DVec2::new(20.0, 3.0), Location::Boundary),
            (DVec2::new(20.0 - near, 3.0), Location::Inside),
            (DVec2::new(20.0 + near, 3.0), Location::Outside),
            (DVec2::new(5.0 + near, 5.0), Location::Inside),
            (DVec2::new(5.0 - near, 0.0), Location::Outside),
            (DVec2::new(-5.0, -20.0 + near), Location::Inside),
            (DVec2::new(5.0, 20.0 + near), Location::Outside),
        ],
    );
}

#[test]
fn a_point_of_a_disc_or_an_annulus_is_inside_outside_or_on_it() {
    let disc = Contour::circle(DVec2::new(8.0, 0.0), 5.0);
    let solid = Body::raised(&disc, &[], ground(), DVec3::Z * 10.0).expect("a disc raises");
    let outer = Contour::circle(DVec2::ZERO, 20.0);
    let ring = Body::raised(
        &outer,
        std::slice::from_ref(&disc),
        ground(),
        DVec3::Z * 10.0,
    )
    .expect("an annulus raises");
    let near = 2.0 * EPS;
    let shared = [
        (DVec2::new(13.0, 0.0), Location::Boundary),
        (DVec2::new(8.0, 5.0 - 0.5 * EPS), Location::Boundary),
        (DVec2::new(8.0, -5.0), Location::Boundary),
        (DVec2::new(8.0 + 3.0, 4.0), Location::Boundary),
    ];
    assert_located(&solid, top_of(&solid), &shared);
    assert_located(&ring, top_of(&ring), &shared);
    assert_located(
        &solid,
        top_of(&solid),
        &[
            (DVec2::new(8.0, 0.0), Location::Inside),
            (DVec2::new(3.0 + near, 0.0), Location::Inside),
            (DVec2::new(3.0 - near, 0.0), Location::Outside),
            (DVec2::new(8.0, 5.0 + near), Location::Outside),
            (DVec2::new(8.0, -5.0 + near), Location::Inside),
            (DVec2::new(13.0, 0.1), Location::Outside),
            (DVec2::new(0.0, 0.0), Location::Outside),
        ],
    );
    assert_located(
        &ring,
        top_of(&ring),
        &[
            (DVec2::new(8.0, 0.0), Location::Outside),
            (DVec2::new(3.0 + near, 0.0), Location::Outside),
            (DVec2::new(3.0 - near, 0.0), Location::Inside),
            (DVec2::new(8.0, 5.0 + near), Location::Inside),
            (DVec2::new(8.0, 15.0), Location::Inside),
            (DVec2::new(8.0, 19.0), Location::Outside),
            (DVec2::new(0.0, 20.0 + near), Location::Outside),
            (DVec2::new(0.0, -20.0 + near), Location::Inside),
            (DVec2::new(-20.0, 0.0), Location::Boundary),
            (DVec2::new(20.0 + near, 0.0), Location::Outside),
        ],
    );
}

fn walls_of(body: &Body) -> Vec<FaceId> {
    body.face_ids()
        .filter(|face| matches!(body.surface(body.face(*face).surface), Surface::Cylinder(_)))
        .collect()
}

#[test]
fn a_point_of_a_band_is_inside_between_its_rings_all_the_way_round() {
    let disc = Contour::circle(DVec2::new(8.0, 0.0), 5.0);
    let body = Body::raised(&disc, &[], ground(), DVec3::Z * 10.0).expect("a disc raises");
    let [band] = walls_of(&body)[..] else {
        panic!("a disc has one wall");
    };
    let near = 2.0 * EPS;
    let mut cases = Vec::new();
    for theta in [-PI, -3.1, -1.0, 0.0, 1e-17, 2.0, PI, 3.1, 7.0, -12.0] {
        cases.extend([
            (DVec2::new(theta, 5.0), Location::Inside),
            (DVec2::new(theta, near), Location::Inside),
            (DVec2::new(theta, -near), Location::Outside),
            (DVec2::new(theta, 10.0 + near), Location::Outside),
            (DVec2::new(theta, 10.0 - 0.5 * EPS), Location::Boundary),
            (DVec2::new(theta, 0.0), Location::Boundary),
            (DVec2::new(theta, 25.0), Location::Outside),
        ]);
    }
    assert_located(&body, band, &cases);
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
fn a_point_of_a_half_band_is_inside_between_its_rulings_even_across_a_half_turn() {
    let body = Body::raised(&slot(), &[], ground(), DVec3::Z * 10.0).expect("a slot raises");
    let walls = walls_of(&body);
    let right = walls
        .iter()
        .copied()
        .find(|face| {
            let Surface::Cylinder(cylinder) = body.surface(body.face(*face).surface) else {
                return false;
            };
            cylinder.origin.x > 0.0
        })
        .expect("the slot has a right end");
    let left = walls
        .into_iter()
        .find(|face| *face != right)
        .expect("and a left one");
    let across = 2.0 * EPS / 5.0;
    let half = std::f64::consts::FRAC_PI_2;
    assert_located(
        &body,
        right,
        &[
            (DVec2::new(0.0, 5.0), Location::Inside),
            (DVec2::new(PI, 5.0), Location::Outside),
            (DVec2::new(half - across, 5.0), Location::Inside),
            (DVec2::new(half + across, 5.0), Location::Outside),
            (DVec2::new(-half + across, 5.0), Location::Inside),
            (DVec2::new(-half - across, 5.0), Location::Outside),
            (DVec2::new(half, 5.0), Location::Boundary),
            (DVec2::new(0.0, 11.0), Location::Outside),
            (DVec2::new(TAU, 5.0), Location::Inside),
        ],
    );
    assert_located(
        &body,
        left,
        &[
            (DVec2::new(PI, 5.0), Location::Inside),
            (DVec2::new(-PI, 5.0), Location::Inside),
            (DVec2::new(-3.0, 5.0), Location::Inside),
            (DVec2::new(3.0, 5.0), Location::Inside),
            (DVec2::new(0.0, 5.0), Location::Outside),
            (DVec2::new(half + across, 5.0), Location::Inside),
            (DVec2::new(half - across, 5.0), Location::Outside),
            (DVec2::new(-half - across, 5.0), Location::Inside),
            (DVec2::new(-half + across, 5.0), Location::Outside),
            (DVec2::new(-half, 5.0), Location::Boundary),
            (DVec2::new(PI, -1.0), Location::Outside),
        ],
    );
}

/// A cylinder's wall that goes all the way round from the ruling at `seam`,
/// its two circles running a whole turn from that ruling back to it.
fn band_cut_open(seam: f64) -> Body {
    use crate::brep::scale::Scale;
    use crate::brep::topology::{Coedge, CurveId, Edge, EdgeId, Face, SurfaceId, Vertex, VertexId};
    let cylinder = Cylinder::about(DVec3::ZERO, DVec3::Z, 5.0);
    let [bottom, top] = [0.0, 10.0].map(|height| cylinder.point(DVec2::new(seam, height)));
    let ruling = Line::through(bottom, DVec3::Z);
    let whole = |curve: u32| Edge {
        curve: CurveId(curve),
        ends: Some([VertexId(curve), VertexId(curve)]),
        from: seam,
        to: seam + TAU,
    };
    let edges = vec![
        whole(0),
        whole(1),
        Edge {
            curve: CurveId(2),
            ends: Some([VertexId(0), VertexId(1)]),
            from: ruling.parameter(bottom),
            to: ruling.parameter(top),
        },
    ];
    let used = |edge: u32, forward: bool| Coedge {
        edge: EdgeId(edge),
        forward,
    };
    Body {
        surfaces: vec![Surface::Cylinder(cylinder)],
        curves: vec![
            Curve::Circle(Circle::on(&cylinder, 0.0)),
            Curve::Circle(Circle::on(&cylinder, 10.0)),
            Curve::Line(ruling),
        ],
        vertices: [bottom, top]
            .map(|point| Vertex {
                point,
                on: vec![SurfaceId(0)],
            })
            .to_vec(),
        edges,
        faces: vec![Face {
            surface: SurfaceId(0),
            flipped: false,
            loops: vec![vec![
                used(0, true),
                used(2, true),
                used(1, false),
                used(2, false),
            ]],
            numbers: Vec::new(),
        }],
        scale: Scale::of(10.0),
        arrivals: Vec::new(),
    }
}

#[test]
fn a_point_of_a_band_cut_open_by_a_ruling_is_inside_on_both_sides_of_it() {
    let across = 2.0 * EPS / 5.0;
    for seam in [0.0, 3.0, -PI, 1.0] {
        let body = band_cut_open(seam);
        let face = FaceId(0);
        let mut cases = vec![
            (DVec2::new(seam, 5.0), Location::Boundary),
            (DVec2::new(seam + TAU, 5.0), Location::Boundary),
            (DVec2::new(seam + across, 5.0), Location::Inside),
            (DVec2::new(seam - across, 5.0), Location::Inside),
            (DVec2::new(seam, 11.0), Location::Outside),
            (DVec2::new(seam + across, -2.0 * EPS), Location::Outside),
            (
                DVec2::new(seam - across, 10.0 + 2.0 * EPS),
                Location::Outside,
            ),
        ];
        for theta in [-3.0, -1.0, 0.5, 2.0, 3.1] {
            cases.push((DVec2::new(theta, 2.0), Location::Inside));
            cases.push((DVec2::new(theta, 12.0), Location::Outside));
        }
        assert_located(&body, face, &cases);
    }
}

/// Frames leaning or not, travels either way, and profiles with every kind of
/// wall: what a raised body can be.
fn raised_bodies() -> Vec<Body> {
    let leaning = Frame {
        origin: DVec3::new(3.0, -2.0, 5.0),
        u: DVec3::new(1.0, 1.0, 0.0) / 2f64.sqrt(),
        v: DVec3::new(-1.0, 1.0, 2f64.sqrt()) / 2.0,
    };
    let plate = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let hole = Contour::circle(DVec2::new(8.0, 0.0), 5.0);
    let square = Contour::rectangle(DVec2::new(-15.0, -15.0), DVec2::new(-5.0, -5.0));
    let profiles: [(Contour, Vec<Contour>); 3] = [
        (plate, vec![hole.clone(), square]),
        (slot(), Vec::new()),
        (Contour::circle(DVec2::new(8.0, 0.0), 5.0), Vec::new()),
    ];
    let mut bodies = Vec::new();
    for frame in [ground(), leaning] {
        for height in [10.0, -7.0] {
            for (outline, holes) in &profiles {
                bodies.push(
                    Body::raised(outline, holes, frame, frame.normal() * height)
                        .expect("a profile raises"),
                );
            }
        }
    }
    bodies
}

#[test]
fn a_raised_face_lies_on_the_left_of_each_of_its_loops_seen_from_outside() {
    const STEP: f64 = 1e-3;
    for body in raised_bodies() {
        for face in body.face_ids() {
            let flipped = body.face(face).flipped;
            for trace in body
                .traces(face)
                .expect("a raised face is traced")
                .iter()
                .flatten()
            {
                let [middle, pace, _] = trace.at(0.5);
                let left = pace.normalize().perp();
                let inward = if flipped { -left } else { left };
                assert_located(
                    &body,
                    face,
                    &[
                        (middle + inward * STEP, Location::Inside),
                        (middle - inward * STEP, Location::Outside),
                    ],
                );
            }
        }
    }
}

/// How far a point of a face stands from its nearest edge, on the surface.
fn clearance(body: &Body, face: FaceId, at: DVec2) -> f64 {
    let radius = match body.surface(body.face(face).surface) {
        Surface::Cylinder(cylinder) => Some(cylinder.radius),
        Surface::Plane(_) => None,
    };
    body.traces(face)
        .expect("a raised face is traced")
        .iter()
        .flatten()
        .map(|trace| distance::distance(trace, at, radius))
        .fold(f64::INFINITY, f64::min)
}

#[test]
fn the_point_inside_a_face_is_inside_and_well_away_from_its_edges() {
    let mut bodies = raised_bodies();
    bodies.extend([0.0, 3.0, -PI].map(band_cut_open));
    for body in bodies {
        for face in body.face_ids() {
            let at = body
                .point_inside(face)
                .expect("a raised face is traced")
                .expect("a face has an inside");
            assert_located(&body, face, &[(at, Location::Inside)]);
            let clear = clearance(&body, face, at);
            assert!(
                clear >= 2.5,
                "{at} on face {face:?} is {clear} from an edge"
            );
        }
    }
}

#[test]
fn the_sampled_cuts_heights_and_distances_agree_with_those_of_a_round() {
    let round = Trace::Round {
        center: DVec2::new(8.0, -1.0),
        radius: 5.0,
        start: 2.5,
        sweep: -4.5,
    };
    let exact: Vec<f64> = crossing::stretches(&round, false)
        .iter()
        .skip(1)
        .map(|stretch| stretch.from)
        .collect();
    let sampled = crossing::sampled(&round, false);
    assert_eq!(exact.len(), sampled.len(), "{exact:?} against {sampled:?}");
    for (exact, sampled) in exact.iter().zip(&sampled) {
        assert!(
            (exact - sampled).abs() <= CLOSE,
            "{exact} against {sampled}"
        );
    }
    for stretch in crossing::stretches(&round, false) {
        let (start, end) = (stretch.start(), stretch.end());
        let target = start + (end - start) * 0.3;
        let heights = crossing::heights(&[vec![round]], target, false);
        let bisected = crossing::bisected(&stretch, target);
        assert!(
            heights
                .iter()
                .any(|height| (height - bisected).abs() <= 1e-9),
            "{bisected} is none of {heights:?}"
        );
    }
    for at in [
        DVec2::new(8.0, 3.0),
        DVec2::new(14.0, -2.0),
        DVec2::new(0.0, 0.0),
    ] {
        let exact = distance::distance(&round, at, None);
        let sampled = distance::sampled(&round, at, None);
        assert!((exact - sampled).abs() <= 1e-9, "{exact} against {sampled}");
    }
}

#[test]
fn a_trace_sampled_on_a_cylinder_is_cut_every_quarter_turn() {
    let level = Trace::Segment {
        from: DVec2::new(3.0, 2.0),
        to: DVec2::new(3.0 - TAU, 2.0),
    };
    let cuts = crossing::sampled(&level, true);
    assert_eq!(cuts.len(), 3);
    for (index, cut) in cuts.iter().enumerate() {
        assert!((cut - (index + 1) as f64 / 4.0).abs() <= CLOSE, "{cuts:?}");
    }
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

#[test]
fn a_point_near_where_a_hole_touches_its_disc_from_inside_is_told_apart() {
    let stock = Contour::circle(DVec2::ZERO, 20.0);
    let hole = Contour::circle(DVec2::new(15.0, 0.0), 5.0);
    let body = Body::raised(&stock, &[hole], ground(), DVec3::Z * 10.0).expect("a bored disc");
    let mut cases = vec![
        (DVec2::new(19.9999, 0.0), Location::Outside),
        (DVec2::new(19.9999, 0.05), Location::Inside),
        (DVec2::new(19.9999, -0.05), Location::Inside),
        (DVec2::new(20.0, 0.0), Location::Boundary),
        (DVec2::new(20.0 - 1e-12, 0.0), Location::Boundary),
        (DVec2::new(20.0 + 2.0 * EPS * 20.0, 0.0), Location::Outside),
        (DVec2::new(15.0, 0.0), Location::Outside),
        (DVec2::new(-19.0, 0.0), Location::Inside),
    ];
    for x in [10.5, 12.0, 17.0, 19.0, 19.99] {
        let inner = (25.0f64 - (x - 15.0) * (x - 15.0)).sqrt();
        let outer = (400.0f64 - x * x).sqrt();
        cases.extend([
            (DVec2::new(x, (inner + outer) / 2.0), Location::Inside),
            (DVec2::new(x, -(inner + outer) / 2.0), Location::Inside),
            (DVec2::new(x, inner / 2.0), Location::Outside),
        ]);
    }
    assert_located(&body, top_of(&body), &cases);
    for face in body.face_ids() {
        let at = body
            .point_inside(face)
            .expect("a raised face is traced")
            .expect("a face has an inside");
        assert_located(&body, face, &[(at, Location::Inside)]);
    }
    let volume = (400.0 - 25.0) * PI * 10.0;
    assert!((body.volume() - volume).abs() <= 1e-12 * volume);
}

/// Three quarters of a disc, as an outline and as a hole in a plate.
fn three_quarters(shift: DVec2) -> Contour {
    use crate::profile::Run;
    Contour {
        corners: vec![
            shift + DVec2::new(5.0, 0.0),
            shift + DVec2::new(0.0, -5.0),
            shift,
        ],
        runs: vec![
            Run::Round {
                center: shift,
                turn: 1.5 * PI,
            },
            Run::Straight,
            Run::Straight,
        ],
    }
}

#[test]
fn a_wall_turning_three_quarters_round_keeps_its_matter_on_the_left_and_its_volume() {
    let leaning = Frame {
        origin: DVec3::new(3.0, -2.0, 5.0),
        u: DVec3::new(1.0, 1.0, 0.0) / 2f64.sqrt(),
        v: DVec3::new(-1.0, 1.0, 2f64.sqrt()) / 2.0,
    };
    let plate = Contour::rectangle(DVec2::new(-30.0, -30.0), DVec2::new(30.0, 30.0));
    let quarter = 0.75 * PI * 25.0;
    let holed = [three_quarters(DVec2::new(-15.0, -15.0))];
    let cases: [(&Contour, &[Contour], f64); 2] = [
        (&three_quarters(DVec2::ZERO), &[], quarter),
        (&plate, &holed, 3600.0 - quarter),
    ];
    for frame in [ground(), leaning] {
        for height in [10.0, -7.0] {
            for (outline, holes, area) in cases {
                let body = Body::raised(outline, holes, frame, frame.normal() * height)
                    .expect("it raises");
                let volume = area * height.abs();
                assert!((body.volume() - volume).abs() <= 1e-12 * volume);
                for face in body.face_ids() {
                    let flipped = body.face(face).flipped;
                    for trace in body.traces(face).expect("traced").iter().flatten() {
                        for share in [0.1, 0.5, 0.9] {
                            let [middle, pace, _] = trace.at(share);
                            let left = pace.normalize().perp() * 1e-3;
                            let inward = if flipped { -left } else { left };
                            assert_located(
                                &body,
                                face,
                                &[
                                    (middle + inward, Location::Inside),
                                    (middle - inward, Location::Outside),
                                ],
                            );
                        }
                    }
                }
            }
        }
    }
}
