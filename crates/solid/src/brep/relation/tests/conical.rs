//! How a cone meets a plane, a cylinder or another cone (#536): every pair of
//! one axis read as two lines of the meridian half-plane, a plane holding the
//! axis as the two rulings it cuts, anything else declined.

use glam::DVec2;

use super::*;
use crate::brep::surface::{Cone, Cylinder};

/// The cone about the line through `point` along `axis` its run from
/// `from` to `to` turns into, each `(h, ρ)` read along `axis` from `point`.
fn cone(point: DVec3, axis: DVec3, from: [f64; 2], to: [f64; 2]) -> Cone {
    Cone::through(point, axis, [DVec2::from(from), DVec2::from(to)])
}

/// A point about Z: tip at height 10, base of radius 5 on the XY plane.
fn point() -> Cone {
    cone(DVec3::ZERO, DVec3::Z, [0.0, 5.0], [10.0, 0.0])
}

#[test]
fn a_plane_square_to_a_cone_s_axis_meets_it_along_a_circle() {
    let tip = point();
    let level = plane(DVec3::new(0.0, 0.0, 4.0), -DVec3::Z);
    let found = relation(&Surface::Cone(tip), &level, scale());
    let Relation::Circle(circle) = found else {
        panic!("a circle expected, got {found:?}");
    };
    assert_eq!(circle, tip.circle(4.0, circle.radius));
    assert!((circle.radius - 3.0).abs() < 1e-12 * REACH, "{circle:?}");
    assert_on_both(&found, &Surface::Cone(tip), &level, 1e-12 * REACH);
    assert_eq!(relation(&level, &Surface::Cone(tip), scale()), found);
}

#[test]
fn a_plane_through_the_apex_square_to_the_axis_touches_it_there() {
    let eps = scale().eps();
    for moved in [0.0, eps / 2.0, -eps / 2.0] {
        let tip = point();
        let at = DVec3::new(0.0, 0.0, 10.0 + moved);
        let level = plane(at, DVec3::Z);
        let found = relation(&Surface::Cone(tip), &level, scale());
        assert_eq!(found, Relation::Apex(at), "{moved}");
        assert!(tip.distance(at).abs() <= eps, "{moved}");
        assert_eq!(relation(&level, &Surface::Cone(tip), scale()), found);
    }
}

/// The two rulings a plane holding the cone's axis cuts, each checked to lie
/// along the cone and on the plane in front of the apex.
fn rulings(tip: &Cone, side: &Surface, within: f64) -> ([Line; 2], Option<DVec3>) {
    let found = relation(&Surface::Cone(*tip), side, scale());
    assert_eq!(relation(side, &Surface::Cone(*tip), scale()), found);
    let Relation::Rulings { lines, apex } = found else {
        panic!("two rulings expected, got {found:?}");
    };
    for line in lines {
        assert!(tip.rules(&line, scale()), "{line:?} along {tip:?}");
        let ahead = along(&Curve::Line(line))
            .into_iter()
            .filter(|point| tip.parameters(*point).y >= tip.apex_at());
        for point in ahead {
            for surface in [&Surface::Cone(*tip), side] {
                assert!(
                    surface.distance(point).abs() <= within,
                    "{point} is {} off {surface:?}",
                    surface.distance(point)
                );
            }
        }
    }
    (lines, apex)
}

#[test]
fn a_plane_holding_the_axis_cuts_two_rulings_crossing_at_the_apex() {
    let tip = point();
    for normal in [
        DVec3::X,
        DVec3::Y,
        DVec3::new(1.0, 1.0, 0.0),
        DVec3::new(-3.0, 1.0, 0.0),
    ] {
        let side = plane(DVec3::ZERO, normal);
        let (lines, apex) = rulings(&tip, &side, 1e-12 * REACH);
        let apex = apex.expect("the apex stands within the box");
        assert!(
            apex.distance(DVec3::new(0.0, 0.0, 10.0)) < 1e-12 * REACH,
            "{apex}"
        );
        for line in lines {
            assert!(line.point(line.parameter(apex)).distance(apex) < 1e-12 * REACH);
        }
        assert_ne!(lines[0], lines[1]);
    }
}

#[test]
fn the_rulings_of_a_near_cylinder_cone_stand_on_it_though_its_apex_is_far() {
    let taper = cone(DVec3::ZERO, DVec3::Z, [0.0, 5.0], [20.0, 5.0 + 1e-6]);
    assert!(taper.apex().length() > 1e6, "{:?}", taper.apex());
    let side = plane(DVec3::ZERO, DVec3::new(1.0, 2.0, 0.0));
    let (_, apex) = rulings(&taper, &side, 1e-12 * REACH);
    assert_eq!(apex, None);
}

#[test]
fn a_plane_a_hair_from_square_is_square() {
    let eps = scale().eps();
    let tilted = DVec3::new(0.25 * eps / REACH, 0.0, 1.0);
    let level = plane(DVec3::new(0.0, 0.0, 4.0), tilted);
    let found = relation(&Surface::Cone(point()), &level, scale());
    let Relation::Circle(circle) = found else {
        panic!("a circle expected, got {found:?}");
    };
    assert!((circle.radius - 3.0).abs() <= eps, "{circle:?}");
    let leaning = DVec3::new(1.0, 0.0, 0.25 * eps / REACH);
    let side = plane(DVec3::ZERO, leaning);
    rulings(&point(), &side, eps);
}

#[test]
fn a_plane_at_a_slant_is_unsupported() {
    for normal in [
        DVec3::new(1.0, 0.0, 1.0),
        DVec3::new(0.0, 1.0, 0.01),
        DVec3::new(1.0, 1.0, 5.0),
    ] {
        let slant = plane(DVec3::new(0.0, 0.0, 5.0), normal);
        assert_eq!(
            relation(&Surface::Cone(point()), &slant, scale()),
            Relation::Unsupported
        );
        assert_eq!(
            relation(&slant, &Surface::Cone(point()), scale()),
            Relation::Unsupported
        );
    }
}

#[test]
fn a_plane_parallel_to_the_axis_off_it_is_unsupported() {
    let eps = scale().eps();
    for off in [2.0 * eps, 1.0, 20.0] {
        let side = plane(DVec3::new(off, 0.0, 0.0), DVec3::X);
        assert_eq!(
            relation(&Surface::Cone(point()), &side, scale()),
            Relation::Unsupported,
            "{off}"
        );
    }
}

#[test]
fn a_coaxial_cylinder_meets_a_cone_along_the_circle_of_its_radius() {
    let tip = point();
    let bore = cylinder(DVec3::new(0.0, 0.0, -7.0), -DVec3::Z, 2.0);
    let found = relation(&Surface::Cone(tip), &bore, scale());
    let Relation::Circle(circle) = found else {
        panic!("a circle expected, got {found:?}");
    };
    assert_eq!(circle.radius, 2.0);
    assert!(
        circle.center.distance(DVec3::new(0.0, 0.0, 6.0)) < 1e-12 * REACH,
        "{circle:?}"
    );
    assert_on_both(&found, &Surface::Cone(tip), &bore, 1e-12 * REACH);
    assert_eq!(relation(&bore, &Surface::Cone(tip), scale()), found);
}

#[test]
fn a_cylinder_at_a_cone_s_rim_gives_the_corner_s_circle_within_rounding() {
    for axis in [DVec3::Z, -DVec3::X, DVec3::new(1.0, 2.0, -2.0)] {
        let through = DVec3::new(3.0, -1.0, 2.0);
        let shaft = Cylinder::about(through, axis, 10.0);
        let chamfer = cone(through, axis, [28.0, 10.0], [30.0, 8.0]);
        let found = relation(&Surface::Cone(chamfer), &Surface::Cylinder(shaft), scale());
        let Relation::Circle(circle) = found else {
            panic!("a circle expected, got {found:?}");
        };
        let rim = Circle::on(&shaft, shaft.axis.dot(through + axis.normalize() * 28.0));
        assert_eq!(circle.radius, rim.radius);
        assert_eq!([circle.axis, circle.u, circle.v], [rim.axis, rim.u, rim.v]);
        assert!(
            circle.center.distance(rim.center) < 1e-12 * REACH,
            "{circle:?} {rim:?}"
        );
    }
}

#[test]
fn a_cylinder_a_hair_off_coaxial_is_coaxial() {
    let eps = scale().eps();
    let bore = cylinder(DVec3::new(eps / 2.0, 0.0, 0.0), DVec3::Z, 2.0);
    let found = relation(&Surface::Cone(point()), &bore, scale());
    let Relation::Circle(circle) = found else {
        panic!("a circle expected, got {found:?}");
    };
    assert_on_both(&found, &Surface::Cone(point()), &bore, eps);
    assert_eq!(circle.radius, 2.0);
    assert!(
        circle.center.distance(DVec3::new(0.0, 0.0, 6.0)) < 1e-12 * REACH,
        "{circle:?}"
    );
}

#[test]
fn a_cylinder_further_off_coaxial_is_unsupported() {
    let eps = scale().eps();
    for off in [2.0 * eps, 0.5, 30.0] {
        let bore = cylinder(DVec3::new(0.0, off, 0.0), DVec3::Z, 2.0);
        assert_eq!(
            relation(&Surface::Cone(point()), &bore, scale()),
            Relation::Unsupported,
            "{off}"
        );
    }
}

#[test]
fn a_radial_cylinder_and_a_cone_at_a_skew_angle_are_unsupported() {
    for axis in [DVec3::X, DVec3::new(1.0, 1.0, 1.0)] {
        let hole = cylinder(DVec3::new(0.0, 0.0, 3.0), axis, 1.0);
        assert_eq!(
            relation(&Surface::Cone(point()), &hole, scale()),
            Relation::Unsupported
        );
        assert_eq!(
            relation(&hole, &Surface::Cone(point()), scale()),
            Relation::Unsupported
        );
    }
}

#[test]
fn a_cone_within_the_tolerance_of_a_plane_or_a_cylinder_across_the_box_is_unsupported() {
    let eps = scale().eps();
    let disc = cone(DVec3::ZERO, DVec3::Z, [0.0, 1.0], [0.1 * eps, 30.0]);
    let level = plane(DVec3::ZERO, DVec3::Z);
    assert_eq!(
        relation(&Surface::Cone(disc), &level, scale()),
        Relation::Unsupported
    );
    let sleeve = cone(DVec3::ZERO, DVec3::Z, [-30.0, 5.0], [30.0, 5.0 + 0.1 * eps]);
    let wall = cylinder(DVec3::ZERO, DVec3::Z, 5.0);
    assert_eq!(
        relation(&Surface::Cone(sleeve), &wall, scale()),
        Relation::Unsupported
    );
}

#[test]
fn the_meridian_solver_gives_the_circle_plane_and_cylinder_give() {
    for axis in [
        DVec3::Z,
        DVec3::new(0.0, -1.0, 0.0),
        DVec3::new(2.0, -1.0, 3.0),
    ] {
        let through = DVec3::new(-4.0, 2.5, 1.0);
        let chamfer = cone(through, axis, [-3.0, 6.0], [5.0, 2.0]);
        for height in [-2.5, 0.0, 1.0, 4.75] {
            let at = through + axis.normalize() * height;
            let level = plane(at, axis);
            let Relation::Circle(on_the_cone) = relation(&Surface::Cone(chamfer), &level, scale())
            else {
                panic!("a circle expected at {height}");
            };
            let wall = cylinder(through, axis, on_the_cone.radius);
            let Relation::Circle(on_the_wall) = relation(&wall, &level, scale()) else {
                panic!("a circle expected at {height}");
            };
            assert_eq!(on_the_cone, on_the_wall, "{axis} {height}");
            let expected = 6.0 - (height + 3.0) / 2.0;
            assert!(
                (on_the_cone.radius - expected).abs() < 1e-12 * REACH,
                "{on_the_cone:?}"
            );
        }
    }
}

/// The relation of two cones, the same whichever comes first.
fn between(one: &Cone, other: &Cone) -> Relation {
    let found = relation(&Surface::Cone(*one), &Surface::Cone(*other), scale());
    assert_eq!(
        relation(&Surface::Cone(*other), &Surface::Cone(*one), scale()),
        found
    );
    found
}

#[test]
fn two_coaxial_cones_of_one_slope_are_one_surface() {
    let eps = scale().eps();
    let tip = point();
    for (other, expected) in [
        (cone(DVec3::ZERO, DVec3::Z, [10.0, 0.0], [0.0, 5.0]), true),
        (cone(DVec3::ZERO, -DVec3::Z, [-2.0, 4.0], [-6.0, 2.0]), true),
        (
            cone(
                DVec3::new(0.0, 0.0, eps / 2.0),
                DVec3::Z,
                [0.0, 5.0],
                [10.0, 0.0],
            ),
            true,
        ),
        (
            cone(
                DVec3::new(0.0, 0.0, 3.0 * eps),
                DVec3::Z,
                [0.0, 5.0],
                [10.0, 0.0],
            ),
            false,
        ),
        (
            cone(DVec3::new(0.0, 0.0, 3.0), DVec3::Z, [0.0, 5.0], [10.0, 0.0]),
            false,
        ),
    ] {
        let found = between(&tip, &other);
        let wanted = if expected {
            Relation::Same { agree: true }
        } else {
            Relation::Apart
        };
        assert_eq!(found, wanted, "{other:?}");
    }
}

#[test]
fn two_coaxial_cones_a_hair_apart_in_slope_cross_along_a_circle() {
    let tip = point();
    let other = cone(DVec3::ZERO, DVec3::Z, [0.0, 5.0], [10.0, 1e-6]);
    let found = between(&tip, &other);
    let Relation::Circle(circle) = found else {
        panic!("a circle expected, got {found:?}");
    };
    let eps = scale().eps();
    assert!((circle.radius - 5.0).abs() < eps, "{circle:?}");
    assert!(circle.center.length() < eps, "{circle:?}");
    assert_on_both(&found, &Surface::Cone(tip), &Surface::Cone(other), eps);
}

#[test]
fn two_coaxial_cones_crossing_beyond_their_apexes_are_apart() {
    let tip = point();
    let other = cone(DVec3::ZERO, DVec3::Z, [12.0, 0.0], [20.0, 8.0]);
    assert_eq!(between(&tip, &other), Relation::Apart);
}

#[test]
fn two_cones_sharing_an_apex_touch_there() {
    let eps = scale().eps();
    let tip = point();
    for other in [
        cone(DVec3::ZERO, DVec3::Z, [10.0, 0.0], [20.0, 5.0]),
        cone(DVec3::ZERO, DVec3::Z, [10.0, 0.0], [0.0, 8.0]),
        cone(DVec3::ZERO, DVec3::Z, [10.0 + eps / 2.0, 0.0], [20.0, 1.0]),
        cone(
            DVec3::ZERO,
            -DVec3::Z,
            [-10.0 + eps / 2.0, 0.0],
            [-11.0, 1e3],
        ),
    ] {
        let found = between(&tip, &other);
        let Relation::Apex(apex) = found else {
            panic!("the apex expected, got {found:?} for {other:?}");
        };
        for surface in [tip, other] {
            assert!(
                surface.distance(apex).abs() <= eps,
                "{apex} off {surface:?}"
            );
        }
    }
}

#[test]
fn a_cone_s_relation_is_the_same_whichever_comes_first() {
    let eps = scale().eps();
    let tip = point();
    let others = [
        cone(
            DVec3::new(eps / 3.0, 0.0, 0.0),
            DVec3::Z,
            [0.0, 5.0],
            [10.0, 1e-6],
        ),
        cone(
            DVec3::new(0.0, eps / 3.0, 0.0),
            -DVec3::Z,
            [-1.0, 1.0],
            [-3.0, 6.0],
        ),
        cone(DVec3::new(1.0, 0.0, 0.0), DVec3::Z, [0.0, 5.0], [10.0, 0.0]),
        cone(DVec3::ZERO, DVec3::X, [0.0, 5.0], [10.0, 0.0]),
    ];
    for other in others {
        between(&tip, &other);
    }
    for surface in [
        plane(DVec3::new(0.0, 0.0, 3.0), DVec3::Z),
        plane(DVec3::ZERO, DVec3::new(1.0, -1.0, 0.0)),
        cylinder(DVec3::new(eps / 3.0, 0.0, 0.0), -DVec3::Z, 1.0),
    ] {
        assert_eq!(
            relation(&Surface::Cone(tip), &surface, scale()),
            relation(&surface, &Surface::Cone(tip), scale())
        );
    }
}

#[test]
fn a_plane_beyond_the_apex_misses_the_cone() {
    let eps = scale().eps();
    for height in [10.0 + 2.0 * eps, 12.0, 39.0] {
        let level = plane(DVec3::new(0.0, 0.0, height), DVec3::Z);
        assert_eq!(
            relation(&Surface::Cone(point()), &level, scale()),
            Relation::Apart,
            "{height}"
        );
    }
}
