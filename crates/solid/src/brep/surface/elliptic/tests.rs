use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::surface::Surface;

const CLOSE: f64 = 1e-12;

/// An ellipse of half-axes 7 and 3 about `(1, -2, 4)`, its axis leaning, its
/// shorter half-axis given first.
fn leaning() -> EllipticCylinder {
    let axis = DVec3::new(0.0, 1.0, 2.0).normalize();
    let first = axis.cross(DVec3::X).normalize() * 3.0;
    EllipticCylinder::about(DVec3::new(1.0, -2.0, 4.0), axis, first, 7.0)
}

#[test]
fn an_elliptic_cylinder_reads_its_major_half_axis_along_u_whichever_was_drawn_first() {
    let wall = EllipticCylinder::about(DVec3::new(5.0, 6.0, 0.0), DVec3::Z, DVec3::Y * 3.0, 7.0);
    assert_eq!((wall.major, wall.minor), (7.0, 3.0));
    assert!((wall.u.abs() - DVec3::X).length() < CLOSE, "{}", wall.u);
    assert!((wall.v - wall.axis.cross(wall.u)).length() < CLOSE);
    assert_eq!(wall.origin, DVec3::new(5.0, 6.0, 0.0));
}

#[test]
fn an_elliptic_cylinder_reached_from_either_way_is_the_same_bits() {
    let one = EllipticCylinder::about(DVec3::new(2.0, 1.0, 9.0), DVec3::Z, DVec3::X * 7.0, 3.0);
    let other =
        EllipticCylinder::about(DVec3::new(2.0, 1.0, -4.0), -DVec3::Z, DVec3::X * -7.0, 3.0);
    let across = EllipticCylinder::about(DVec3::new(2.0, 1.0, 0.0), DVec3::Z, DVec3::Y * 3.0, 7.0);
    assert_eq!(one, other);
    assert_eq!(one, across);
}

#[test]
fn a_point_of_an_elliptic_cylinder_comes_back_from_its_parameters() {
    let wall = leaning();
    for at in [
        DVec2::new(0.3, 2.0),
        DVec2::new(-2.9, -5.0),
        DVec2::new(PI, 0.0),
        DVec2::new(FRAC_PI_2, 1.5),
    ] {
        let point = wall.point(at);
        assert!(wall.distance(point).abs() < CLOSE * 10.0, "{at}");
        let back = wall.parameters(point);
        assert!((back - at).length() < CLOSE, "{at} came back as {back}");
    }
}

#[test]
fn the_normal_of_an_elliptic_cylinder_is_square_to_its_wall_and_points_away_from_its_axis() {
    let wall = leaning();
    for step in 0..12 {
        let theta = TAU * f64::from(step) / 12.0;
        let normal = wall.normal(theta);
        let running =
            wall.point(DVec2::new(theta + 1e-6, 0.0)) - wall.point(DVec2::new(theta - 1e-6, 0.0));
        let out = wall.point(DVec2::new(theta, 0.0)) - wall.origin;
        assert!((normal.length() - 1.0).abs() < CLOSE);
        assert!(
            normal.dot(running).abs() < 1e-9 * running.length(),
            "at {theta}"
        );
        assert!(normal.dot(wall.axis).abs() < CLOSE);
        assert!(normal.dot(out) > 0.0, "at {theta}");
    }
}

#[test]
fn a_point_off_an_elliptic_cylinder_stands_as_far_as_its_foot_either_side() {
    let wall = leaning();
    for step in 0..24 {
        let theta = TAU * f64::from(step) / 24.0 + 0.1;
        let foot = wall.point(DVec2::new(theta, 2.5));
        for off in [1e-9, 1e-3, 0.5, 2.0, -1e-9, -1e-3, -0.4] {
            let point = foot + wall.normal(theta) * off;
            let distance = wall.distance(point);
            assert!(
                (distance - off).abs() < 1e-12,
                "{off} off at {theta} reads {distance}",
            );
        }
    }
}

#[test]
fn a_point_on_an_axis_of_the_section_finds_its_foot_off_that_axis_where_it_lies_deep() {
    let wall = EllipticCylinder::about(DVec3::ZERO, DVec3::Z, DVec3::X * 5.0, 3.0);
    let inside = DVec3::new(1.0, 0.0, 4.0);
    let across = 25.0 / 16.0;
    let foot = DVec2::new(across, 3.0 * (1.0 - (across / 5.0).powi(2)).sqrt());
    let expected = -(foot - DVec2::new(1.0, 0.0)).length();
    assert!(
        (wall.distance(inside) - expected).abs() < CLOSE,
        "{}",
        wall.distance(inside)
    );
    assert!((wall.distance(DVec3::ZERO) + 3.0).abs() < CLOSE);
    assert!((wall.distance(DVec3::new(0.0, -1.0, 0.0)) + 2.0).abs() < CLOSE);
    assert!((wall.distance(DVec3::new(-8.0, 0.0, 0.0)) - 3.0).abs() < CLOSE);
    assert!((wall.distance(DVec3::new(0.0, 4.5, -2.0)) - 1.5).abs() < CLOSE);
    assert!((wall.distance(DVec3::new(4.9, 0.0, 0.0)) + 0.1).abs() < CLOSE);
}

#[test]
fn the_support_of_an_elliptic_cylinder_is_how_far_it_reaches_along_a_way_square_to_its_axis() {
    let wall = leaning();
    assert!((wall.support(wall.u) - 7.0).abs() < CLOSE);
    assert!((wall.support(-wall.v) - 3.0).abs() < CLOSE);
    let way = (wall.u * 2.0 - wall.v).normalize();
    let reached = (0..100_000)
        .map(|step| {
            let theta = TAU * f64::from(step) / 100_000.0;
            way.dot(wall.point(DVec2::new(theta, 0.0)) - wall.origin)
        })
        .fold(f64::NEG_INFINITY, f64::max);
    assert!((wall.support(way) - reached).abs() < 1e-7, "{reached}");
}

#[test]
fn an_elliptic_wall_closes_on_itself_after_a_turn() {
    let surface = Surface::Elliptic(leaning());
    assert_eq!(surface.period(), Some(TAU));
    let at = DVec2::new(1.0, 2.0);
    let point = surface.point(at);
    assert!(surface.distance(point).abs() < 1e-11);
    assert!((surface.parameters(point) - at).length() < CLOSE);
    assert!((surface.normal(at) - leaning().normal(1.0)).length() < CLOSE);
}

#[test]
fn an_elliptic_wall_moved_keeps_its_axis_and_half_axes_to_the_bit() {
    let wall = leaning();
    let by = DVec3::new(3.0, -4.0, 5.0);
    let moved = wall.moved(by);
    assert_eq!(
        (moved.axis, moved.u, moved.v, moved.major, moved.minor),
        (wall.axis, wall.u, wall.v, wall.major, wall.minor),
    );
    assert!(moved.origin.dot(moved.axis).abs() < CLOSE);
    let at = DVec2::new(0.7, 1.0);
    assert!(moved.distance(wall.point(at) + by).abs() < 1e-11);
}

#[test]
fn two_elliptic_walls_stand_as_far_apart_as_their_axes_half_axes_and_turn_part_them_the_turn_by_the_half_axes_difference()
 {
    let wall = EllipticCylinder::about(DVec3::ZERO, DVec3::Z, DVec3::X * 5.0, 3.0);
    let beside = EllipticCylinder::about(DVec3::new(0.0, 2e-3, 7.0), DVec3::Z, DVec3::X * 5.0, 3.0);
    let wider = EllipticCylinder::about(DVec3::ZERO, DVec3::Z, DVec3::X * 5.25, 2.5);
    let turned = EllipticCylinder::about(
        DVec3::ZERO,
        DVec3::Z,
        DVec3::new(1.0, 1e-4, 0.0).normalize() * 5.0,
        3.0,
    );
    assert!((wall.gap(&beside) - 2e-3).abs() < CLOSE);
    assert!((wall.gap(&wider) - 0.75).abs() < CLOSE);
    assert!((wall.gap(&turned) - 2e-4).abs() < 1e-11);
    assert_eq!(wall.gap(&wall), 0.0);
}

#[test]
fn an_elliptic_wall_a_hair_from_round_turned_any_way_stands_within_its_half_axes_difference() {
    let wall = EllipticCylinder::about(DVec3::ZERO, DVec3::Z, DVec3::X * (5.0 + 3e-9), 5.0);
    for degrees in [1e-3, 1.0, 45.0, 90.0, 135.0] {
        let turned = EllipticCylinder::about(
            DVec3::ZERO,
            DVec3::Z,
            DVec2::from_angle(f64::to_radians(degrees)).extend(0.0) * (5.0 + 3e-9),
            5.0,
        );
        assert!(wall.within(&turned, 3.1e-9), "{degrees}");
        assert!(wall.gap(&turned) <= 3.1e-9, "{degrees}");
    }
    let rounder = EllipticCylinder::about(DVec3::ZERO, DVec3::Z, DVec3::Y * (5.0 + 1e-9), 5.0);
    assert!(!wall.within(&rounder, 1.9e-9));
}

#[test]
fn an_elliptic_section_runs_slowest_at_the_ends_of_its_major_half_axis() {
    let wall = leaning();
    assert!((wall.speed(0.0) - wall.minor).abs() < CLOSE);
    assert!((wall.speed(PI) - wall.minor).abs() < CLOSE);
    assert!((wall.speed(FRAC_PI_2) - wall.major).abs() < CLOSE);
    let theta = 0.4;
    let step = 1e-6;
    let run = wall.point(DVec2::new(theta + step, 0.0)) - wall.point(DVec2::new(theta - step, 0.0));
    assert!((wall.speed(theta) - run.length() / (2.0 * step)).abs() < 1e-8);
}

#[test]
fn the_point_of_an_elliptic_wall_nearest_a_place_is_its_foot_at_the_place_s_height() {
    let wall = leaning();
    for (theta, off) in [(0.3, 0.5), (2.0, -0.2), (-1.0, 3.0)] {
        let foot = wall.point(DVec2::new(theta, 4.0));
        let place = foot + wall.normal(theta) * off;
        let nearest = wall.nearest(place);
        assert!(nearest.distance(foot) < 1e-11, "{nearest} for {foot}");
    }
}
