use std::f64::consts::{FRAC_PI_2, PI};

use glam::{DVec2, DVec3};

use super::Sphere;
use crate::brep::curve::Circle;
use crate::brep::surface::Cylinder;

const CLOSE: f64 = 1e-12;

/// A ball of radius 3 about `(h = 5)` on a slanted axis through a point off
/// the origin.
fn slanted() -> Sphere {
    Sphere::through(
        DVec3::new(1.0, -2.0, 3.0),
        DVec3::new(0.0, 1.0, 1.0),
        5.0,
        3.0,
    )
}

#[test]
fn a_sphere_shares_origin_and_reference_with_a_cylinder_of_its_axis() {
    let point = DVec3::new(2.0, -5.0, 1.0);
    let axis = DVec3::new(1.0, 2.0, -0.5);
    let sphere = Sphere::through(point, axis, 4.0, 2.0);
    let cylinder = Cylinder::about(point, axis, 4.0);
    assert_eq!(
        [sphere.origin, sphere.axis, sphere.u, sphere.v],
        [cylinder.origin, cylinder.axis, cylinder.u, cylinder.v]
    );
    assert_eq!(sphere.circle(3.0, 4.0), Circle::on(&cylinder, 3.0));
}

#[test]
fn a_sphere_turned_about_the_axis_either_way_is_the_same_bits() {
    let point = DVec3::new(3.0, 7.0, -2.0);
    let axis = DVec3::new(0.3, -1.0, 0.2);
    let forwards = Sphere::through(point, axis, 2.5, 1.5);
    let backwards = Sphere::through(point, -axis, -2.5, 1.5);
    assert_eq!(forwards, backwards);
    assert!(forwards.middle().distance(point + axis.normalize() * 2.5) < CLOSE);
}

#[test]
fn a_point_of_a_sphere_reads_back_its_parameters() {
    let sphere = slanted();
    for at in [
        DVec2::new(0.3, 0.2),
        DVec2::new(-2.9, -1.2),
        DVec2::new(3.1, 1.5),
        DVec2::new(1.0, 0.0),
    ] {
        let point = sphere.point(at);
        assert!(sphere.distance(point).abs() < CLOSE, "{at}");
        let back = sphere.parameters(point);
        assert!((back - at).length() < CLOSE, "{at} came back as {back}");
        assert!(sphere.nearest(point).distance(point) < CLOSE);
    }
}

#[test]
fn a_sphere_s_parameters_turn_as_its_own_normal_the_poles_included() {
    let step = 1e-6;
    let sphere = slanted();
    for at in [
        DVec2::new(0.4, 0.3),
        DVec2::new(-2.0, -1.0),
        DVec2::new(1.0, FRAC_PI_2 - 1e-3),
        DVec2::new(1.0, -FRAC_PI_2 + 1e-3),
    ] {
        let here = sphere.point(at);
        let round = sphere.point(at + DVec2::new(step, 0.0)) - here;
        let along = sphere.point(at + DVec2::new(0.0, step)) - here;
        let turned = round.cross(along).normalize();
        let normal = sphere.normal(at);
        assert!((normal.length() - 1.0).abs() < CLOSE);
        assert!(
            turned.distance(normal) < 1e-5,
            "{turned} against {normal} at {at}"
        );
        assert!(
            normal.dot(here - sphere.middle()) > 0.0,
            "the normal points into the ball at {at}"
        );
    }
    for pole in [-FRAC_PI_2, FRAC_PI_2] {
        let normal = sphere.normal(DVec2::new(0.7, pole));
        assert!(normal.distance(sphere.axis * pole.signum()) < CLOSE);
    }
}

#[test]
fn a_sphere_s_distance_is_signed_outward() {
    let sphere = slanted();
    let middle = sphere.middle();
    let away = DVec3::new(1.0, 2.0, 3.0).normalize();
    assert!((sphere.distance(middle + away * 4.0) - 1.0).abs() < CLOSE);
    assert!((sphere.distance(middle + away * 2.0) + 1.0).abs() < CLOSE);
    assert!((sphere.distance(middle) + 3.0).abs() < CLOSE);
    assert_eq!(sphere.poles(), [-FRAC_PI_2, FRAC_PI_2]);
}

#[test]
fn a_latitude_circle_is_held_and_a_meridian_is_told_from_it() {
    let sphere = slanted();
    let eps = 1e-8;
    let phi: f64 = 0.4;
    let (h, rho) = (sphere.center + 3.0 * phi.sin(), 3.0 * phi.cos());
    let latitude = sphere.circle(h, rho);
    assert!(sphere.holds(&latitude, eps));
    assert!(!sphere.meridian(&latitude, eps));
    assert!(!sphere.holds(&sphere.circle(h, rho + 10.0 * eps), eps));
    let mut leaning = latitude;
    leaning.axis = (leaning.axis + leaning.u * 1e-6).normalize();
    assert!(!sphere.holds(&leaning, eps));

    let meridian = Circle::about(sphere.middle(), sphere.u, 3.0);
    assert!(sphere.meridian(&meridian, eps));
    assert!(!sphere.holds(&meridian, eps));
    let off = Circle::about(sphere.middle() + sphere.u * 10.0 * eps, sphere.u, 3.0);
    assert!(!sphere.meridian(&off, eps));
    let beside = Circle::about(sphere.middle() + sphere.axis * 10.0 * eps, sphere.v, 3.0);
    assert!(!sphere.meridian(&beside, eps));
    let smaller = Circle::about(sphere.middle(), sphere.radial(0.3), 3.0 - 10.0 * eps);
    assert!(!sphere.meridian(&smaller, eps));
}

#[test]
fn a_line_crosses_a_ball_twice_touches_it_once_and_passes_it_by() {
    let sphere = slanted();
    let middle = sphere.middle();
    let direction = DVec3::new(1.0, 0.5, -0.2).normalize();
    let origin = middle - direction * 10.0;
    let [Some(near), Some(far)] = sphere.roots(origin, direction) else {
        panic!("a line through the centre crosses twice");
    };
    assert!((near - 7.0).abs() < CLOSE && (far - 13.0).abs() < CLOSE);
    for t in [near, far] {
        assert!(sphere.distance(origin + direction * t).abs() < CLOSE);
    }

    let across = direction.cross(sphere.axis).normalize();
    let grazing = sphere.roots(origin + across * 3.0, direction);
    let [Some(touch), None] = grazing else {
        panic!("a tangent line touches once: {grazing:?}");
    };
    assert!(
        sphere
            .distance(origin + across * 3.0 + direction * touch)
            .abs()
            < 1e-9
    );

    assert_eq!(
        sphere.roots(origin + across * (3.0 + 1e-9), direction),
        [None, None]
    );
    let [Some(_), Some(_)] = sphere.roots(origin + across * (3.0 - 1e-6), direction) else {
        panic!("a line a hair inside crosses twice");
    };
}

#[test]
fn a_sphere_reaches_its_radius_either_way_along_any_direction() {
    let sphere = slanted();
    let direction = DVec3::new(0.2, -0.7, 0.4).normalize();
    let [low, high] = sphere.extent(direction);
    let at = sphere.middle().dot(direction);
    assert!((high - (at + 3.0)).abs() < CLOSE && (low - (at - 3.0)).abs() < CLOSE);
    for theta in [-PI, -1.0, 0.5, 3.0] {
        let point = sphere.point(DVec2::new(theta, 0.2));
        let along = point.dot(direction);
        assert!(low - CLOSE <= along && along <= high + CLOSE);
    }
}
