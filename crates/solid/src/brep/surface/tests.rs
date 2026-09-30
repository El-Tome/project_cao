use glam::{DVec2, DVec3};

use super::*;

const CLOSE: f64 = 1e-12;

#[test]
fn a_plane_reached_from_either_side_is_the_same_plane_turned_round() {
    let (up, turned_up) = Plane::through(DVec3::new(3.0, -2.0, 10.0), DVec3::Z);
    let (down, turned_down) = Plane::through(DVec3::new(-7.0, 5.0, 10.0), -DVec3::Z);
    assert_eq!(up, down);
    assert!(!turned_up && turned_down);
    assert_eq!(up.origin, DVec3::new(0.0, 0.0, 10.0));
}

#[test]
fn a_plane_tilted_by_a_hair_keeps_the_sign_of_the_plane_it_nearly_is() {
    let (plane, turned) = Plane::through(DVec3::ZERO, DVec3::new(-1e-9, 0.0, 1.0));
    assert!(!turned);
    assert!(plane.normal.z > 0.0);
}

#[test]
fn a_point_of_a_plane_comes_back_from_its_parameters() {
    let (plane, _) = Plane::through(DVec3::new(1.0, 2.0, 3.0), DVec3::new(1.0, 2.0, 2.0));
    let at = DVec2::new(4.5, -7.25);
    let back = plane.parameters(plane.point(at));
    assert!((back - at).length() < CLOSE, "{back}");
    assert!(plane.distance(plane.point(at)).abs() < CLOSE);
}

#[test]
fn parallel_cylinders_start_their_angles_from_the_same_direction() {
    let one = Cylinder::about(DVec3::new(8.0, 0.0, 3.0), DVec3::Z, 5.0);
    let other = Cylinder::about(DVec3::new(-2.0, 7.0, -1.0), -DVec3::Z, 20.0);
    assert_eq!(one.u, other.u);
    assert_eq!(one.v, other.v);
    assert_eq!(one.axis, other.axis);
    assert_eq!(one.origin, DVec3::new(8.0, 0.0, 0.0));
}

#[test]
fn the_angles_a_plane_of_the_origin_touches_a_cylinder_at_are_quarter_turns() {
    for axis in [DVec3::X, DVec3::Y, DVec3::Z] {
        let cylinder = Cylinder::about(DVec3::ZERO, axis, 1.0);
        for quarter in 0..4 {
            let radial = cylinder.radial(f64::from(quarter) * std::f64::consts::FRAC_PI_2);
            let along_an_axis = DVec3::AXES
                .iter()
                .any(|world| (radial.abs() - *world).length() < CLOSE);
            assert!(along_an_axis, "{axis} at quarter {quarter}: {radial}");
        }
    }
}

#[test]
fn a_point_of_a_cylinder_comes_back_from_its_parameters() {
    let cylinder = Cylinder::about(DVec3::new(1.0, -3.0, 2.0), DVec3::new(0.0, 1.0, 1.0), 4.0);
    for at in [
        DVec2::new(0.3, 2.0),
        DVec2::new(-2.9, -5.0),
        DVec2::new(3.1, 0.0),
    ] {
        let point = cylinder.point(at);
        assert!(cylinder.distance(point).abs() < CLOSE);
        let back = cylinder.parameters(point);
        assert!((back - at).length() < CLOSE, "{at} came back as {back}");
    }
}
