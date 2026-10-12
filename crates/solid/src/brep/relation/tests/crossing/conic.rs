//! Where the conic a plane cuts a wall at a slant crosses a third surface:
//! through the triple's most degenerate pair, by the very call the other
//! curve of the corner makes, so that both give the corner the same bits.

use glam::DVec2;

use super::*;
use crate::brep::conic::{Arm, Conic, Round};
use crate::brep::surface::Cone;

/// A plane leaning on X through `(0, 0, 10)`: it cuts the wall about Z along
/// an ellipse.
fn slanted() -> Plane {
    Plane::through(DVec3::new(0.0, 0.0, 10.0), DVec3::new(0.3, 0.0, 1.0)).0
}

/// A plane leaning on X through `(0, 0, 5)`, half way up the point: it cuts
/// it along an ellipse.
fn lower() -> Plane {
    Plane::through(DVec3::new(0.0, 0.0, 5.0), DVec3::new(0.2, 0.0, 1.0)).0
}

fn wall() -> Cylinder {
    Cylinder::about(DVec3::ZERO, DVec3::Z, 5.0)
}

/// A point about Z: tip at height 10, base of radius 5 on the XY plane.
fn point() -> Cone {
    Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, 5.0), DVec2::new(10.0, 0.0)],
    )
}

fn cut(plane: &Plane, round: Round) -> Vec<Conic> {
    Conic::cut(plane, &round, scale())
}

fn ellipse(plane: &Plane, round: Round) -> Curve {
    let [conic] = cut(plane, round)[..] else {
        panic!("one conic expected");
    };
    Curve::Conic(conic)
}

/// The points of the crossings, as bits, sorted.
fn bits(found: &Crossings) -> Vec<[u64; 3]> {
    let Crossings::At(found) = found else {
        panic!("points expected, got {found:?}");
    };
    let mut bits: Vec<[u64; 3]> = found
        .iter()
        .map(|crossing| crossing.point.to_array().map(f64::to_bits))
        .collect();
    bits.sort();
    bits
}

/// What the other curves a triple holds give against the surface left: the
/// crossings each makes, together.
fn through(curves: &[Curve], surface: &Surface, touching: &Touches) -> Crossings {
    let mut all = Vec::new();
    for curve in curves {
        let Crossings::At(found) = crossings_given(curve, surface, scale(), touching) else {
            panic!("points expected");
        };
        all.extend(found);
    }
    Crossings::At(all)
}

/// The crossings of a conic, each checked to stand within the tolerance of
/// the conic at its parameter and of the surface, and sorted along it.
fn on_the_conic(curve: &Curve, surface: &Surface, touching: &Touches) -> Vec<Crossing> {
    let eps = scale().eps();
    let Crossings::At(found) = crossings_given(curve, surface, scale(), touching) else {
        panic!("points expected");
    };
    assert!(
        found
            .windows(2)
            .all(|pair| pair[0].parameter < pair[1].parameter)
    );
    for crossing in &found {
        assert!(curve.point(crossing.parameter).distance(crossing.point) <= eps);
        assert!(
            surface.distance(crossing.point).abs() <= eps,
            "{crossing:?}"
        );
    }
    found
}

#[test]
fn a_conic_crosses_a_plane_where_the_line_the_planes_share_crosses_its_wall_to_the_bit() {
    let none = Touches::default();
    let side = plane(DVec3::new(2.0, 0.0, 0.0), DVec3::new(1.0, 0.4, 0.1));
    for (slant, round) in [
        (slanted(), Round::Cylinder(wall())),
        (lower(), Round::Cone(point())),
    ] {
        let surface = match round {
            Round::Cylinder(cylinder) => Surface::Cylinder(cylinder),
            Round::Cone(cone) => Surface::Cone(cone),
            Round::Elliptic(wall) => Surface::Elliptic(wall),
        };
        let conic = ellipse(&slant, round);
        let found = on_the_conic(&conic, &side, &none);
        assert_eq!(found.len(), 2, "{round:?}");
        assert!(found.iter().all(|crossing| !crossing.tangent));
        let shared = relation(&Surface::Plane(slant), &side, scale()).curves();
        assert_eq!(
            bits(&Crossings::At(found)),
            bits(&through(&shared, &surface, &none)),
            "{round:?}"
        );
    }
}

#[test]
fn a_conic_lies_along_its_own_plane_and_wall_and_whatever_stands_within_the_tolerance_of_them() {
    let eps = scale().eps();
    let conic = ellipse(&slanted(), Round::Cylinder(wall()));
    let lifted = Plane::through(DVec3::new(0.0, 0.0, 10.0 + eps / 4.0), slanted().normal).0;
    for surface in [
        Surface::Plane(slanted()),
        Surface::Plane(lifted),
        Surface::Cylinder(wall()),
        cylinder(DVec3::new(eps / 4.0, 0.0, 0.0), DVec3::Z, 5.0),
    ] {
        assert_eq!(
            crossings(&conic, &surface, scale()),
            Crossings::Along,
            "{surface:?}"
        );
    }
    let level = plane(DVec3::new(0.0, 0.0, 30.0), DVec3::Z);
    assert!(on_the_conic(&conic, &level, &Touches::default()).is_empty());
    let narrower = cylinder(DVec3::ZERO, DVec3::Z, 4.0);
    assert!(on_the_conic(&conic, &narrower, &Touches::default()).is_empty());
}

#[test]
fn a_conic_touches_a_plane_once_where_that_line_passes_within_the_tolerance_outside_its_wall() {
    let eps = scale().eps();
    let none = Touches::default();
    let conic = ellipse(&slanted(), Round::Cylinder(wall()));
    let beside = |y: f64| plane(DVec3::new(0.0, y, 0.0), DVec3::Y);
    let touched = on_the_conic(&conic, &beside(5.0 + eps / 2.0), &none);
    assert_eq!(tangents(&touched), [true]);
    assert!(on_the_conic(&conic, &beside(5.0 + 2.0 * eps), &none).is_empty());
    let crossed = on_the_conic(&conic, &beside(5.0 - 2.0 * eps), &none);
    assert_eq!(tangents(&crossed), [false, false]);
    let decided = Touches {
        along: true,
        at: Vec::new(),
    };
    let inside = on_the_conic(&conic, &beside(5.0 - eps / 2.0), &decided);
    assert_eq!(tangents(&inside), [true]);
}

#[test]
fn a_conic_crosses_a_wall_parallel_to_its_own_where_their_lines_cross_its_plane() {
    let none = Touches::default();
    let slant = slanted();
    let conic = ellipse(&slant, Round::Cylinder(wall()));
    let crossing = cylinder(DVec3::new(6.0, 0.0, 0.0), DVec3::Z, 5.0);
    let found = on_the_conic(&conic, &crossing, &none);
    assert_eq!(tangents(&found), [false, false]);
    let lines = relation(&Surface::Cylinder(wall()), &crossing, scale()).curves();
    assert_eq!(
        bits(&Crossings::At(found)),
        bits(&through(&lines, &Surface::Plane(slant), &none))
    );

    let touching = cylinder(DVec3::new(10.0, 0.0, 0.0), DVec3::Z, 5.0);
    let touched = on_the_conic(&conic, &touching, &none);
    assert_eq!(tangents(&touched), [true]);
    assert!(touched[0].point.distance(DVec3::new(5.0, 0.0, 8.5)) <= 1e-12 * REACH);
}

#[test]
fn a_conic_crosses_a_perpendicular_wall_where_their_meet_crosses_its_plane_to_the_bit() {
    let none = Touches::default();
    let slant = slanted();
    let conic = ellipse(&slant, Round::Cylinder(wall()));
    let lying = cylinder(DVec3::new(0.0, 0.0, 10.0), DVec3::X, 3.0);
    let found = on_the_conic(&conic, &lying, &none);
    assert_eq!(found.len(), 4, "{found:?}");
    let meets = relation(&Surface::Cylinder(wall()), &lying, scale()).curves();
    assert!(meets.iter().all(|curve| matches!(curve, Curve::Meet(_))));
    assert_eq!(
        bits(&Crossings::At(found)),
        bits(&through(&meets, &Surface::Plane(slant), &none))
    );
}

#[test]
fn a_conic_on_a_cone_crosses_a_coaxial_cylinder_on_their_circle() {
    let none = Touches::default();
    let slant = lower();
    let conic = ellipse(&slant, Round::Cone(point()));
    let coaxial = cylinder(DVec3::ZERO, DVec3::Z, 2.5);
    let found = on_the_conic(&conic, &coaxial, &none);
    assert_eq!(tangents(&found), [false, false]);
    let circles = relation(&Surface::Cone(point()), &coaxial, scale()).curves();
    let [Curve::Circle(circle)] = circles[..] else {
        panic!("one circle expected, got {circles:?}");
    };
    assert!((circle.center.z - 5.0).abs() <= 1e-12 * REACH);
    assert_eq!(
        bits(&Crossings::At(found)),
        bits(&through(&circles, &Surface::Plane(slant), &none))
    );
}

#[test]
fn two_conics_of_one_plane_cross_where_their_walls_meet_it() {
    let none = Touches::default();
    let slant = slanted();
    let other = Cylinder::about(DVec3::new(6.0, 0.0, 0.0), DVec3::Z, 5.0);
    let one = ellipse(&slant, Round::Cylinder(wall()));
    let two = ellipse(&slant, Round::Cylinder(other));
    let from_one = on_the_conic(&one, &Surface::Cylinder(other), &none);
    let from_two = on_the_conic(&two, &Surface::Cylinder(wall()), &none);
    assert_eq!(from_one.len(), 2);
    assert_eq!(
        bits(&Crossings::At(from_one)),
        bits(&Crossings::At(from_two))
    );
}

#[test]
fn an_arm_keeps_only_the_crossings_on_its_own_side() {
    let none = Touches::default();
    let hair = Plane::through(DVec3::new(2.0, 0.0, 0.0), DVec3::new(1.0, 0.0, 1e-6)).0;
    let arms = cut(&hair, Round::Cylinder(wall()));
    assert_eq!(arms.len(), 2, "{arms:?}");
    let level = plane(DVec3::new(0.0, 0.0, 3.0), DVec3::Z);
    let mut sides = Vec::new();
    for arm in arms {
        let Arm::Split { side } = arm.arm else {
            panic!("arms expected, got {arm:?}");
        };
        let found = on_the_conic(&Curve::Conic(arm), &level, &none);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].point.y * f64::from(side) > 0.0, "{found:?}");
        sides.push(side);
    }
    sides.sort();
    assert_eq!(sides, [-1, 1]);
}

#[test]
fn a_conic_against_a_skew_wall_or_a_cone_off_its_axis_is_unsupported() {
    let on_the_wall = ellipse(&slanted(), Round::Cylinder(wall()));
    let on_the_point = ellipse(&lower(), Round::Cone(point()));
    let skew = cylinder(DVec3::ZERO, DVec3::new(1.0, 0.0, 1.0), 2.0);
    let beside = cylinder(DVec3::new(1.0, 0.0, 0.0), DVec3::Z, 2.0);
    let off = Surface::Cone(Cone::through(
        DVec3::new(1.0, 0.0, 0.0),
        DVec3::Z,
        [DVec2::new(0.0, 3.0), DVec2::new(6.0, 0.0)],
    ));
    for (conic, surface) in [
        (&on_the_wall, &skew),
        (&on_the_wall, &off),
        (&on_the_point, &skew),
        (&on_the_point, &beside),
    ] {
        assert_eq!(
            crossings(conic, surface, scale()),
            Crossings::Unsupported,
            "{surface:?}"
        );
    }
}

#[test]
fn a_conic_crosses_a_plane_read_along_its_wall_s_axis_where_that_plane_s_lines_cross_its_own_to_the_bit()
 {
    let eps = scale().eps();
    let none = Touches::default();
    let slant = slanted();
    let conic = ellipse(&slant, Round::Cylinder(wall()));
    let lean = 0.8 * eps / (2.0 * scale().reach());
    let inside = plane(DVec3::new(5.0 - 2e-7, 0.0, 0.0), DVec3::new(1.0, 0.0, lean));
    let found = on_the_conic(&conic, &inside, &none);
    assert_eq!(tangents(&found), [false, false]);
    let lines = relation(&Surface::Cylinder(wall()), &inside, scale()).curves();
    assert!(lines.iter().all(|curve| matches!(curve, Curve::Line(_))));
    assert_eq!(
        bits(&Crossings::At(found)),
        bits(&through(&lines, &Surface::Plane(slant), &none))
    );
}
