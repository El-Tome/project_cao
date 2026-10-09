use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::surface::Cone;
use crate::turning::{Axis, Corner, Straight, Turn};

/// A profile of one contour laid on the side of X, its corners `(h, r)`,
/// turned by `degrees` about Y.
fn turned(corners: &[[f64; 2]], degrees: f64) -> Body {
    let contour = corners
        .iter()
        .enumerate()
        .map(|(run, at)| Corner {
            at: DVec2::from(*at),
            run: run as u32,
        })
        .collect();
    let straight = Straight {
        side: -1.0,
        contours: vec![contour],
        runs: corners.len() as u32,
        last_off_the_axis: Some(corners.len() as u32 - 1),
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle: degrees.to_radians(),
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(&straight, ground(), &turn).expect("the profile turns")
}

/// A point of radius 5, its apex 10 along Y.
const POINT: [[f64; 2]; 3] = [[0.0, 0.0], [10.0, 0.0], [0.0, 5.0]];

/// A point of radius 1, its apex 1000 along Y.
const NEEDLE: [[f64; 2]; 3] = [[0.0, 0.0], [1000.0, 0.0], [0.0, 1.0]];

fn on_the_cone(body: &Body) -> (FaceId, Cone) {
    body.face_ids()
        .find_map(|face| match body.surface(body.face(face).surface) {
            Surface::Cone(cone) => Some((face, *cone)),
            Surface::Plane(_) | Surface::Cylinder(_) => None,
        })
        .expect("a face lies on a cone")
}

/// An angle moved by whole turns to stand within half a turn of nought.
fn wrapped(angle: f64) -> f64 {
    angle - TAU * (angle / TAU).round()
}

/// How far a loop turns about the axis, net, from each trace to the next
/// the nearest way round.
fn turning(lap: &[Trace]) -> f64 {
    (0..lap.len())
        .map(|rank| {
            let next = lap[(rank + 1) % lap.len()].start().x;
            lap[rank].end().x - lap[rank].start().x + wrapped(next - lap[rank].end().x)
        })
        .sum()
}

/// The angles of the rulings bounding a face.
fn rulings(body: &Body, face: FaceId) -> Vec<f64> {
    body.traces(face)
        .expect("the face is traced")
        .iter()
        .flatten()
        .filter_map(|trace| match *trace {
            Trace::Segment { from, to } if from.x == to.x => Some(from.x),
            _ => None,
        })
        .collect()
}

#[test]
fn a_ruling_is_seen_at_its_own_angle_all_the_way_down_to_the_apex() {
    let cone = Cone::through(
        DVec3::ZERO,
        DVec3::Y,
        [DVec2::new(0.0, 5.0), DVec2::new(10.0, 0.0)],
    );
    for angle in [0.3, -2.0, PI] {
        let foot = cone.point(DVec2::new(angle, 0.0));
        let line = Line::through(foot, cone.apex() - foot);
        let [from, to] = [line.parameter(foot), line.parameter(cone.apex())];
        let trace = traced(&Curve::Line(line), &Surface::Cone(cone), from, to, EPS);
        let Ok(Trace::Segment {
            from: low,
            to: high,
        }) = trace
        else {
            panic!("a segment, not {trace:?}");
        };
        assert!(wrapped(low.x - angle).abs() <= CLOSE, "{low} at {angle}");
        assert_eq!(low.x, high.x, "the apex keeps the ruling's angle");
        assert!((high.y - cone.apex_at()).abs() <= CLOSE, "{high}");
        for u in [0.0, 0.3, 1.0] {
            let seen = cone.point(trace.unwrap().at(u)[0]);
            let point = line.point(from + (to - from) * u);
            assert!(seen.distance(point) <= CLOSE, "{seen} against {point}");
        }
    }
}

#[test]
fn a_circle_about_a_cone_s_axis_is_seen_at_its_rim_s_length_turning_the_way_its_axis_says() {
    let cone = Cone::through(
        DVec3::new(1.0, 2.0, 3.0),
        DVec3::new(0.0, 1.0, 1.0).normalize(),
        [DVec2::new(-4.0, 6.0), DVec2::new(3.0, 2.0)],
    );
    let circle = cone.circle(1.0, cone.section(1.0));
    let backwards = Circle {
        axis: -circle.axis,
        v: -circle.v,
        ..circle
    };
    for circle in [circle, backwards] {
        let trace = traced(&Curve::Circle(circle), &Surface::Cone(cone), 0.5, 2.0, EPS)
            .expect("a circle about the axis is a segment");
        assert!(matches!(trace, Trace::Segment { from, to } if from.y == to.y));
        for u in [0.0, 0.3, 1.0] {
            let seen = cone.point(trace.at(u)[0]);
            let point = circle.point(0.5 + 1.5 * u);
            assert!(seen.distance(point) <= CLOSE, "{seen} against {point}");
        }
    }
}

#[test]
fn a_loop_through_the_apex_is_closed_along_the_floor() {
    for degrees in [90.0, 270.0, -200.0] {
        let body = turned(&POINT, degrees);
        let (face, cone) = on_the_cone(&body);
        let open = body.traces(face).expect("the face is traced");
        let closed = body.closed_traces(face).expect("the face is traced");
        assert_eq!(open.len(), 1);
        let [lap] = &closed[..] else {
            panic!("one loop at {degrees}°, not {closed:?}");
        };
        assert_eq!(lap.len(), open[0].len() + 1, "one floor at {degrees}°");
        let floors: Vec<usize> = (0..lap.len())
            .filter(|rank| !open[0].contains(&lap[*rank]))
            .collect();
        let [floor] = floors[..] else {
            panic!("one floor at {degrees}°, not {lap:?}");
        };
        let Trace::Segment { from, to } = lap[floor] else {
            panic!("a floor segment at {degrees}°, not {:?}", lap[floor]);
        };
        for end in [from, to] {
            assert!(
                (end.y - cone.apex_at()).abs() <= body.scale().eps(),
                "{end}"
            );
        }
        let [before, after] = [floor + lap.len() - 1, floor + 1].map(|rank| lap[rank % lap.len()]);
        assert_eq!(from, before.end());
        assert!(wrapped(after.start().x - to.x).abs() <= CLOSE);
        assert!(
            turning(lap).abs() <= 1e-9,
            "{degrees}° turns {}",
            turning(lap)
        );
    }
}

#[test]
fn a_whole_point_is_closed_below_by_a_whole_floor_turning_against_its_rim() {
    let body = turned(&POINT, 360.0);
    let (face, cone) = on_the_cone(&body);
    let closed = body.closed_traces(face).expect("the face is traced");
    let [rim, floor] = &closed[..] else {
        panic!("the rim and the floor, not {closed:?}");
    };
    assert!((turning(rim).abs() - TAU).abs() <= 1e-9);
    assert!((turning(floor) + turning(rim)).abs() <= 1e-9);
    assert!(floor.iter().all(|trace| trace.start().y == cone.apex_at()));
}

#[test]
fn a_whole_point_has_a_point_inside_away_from_its_tip() {
    let body = turned(&POINT, 360.0);
    let (face, cone) = on_the_cone(&body);
    let at = body
        .point_inside(face)
        .expect("the face is traced")
        .expect("a whole point has an inside");
    assert_eq!(
        body.locate(face, at, body.scale().eps()),
        Ok(Location::Inside)
    );
    assert!(cone.point(at).distance(cone.apex()) > 1.0, "{at}");
}

#[test]
fn a_place_near_a_whole_point_s_tip_is_on_its_boundary_as_a_vertex_of_its_face() {
    let body = turned(&POINT, 360.0);
    let (face, cone) = on_the_cone(&body);
    let eps = body.scale().eps();
    let floor = cone.apex_at();
    let rim = cone.parameters(DVec3::X * 5.0).y;
    for angle in [0.3, -2.0, PI] {
        let locate = |l: f64| body.locate(face, DVec2::new(angle, l), eps);
        assert_eq!(locate(floor + eps / 2.0), Ok(Location::Boundary));
        assert_eq!(locate(floor + 3.0 * eps), Ok(Location::Inside));
        assert_eq!(locate(floor + 1.0), Ok(Location::Inside));
        assert_eq!(locate(rim + 1.0), Ok(Location::Outside));
    }
}

#[test]
fn a_place_near_a_partial_point_s_apex_is_on_its_boundary() {
    for degrees in [90.0, 270.0, -200.0] {
        let body = turned(&POINT, degrees);
        let (face, cone) = on_the_cone(&body);
        let eps = body.scale().eps();
        let floor = cone.apex_at();
        let inside = body
            .point_inside(face)
            .expect("the face is traced")
            .expect("a partial point has an inside");
        let locate = |angle: f64, l: f64| body.locate(face, DVec2::new(angle, l), eps);
        assert_eq!(locate(inside.x, floor + eps / 2.0), Ok(Location::Boundary));
        assert_eq!(locate(inside.x, floor + 1.0), Ok(Location::Inside));
        let [one, other] = rulings(&body, face)[..] else {
            panic!("two rulings bound the point at {degrees}°");
        };
        let span = (other - one).rem_euclid(TAU);
        let across = if (inside.x - one).rem_euclid(TAU) < span {
            other + (TAU - span) / 2.0
        } else {
            one + span / 2.0
        };
        assert_eq!(
            locate(across, floor + 1.0),
            Ok(Location::Outside),
            "{degrees}°"
        );
    }
}

#[test]
fn a_place_of_the_other_nappe_is_outside() {
    for degrees in [360.0, 90.0, 270.0] {
        let body = turned(&POINT, degrees);
        let (face, cone) = on_the_cone(&body);
        let eps = body.scale().eps();
        for angle in [0.3, -2.0, PI] {
            for beyond in [3.0 * eps, 1.0, 4.0] {
                let at = DVec2::new(angle, cone.apex_at() - beyond);
                assert_eq!(
                    body.locate(face, at, eps),
                    Ok(Location::Outside),
                    "{degrees}° at {at}"
                );
            }
        }
    }
}

#[test]
fn a_place_a_hair_from_a_ruling_of_a_slender_cone_is_on_its_boundary() {
    let body = turned(&NEEDLE, 90.0);
    let (face, cone) = on_the_cone(&body);
    let eps = body.scale().eps();
    let inside = body
        .point_inside(face)
        .expect("the face is traced")
        .expect("a partial needle has an inside");
    let rulings = rulings(&body, face);
    assert_eq!(rulings.len(), 2, "two rulings bound the needle");
    let l = cone.apex_at() / 2.0;
    let radius = cone.radius_at(l);
    for ruling in rulings {
        let inward = wrapped(inside.x - ruling).signum();
        let off = |hair: f64| DVec2::new(ruling + inward * hair * eps / radius, l);
        assert_eq!(body.locate(face, off(0.5), eps), Ok(Location::Boundary));
        assert_eq!(body.locate(face, off(3.0), eps), Ok(Location::Inside));
        assert_eq!(body.locate(face, off(-3.0), eps), Ok(Location::Outside));
    }
}
