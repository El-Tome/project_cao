//! A plane against an elliptic wall (#571), at every angle, and the same
//! ellipse twice.

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::conic::Arm;
use crate::brep::surface::EllipticCylinder;

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

/// The plane along the axis whose normal turns `angle` off X, standing
/// `offset` beyond the axis along it.
fn along(angle: f64, offset: f64) -> (Surface, DVec3) {
    let normal = DVec3::new(angle.cos(), angle.sin(), 0.0);
    (plane(oval().origin + normal * offset, normal), normal)
}

#[test]
fn a_plane_square_to_an_elliptic_wall_s_axis_meets_it_along_its_whole_section() {
    let wall = Surface::Elliptic(oval());
    let top = plane(DVec3::new(0.0, 0.0, 10.0), -DVec3::Z);
    let found = relation(&wall, &top, scale());
    let Relation::Conic(conics) = &found else {
        panic!("the section expected, got {found:?}");
    };
    assert_eq!(conics.len(), 1);
    assert_eq!(conics[0].arm, Arm::Whole);
    assert_on_both(&found, &wall, &top, 1e-12 * REACH);
    assert_eq!(relation(&top, &wall, scale()), found);
}

#[test]
fn a_plane_at_a_slant_to_an_elliptic_wall_meets_it_along_a_conic() {
    let wall = Surface::Elliptic(oval());
    for normal in [
        DVec3::new(0.3, 0.2, 1.0),
        DVec3::new(1.0, -0.5, 0.4),
        DVec3::new(-0.2, 1.0, 0.1),
    ] {
        let slant = plane(DVec3::new(1.0, -2.0, 4.0), normal);
        let found = relation(&slant, &wall, scale());
        let Relation::Conic(conics) = &found else {
            panic!("a conic expected, got {found:?}");
        };
        assert_eq!(conics.len(), 1);
        assert_on_both(&found, &wall, &slant, 1e-12 * REACH);
    }
}

#[test]
fn a_plane_along_an_elliptic_wall_s_axis_cuts_it_along_two_rulings_touches_it_along_one_or_misses_it()
 {
    let eps = scale().eps();
    let wall = Surface::Elliptic(oval());
    for angle in [0.0, 0.5, 2.0, -1.2] {
        let (_, normal) = along(angle, 0.0);
        let reach = oval().support(normal);
        for (offset, expected) in [
            (0.0, 2),
            (reach - 1.0, 2),
            (reach, 1),
            (reach + eps / 2.0, 1),
            (reach - eps / 2.0, 1),
            (reach + 2.0 * eps, 0),
            (reach - 2.0 * eps, 2),
            (-reach - eps / 2.0, 1),
            (-reach - 2.0 * eps, 0),
            (reach + 3.0, 0),
        ] {
            let (side, _) = along(angle, offset);
            let found = relation(&wall, &side, scale());
            let lines = match &found {
                Relation::Lines(lines) => lines.to_vec(),
                Relation::Tangent(line) => vec![*line],
                Relation::Apart => Vec::new(),
                _ => panic!("rulings expected, got {found:?}"),
            };
            assert_eq!(lines.len(), expected, "at {offset} off {angle}");
            for line in &lines {
                assert!(line.direction.cross(DVec3::Z).length() < 1e-15);
                for t in [-REACH, 0.0, REACH] {
                    let point = line.point(t);
                    assert!(side.distance(point).abs() <= 1e-12 * REACH);
                    assert!(wall.distance(point).abs() <= eps);
                }
            }
            if expected == 1 {
                let wall = oval();
                let reached = wall.u * (wall.major * wall.major * normal.dot(wall.u))
                    + wall.v * (wall.minor * wall.minor * normal.dot(wall.v));
                let support = wall.origin + reached * (offset.signum() / reach);
                let flat = |point: DVec3| DVec2::new(point.x, point.y);
                let touch = flat(lines[0].origin);
                assert!(touch.distance(flat(support)) <= 2.0 * eps, "{touch}");
            }
            assert_eq!(relation(&side, &wall, scale()), found);
        }
    }
}

#[test]
fn the_same_ellipse_reached_twice_is_one_surface_and_another_meets_it_along_rulings() {
    let eps = scale().eps();
    let wall = oval();
    let again = |center: DVec3, first: DVec3, second: f64| {
        Surface::Elliptic(EllipticCylinder::about(center, -DVec3::Z, first, second))
    };
    let turned = 30f64.to_radians();
    let major = DVec3::new(turned.cos(), turned.sin(), 0.0) * 7.0;
    for (other, expected) in [
        (again(DVec3::new(1.0, -2.0, 9.0), -major, 3.0), "same"),
        (
            again(DVec3::new(1.0, -2.0 + eps / 2.0, 0.0), major, 3.0),
            "same",
        ),
        (
            again(
                DVec3::new(1.0, -2.0, 0.0),
                DVec3::Z.cross(major) * 3.0 / 7.0,
                7.0,
            ),
            "same",
        ),
        (
            again(DVec3::new(1.0, -2.0, 0.0), major, 3.0 + 2.0 * eps),
            "parallel",
        ),
        (
            again(DVec3::new(1.0, -2.0 + 2.0 * eps, 0.0), major, 3.0),
            "lines",
        ),
    ] {
        let found = relation(&Surface::Elliptic(wall), &other, scale());
        assert_eq!(class(&found), expected, "{other:?} gives {found:?}");
    }
    let round = cylinder(DVec3::new(1.0, -2.0, 0.0), DVec3::Z, 5.0);
    let found = relation(&Surface::Elliptic(wall), &round, scale());
    let Relation::Parallel(rulings) = &found else {
        panic!("four crossings expected, got {found:?}");
    };
    assert!(rulings.len() == 4 && rulings.iter().all(|ruled| !ruled.touching));
}
