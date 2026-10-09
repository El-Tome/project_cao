//! A plane at a slant to a cylinder or a cone (#552), decided as the kernel
//! decides it, `planar.rs` and `conical.rs` asking `sliced.rs` for the pairs
//! `at_a_slant` picks out: a conic, the rulings through the apex, the ruling
//! of touch, the apex alone, or nothing.

use glam::DVec2;

use super::*;
use crate::brep::canonical::distance;
use crate::brep::conic::{Arm, Conic};
use crate::brep::surface::Cone;

/// The relation of two surfaces, the same whichever came first, and for a
/// pair at a slant what `sliced.rs` alone decides.
fn decided(one: &Surface, other: &Surface) -> Relation {
    let found = relation(one, other, scale());
    assert_eq!(
        relation(other, one, scale()),
        found,
        "{one:?} and {other:?} in either order"
    );
    if at_a_slant(one, other, scale()) {
        assert_eq!(found, sliced_alone(one, other), "{one:?} and {other:?}");
    }
    found
}

/// The conics of a relation that cuts some.
fn conics(found: &Relation) -> &[Conic] {
    let Relation::Conic(conics) = found else {
        panic!("a conic expected, got {found:?}");
    };
    conics
}

/// Points of a conic within the box: round its period, and at parameters
/// from a trillionth to a thousand either way of its anchor, spread by their
/// logarithm, for an arm a hair from an apex or a hairpin runs across the box
/// within a sliver of its parameter.
fn in_the_box(conic: &Conic) -> Vec<DVec3> {
    let round = conic
        .period()
        .map(|period| (0..256).map(move |step| period * step as f64 / 256.0))
        .into_iter()
        .flatten();
    let spread = (0..=512)
        .map(|step| 10_f64.powf(-12.0 + 15.0 * step as f64 / 512.0))
        .flat_map(|t| [t, -t]);
    round
        .chain(spread)
        .map(|t| conic.point(t))
        .filter(|point| point.abs().max_element() <= REACH)
        .collect()
}

/// Every point of every conic in the box stands on both surfaces to a tenth
/// of the tolerance, and there are some.
fn assert_cut_on_both(found: &Relation, one: &Surface, other: &Surface) {
    let held = scale().eps() / 10.0;
    let mut count = 0;
    for conic in conics(found) {
        for point in in_the_box(conic) {
            count += 1;
            for surface in [one, other] {
                assert!(
                    surface.distance(point).abs() <= held,
                    "{point} is {} off {surface:?}",
                    surface.distance(point)
                );
            }
        }
    }
    assert!(count > 16, "{count} points of {found:?} in the box");
}

/// A shaft of radius 5 about the Z axis through `(1, −2)`.
fn shaft() -> Surface {
    cylinder(DVec3::new(1.0, -2.0, 0.0), DVec3::Z, 5.0)
}

/// The plane's normal turned from `from` towards `to` by `angle`.
fn leaning(from: DVec3, to: DVec3, angle: f64) -> DVec3 {
    from * angle.cos() + to * angle.sin()
}

/// The angle `parallel` and `square` read directions at.
fn tol() -> f64 {
    scale().eps() / (2.0 * REACH)
}

#[test]
fn a_plane_at_a_slant_cuts_a_cylinder_along_one_closed_ellipse() {
    let tilt = 0.5_f64;
    let slant = plane(
        DVec3::new(1.0, -2.0, 3.0),
        leaning(DVec3::Z, DVec3::X, tilt),
    );
    let found = decided(&shaft(), &slant);
    let [ellipse] = conics(&found) else {
        panic!("one ellipse expected, got {found:?}");
    };
    assert_eq!(ellipse.arm, Arm::Whole);
    let period = ellipse.period().expect("an ellipse closes");
    let across = ellipse.point(0.0).distance(ellipse.point(period / 2.0));
    assert!(
        (across - 10.0 / tilt.cos()).abs() <= 1e-12 * REACH,
        "{across} between the vertices"
    );
    assert_cut_on_both(&found, &shaft(), &slant);
}

#[test]
fn a_plane_tangent_to_a_cylinder_along_a_ruling_a_hair_off_parallel_is_that_ruling() {
    let touch = DVec3::new(6.0, -2.0, 0.5);
    for share in [0.0, 0.25, -0.25] {
        let side = plane(touch, leaning(DVec3::X, DVec3::Z, share * tol()));
        assert!(!at_a_slant(&shaft(), &side, scale()), "{share}");
        let found = decided(&shaft(), &side);
        let Relation::Tangent(line) = found else {
            panic!("the ruling of touch expected at {share}, got {found:?}");
        };
        assert!(line.direction.cross(DVec3::Z).length() <= 1e-12);
        assert!((line.origin - DVec3::new(6.0, -2.0, 0.0)).length() <= scale().eps());
    }
    let side = plane(touch, leaning(DVec3::X, DVec3::Z, 4.0 * tol()));
    let found = decided(&shaft(), &side);
    let [hairpin] = conics(&found) else {
        panic!("one ellipse expected, got {found:?}");
    };
    assert_eq!(hairpin.arm, Arm::Whole);
    let off = (hairpin.anchor - touch).truncate().length();
    assert!(
        off <= scale().eps() && hairpin.anchor.abs().max_element() <= REACH,
        "{hairpin:?} anchored off the line of touch"
    );
    assert_cut_on_both(&found, &shaft(), &side);
}

#[test]
fn a_plane_a_hair_past_square_to_a_cylinder_s_axis_still_cuts_the_same_circle() {
    let eps = scale().eps();
    let level = DVec3::new(1.0, -2.0, 3.0);
    let ring = Circle::on(
        &Cylinder::about(DVec3::new(1.0, -2.0, 0.0), DVec3::Z, 5.0),
        3.0,
    );
    let flat = plane(level, leaning(DVec3::Z, DVec3::X, 0.25 * tol()));
    assert!(!at_a_slant(&shaft(), &flat, scale()));
    let Relation::Circle(circle) = decided(&shaft(), &flat) else {
        panic!("the circle expected");
    };
    assert!(circle.center.distance(ring.center) <= eps, "{circle:?}");
    let tilted = plane(level, leaning(DVec3::Z, DVec3::X, 4.0 * tol()));
    let found = decided(&shaft(), &tilted);
    let [ellipse] = conics(&found) else {
        panic!("one ellipse expected, got {found:?}");
    };
    assert_eq!(ellipse.arm, Arm::Whole);
    for point in in_the_box(ellipse) {
        let off = distance(&Curve::Circle(ring), point);
        assert!(off <= eps, "{point} stands {off} off the circle");
    }
    assert_cut_on_both(&found, &shaft(), &tilted);
}

#[test]
fn a_plane_a_hair_from_parallel_to_a_cylinder_s_axis_cuts_two_arms_or_is_its_lines() {
    let through = DVec3::new(3.0, -2.0, 0.5);
    let side = plane(through, leaning(DVec3::X, DVec3::Z, 0.25 * tol()));
    assert_eq!(class(&decided(&shaft(), &side)), "lines");
    let leaning_side = plane(through, leaning(DVec3::X, DVec3::Z, 4.0 * tol()));
    let found = decided(&shaft(), &leaning_side);
    let arms = conics(&found);
    let sides: Vec<Arm> = arms.iter().map(|conic| conic.arm).collect();
    assert_eq!(
        sides,
        [Arm::Split { side: 1 }, Arm::Split { side: -1 }],
        "{found:?}"
    );
    for arm in arms {
        assert!(arm.anchor.abs().max_element() <= REACH, "{arm:?}");
    }
    assert_cut_on_both(&found, &shaft(), &leaning_side);
}

/// A point about the Z axis: its tip at height 10, its base of radius 5 on
/// the XY plane, its slope a half.
fn tip() -> Cone {
    Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, 5.0), DVec2::new(10.0, 0.0)],
    )
}

fn point() -> Surface {
    Surface::Cone(tip())
}

const APEX: DVec3 = DVec3::new(0.0, 0.0, 10.0);

/// The normal of a plane holding the point's ruling at `−X`.
fn along_the_ruling() -> DVec3 {
    DVec3::new(2.0, 0.0, -1.0).normalize()
}

#[test]
fn a_plane_parallel_to_a_cone_s_axis_off_it_cuts_one_branch_of_a_hyperbola() {
    for off in [2.0, 2.0 * scale().eps(), -4.5] {
        let side = plane(DVec3::new(off, 0.0, 0.0), DVec3::X);
        assert!(at_a_slant(&point(), &side, scale()), "{off}");
        let found = decided(&point(), &side);
        let [branch] = conics(&found) else {
            panic!("one branch expected at {off}, got {found:?}");
        };
        assert!(branch.k < 0.0 && branch.period().is_none(), "{branch:?}");
        assert_cut_on_both(&found, &point(), &side);
    }
}

#[test]
fn a_plane_parallel_to_a_ruling_cuts_a_parabola_on_the_cone_s_own_nappe() {
    let slope = plane(DVec3::new(1.0, 0.0, 0.0), along_the_ruling());
    let found = decided(&point(), &slope);
    let [parabola] = conics(&found) else {
        panic!("one parabola expected, got {found:?}");
    };
    assert!(parabola.k.abs() <= 1e-12, "{parabola:?}");
    assert!(parabola.period().is_none());
    assert!(
        parabola.anchor.distance(DVec3::new(3.0, 0.0, 4.0)) <= 1e-12 * REACH,
        "{parabola:?} from its vertex"
    );
    assert_cut_on_both(&found, &point(), &slope);
}

#[test]
fn a_plane_at_a_slant_across_a_cone_cuts_an_ellipse_on_its_own_nappe_or_nothing() {
    let normal = leaning(DVec3::Z, DVec3::X, 0.3);
    let across = plane(DVec3::new(0.0, 0.0, 4.0), normal);
    let found = decided(&point(), &across);
    let [ellipse] = conics(&found) else {
        panic!("one ellipse expected, got {found:?}");
    };
    assert!(ellipse.k > 0.0 && ellipse.period().is_some(), "{ellipse:?}");
    assert_eq!(ellipse.arm, Arm::Whole);
    assert_cut_on_both(&found, &point(), &across);
    let above = plane(DVec3::new(0.0, 0.0, 14.0), normal);
    assert_eq!(decided(&point(), &above), Relation::Apart);
}

/// The two rulings through the apex a relation holds, each checked to run
/// along the cone, through the apex and in the plane across the box.
fn through_the_apex(found: &Relation, side: &Surface) -> [Line; 2] {
    let Relation::Rulings { lines, apex } = found else {
        panic!("two rulings expected, got {found:?}");
    };
    assert!(apex.is_some_and(|apex| apex.distance(APEX) <= scale().eps()));
    for line in lines {
        assert!(tip().rules(line, scale()), "{line:?} along {:?}", tip());
        assert!(distance(&Curve::Line(*line), APEX) <= scale().eps());
        for t in [-2.0 * REACH, 0.0, 2.0 * REACH] {
            let off = side.distance(line.point(line.parameter(DVec3::ZERO) + t));
            assert!(off.abs() <= scale().eps(), "{line:?} {off} off the plane");
        }
    }
    *lines
}

#[test]
fn a_plane_through_a_cone_s_apex_at_a_slant_cuts_two_rulings_through_their_feet() {
    let eps = scale().eps();
    let normal = leaning(DVec3::X, DVec3::Z, 0.3);
    for off in [0.0, eps / 2.0, -eps / 2.0] {
        let steep = plane(APEX + normal * off, normal);
        assert!(at_a_slant(&point(), &steep, scale()));
        let lines = through_the_apex(&decided(&point(), &steep), &steep);
        assert!(lines[0].direction.cross(lines[1].direction).length() > 0.1);
    }
}

#[test]
fn a_plane_through_the_apex_flatter_than_the_slope_meets_the_cone_at_its_apex_alone() {
    let flat = plane(APEX, leaning(DVec3::Z, DVec3::X, 0.3));
    let found = decided(&point(), &flat);
    let Relation::Apex(apex) = found else {
        panic!("the apex expected, got {found:?}");
    };
    assert!(apex.distance(APEX) <= scale().eps());
}

#[test]
fn a_plane_touching_a_cone_along_a_ruling_is_that_ruling_and_its_apex() {
    let ruling = DVec3::new(-1.0, 0.0, -2.0).normalize();
    let turned = |share: f64| leaning(along_the_ruling(), ruling, share * tol());
    for share in [0.0, 0.5, -0.5] {
        let touching = plane(APEX, turned(share));
        let found = decided(&point(), &touching);
        let Relation::Ruling { line, apex } = found else {
            panic!("the ruling of touch expected at {share}, got {found:?}");
        };
        assert!(apex.is_some_and(|apex| apex.distance(APEX) <= 1e-12 * REACH));
        assert!(tip().rules(&line, scale()), "{line:?}");
        assert!(line.direction.cross(ruling).length() <= 1e-12, "{line:?}");
        let foot = DVec3::new(-5.0, 0.0, 0.0);
        assert!(distance(&Curve::Line(line), foot) <= 1e-12 * REACH);
    }
    let mut both = [4.0, -4.0].map(|share| class(&decided(&point(), &plane(APEX, turned(share)))));
    both.sort();
    assert_eq!(both, ["apex", "rulings"]);
}

#[test]
fn a_plane_a_hair_from_the_apex_is_decided_through_it_and_two_hairs_off_cuts_a_conic() {
    let eps = scale().eps();
    let normal = leaning(DVec3::X, DVec3::Z, 0.3);
    for off in [0.9 * eps, -0.9 * eps] {
        let steep = plane(APEX + normal * off, normal);
        through_the_apex(&decided(&point(), &steep), &steep);
    }
    for off in [2.0 * eps, -2.0 * eps, 1e3 * eps] {
        let steep = plane(APEX + normal * off, normal);
        let found = decided(&point(), &steep);
        let [branch] = conics(&found) else {
            panic!("one branch expected at {off}, got {found:?}");
        };
        assert!(branch.k < 0.0, "{branch:?}");
        assert_cut_on_both(&found, &point(), &steep);
    }
}

#[test]
fn a_conic_closer_to_the_axis_than_the_tolerance_is_the_apex() {
    let eps = scale().eps();
    let normal = leaning(DVec3::Z, DVec3::X, 0.3);
    let near = plane(APEX - normal * (1.5 * eps), normal);
    let found = decided(&point(), &near);
    let Relation::Apex(apex) = found else {
        panic!("the apex expected, got {found:?}");
    };
    assert!(apex.distance(APEX) <= 2.0 * eps, "{apex}");
    assert!(
        near.distance(apex).abs() <= 1e-15 * REACH,
        "{apex} off the plane"
    );
    let further = plane(APEX - normal * (4.0 * eps), normal);
    let [ring] = conics(&decided(&point(), &further))[..] else {
        panic!("one ellipse expected");
    };
    assert!(ring.k > 0.0, "{ring:?}");
}

#[test]
fn a_plane_meeting_only_the_other_nappe_stands_apart() {
    let eps = scale().eps();
    let flat = leaning(DVec3::Z, DVec3::X, 0.3);
    for (through, normal) in [
        (APEX + flat * (2.0 * eps), flat),
        (DVec3::new(0.0, 0.0, 14.0), flat),
        (DVec3::new(-1.0, 0.0, 12.0), along_the_ruling()),
    ] {
        let beyond = plane(through, normal);
        assert!(at_a_slant(&point(), &beyond, scale()));
        assert_eq!(decided(&point(), &beyond), Relation::Apart, "{through}");
    }
}

#[test]
fn skew_walls_and_a_wall_off_a_cone_s_axis_stay_unsupported() {
    let skew = cylinder(DVec3::new(0.0, 0.0, 5.0), DVec3::new(1.0, 1.0, 0.3), 2.0);
    let off_the_axis = cylinder(DVec3::new(1.0, 0.0, 0.0), DVec3::Z, 2.0);
    let leaning_point = Surface::Cone(Cone::through(
        DVec3::new(0.0, 1.0, 0.0),
        DVec3::new(0.2, 0.0, 1.0),
        [DVec2::new(0.0, 3.0), DVec2::new(6.0, 0.0)],
    ));
    for (one, other) in [
        (&shaft(), &skew),
        (&point(), &skew),
        (&point(), &off_the_axis),
        (&point(), &leaning_point),
    ] {
        assert!(!at_a_slant(one, other, scale()));
        assert_eq!(decided(one, other), Relation::Unsupported, "{other:?}");
    }
}
