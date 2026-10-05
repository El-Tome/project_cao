use glam::DVec3;

use super::*;
use crate::brep::surface::Cylinder;

const CLOSE: f64 = 1e-12;

#[test]
fn a_line_reached_from_two_points_either_way_is_the_same_line() {
    let one = Line::through(DVec3::new(20.0, 0.0, 3.0), DVec3::Z);
    let other = Line::through(DVec3::new(20.0, 0.0, -8.0), -DVec3::Z);
    assert_eq!(one, other);
    assert_eq!(one.origin, DVec3::new(20.0, 0.0, 0.0));
}

#[test]
fn a_point_on_a_line_comes_back_from_its_parameter() {
    let line = Line::through(DVec3::new(1.0, 2.0, 3.0), DVec3::new(1.0, -1.0, 2.0));
    let point = line.point(7.5);
    assert!((line.parameter(point) - 7.5).abs() < CLOSE);
}

#[test]
fn a_circle_of_a_cylinder_turns_with_the_cylinder_s_angle() {
    let cylinder = Cylinder::about(DVec3::new(8.0, 0.0, 0.0), DVec3::Z, 5.0);
    let circle = Circle::on(&cylinder, 10.0);
    for theta in [-3.0, -1.0, 0.0, 0.5, 2.9] {
        let on_cylinder = cylinder.point(glam::DVec2::new(theta, 10.0));
        assert!((circle.point(theta) - on_cylinder).length() < CLOSE);
        assert!((circle.parameter(on_cylinder) - theta).abs() < CLOSE);
    }
}

#[test]
fn a_circle_s_derivative_is_its_speed_along_its_angle() {
    let cylinder = Cylinder::about(DVec3::ZERO, DVec3::X, 3.0);
    let circle = Curve::Circle(Circle::on(&cylinder, -2.0));
    let (theta, step) = (0.7, 1e-6);
    let numeric = (circle.point(theta + step) - circle.point(theta - step)) / (2.0 * step);
    assert!((numeric - circle.derivative(theta)).length() < 1e-8);
}
