//! Where a curve crosses a surface: what the corners of a triple stand on.

use super::*;
use crate::brep::curve::Meet;
use crate::brep::meet::Meeting;

/// The crossings found, each checked to lie on the curve at its parameter and
/// on the surface, and the list sorted.
fn found(curve: &Curve, surface: &Surface, within: f64) -> Vec<Crossing> {
    let Crossings::At(found) = crossings(curve, surface, scale()) else {
        panic!("points expected");
    };
    assert!(
        found
            .windows(2)
            .all(|pair| pair[0].parameter < pair[1].parameter)
    );
    for crossing in &found {
        assert!((curve.point(crossing.parameter) - crossing.point).length() < 1e-12 * REACH);
        assert!(
            surface.distance(crossing.point).abs() <= within,
            "{crossing:?} is {} off {surface:?}",
            surface.distance(crossing.point)
        );
    }
    found
}

fn tangents(found: &[Crossing]) -> Vec<bool> {
    found.iter().map(|crossing| crossing.tangent).collect()
}

#[test]
fn a_line_crosses_a_plane_once_unless_it_runs_along_or_beside_it() {
    let eps = scale().eps();
    let line = Curve::Line(Line::through(
        DVec3::new(3.0, 4.0, 0.0),
        DVec3::new(1.0, 0.0, 2.0),
    ));
    let top = plane(DVec3::new(0.0, 0.0, 10.0), DVec3::Z);
    let once = found(&line, &top, 1e-12 * REACH);
    assert_eq!(tangents(&once), [false]);
    assert!((once[0].point - DVec3::new(8.0, 4.0, 10.0)).length() < 1e-12 * REACH);

    let flat = Curve::Line(Line::through(
        DVec3::new(3.0, 4.0, 10.0 + eps / 2.0),
        DVec3::X,
    ));
    assert_eq!(crossings(&flat, &top, scale()), Crossings::Along);
    let above = Curve::Line(Line::through(
        DVec3::new(3.0, 4.0, 10.0 + 2.0 * eps),
        DVec3::X,
    ));
    assert!(found(&above, &top, 0.0).is_empty());
}

#[test]
fn a_line_crosses_a_cylinder_twice_touches_it_once_or_misses_it() {
    let eps = scale().eps();
    let hole = cylinder(DVec3::new(15.0, 0.0, 0.0), DVec3::Z, 5.0);
    let across = |x: f64| {
        Curve::Line(Line::through(
            DVec3::new(x, 0.0, 4.0),
            DVec3::new(0.0, 1.0, 1.0),
        ))
    };
    assert_eq!(
        tangents(&found(&across(17.0), &hole, 1e-12 * REACH)),
        [false, false]
    );
    for x in [20.0, 20.0 + eps / 2.0, 20.0 - eps / 2.0, 10.0 + eps / 2.0] {
        let touch = found(&across(x), &hole, eps);
        assert_eq!(tangents(&touch), [true], "{x}");
        assert!((touch[0].point.x - x).abs() <= eps);
    }
    assert!(found(&across(20.0 + 2.0 * eps), &hole, 0.0).is_empty());
    assert_eq!(
        tangents(&found(&across(20.0 - 2.0 * eps), &hole, 1e-12 * REACH)),
        [false, false]
    );

    let ruling = Curve::Line(Line::through(
        DVec3::new(20.0 + eps / 2.0, 0.0, 0.0),
        DVec3::Z,
    ));
    assert_eq!(crossings(&ruling, &hole, scale()), Crossings::Along);
    let beside = Curve::Line(Line::through(
        DVec3::new(20.0 + 2.0 * eps, 0.0, 0.0),
        DVec3::Z,
    ));
    assert!(found(&beside, &hole, 0.0).is_empty());
}

fn circle(center: DVec3, radius: f64, height: f64) -> Curve {
    Curve::Circle(Circle::on(
        &Cylinder::about(center, DVec3::Z, radius),
        height,
    ))
}

#[test]
fn a_circle_crosses_a_plane_twice_touches_it_once_or_lies_in_it() {
    let eps = scale().eps();
    let rim = circle(DVec3::new(8.0, 0.0, 0.0), 5.0, 10.0);
    let wall = |x: f64| plane(DVec3::new(x, 0.0, 0.0), DVec3::X);
    let twice = found(&rim, &wall(10.0), 1e-12 * REACH);
    assert_eq!(tangents(&twice), [false, false]);
    assert!((twice[0].point - DVec3::new(10.0, -(21.0_f64.sqrt()), 10.0)).length() < 1e-12 * REACH);
    for x in [13.0, 13.0 + eps / 2.0, 3.0 - eps / 2.0] {
        assert_eq!(tangents(&found(&rim, &wall(x), eps)), [true], "{x}");
    }
    assert!(found(&rim, &wall(13.0 + 2.0 * eps), 0.0).is_empty());
    assert_eq!(
        tangents(&found(&rim, &wall(13.0 - 2.0 * eps), 1e-12 * REACH)),
        [false, false]
    );

    let top = |z: f64| plane(DVec3::new(0.0, 0.0, z), DVec3::Z);
    assert_eq!(
        crossings(&rim, &top(10.0 - eps / 2.0), scale()),
        Crossings::Along
    );
    assert!(found(&rim, &top(10.0 + 2.0 * eps), 0.0).is_empty());
    let slope = plane(DVec3::new(8.0, 0.0, 10.0), DVec3::new(1.0, 1.0, 1.0));
    assert_eq!(
        tangents(&found(&rim, &slope, 1e-12 * REACH)),
        [false, false]
    );
}

#[test]
fn a_circle_crosses_a_parallel_cylinder_as_two_circles_cross() {
    let eps = scale().eps();
    let rim = circle(DVec3::ZERO, 20.0, 10.0);
    let tool = |x: f64, radius: f64| cylinder(DVec3::new(x, 0.0, 0.0), DVec3::Z, radius);
    assert_eq!(
        tangents(&found(&rim, &tool(20.0, 5.0), 1e-12 * REACH)),
        [false, false]
    );
    for x in [15.0, 15.0 + eps / 2.0, 25.0, 25.0 - eps / 2.0, -15.0] {
        let touch = found(&rim, &tool(x, 5.0), eps);
        assert_eq!(tangents(&touch), [true], "{x}");
        assert!((touch[0].point - DVec3::new(20.0 * x.signum(), 0.0, 10.0)).length() <= eps);
    }
    assert!(found(&rim, &tool(15.0 - 2.0 * eps, 5.0), 0.0).is_empty());
    assert!(found(&rim, &tool(8.0, 5.0), 0.0).is_empty());
    assert_eq!(
        crossings(&rim, &tool(eps / 2.0, 20.0), scale()),
        Crossings::Along
    );
}

#[test]
fn a_circle_crosses_a_perpendicular_cylinder_where_its_plane_cuts_it() {
    let rim = circle(DVec3::ZERO, 20.0, 5.0);
    let across = cylinder(DVec3::new(0.0, 15.0, 5.0), DVec3::X, 5.0);
    let three = found(&rim, &across, 1e-12 * REACH);
    assert_eq!(three.len(), 3);
    assert_eq!(three.iter().filter(|crossing| crossing.tangent).count(), 1);
    let below = cylinder(DVec3::new(0.0, 0.0, 10.0), DVec3::X, 5.0);
    assert_eq!(tangents(&found(&rim, &below, 1e-12 * REACH)), [true, true]);
    let skew = cylinder(DVec3::ZERO, DVec3::new(1.0, 0.0, 1.0), 5.0);
    assert_eq!(crossings(&rim, &skew, scale()), Crossings::Unsupported);
}

/// The loop a cylinder of radius 3 along X through `(0, 4, 0)` makes with one
/// of radius 5 standing on Z: it spans `y` from 1 to 5, and reaches up to
/// `z = 3` where `y = 4`, at `x = ±3`.
fn one_loop() -> (Curve, Surface, Surface) {
    let stock = Cylinder::about(DVec3::ZERO, DVec3::Z, 5.0);
    let across = Cylinder::about(DVec3::new(0.0, 4.0, 0.0), DVec3::X, 3.0);
    let meet = Meet {
        first: stock,
        second: across,
        component: 0,
    };
    (
        Curve::Meet(meet),
        Surface::Cylinder(stock),
        Surface::Cylinder(across),
    )
}

#[test]
fn a_meet_crosses_a_plane_where_its_arcs_pass_through_it() {
    let (meet, ..) = one_loop();
    let at = |normal: DVec3, offset: f64| plane(normal * offset, normal);
    for (surface, count) in [
        (at(DVec3::Z, 0.0), 2),
        (at(DVec3::Y, 2.0), 4),
        (at(DVec3::X, 0.0), 2),
        (at(DVec3::Z, 2.0), 2),
        (at(DVec3::Y, 6.0), 0),
    ] {
        let crossed = found(&meet, &surface, 1e-12 * REACH);
        assert_eq!(tangents(&crossed), vec![false; count], "{surface:?}");
    }
    let bottom = found(&meet, &at(DVec3::Z, 0.0), 1e-12 * REACH);
    for (crossing, x) in bottom.iter().zip([24.0_f64.sqrt(), -(24.0_f64.sqrt())]) {
        assert!((crossing.point - DVec3::new(x, 1.0, 0.0)).length() < 1e-12 * REACH);
    }
}

#[test]
fn a_meet_touching_a_plane_touches_it_once_at_each_top() {
    let eps = scale().eps();
    let (meet, ..) = one_loop();
    let top = |z: f64| plane(DVec3::new(0.0, 0.0, z), DVec3::Z);
    for z in [3.0, 3.0 + eps / 2.0, 3.0 - eps / 2.0] {
        let touches = found(&meet, &top(z), eps);
        assert_eq!(tangents(&touches), [true, true], "{z}");
        for touch in touches {
            assert!(
                (touch.point - DVec3::new(3.0 * touch.point.x.signum(), 4.0, 3.0)).length() < 1e-6
            );
        }
    }
    assert!(found(&meet, &top(3.0 + 2.0 * eps), 0.0).is_empty());
    assert_eq!(
        tangents(&found(&meet, &top(3.0 - 2.0 * eps), 1e-12 * REACH)),
        [false; 4]
    );
}

#[test]
fn a_meet_touching_a_plane_where_it_starts_gives_the_touch_within_its_first_period() {
    let eps = scale().eps();
    let centre = DVec3::new(1.0, 2.0, 0.0);
    for step in 0..400 {
        let angle = 0.013 + step as f64 * 0.0157;
        let along = DVec3::new(angle.cos(), angle.sin(), 0.0);
        let across = DVec3::Z.cross(along);
        for (a, b, d, e) in [
            (5.0, 3.0, 4.0, 2.0),
            (5.0, 3.0, 0.5, 0.0),
            (5.0, 3.0, 2.0, 1.0),
            (4.0, 4.0, 0.0, 3.0),
        ] {
            let first = Cylinder::about(centre, DVec3::Z, a);
            let second = Cylinder::about(centre + across * d + DVec3::Z * e, along, b);
            for meet in Meeting::of(&first, &second, scale()).components {
                let period = meet.period().expect("a component closes on itself");
                for y in [(-a).max(d - b), a.min(d + b)] {
                    let side = plane(centre + across * y, across);
                    for crossing in found(&Curve::Meet(meet), &side, eps) {
                        assert!(
                            (0.0..period).contains(&crossing.parameter),
                            "{angle} {a} {b} {d} {e}: {crossing:?} is not within {period}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn a_meet_crosses_a_third_cylinder_everywhere_its_distance_changes_sign() {
    let (meet, stock, across) = one_loop();
    let Curve::Meet(inner) = meet else {
        unreachable!()
    };
    let period = inner.period().expect("one loop");
    for third in [
        cylinder(DVec3::new(2.0, 3.0, 0.0), DVec3::Z, 2.5),
        cylinder(DVec3::new(0.0, 0.0, 1.0), DVec3::Y, 2.0),
        cylinder(DVec3::new(0.0, 5.0, 1.0), DVec3::X, 2.0),
    ] {
        let crossed = found(&meet, &third, 1e-12 * REACH);
        let side = |t: f64| third.distance(meet.point(t));
        let changes = (0..2000)
            .filter(|&k| {
                let [from, to] = [k, k + 1].map(|k| period * k as f64 / 2000.0);
                side(from) * side(to) < 0.0
            })
            .count();
        assert_eq!(crossed.len(), changes, "{third:?}");
        assert!(changes > 0);
    }
    assert_eq!(crossings(&meet, &stock, scale()), Crossings::Along);
    assert_eq!(crossings(&meet, &across, scale()), Crossings::Along);
}

/// Seed 1044340 of the campaign: a post along Y grooved by a bar along X,
/// then cut by a post of its radius a tenth of a micron aside. The two posts
/// cross along two lines at a grazing angle, and the curve the first meets
/// the bar along crosses the second where those lines pass through the bar:
/// found by scanning the curve, where the second stands within the
/// tolerance over a stretch millimetres long, it came out eight tenths of a
/// micron off. A triple is solved from its most degenerate pair.
#[test]
fn a_meet_crosses_a_cylinder_parallel_to_one_of_its_own_where_their_lines_cross_the_other() {
    let post = Cylinder::about(DVec3::new(10.0, 0.0, 30.0), DVec3::Y, 20.0);
    let bar = Cylinder::about(DVec3::new(0.0, 25.0, 50.0), DVec3::X, 8.0);
    let aside = cylinder(DVec3::new(10.000_000_1, 0.0, 30.0), DVec3::Y, 20.0);
    let scale = Scale::of(60.0);
    let mut crossed = Vec::new();
    for meet in Meeting::of(&post, &bar, scale).components {
        let Crossings::At(found) = crossings(&Curve::Meet(meet), &aside, scale) else {
            panic!("points expected");
        };
        for crossing in &found {
            let point = Curve::Meet(meet).point(crossing.parameter);
            assert!(
                (point - crossing.point).length() < 1e-12 * REACH,
                "{crossing:?}"
            );
        }
        crossed.extend(found);
    }
    assert_eq!(crossed.len(), 2, "{crossed:?}");
    for crossing in crossed {
        assert!(
            (crossing.point.x - 10.000_000_05).abs() < 1e-12 * REACH,
            "{crossing:?}"
        );
        assert!((crossing.point.z - 50.0).abs() < 1e-9, "{crossing:?}");
    }
}

#[test]
fn an_ellipse_of_two_equal_cylinders_lies_along_its_own_plane() {
    let first = Cylinder::about(DVec3::ZERO, DVec3::Z, 4.0);
    let second = Cylinder::about(DVec3::new(0.0, 0.0, 1.0), DVec3::X, 4.0);
    let ellipse = Curve::Meet(Meet {
        first,
        second,
        component: 0,
    });
    let own = plane(DVec3::new(0.0, 0.0, 1.0), DVec3::new(1.0, 0.0, -1.0));
    let other = plane(DVec3::new(0.0, 0.0, 1.0), DVec3::new(1.0, 0.0, 1.0));
    assert_eq!(crossings(&ellipse, &own, scale()), Crossings::Along);
    assert_eq!(found(&ellipse, &other, 1e-12 * REACH).len(), 2);
}
