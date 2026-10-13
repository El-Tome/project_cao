//! Two walls square to each other, an elliptic one among them (#571): they
//! meet along the curve `meet.rs` builds, or stand apart; at any other
//! angle the pair is unsupported.

use glam::DVec3;

use super::*;
use crate::brep::surface::EllipticCylinder;

/// An elliptic post on Z, half-axes 5 along X and 3 along Y, turned by
/// `degrees` about its axis.
fn post(degrees: f64) -> Surface {
    let (sin, cos) = degrees.to_radians().sin_cos();
    Surface::Elliptic(EllipticCylinder::about(
        DVec3::ZERO,
        DVec3::Z,
        DVec3::new(cos, sin, 0.0) * 5.0,
        3.0,
    ))
}

/// An elliptic bar along `axis` through `center`, its first half-axis
/// `first`.
fn bar(center: DVec3, axis: DVec3, first: DVec3, second: f64) -> Surface {
    Surface::Elliptic(EllipticCylinder::about(center, axis, first, second))
}

#[test]
fn a_round_wall_square_to_an_elliptic_one_meets_it_along_the_curve_on_both() {
    for (round, components) in [
        (cylinder(DVec3::new(0.0, 1.0, 2.0), DVec3::X, 1.5), 2),
        (cylinder(DVec3::new(0.0, 2.5, 2.0), DVec3::X, 1.5), 1),
        (cylinder(DVec3::new(0.0, 0.5, 2.0), DVec3::Y, 6.0), 2),
    ] {
        for turned in [0.0, 40.0] {
            let wall = post(turned);
            let found = relation(&wall, &round, scale());
            let Relation::Meet(meeting) = &found else {
                panic!("a meet expected, got {found:?}");
            };
            assert_eq!(meeting.components.len(), components, "{round:?}");
            assert_on_both(&found, &wall, &round, 1e-12 * REACH);
            assert_eq!(relation(&round, &wall, scale()), found);
        }
    }
}

#[test]
fn two_elliptic_walls_square_to_each_other_meet_along_the_curve_on_both() {
    let wall = post(25.0);
    let across = bar(
        DVec3::new(0.0, 0.5, 4.0),
        DVec3::X,
        DVec3::new(0.0, 0.6, 0.8) * 2.5,
        1.0,
    );
    let found = relation(&wall, &across, scale());
    let Relation::Meet(meeting) = &found else {
        panic!("a meet expected, got {found:?}");
    };
    assert_eq!(meeting.configuration, Configuration::TwoLoops);
    assert_on_both(&found, &wall, &across, 1e-12 * REACH);
}

#[test]
fn an_elliptic_wall_square_to_another_wall_beyond_its_reach_stands_apart() {
    let far = cylinder(DVec3::new(0.0, 6.0, 2.0), DVec3::X, 1.5);
    assert_eq!(relation(&post(0.0), &far, scale()), Relation::Apart);
    let far = bar(DVec3::new(-9.0, 0.0, 2.0), DVec3::Y, DVec3::Z * 2.0, 1.0);
    assert_eq!(relation(&post(0.0), &far, scale()), Relation::Apart);
}

#[test]
fn an_elliptic_wall_at_a_skew_angle_to_another_wall_is_unsupported() {
    let skew = cylinder(DVec3::new(0.0, 1.0, 2.0), DVec3::new(1.0, 0.0, 0.3), 1.5);
    assert_eq!(relation(&post(0.0), &skew, scale()), Relation::Unsupported);
    let skew = bar(
        DVec3::new(0.0, 1.0, 2.0),
        DVec3::new(1.0, 0.2, 0.3),
        DVec3::Z * 2.0,
        1.0,
    );
    assert_eq!(relation(&skew, &post(0.0), scale()), Relation::Unsupported);
}
