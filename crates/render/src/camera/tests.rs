//! What render · camera.rs is held to.

use super::*;

/// Every zone — face, edge or corner — must end up with the camera looking
/// straight back down the direction of that zone.
#[test]
fn zone_views_look_straight_at_the_zone() {
    for zone in every_zone() {
        let mut camera = OrbitCamera::default();
        let mut transition = ViewTransition::to_zone(&camera, zone);
        while transition.advance(&mut camera, 0.1) {}

        let expected = -zone.direction();
        let forward = camera.forward();
        assert!(
            (forward - expected).length() < 1e-4,
            "{zone:?}: looking along {forward:?}, expected {expected:?}"
        );
    }
}

/// An edge or corner names its faces in a fixed order, so the same zone
/// picked from two different faces compares equal.
#[test]
fn zones_are_order_independent() {
    assert_eq!(
        CubeZone::edge(CubeFace::PlusZ, CubeFace::MinusY),
        CubeZone::edge(CubeFace::MinusY, CubeFace::PlusZ)
    );
    assert_eq!(
        CubeZone::corner(CubeFace::PlusZ, CubeFace::MinusY, CubeFace::PlusX),
        CubeZone::corner(CubeFace::PlusX, CubeFace::PlusZ, CubeFace::MinusY)
    );
}

/// Only a face lands on a work plane; oblique views do not.
#[test]
fn only_faces_carry_a_plane() {
    for zone in every_zone() {
        assert_eq!(
            zone.face().is_some(),
            matches!(zone, CubeZone::Face(_)),
            "{zone:?}"
        );
    }
}

fn every_zone() -> Vec<CubeZone> {
    let mut zones: Vec<CubeZone> = CubeFace::ALL.map(CubeZone::Face).into();
    for a in CubeFace::ALL {
        for b in CubeFace::ALL {
            if a.normal().dot(b.normal()).abs() > 0.5 {
                continue;
            }
            zones.push(CubeZone::edge(a, b));
            for c in CubeFace::ALL {
                if c.normal().dot(a.normal()).abs() > 0.5 || c.normal().dot(b.normal()).abs() > 0.5
                {
                    continue;
                }
                zones.push(CubeZone::corner(a, b, c));
            }
        }
    }
    zones
}

#[test]
fn top_and_bottom_views_stay_well_defined() {
    for pitch in [FRAC_PI_2, -FRAC_PI_2] {
        let mut camera = OrbitCamera::default();
        camera.set_view_angles(0.0, pitch);
        assert!(camera.view().is_finite());
        assert!((camera.up().length() - 1.0).abs() < 1e-5);
    }
}

#[test]
fn orbit_cannot_flip_past_the_poles() {
    let mut camera = OrbitCamera::default();
    camera.orbit(Vec2::new(0.0, 10_000.0), 0.01);
    assert!(camera.pitch() <= FRAC_PI_2 + 1e-6);
    camera.orbit(Vec2::new(0.0, -20_000.0), 0.01);
    assert!(camera.pitch() >= -FRAC_PI_2 - 1e-6);
}

#[test]
fn zoom_is_symmetric_and_bounded() {
    let mut camera = OrbitCamera::default();
    let start = camera.distance();
    camera.zoom(100.0, 0.0015);
    camera.zoom(-100.0, 0.0015);
    assert!((camera.distance() - start).abs() < 1e-2);

    camera.zoom(1e6, 0.0015);
    assert!(camera.distance() >= 1e-3);
}

/// Looking straight down must not turn the view: the sketch axes have to
/// land the way round they are drawn.
#[test]
fn a_top_view_is_not_turned_half_a_turn() {
    for (direction, name) in [(Vec3::Z, "top"), (Vec3::NEG_Z, "bottom")] {
        let (yaw, _) = view_angles_towards(direction);
        assert!(
            yaw.abs() < 1e-4,
            "{name}: yaw {} instead of 0",
            yaw.to_degrees()
        );
    }

    let mut camera = OrbitCamera::default();
    let (yaw, pitch) = view_angles_towards(Vec3::Z);
    camera.set_view_angles(yaw, pitch);
    assert!(
        (camera.right() - Vec3::X).length() < 1e-4,
        "X must point right, not {:?}",
        camera.right()
    );
    assert!((camera.up() - Vec3::Y).length() < 1e-4);
}

/// A ray through the middle of the screen must run straight down the
/// camera's own direction, and one off to the side must lean away from it.
#[test]
fn rays_follow_the_camera() {
    let camera = OrbitCamera::default();
    let (origin, direction) = camera.ray(Vec2::ZERO, 1.6);

    assert!(
        (direction - camera.forward()).length() < 1e-3,
        "centre ray points along {direction:?}, expected {:?}",
        camera.forward()
    );
    assert!((origin - camera.eye()).length() < camera.distance());

    let (_, right_edge) = camera.ray(Vec2::new(0.9, 0.0), 1.6);
    assert!(right_edge.dot(camera.right()) > 0.1);
}

/// Framing a sphere must put it fully inside the view: the angle from the
/// eye to its rim stays under the half field of view.
#[test]
fn focusing_frames_the_whole_sphere() {
    for (radius, aspect) in [(1.0, 1.6), (500.0, 0.6), (0.01, 1.0)] {
        let mut camera = OrbitCamera::default();
        camera.focus_on(Vec3::new(3.0, -4.0, 5.0), radius, aspect);

        assert_eq!(camera.target(), Vec3::new(3.0, -4.0, 5.0));
        let half_vertical = 45f32.to_radians() * 0.5;
        let half_horizontal = (half_vertical.tan() * aspect).atan();
        let rim_angle = (radius / camera.distance()).asin();
        assert!(
            rim_angle < half_vertical.min(half_horizontal),
            "radius {radius} at aspect {aspect} does not fit"
        );
    }
}

#[test]
fn zoom_respects_configured_limits() {
    let mut camera = OrbitCamera::default();
    camera.set_distance_limits(0.5, 5_000.0);

    camera.zoom(-1e6, 0.0015);
    assert!(camera.distance() <= 5_000.0);
    camera.zoom(1e6, 0.0015);
    assert!(camera.distance() >= 0.5);
}

/// Zooming out must keep going well past the few tens of metres a part is
/// modelled at, so a whole assembly can be framed.
#[test]
fn zooming_out_reaches_far_beyond_a_part() {
    let mut camera = OrbitCamera::default();
    for _ in 0..200 {
        camera.zoom(-100.0, 0.0015);
    }
    assert!(
        camera.distance() > 1e6,
        "stuck at {} units",
        camera.distance()
    );
}
