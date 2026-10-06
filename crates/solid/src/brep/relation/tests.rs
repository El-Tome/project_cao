use glam::DVec3;

use super::*;
use crate::brep::curve::Curve;
use crate::brep::meet::Configuration;
use crate::brep::surface::{Cylinder, Plane};

const REACH: f64 = 40.0;

fn scale() -> Scale {
    Scale::of(REACH)
}

fn plane(point: DVec3, normal: DVec3) -> Surface {
    Surface::Plane(Plane::through(point, normal).0)
}

fn cylinder(point: DVec3, axis: DVec3, radius: f64) -> Surface {
    Surface::Cylinder(Cylinder::about(point, axis, radius))
}

/// Points along a curve: a line across the box, a closed curve all round.
fn along(curve: &Curve) -> Vec<DVec3> {
    let (from, to) = match curve.period() {
        Some(period) => (0.0, period),
        None => (-2.0 * REACH, 2.0 * REACH),
    };
    (0..=64)
        .map(|step| curve.point(from + (to - from) * step as f64 / 64.0))
        .collect()
}

fn assert_on_both(relation: &Relation, one: &Surface, other: &Surface, within: f64) {
    for curve in relation.curves() {
        for point in along(&curve) {
            for surface in [one, other] {
                assert!(
                    surface.distance(point).abs() <= within,
                    "{relation:?}: {point} is {} off {surface:?}",
                    surface.distance(point)
                );
            }
        }
    }
}

#[test]
fn two_planes_across_each_other_meet_along_one_line_on_both() {
    let top = plane(DVec3::new(0.0, 0.0, 10.0), DVec3::Z);
    let side = plane(DVec3::new(20.0, 0.0, 0.0), -DVec3::X);
    let found = relation(&top, &side, scale());
    let Relation::Line(line) = found else {
        panic!("one line expected, got {found:?}");
    };
    assert_eq!(line, Line::through(DVec3::new(20.0, 0.0, 10.0), DVec3::Y));
    assert_on_both(&found, &top, &side, 1e-12 * REACH);
}

#[test]
fn parallel_planes_are_one_within_the_tolerance_and_apart_beyond_it() {
    let eps = scale().eps();
    for normal in [DVec3::X, DVec3::Y, DVec3::Z] {
        let at = |offset: f64, towards: f64| plane(normal * offset, normal * towards);
        let top = at(10.0, 1.0);
        for (moved, facing, expected) in [
            (0.0, 1.0, Relation::Same { agree: true }),
            (0.0, -1.0, Relation::Same { agree: true }),
            (eps / 2.0, 1.0, Relation::Same { agree: true }),
            (-eps / 2.0, -1.0, Relation::Same { agree: true }),
            (2.0 * eps, 1.0, Relation::Apart),
            (-2.0 * eps, -1.0, Relation::Apart),
        ] {
            let other = at(10.0 + moved, facing);
            assert_eq!(
                relation(&top, &other, scale()),
                expected,
                "{moved} {facing}"
            );
            assert_eq!(
                relation(&other, &top, scale()),
                expected,
                "{moved} {facing}"
            );
        }
    }
}

#[test]
fn a_plane_square_to_a_cylinder_s_axis_meets_it_along_a_circle() {
    let wall = cylinder(DVec3::new(8.0, 0.0, 0.0), DVec3::Z, 5.0);
    let top = plane(DVec3::new(0.0, 0.0, 10.0), -DVec3::Z);
    let found = relation(&wall, &top, scale());
    let Relation::Circle(circle) = found else {
        panic!("a circle expected, got {found:?}");
    };
    let Surface::Cylinder(wall_cylinder) = wall else {
        unreachable!()
    };
    assert_eq!(circle, Circle::on(&wall_cylinder, 10.0));
    assert_on_both(&found, &wall, &top, 1e-12 * REACH);
    assert_eq!(relation(&top, &wall, scale()), found);
}

#[test]
fn a_plane_along_a_cylinder_s_axis_cuts_it_along_two_lines_or_touches_it_along_one() {
    let eps = scale().eps();
    let hole = cylinder(DVec3::new(15.0, 0.0, 0.0), DVec3::Z, 5.0);
    for (side, expected) in [
        (17.0, "lines"),
        (20.0, "tangent"),
        (20.0 + eps / 2.0, "tangent"),
        (20.0 - eps / 2.0, "tangent"),
        (20.0 + 2.0 * eps, "apart"),
        (20.0 - 2.0 * eps, "lines"),
        (10.0 - eps / 2.0, "tangent"),
        (10.0 - 2.0 * eps, "apart"),
        (25.0, "apart"),
    ] {
        let wall = plane(DVec3::new(side, 0.0, 0.0), DVec3::X);
        let found = relation(&hole, &wall, scale());
        let class = match &found {
            Relation::Lines(_) => "lines",
            Relation::Tangent(_) => "tangent",
            Relation::Apart => "apart",
            _ => "other",
        };
        assert_eq!(class, expected, "{side}: {found:?}");
        let within = if (side - 20.0_f64).abs().min((side - 10.0).abs()) < 1e-6 {
            eps
        } else {
            1e-12 * REACH
        };
        assert_on_both(&found, &hole, &wall, within);
        assert_eq!(relation(&wall, &hole, scale()), found);
    }
}

#[test]
fn a_plane_touching_a_cylinder_exactly_meets_it_along_the_line_where_it_touches() {
    let hole = cylinder(DVec3::new(15.0, 0.0, 0.0), DVec3::Z, 5.0);
    let wall = plane(DVec3::new(20.0, 0.0, 0.0), DVec3::X);
    assert_eq!(
        relation(&hole, &wall, scale()),
        Relation::Tangent(Line::through(DVec3::new(20.0, 0.0, 0.0), DVec3::Z))
    );
}

#[test]
fn a_plane_oblique_to_a_cylinder_s_axis_is_not_supported() {
    let hole = cylinder(DVec3::ZERO, DVec3::Z, 5.0);
    let slope = plane(DVec3::ZERO, DVec3::new(1.0, 0.0, 1.0));
    assert_eq!(relation(&hole, &slope, scale()), Relation::Unsupported);
    assert_eq!(relation(&slope, &hole, scale()), Relation::Unsupported);
}

fn class(relation: &Relation) -> &'static str {
    match relation {
        Relation::Apart => "apart",
        Relation::Same { .. } => "same",
        Relation::Line(_) => "line",
        Relation::Lines(_) => "lines",
        Relation::Tangent(_) => "tangent",
        Relation::Circle(_) => "circle",
        Relation::Meet(_) => "meet",
        Relation::Rulings { .. } => "rulings",
        Relation::Apex(_) => "apex",
        Relation::Unsupported => "unsupported",
    }
}

#[test]
fn parallel_cylinders_cross_touch_or_are_one_as_their_axes_stand() {
    let eps = scale().eps();
    let stock = cylinder(DVec3::ZERO, DVec3::Z, 20.0);
    for (x, radius, expected) in [
        (0.0, 20.0, "same"),
        (eps / 2.0, 20.0 + eps / 2.0, "same"),
        (0.0, 20.0 + 2.0 * eps, "apart"),
        (2.0 * eps, 20.0, "lines"),
        (0.0, 5.0, "apart"),
        (8.0, 5.0, "apart"),
        (15.0, 5.0, "tangent"),
        (15.0 + eps / 2.0, 5.0, "tangent"),
        (15.0 - eps / 2.0, 5.0, "tangent"),
        (15.0 + 2.0 * eps, 5.0, "lines"),
        (15.0 - 2.0 * eps, 5.0, "apart"),
        (20.0, 5.0, "lines"),
        (25.0, 5.0, "tangent"),
        (25.0 + eps / 2.0, 5.0, "tangent"),
        (25.0 - 2.0 * eps, 5.0, "lines"),
        (25.0 + 2.0 * eps, 5.0, "apart"),
        (-25.0 + eps / 2.0, 5.0, "tangent"),
        (40.0, 5.0, "apart"),
    ] {
        let tool = cylinder(DVec3::new(x, 0.0, 0.0), -DVec3::Z, radius);
        let found = relation(&stock, &tool, scale());
        assert_eq!(class(&found), expected, "{x} {radius}: {found:?}");
        let near_a_touch = [15.0_f64, 25.0, -25.0]
            .iter()
            .any(|touch| (x - touch).abs() < 1e-6);
        let within = if near_a_touch { eps } else { 1e-12 * REACH };
        assert_on_both(&found, &stock, &tool, within);
        assert_eq!(relation(&tool, &stock, scale()), found);
    }
}

#[test]
fn parallel_cylinders_a_hair_off_one_axis_touch_inside_where_their_gap_closes() {
    let eps = scale().eps();
    let stock = cylinder(DVec3::ZERO, DVec3::Z, 20.0);
    for (x, expected) in [
        (0.9 * eps, "tangent"),
        (0.6 * eps, "tangent"),
        (0.4 * eps, "apart"),
        (0.0, "apart"),
    ] {
        let tool = cylinder(DVec3::new(x, 0.0, 0.0), DVec3::Z, 20.0 - 1.5 * eps);
        let found = relation(&stock, &tool, scale());
        assert_eq!(class(&found), expected, "{x}: {found:?}");
        assert_on_both(&found, &stock, &tool, eps);
        assert_eq!(relation(&tool, &stock, scale()), found);
    }
}

#[test]
fn parallel_cylinders_touching_exactly_meet_along_the_line_where_they_touch() {
    let stock = cylinder(DVec3::ZERO, DVec3::Z, 20.0);
    for (x, touch) in [(15.0, 20.0), (25.0, 20.0), (-15.0, -20.0)] {
        let tool = cylinder(DVec3::new(x, 0.0, 0.0), DVec3::Z, 5.0);
        assert_eq!(
            relation(&stock, &tool, scale()),
            Relation::Tangent(Line::through(DVec3::new(touch, 0.0, 0.0), DVec3::Z))
        );
    }
}

#[test]
fn perpendicular_cylinders_meet_along_their_curve_and_skew_ones_are_not_supported() {
    let stock = cylinder(DVec3::ZERO, DVec3::Z, 20.0);
    let across = cylinder(DVec3::new(0.0, 15.0, 5.0), DVec3::X, 5.0);
    let found = relation(&stock, &across, scale());
    let Relation::Meet(meeting) = &found else {
        panic!("a meeting expected, got {found:?}");
    };
    assert_eq!(meeting.configuration, Configuration::FigureOfEight);
    assert_eq!(found.points().len(), 1);
    assert_on_both(&found, &stock, &across, 1e-12 * REACH);
    assert_eq!(relation(&across, &stock, scale()), found);

    let far = cylinder(DVec3::new(0.0, 40.0, 5.0), DVec3::X, 5.0);
    assert_eq!(relation(&stock, &far, scale()), Relation::Apart);

    let skew = cylinder(DVec3::ZERO, DVec3::new(1.0, 0.0, 1.0), 5.0);
    assert_eq!(relation(&stock, &skew, scale()), Relation::Unsupported);
}

#[test]
fn two_planes_one_within_the_tolerance_say_when_their_canonical_normals_part() {
    let tilted = DVec3::new(1.0, -1.0 - 1e-12, 0.0);
    let one = plane(DVec3::new(3.0, -3.0, 5.0), DVec3::new(1.0, -1.0, 0.0));
    let other = plane(DVec3::new(3.0, -3.0, 5.0), tilted);
    assert_eq!(
        relation(&one, &other, scale()),
        Relation::Same { agree: false }
    );
}

mod crossing;
mod lattice;
