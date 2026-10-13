//! Two parallel walls, an elliptic one among them, in every configuration
//! their sections can stand in.

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::surface::Surface;

const REACH: f64 = 40.0;

fn scale() -> Scale {
    Scale::of(REACH)
}

/// An ellipse of half-axes 7 and 3 about `(1, −2)`, its major half-axis
/// turned 30° off X, raised along Z.
fn oval() -> EllipticCylinder {
    let turned = 30f64.to_radians();
    EllipticCylinder::about(
        DVec3::new(1.0, -2.0, 0.0),
        DVec3::Z,
        DVec3::new(turned.cos(), turned.sin(), 0.0) * 7.0,
        3.0,
    )
}

/// The point of the ellipse's section `(x, y)` along its own half-axes.
fn within(wall: &EllipticCylinder, x: f64, y: f64) -> DVec3 {
    wall.origin + wall.u * x + wall.v * y
}

fn round(center: DVec3, radius: f64) -> Cylinder {
    Cylinder::about(center, DVec3::Z, radius)
}

/// The rulings of a relation, each with whether it touches.
fn rulings(found: &Relation) -> Vec<(Line, bool)> {
    match found {
        Relation::Apart => Vec::new(),
        Relation::Lines(lines) => lines.iter().map(|line| (*line, false)).collect(),
        Relation::Tangent(line) => vec![(*line, true)],
        Relation::Parallel(rulings) => rulings
            .iter()
            .map(|ruled| (ruled.line, ruled.touching))
            .collect(),
        other => panic!("rulings expected, got {other:?}"),
    }
}

/// The relation of the two walls, the same both ways round, its rulings
/// along the axis and on both walls: a crossing to rounding, a touch within
/// the tolerance. How many cross, and how many touch.
fn counted(one: &Surface, other: &Surface) -> (usize, usize) {
    let found = super::super::relation(one, other, scale());
    assert_eq!(super::super::relation(other, one, scale()), found);
    let eps = scale().eps();
    let mut counts = (0, 0);
    for (line, touching) in rulings(&found) {
        assert!(line.direction.cross(DVec3::Z).length() < 1e-15);
        let within = if touching { eps } else { 1e-12 * REACH };
        for surface in [one, other] {
            let off = surface.distance(line.origin).abs();
            assert!(off <= within, "{line:?} is {off} off {surface:?}");
        }
        if touching {
            counts.1 += 1;
        } else {
            counts.0 += 1;
        }
    }
    counts
}

#[test]
fn a_circle_apart_from_an_ellipse_outside_it_or_inside_it_meets_it_nowhere() {
    let wall = Surface::Elliptic(oval());
    let beside = round(within(&oval(), 11.0, 0.0), 3.0);
    let inside = round(within(&oval(), 1.0, 0.5), 1.0);
    let around = round(within(&oval(), 0.0, 0.0), 8.0);
    for circle in [beside, inside, around] {
        assert_eq!(counted(&wall, &Surface::Cylinder(circle)), (0, 0));
    }
}

#[test]
fn a_circle_crossing_an_ellipse_crosses_it_along_two_rulings_or_four() {
    let wall = Surface::Elliptic(oval());
    let at_its_end = round(within(&oval(), 7.0, 0.0), 2.0);
    assert_eq!(counted(&wall, &Surface::Cylinder(at_its_end)), (2, 0));
    let about_its_centre = round(within(&oval(), 0.2, 0.1), 5.0);
    assert_eq!(counted(&wall, &Surface::Cylinder(about_its_centre)), (4, 0));
}

#[test]
fn a_circle_inscribed_across_the_minor_axis_touches_along_two_rulings() {
    let wall = Surface::Elliptic(oval());
    let eps = scale().eps();
    for hair in [0.0, eps / 2.0, -eps / 2.0] {
        let inscribed = round(within(&oval(), 0.0, 0.0), 3.0 + hair);
        assert_eq!(
            counted(&wall, &Surface::Cylinder(inscribed)),
            (0, 2),
            "{hair}"
        );
    }
    let grown = round(within(&oval(), 0.0, 0.0), 3.0 + 2.0 * eps);
    assert_eq!(counted(&wall, &Surface::Cylinder(grown)), (4, 0));
    let shrunk = round(within(&oval(), 0.0, 0.0), 3.0 - 2.0 * eps);
    assert_eq!(counted(&wall, &Surface::Cylinder(shrunk)), (0, 0));
}

#[test]
fn a_circle_touching_an_ellipse_at_a_vertex_touches_along_one_ruling_and_may_cross_it_twice() {
    let wall = Surface::Elliptic(oval());
    let curvature = 3.0 * 3.0 / 7.0;
    let outside = round(within(&oval(), 9.0, 0.0), 2.0);
    assert_eq!(counted(&wall, &Surface::Cylinder(outside)), (0, 1));
    let tighter = round(within(&oval(), 7.0 - curvature / 2.0, 0.0), curvature / 2.0);
    assert_eq!(counted(&wall, &Surface::Cylinder(tighter)), (0, 1));
    let osculating = round(within(&oval(), 7.0 - curvature, 0.0), curvature);
    assert_eq!(counted(&wall, &Surface::Cylinder(osculating)), (0, 1));
    let looser = round(within(&oval(), 4.0, 0.0), 3.0);
    assert_eq!(counted(&wall, &Surface::Cylinder(looser)), (2, 1));
    let at_the_minor_vertex = round(within(&oval(), 0.0, -5.0), 2.0);
    assert_eq!(
        counted(&wall, &Surface::Cylinder(at_the_minor_vertex)),
        (0, 1)
    );
}

#[test]
fn a_circle_a_hair_from_touching_an_ellipse_outside_touches_it_within_the_tolerance() {
    let wall = Surface::Elliptic(oval());
    let eps = scale().eps();
    for (hair, expected) in [
        (eps / 2.0, (0, 1)),
        (-eps / 2.0, (0, 1)),
        (2.0 * eps, (0, 0)),
        (-2.0 * eps, (2, 0)),
    ] {
        let circle = round(within(&oval(), 9.0 + hair, 0.0), 2.0);
        assert_eq!(
            counted(&wall, &Surface::Cylinder(circle)),
            expected,
            "{hair}"
        );
    }
}

#[test]
fn two_ellipses_crossing_touching_or_one_inside_the_other_meet_as_their_sections_do() {
    let wall = Surface::Elliptic(oval());
    let other = |center: DVec3, first: DVec3, second: f64| {
        Surface::Elliptic(EllipticCylinder::about(center, DVec3::Z, first, second))
    };
    let at = |x: f64, y: f64| within(&oval(), x, y);
    let crossed = other(at(0.0, 0.0), oval().v * 6.0, 2.0);
    assert_eq!(counted(&wall, &crossed), (4, 0));
    let beside = other(at(9.0, 0.0), oval().u * 2.0, 1.0);
    assert_eq!(counted(&wall, &beside), (0, 1));
    let across = other(at(0.0, -5.0), oval().u * 4.0, 2.0);
    assert_eq!(counted(&wall, &across), (0, 1));
    let shrunk = other(at(7.0 - 7.0 * 0.5, 0.0), oval().u * 3.5, 1.5);
    assert_eq!(counted(&wall, &shrunk), (0, 1));
    let grown = other(at(0.0, 0.0), oval().u * 7.5, 3.0);
    assert_eq!(counted(&wall, &grown), (0, 2));
    let moved = other(at(0.5, 0.0), oval().u * 7.0, 3.0);
    assert_eq!(counted(&wall, &moved), (2, 0));
    let apart = other(at(20.0, 0.0), oval().u * 7.0, 3.0);
    assert_eq!(counted(&wall, &apart), (0, 0));
    let inside = other(at(0.0, 0.0), oval().u * 6.0, 2.0);
    assert_eq!(counted(&wall, &inside), (0, 0));
}

#[test]
fn the_same_ellipse_reached_again_is_one_surface_and_one_turned_a_little_crosses_it_four_times() {
    let wall = Surface::Elliptic(oval());
    let again = Surface::Elliptic(EllipticCylinder::about(
        within(&oval(), 0.0, 0.0) + DVec3::Z * 5.0,
        -DVec3::Z,
        -oval().u * 7.0,
        3.0,
    ));
    assert_eq!(
        super::super::relation(&wall, &again, scale()),
        Relation::Same { agree: true }
    );
    let turned = DVec2::from_angle(1e-4).rotate(DVec2::new(oval().u.x, oval().u.y));
    let turned = Surface::Elliptic(EllipticCylinder::about(
        within(&oval(), 0.0, 0.0),
        DVec3::Z,
        DVec3::new(turned.x, turned.y, 0.0) * 7.0,
        3.0,
    ));
    assert_eq!(counted(&wall, &turned), (4, 0));
}

#[test]
fn an_elliptic_wall_and_a_wall_not_parallel_to_it_are_unsupported() {
    let wall = Surface::Elliptic(oval());
    let square = Surface::Cylinder(Cylinder::about(DVec3::ZERO, DVec3::X, 2.0));
    assert_eq!(
        super::super::relation(&wall, &square, scale()),
        Relation::Unsupported
    );
}

#[test]
fn a_circle_and_an_ellipse_a_hair_from_round_within_twenty_tolerances_of_it_all_round_are_unsupported()
 {
    let eps = scale().eps();
    let nearly = Surface::Elliptic(EllipticCylinder::about(
        DVec3::new(1.0, -2.0, 0.0),
        DVec3::Z,
        DVec3::X * (5.0 + 10.0 * eps),
        5.0,
    ));
    for (x, radius) in [(1.0, 5.0 + 5.0 * eps), (1.0 + 3.0 * eps, 5.0), (1.0, 5.0)] {
        let round = Surface::Cylinder(Cylinder::about(DVec3::new(x, -2.0, 0.0), DVec3::Z, radius));
        assert_eq!(
            super::super::relation(&nearly, &round, scale()),
            Relation::Unsupported,
            "{x} {radius}"
        );
    }
    let further = Surface::Cylinder(Cylinder::about(
        DVec3::new(1.0, -2.0, 0.0),
        DVec3::Z,
        5.0 + 30.0 * eps,
    ));
    assert_eq!(
        super::super::relation(&nearly, &further, scale()),
        Relation::Apart
    );
}
