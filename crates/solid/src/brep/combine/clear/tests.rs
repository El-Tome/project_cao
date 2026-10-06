use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::*;
use crate::profile::{Contour, Frame, Run};

const TOLERANCE: f64 = 1e-9;

/// A whole cylinder about the line through `origin` along `axis`, from there
/// to `length` along it.
fn rod(origin: DVec3, axis: DVec3, radius: f64, length: f64) -> Body {
    let axis = axis.normalize();
    let u = axis.any_orthonormal_vector();
    let frame = Frame {
        origin,
        u,
        v: axis.cross(u),
    };
    let outline = Contour {
        corners: vec![DVec2::X * radius],
        runs: vec![Run::Round {
            center: DVec2::ZERO,
            turn: TAU,
        }],
    };
    Body::raised(&outline, &[], frame, axis * length).expect("a rod raises")
}

/// A square plate lying in the plane through `origin` square to `normal`.
fn plate(origin: DVec3, normal: DVec3, half: f64) -> Body {
    let normal = normal.normalize();
    let u = normal.any_orthonormal_vector();
    let frame = Frame {
        origin,
        u,
        v: normal.cross(u),
    };
    let outline = Contour::rectangle(DVec2::splat(-half), DVec2::splat(half));
    Body::raised(&outline, &[], frame, normal).expect("a plate raises")
}

fn wall(body: &Body) -> FaceId {
    body.face_ids()
        .find(|&face| matches!(body.surface(body.face(face).surface), Surface::Cylinder(_)))
        .expect("a wall")
}

/// The face of a body lying on a plane whose normal is `normal`, either way.
fn flat(body: &Body, normal: DVec3) -> FaceId {
    body.face_ids()
        .find(|&face| match body.surface(body.face(face).surface) {
            Surface::Plane(plane) => plane.normal.cross(normal.normalize()).length() < TOLERANCE,
            Surface::Cylinder(_) | Surface::Cone(_) => false,
        })
        .expect("a flat face")
}

#[test]
fn a_disc_reaches_along_a_slanted_direction_as_far_as_its_rim_turns() {
    let disc = rod(DVec3::ZERO, DVec3::Z, 2.0, 1.0);
    let direction = DVec3::new(1.0, 0.0, 1.0).normalize();
    let [low, high] = reach((&disc, flat(&disc, DVec3::Z)), direction).expect("a reach");
    let expected = 2.0 * direction.x;
    assert!(
        (low + expected).abs() < TOLERANCE && (high - expected).abs() < TOLERANCE,
        "the floor reaches from {low} to {high}"
    );
}

#[test]
fn a_wall_reaches_along_its_axis_from_its_floor_to_its_top() {
    let axis = DVec3::new(1.0, 2.0, 2.0).normalize();
    let body = rod(axis * 3.0, axis, 1.5, 4.0);
    let [low, high] = reach((&body, wall(&body)), axis).expect("a reach");
    assert!(
        (low - 3.0).abs() < TOLERANCE && (high - 7.0).abs() < TOLERANCE,
        "the wall reaches from {low} to {high}"
    );
}

#[test]
fn two_walls_whose_axes_cross_beyond_their_ends_stand_clear() {
    let one = rod(DVec3::new(5.0, 0.0, 0.0), DVec3::X, 1.0, 10.0);
    let other = rod(
        DVec3::new(0.0, 3.0, 0.0),
        DVec3::new(-1.0, 1.0, 0.3),
        1.0,
        10.0,
    );
    assert!(apart((&one, wall(&one)), (&other, wall(&other)), 1e-8));
}

#[test]
fn two_walls_at_a_skew_angle_crossing_each_other_do_not_stand_clear() {
    let one = rod(DVec3::new(-5.0, 0.0, 0.0), DVec3::X, 1.0, 10.0);
    let other = rod(
        DVec3::new(0.0, -5.0, 0.5),
        DVec3::new(0.3, 1.0, 0.0),
        1.0,
        10.0,
    );
    assert!(!apart((&one, wall(&one)), (&other, wall(&other)), 1e-8));
}

#[test]
fn a_wall_ending_short_of_a_slanted_plane_stands_clear_of_it() {
    let body = rod(DVec3::ZERO, DVec3::Z, 1.0, 3.0);
    let normal = DVec3::new(1.0, 0.0, 1.0);
    let slab = plate(DVec3::Z * 6.0, normal, 20.0);
    assert!(apart(
        (&body, wall(&body)),
        (&slab, flat(&slab, normal)),
        1e-8
    ));
}

#[test]
fn a_wall_passing_through_a_slanted_plane_does_not_stand_clear_of_it() {
    let body = rod(DVec3::ZERO, DVec3::Z, 1.0, 8.0);
    let normal = DVec3::new(1.0, 0.0, 1.0);
    let slab = plate(DVec3::Z * 6.0, normal, 20.0);
    assert!(!apart(
        (&body, wall(&body)),
        (&slab, flat(&slab, normal)),
        1e-8
    ));
}

#[test]
fn a_wall_beside_a_small_slanted_plate_its_plane_crosses_stands_clear_of_it() {
    let body = rod(DVec3::ZERO, DVec3::Z, 1.0, 8.0);
    let normal = DVec3::new(1.0, 0.0, 1.0);
    let slab = plate(DVec3::new(0.0, 6.0, 4.0), normal, 2.0);
    assert!(apart(
        (&body, wall(&body)),
        (&slab, flat(&slab, normal)),
        1e-8
    ));
}

#[test]
fn two_stretches_side_by_side_stand_as_far_apart_as_their_lines() {
    let distance = between_stretches(
        [DVec3::ZERO, DVec3::X * 4.0],
        [DVec3::new(1.0, 2.0, 0.0), DVec3::new(3.0, 2.0, 0.0)],
    );
    assert!((distance - 2.0).abs() < TOLERANCE, "{distance}");
}

#[test]
fn two_stretches_whose_lines_cross_beyond_one_end_stand_as_far_as_that_end() {
    let distance = between_stretches(
        [DVec3::new(2.0, 0.0, 0.0), DVec3::new(5.0, 0.0, 0.0)],
        [DVec3::new(0.0, -1.0, 0.0), DVec3::new(0.0, 1.0, 0.0)],
    );
    assert!((distance - 2.0).abs() < TOLERANCE, "{distance}");
}
