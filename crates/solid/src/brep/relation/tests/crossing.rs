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

/// The same, the curve lying on a surface decided to touch `surface`.
fn found_touching(curve: &Curve, surface: &Surface, within: f64) -> Vec<Crossing> {
    let along = Touches {
        along: true,
        at: Vec::new(),
    };
    let Crossings::At(found) = crossings_given(curve, surface, scale(), &along) else {
        panic!("points expected");
    };
    for crossing in &found {
        assert!(surface.distance(crossing.point).abs() <= within);
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
    for x in [20.0, 20.0 + eps / 2.0] {
        let touch = found(&across(x), &hole, eps);
        assert_eq!(tangents(&touch), [true], "{x}");
        assert!((touch[0].point.x - x).abs() <= eps);
    }
    for x in [20.0 - eps / 2.0, 10.0 + eps / 2.0] {
        let twice = found(&across(x), &hole, 1e-12 * REACH);
        assert_eq!(tangents(&twice), [false, false], "{x}");
        assert!((twice[1].point - twice[0].point).length() > 100.0 * eps);
        let touch = found_touching(&across(x), &hole, eps);
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
        tangents(&found(&rim, &wall(13.0 - eps / 2.0), 1e-12 * REACH)),
        [false, false]
    );
    assert_eq!(
        tangents(&found_touching(&rim, &wall(13.0 - eps / 2.0), eps)),
        [true]
    );
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

/// A round of radius 1 along Y whose axis stands 7 from Z, so that it touches
/// the wall of radius 8 about Z from inside at `(8, 0, 3)`, and the height a
/// hair under that touch: the plane there cuts the round 7 + √(1 − δ²) from
/// the axis, a hair inside the wall by less than a coordinate holds, so the
/// line as rounded stands on the wall. Its two crossings are √8 δ either side
/// of the touch, many tolerances apart.
fn a_hair_under_a_round_touching_inside() -> (Cylinder, Surface, f64, f64) {
    let delta = 3e-8;
    let wall = Cylinder::about(DVec3::ZERO, DVec3::Z, 8.0);
    let round = cylinder(DVec3::new(7.0, 0.0, 3.0), DVec3::Y, 1.0);
    (wall, round, 3.0 - delta, 8.0_f64.sqrt() * delta)
}

#[test]
fn a_circle_a_hair_under_a_round_touching_its_wall_inside_crosses_it_twice() {
    let (wall, round, height, half) = a_hair_under_a_round_touching_inside();
    assert!(half > 2.0 * scale().eps());
    let rim = Curve::Circle(Circle::on(&wall, height));
    let crossed = found(&rim, &round, 1e-12 * REACH);
    assert_eq!(tangents(&crossed), [false; 4]);
    let near: Vec<&Crossing> = crossed
        .iter()
        .filter(|crossing| crossing.point.x > 7.5)
        .collect();
    assert_eq!(near.len(), 2);
    for crossing in near {
        assert!(
            (crossing.point.y.abs() - half).abs() < 1e-12 * REACH,
            "{crossing:?}"
        );
    }
}

#[test]
fn the_curve_two_cylinders_meet_along_a_hair_from_their_touch_crosses_a_plane_twice_there() {
    let (wall, round, height, half) = a_hair_under_a_round_touching_inside();
    let Surface::Cylinder(round) = round else {
        unreachable!()
    };
    let meet = Curve::Meet(Meet {
        first: wall,
        second: round,
        component: 0,
    });
    let level = plane(DVec3::new(0.0, 0.0, height), DVec3::Z);
    let crossed = found(&meet, &level, 1e-12 * REACH);
    let near: Vec<&Crossing> = crossed
        .iter()
        .filter(|crossing| crossing.point.x > 7.5)
        .collect();
    assert_eq!(near.len(), 2, "{crossed:?}");
    for crossing in near {
        assert!(!crossing.tangent);
        assert!(
            (crossing.point.y.abs() - half).abs() < 1e-12 * REACH,
            "{crossing:?}"
        );
    }
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

/// A point about Z: tip at height 10, base of radius 5 on the XY plane; its
/// section at height `h` is of radius `5 − h/2`.
fn point() -> Surface {
    Surface::Cone(crate::brep::surface::Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [glam::DVec2::new(0.0, 5.0), glam::DVec2::new(10.0, 0.0)],
    ))
}

fn line(through: [f64; 3], direction: [f64; 3]) -> Curve {
    Curve::Line(Line::through(DVec3::from(through), DVec3::from(direction)))
}

#[test]
fn a_ruling_lies_along_its_cone() {
    let ruling = line([5.0, 0.0, 0.0], [-5.0, 0.0, 10.0]);
    assert_eq!(crossings(&ruling, &point(), scale()), Crossings::Along);
    let near_cylinder = Surface::Cone(crate::brep::surface::Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [
            glam::DVec2::new(0.0, 10.0),
            glam::DVec2::new(20.0, 10.0 + 1e-6),
        ],
    ));
    let far_ruling = line([0.0, 10.0, 0.0], [0.0, 1e-6, 20.0]);
    assert_eq!(
        crossings(&far_ruling, &near_cylinder, scale()),
        Crossings::Along
    );
}

#[test]
fn the_axis_crosses_a_cone_once_at_its_apex() {
    let axis = line([0.0, 0.0, -3.0], [0.0, 0.0, 1.0]);
    let once = found(&axis, &point(), 1e-12 * REACH);
    assert_eq!(tangents(&once), [false]);
    assert!((once[0].point - DVec3::new(0.0, 0.0, 10.0)).length() < 1e-12 * REACH);
}

#[test]
fn a_line_through_the_apex_outside_the_cone_only_touches_it() {
    let eps = scale().eps();
    for moved in [0.0, eps / 2.0] {
        let flat = line([0.0, moved, 10.0], [1.0, 0.0, 0.1]);
        let once = found(&flat, &point(), eps);
        assert_eq!(tangents(&once), [true], "{moved}");
    }
    let steep = line([0.0, 0.0, 10.0], [1.0, 0.0, 3.0]);
    assert_eq!(tangents(&found(&steep, &point(), 1e-12 * REACH)), [false]);
}

#[test]
fn a_line_square_to_the_axis_grazing_a_cone_touches_it_once() {
    let eps = scale().eps();
    let at = |y: f64| line([0.0, y, 4.0], [1.0, 0.0, 0.0]);
    let touch = found(&at(3.0 + eps / 2.0), &point(), eps);
    assert_eq!(tangents(&touch), [true]);
    assert!((touch[0].point - DVec3::new(0.0, 3.0 + eps / 2.0, 4.0)).length() < 1e-12 * REACH);
    assert_eq!(
        tangents(&found(&at(3.0 - 2.0 * eps), &point(), 1e-12 * REACH)),
        [false, false]
    );
    assert!(found(&at(3.0 + 2.0 * eps), &point(), 0.0).is_empty());
    assert_eq!(
        tangents(&found(&at(3.0 - eps / 2.0), &point(), eps)),
        [false, false]
    );
    assert_eq!(
        tangents(&found_touching(&at(3.0 - eps / 2.0), &point(), eps)),
        [true]
    );
}

#[test]
fn a_skew_line_grazing_a_cone_touches_it_once() {
    let eps = scale().eps();
    let direction = (DVec3::X + DVec3::new(0.0, -1.0, 2.0) * 0.2).normalize();
    let normal = DVec3::new(0.0, 2.0, 1.0).normalize();
    let on = DVec3::new(0.0, 3.0, 4.0);
    for off in [0.0, eps / 2.0, -eps / 2.0] {
        let grazing = Curve::Line(Line::through(on + normal * off, direction));
        let crossed = found(&grazing, &point(), eps);
        assert!(crossed.len() <= 2, "{off}: {crossed:?}");
        if off >= 0.0 {
            assert_eq!(tangents(&crossed), [true], "{off}");
        }
    }
}

#[test]
fn a_skew_line_crosses_a_cone_at_its_roots_on_its_nappe_only() {
    let twice = line([0.0, 0.5, 4.0], [1.0, 0.0, 1.0]);
    let crossed = found(&twice, &point(), 1e-12 * REACH);
    assert_eq!(tangents(&crossed), [false, false]);
    let through_both = line([0.0, 0.5, 4.0], [1.0, 0.0, 3.0]);
    let once = found(&through_both, &point(), 1e-12 * REACH);
    assert_eq!(tangents(&once), [false]);
    assert!(once[0].point.z < 10.0, "{once:?}");
}

#[test]
fn a_line_parallel_to_a_ruling_crosses_once() {
    let beside = line([0.0, 1.0, 0.0], [-1.0, 0.0, 2.0]);
    let once = found(&beside, &point(), 1e-12 * REACH);
    assert_eq!(tangents(&once), [false]);
    assert!((once[0].point - DVec3::new(-2.4, 1.0, 4.8)).length() < 1e-12 * REACH);
}

#[test]
fn a_coaxial_circle_lies_along_a_cone_or_misses_it() {
    let eps = scale().eps();
    for radius in [3.0, 3.0 + eps / 2.0] {
        let rim = circle(DVec3::ZERO, radius, 4.0);
        assert_eq!(crossings(&rim, &point(), scale()), Crossings::Along);
    }
    assert!(found(&circle(DVec3::ZERO, 3.5, 4.0), &point(), 0.0).is_empty());
    assert!(found(&circle(DVec3::ZERO, 1.0, 12.0), &point(), 0.0).is_empty());
}

#[test]
fn a_circle_square_to_the_axis_crosses_a_cone_where_it_crosses_its_section() {
    let across = found(
        &circle(DVec3::new(3.0, 0.0, 0.0), 1.0, 4.0),
        &point(),
        1e-12 * REACH,
    );
    assert_eq!(tangents(&across), [false, false]);
    let inside = found(
        &circle(DVec3::new(2.0, 0.0, 0.0), 1.0, 4.0),
        &point(),
        1e-12 * REACH,
    );
    assert_eq!(tangents(&inside), [true]);
    assert!((inside[0].point - DVec3::new(3.0, 0.0, 4.0)).length() < 1e-12 * REACH);
    let at_the_apex = found(
        &circle(DVec3::new(2.0, 0.0, 0.0), 2.0, 10.0),
        &point(),
        1e-12 * REACH,
    );
    assert_eq!(tangents(&at_the_apex), [true]);
}

#[test]
fn a_circle_in_a_plane_holding_the_axis_crosses_its_rulings_on_its_nappe_only() {
    let upright = |center: DVec3, radius: f64| {
        Curve::Circle(Circle {
            center,
            axis: DVec3::Y,
            radius,
            u: DVec3::Z,
            v: DVec3::X,
        })
    };
    let below = found(
        &upright(DVec3::new(0.0, 0.0, 5.0), 3.0),
        &point(),
        1e-12 * REACH,
    );
    assert_eq!(tangents(&below), [false; 4]);
    let over = found(
        &upright(DVec3::new(0.0, 0.0, 10.0), 2.0),
        &point(),
        1e-12 * REACH,
    );
    assert_eq!(tangents(&over), [false, false]);
    for crossing in over {
        assert!(crossing.point.z < 10.0, "{crossing:?}");
    }
}

#[test]
fn a_circle_at_a_slant_to_a_cone_is_unsupported() {
    let slanted = Curve::Circle(Circle {
        center: DVec3::new(0.0, 0.0, 4.0),
        axis: DVec3::new(0.0, 0.6, 0.8),
        radius: 3.0,
        u: DVec3::X,
        v: DVec3::new(0.0, 0.8, -0.6),
    });
    assert_eq!(
        crossings(&slanted, &point(), scale()),
        Crossings::Unsupported
    );
}

#[test]
fn a_circle_through_the_apex_in_a_plane_holding_the_axis_touches_it_there_unless_it_enters() {
    let upright = |center: DVec3| {
        Curve::Circle(Circle {
            center,
            axis: DVec3::Y,
            radius: 2.0,
            u: DVec3::Z,
            v: DVec3::X,
        })
    };
    let apex = DVec3::new(0.0, 0.0, 10.0);
    let above = found(
        &upright(DVec3::new(0.0, 0.0, 12.0)),
        &point(),
        1e-12 * REACH,
    );
    assert_eq!(tangents(&above), [true], "{above:?}");
    assert!((above[0].point - apex).length() < 1e-12 * REACH);
    let beside = found(
        &upright(DVec3::new(2.0, 0.0, 10.0)),
        &point(),
        1e-12 * REACH,
    );
    assert_eq!(tangents(&beside), [false, false], "{beside:?}");
    assert!(
        beside
            .iter()
            .any(|crossing| (crossing.point - apex).length() < 1e-12 * REACH)
    );
    assert!(
        beside
            .iter()
            .any(|crossing| (crossing.point - DVec3::new(0.8, 0.0, 8.4)).length() < 1e-12 * REACH)
    );
}
