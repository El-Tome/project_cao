//! What render · cube.rs is held to.

use super::*;
use crate::camera::{CubeZone, OrbitCamera};
use std::f32::consts::FRAC_PI_2;

/// Looking straight at a face, the centre of the cube's viewport must pick
/// that same face.
#[test]
fn clicking_the_centre_picks_the_face_we_look_at() {
    for face in CubeFace::ALL {
        let mut camera = OrbitCamera::default();
        let mut transition = crate::ViewTransition::to_zone(&camera, CubeZone::Face(face));
        while transition.advance(&mut camera, 0.1) {}

        assert_eq!(
            pick_zone(camera.rotation(), Vec2::ZERO),
            Some(CubeZone::Face(face)),
            "looking at {face:?}"
        );
    }
}

/// Straight-on at a face, the border strips select the neighbouring edges
/// and the four corner tiles select corners.
#[test]
fn borders_of_a_face_pick_edges_and_corners() {
    let mut camera = OrbitCamera::default();
    camera.set_view_angles(0.0, FRAC_PI_2);

    // The cube spans 0.5 of the orthographic half-size, so a face border
    // sits around 0.4 in normalized device coordinates.
    let near_border = 0.45;
    assert!(matches!(
        pick_zone(camera.rotation(), Vec2::new(near_border, 0.0)),
        Some(CubeZone::Edge(_))
    ));
    assert!(matches!(
        pick_zone(camera.rotation(), Vec2::new(0.0, near_border)),
        Some(CubeZone::Edge(_))
    ));
    assert!(matches!(
        pick_zone(camera.rotation(), Vec2::new(near_border, near_border)),
        Some(CubeZone::Corner(_))
    ));
    assert!(matches!(
        pick_zone(camera.rotation(), Vec2::ZERO),
        Some(CubeZone::Face(_))
    ));
}

/// An edge picked from one of its two faces is the same zone as when it is
/// picked from the other, so the highlight covers both.
#[test]
fn an_edge_is_the_same_zone_from_either_face() {
    let top = CubeZone::Face(CubeFace::PlusZ);
    let front = CubeZone::Face(CubeFace::MinusY);

    let from_top = zone_at(CubeFace::PlusZ, 0.0, -0.45);
    let from_front = zone_at(CubeFace::MinusY, 0.0, 0.45);

    assert_eq!(from_top, from_front);
    assert_ne!(from_top, top);
    assert_ne!(from_top, front);
}

#[test]
fn clicking_outside_the_cube_picks_nothing() {
    let mut camera = OrbitCamera::default();
    camera.set_view_angles(0.0, FRAC_PI_2);
    assert_eq!(pick_zone(camera.rotation(), Vec2::new(0.99, 0.99)), None);
}

/// Exactly three faces are visible from a general viewpoint, and never a
/// face and its opposite at once.
#[test]
fn opposite_faces_are_never_both_visible() {
    let camera = OrbitCamera::default();
    let forward = camera.forward();
    for face in CubeFace::ALL {
        if is_visible(face, forward) {
            let opposite = CubeFace::ALL
                .into_iter()
                .find(|other| other.normal() == -face.normal())
                .expect("every face has an opposite");
            assert!(!is_visible(opposite, forward));
        }
    }
}
