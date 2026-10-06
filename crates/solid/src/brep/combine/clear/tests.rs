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

/// The body a profile of corners `(h, r)` turns into about the line through
/// `origin` along `axis`, turned by `angle`.
fn turned(origin: DVec3, axis: DVec3, corners: &[[f64; 2]], angle: f64) -> Body {
    use crate::turning::{Axis, Corner, Straight, Turn};
    let axis = axis.normalize();
    let frame = Frame {
        origin,
        u: axis.any_orthonormal_vector(),
        v: axis,
    };
    let straight = Straight {
        side: -1.0,
        contours: vec![
            corners
                .iter()
                .enumerate()
                .map(|(run, at)| Corner {
                    at: DVec2::from(*at),
                    run: run as u32,
                })
                .collect(),
        ],
        runs: corners.len() as u32,
        last_off_the_axis: Some(corners.len() as u32 - 1),
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(&straight, frame, &turn).expect("the profile turns")
}

fn nappe(body: &Body) -> FaceId {
    body.face_ids()
        .find(|&face| matches!(body.surface(body.face(face).surface), Surface::Cone(_)))
        .expect("a face on a cone")
}

/// A countersink about Z: a cone from radius 2 at the plate's floor, `z =
/// 0`, to radius 7 at its top, `z = 5`, as the tool cutting it.
fn countersink() -> Body {
    turned(
        DVec3::ZERO,
        DVec3::Z,
        &[[0.0, 0.0], [0.0, 2.0], [5.0, 7.0], [5.0, 0.0]],
        TAU,
    )
}

#[test]
fn a_cone_far_from_an_oblique_face_stands_clear() {
    let tool = countersink();
    let normal = DVec3::new(1.0, 0.0, 1.0);
    for slab in [
        plate(DVec3::Z * 15.0, normal, 20.0),
        plate(DVec3::new(0.0, 12.0, 2.0), normal, 3.0),
    ] {
        assert!(apart(
            (&tool, nappe(&tool)),
            (&slab, flat(&slab, normal)),
            1e-8
        ));
    }
}

#[test]
fn a_cone_crossing_an_oblique_face_does_not_stand_clear() {
    let tool = countersink();
    let normal = DVec3::new(1.0, 0.0, 1.0);
    let slab = plate(DVec3::Z * 2.5, normal, 20.0);
    assert!(!apart(
        (&tool, nappe(&tool)),
        (&slab, flat(&slab, normal)),
        1e-8
    ));
}

#[test]
fn a_bolt_hole_beside_a_countersink_stands_clear() {
    let tool = countersink();
    for (at, expected) in [(12.0, true), (9.5, true), (8.5, false), (6.0, false)] {
        let bolt = rod(DVec3::new(at, 0.0, -5.0), DVec3::Z, 2.0, 15.0);
        assert_eq!(
            apart((&tool, nappe(&tool)), (&bolt, wall(&bolt)), 1e-8),
            expected,
            "{at}"
        );
    }
}

#[test]
fn a_bore_inside_a_countersink_s_narrow_end_stands_clear() {
    let tool = countersink();
    for (at, radius, expected) in [(0.5, 1.0, true), (0.0, 1.9, true), (0.5, 1.6, false)] {
        let bore = rod(DVec3::new(0.0, at, -5.0), DVec3::Z, radius, 15.0);
        assert_eq!(
            apart((&tool, nappe(&tool)), (&bore, wall(&bore)), 1e-8),
            expected,
            "{at} {radius}"
        );
        assert_eq!(
            apart((&bore, wall(&bore)), (&tool, nappe(&tool)), 1e-8),
            expected,
            "{at} {radius}"
        );
    }
}

#[test]
fn a_whole_point_s_tip_counts_in_its_reach() {
    let point = turned(
        DVec3::ZERO,
        DVec3::Z,
        &[[0.0, 0.0], [0.0, 5.0], [10.0, 0.0]],
        TAU,
    );
    let [low, high] = reach((&point, nappe(&point)), DVec3::Z).expect("a reach");
    assert!(
        low.abs() < TOLERANCE && (high - 10.0).abs() < TOLERANCE,
        "the point reaches from {low} to {high}"
    );
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
