//! What sketch · plane.rs is held to.
//!
//! Closes #358.
//! - a sketch on a face has its origin on a corner of it, the face spanning no
//!   negative size — `a_face_is_read_from_a_corner_and_spans_no_negative_size`
//! - the face reads the way up it was being looked at —
//!   `a_face_reads_the_way_up_it_is_looked_at`
//! - a sketch on one of the three base planes is unchanged —
//!   `the_planes_of_the_origin_come_back_as_they_are`
//! - the drawing's own axes are drawn at that origin — no test: emitted by
//!   `push_plane_axes` in `cao_render`, which holds the assertion for it
//! - a sketch reopened finds the axes it was created with — no test: the plane
//!   is stored on `CreateSketch` and read back, and nothing derives it again

use super::*;

const TOLERANCE: f64 = 1e-9;

/// The three planes of the origin are the case everything else is judged
/// against: their own corners and their own screen up have to give them
/// back exactly as they are written.
#[test]
fn the_planes_of_the_origin_come_back_as_they_are() {
    for (plane, up) in [
        (WorkPlane::XY, DVec3::Y),
        (WorkPlane::XZ, DVec3::Z),
        (WorkPlane::YZ, DVec3::Z),
    ] {
        let corners: Vec<DVec3> = [(0.0, 0.0), (50.0, 0.0), (50.0, 30.0), (0.0, 30.0)]
            .into_iter()
            .map(|(x, y)| plane.to_world(DVec2::new(x, y)))
            .collect();
        let read = WorkPlane::from_face(&corners, plane.normal(), up);

        assert!((read.u - plane.u).length() < TOLERANCE, "{read:?}");
        assert!((read.v - plane.v).length() < TOLERANCE, "{read:?}");
        assert!(
            (read.origin - plane.origin).length() < TOLERANCE,
            "{read:?}"
        );
    }
}

#[test]
fn a_face_is_read_from_a_corner_and_spans_no_negative_size() {
    let height = 40.0;
    let corners: Vec<DVec3> = [(10.0, 20.0), (70.0, 20.0), (70.0, 55.0), (10.0, 55.0)]
        .into_iter()
        .map(|(x, y)| DVec3::new(x, y, height))
        .collect();

    let plane = WorkPlane::from_face(&corners, DVec3::Z, DVec3::Y);

    assert!(
        corners.contains(&plane.origin),
        "the origin has to be a corner of the face, not {:?}",
        plane.origin,
    );
    for corner in &corners {
        let local = plane.to_local(*corner);
        assert!(
            local.x >= -TOLERANCE && local.y >= -TOLERANCE,
            "{corner:?} sits at {local:?}, behind the corner the face is read from",
        );
    }
}

#[test]
fn a_face_reads_the_way_up_it_is_looked_at() {
    let corners = [
        DVec3::new(0.0, 0.0, 5.0),
        DVec3::new(60.0, 0.0, 5.0),
        DVec3::new(60.0, 60.0, 5.0),
        DVec3::new(0.0, 60.0, 5.0),
    ];

    let upright = WorkPlane::from_face(&corners, DVec3::Z, DVec3::Y);
    let turned = WorkPlane::from_face(&corners, DVec3::Z, DVec3::X);

    assert!((upright.v - DVec3::Y).length() < TOLERANCE);
    assert!((turned.v - DVec3::X).length() < TOLERANCE);
    assert!(
        (turned.origin - DVec3::new(0.0, 60.0, 5.0)).length() < TOLERANCE,
        "turning the view a quarter turn moves the corner it is read from",
    );
}

/// Looking along the normal leaves nothing of the screen up to square the
/// axes with, and a face still has to come back with a usable plane.
#[test]
fn a_face_looked_at_along_its_own_normal_still_gets_its_axes() {
    let corners = [DVec3::ZERO, DVec3::X * 10.0, DVec3::new(10.0, 10.0, 0.0)];

    let plane = WorkPlane::from_face(&corners, DVec3::Z, DVec3::Z);

    assert!((plane.normal() - DVec3::Z).length() < 1e-9);
    assert!(plane.u.dot(plane.v).abs() < TOLERANCE);
}

#[test]
fn local_and_world_coordinates_round_trip() {
    for plane in WorkPlane::ORIGIN_PLANES {
        for point in [
            DVec2::ZERO,
            DVec2::new(12.5, -3.0),
            DVec2::new(-400.0, 900.0),
        ] {
            let round_trip = plane.to_local(plane.to_world(point));
            assert!((round_trip - point).length() < 1e-3, "{plane:?} {point:?}");
        }
    }
}

#[test]
fn a_plane_built_from_a_normal_keeps_that_normal() {
    for normal in [
        DVec3::Z,
        DVec3::X,
        DVec3::new(1.0, 1.0, 1.0),
        DVec3::new(-2.0, 0.5, 3.0),
    ] {
        let plane = WorkPlane::from_normal(DVec3::ZERO, normal);
        let expected = normal.normalize();
        assert!(
            (plane.normal() - expected).length() < 1e-4,
            "{normal:?} gave {:?}",
            plane.normal()
        );
        assert!((plane.u.length() - 1.0).abs() < 1e-4);
        assert!(plane.u.dot(plane.v).abs() < 1e-4);
    }
}

#[test]
fn a_ray_meets_the_plane_where_it_should() {
    let plane = WorkPlane::XY;
    let hit = plane
        .ray_intersection(DVec3::new(3.0, -2.0, 10.0), DVec3::NEG_Z)
        .expect("ray points at the plane");
    assert!((hit - DVec2::new(3.0, -2.0)).length() < 1e-4);

    assert!(plane.ray_intersection(DVec3::Z * 10.0, DVec3::Z).is_none());
    assert!(plane.ray_intersection(DVec3::Z * 10.0, DVec3::X).is_none());
}

#[test]
fn a_plane_is_recognised_by_where_it_sits_and_which_way_it_faces() {
    let through_origin = [
        (WorkPlane::XY, PlaneKind::OriginXY),
        (WorkPlane::XZ, PlaneKind::OriginXZ),
        (WorkPlane::YZ, PlaneKind::OriginYZ),
        (
            WorkPlane::from_normal(DVec3::ZERO, DVec3::new(1.0, 1.0, 0.0)),
            PlaneKind::OriginSlanted,
        ),
    ];

    for (plane, kind) in through_origin {
        assert_eq!(plane.kind(), kind, "{plane:?}");
    }
}

#[test]
fn a_plane_parallel_to_an_axis_plane_but_off_the_origin_is_not_that_plane() {
    let raised = WorkPlane::from_normal(DVec3::Z * 12.0, DVec3::Z);

    assert_eq!(raised.kind(), PlaneKind::OffOrigin);
}

#[test]
fn the_near_side_is_the_one_the_eye_stands_on() {
    let plane = WorkPlane::XY;

    let (above, offset) = plane.near_side(DVec3::new(0.0, 0.0, 12.0));

    assert!(above.distance(DVec3::Z) <= TOLERANCE, "got {above:?}");
    assert!(offset.abs() <= TOLERANCE);
}

#[test]
fn looking_from_under_a_plane_turns_its_near_side_over() {
    let plane = WorkPlane::XY;

    let (below, _) = plane.near_side(DVec3::new(0.0, 0.0, -12.0));

    assert!(below.distance(DVec3::NEG_Z) <= TOLERANCE, "got {below:?}");
}

#[test]
fn a_plane_away_from_the_origin_says_how_far_along_it_sits() {
    let plane = WorkPlane::from_normal(DVec3::new(0.0, 0.0, 4.0), DVec3::Z);

    let (towards, offset) = plane.near_side(DVec3::new(0.0, 0.0, 12.0));

    assert!((towards.dot(DVec3::Z) - 1.0).abs() <= TOLERANCE);
    assert!(
        (offset - 4.0).abs() <= TOLERANCE,
        "the plane stands four units along its own normal, got {offset}"
    );
}
