//! Where a line or a circle crosses a sphere or a torus (#576).

use glam::{DVec2, DVec3};

use super::{found, found_touching, tangents};
use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::relation::tests::{REACH, scale};
use crate::brep::relation::{Crossing, Crossings, Relation, crossings, relation};
use crate::brep::surface::{Cone, Cylinder, Plane, Sphere, Surface, Torus};

/// A ball of radius 3 about `(0, 0, 5)`, turned about Z.
fn ball() -> Surface {
    Surface::Sphere(Sphere::through(DVec3::ZERO, DVec3::Z, 5.0, 3.0))
}

/// A ring of tube radius 1 about a circle of radius 4 at `z = 2`.
fn ring() -> Surface {
    Surface::Torus(Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(2.0, 4.0),
        1.0,
        DVec2::new(2.0, 5.0),
    ))
}

#[test]
fn a_line_crosses_a_ball_twice_touches_it_within_the_tolerance_and_passes_it_by() {
    let eps = scale().eps();
    let through = Curve::Line(Line::through(DVec3::new(0.0, 0.0, 5.0), DVec3::X));
    let twice = found(&through, &ball(), 1e-12 * REACH);
    assert_eq!(tangents(&twice), [false, false]);
    assert!((twice[0].point - DVec3::new(-3.0, 0.0, 5.0)).length() < 1e-12 * REACH);
    assert!((twice[1].point - DVec3::new(3.0, 0.0, 5.0)).length() < 1e-12 * REACH);

    let grazing = Curve::Line(Line::through(
        DVec3::new(0.0, 3.0 + eps / 2.0, 5.0),
        DVec3::X,
    ));
    let once = found(&grazing, &ball(), eps);
    assert_eq!(tangents(&once), [true]);
    assert!((once[0].point - DVec3::new(0.0, 3.0, 5.0)).length() < eps);

    let inside = Curve::Line(Line::through(
        DVec3::new(0.0, 3.0 - eps / 2.0, 5.0),
        DVec3::X,
    ));
    assert_eq!(tangents(&found(&inside, &ball(), eps)), [false, false]);
    assert_eq!(tangents(&found_touching(&inside, &ball(), eps)), [true]);

    let beside = Curve::Line(Line::through(
        DVec3::new(0.0, 3.0 + 2.0 * eps, 5.0),
        DVec3::X,
    ));
    assert_eq!(
        crossings(&beside, &ball(), scale()),
        Crossings::At(Vec::new())
    );
}

#[test]
fn a_line_in_a_plane_holding_the_axis_crosses_a_ring_four_times() {
    let across = Curve::Line(Line::through(DVec3::new(0.0, 0.0, 2.0), DVec3::X));
    let four = found(&across, &ring(), 1e-12 * REACH);
    assert_eq!(tangents(&four), [false; 4]);
    for (crossing, x) in four.iter().zip([-5.0, -3.0, 3.0, 5.0]) {
        assert!((crossing.point - DVec3::new(x, 0.0, 2.0)).length() < 1e-12 * REACH);
    }

    let along = Curve::Line(Line::through(DVec3::new(4.0, 0.0, 0.0), DVec3::Z));
    let twice = found(&along, &ring(), 1e-12 * REACH);
    assert_eq!(tangents(&twice), [false, false]);
    assert!((twice[0].point.z - 1.0).abs() < 1e-12 && (twice[1].point.z - 3.0).abs() < 1e-12);

    let hole = Curve::Line(Line::through(DVec3::new(1.0, 0.0, 0.0), DVec3::Z));
    assert_eq!(
        crossings(&hole, &ring(), scale()),
        Crossings::At(Vec::new())
    );

    let eps = scale().eps();
    let crest = Curve::Line(Line::through(
        DVec3::new(0.0, 0.0, 3.0 + eps / 2.0),
        DVec3::X,
    ));
    let touches = found(&crest, &ring(), eps);
    assert_eq!(tangents(&touches), [true, true]);
}

#[test]
fn a_line_at_a_slant_to_a_torus_s_axis_is_unsupported() {
    let skew = Curve::Line(Line::through(
        DVec3::new(0.0, 1.0, 2.0),
        DVec3::new(1.0, 0.0, 0.5),
    ));
    assert_eq!(crossings(&skew, &ring(), scale()), Crossings::Unsupported);
    let slanted = Curve::Line(Line::through(
        DVec3::new(0.0, 0.0, 5.0),
        DVec3::new(1.0, 0.0, 0.5),
    ));
    assert_eq!(
        tangents(&found(&slanted, &ball(), 1e-12 * REACH)),
        [false, false]
    );
}

#[test]
fn a_circle_a_round_holds_lies_along_it_and_one_about_the_axis_off_it_crosses_it_nowhere() {
    let Surface::Sphere(sphere) = ball() else {
        unreachable!()
    };
    let latitude = Curve::Circle(sphere.circle(5.0 + 3.0 * 0.6, 3.0 * 0.8));
    assert_eq!(crossings(&latitude, &ball(), scale()), Crossings::Along);
    let meridian = Curve::Circle(Circle::about(sphere.middle(), DVec3::Y, 3.0));
    assert_eq!(crossings(&meridian, &ball(), scale()), Crossings::Along);
    let off = Curve::Circle(sphere.circle(5.0, 2.0));
    assert_eq!(crossings(&off, &ball(), scale()), Crossings::At(Vec::new()));

    let Surface::Torus(torus) = ring() else {
        unreachable!()
    };
    let tube = Curve::Circle(Circle::about(DVec3::new(4.0, 0.0, 2.0), DVec3::Y, 1.0));
    assert_eq!(crossings(&tube, &ring(), scale()), Crossings::Along);
    assert_eq!(
        crossings(&Curve::Circle(torus.circle(2.0, 5.0)), &ring(), scale()),
        Crossings::Along
    );
    assert_eq!(
        crossings(&Curve::Circle(torus.circle(2.0, 4.0)), &ring(), scale()),
        Crossings::At(Vec::new())
    );
}

#[test]
fn a_circle_in_a_plane_off_the_axis_is_unsupported_against_a_round() {
    let leaning = Curve::Circle(Circle::about(DVec3::new(4.0, 0.0, 2.0), DVec3::X, 1.0));
    assert_eq!(
        crossings(&leaning, &ring(), scale()),
        Crossings::Unsupported
    );
    let beside = Curve::Circle(Circle::about(DVec3::new(1.0, 0.0, 5.0), DVec3::Z, 2.0));
    assert_eq!(crossings(&beside, &ball(), scale()), Crossings::Unsupported);
    let slanted = Curve::Circle(Circle::about(
        DVec3::new(0.0, 0.0, 5.0),
        DVec3::new(1.0, 0.0, 1.0),
        2.0,
    ));
    assert_eq!(
        crossings(&slanted, &ball(), scale()),
        Crossings::Unsupported
    );
}

/// Whether every crossing stands on a circle of the pair's rims, within
/// `within`.
fn on_the_rims(crossings: &[Crossing], one: &Surface, other: &Surface, within: f64) -> bool {
    let Relation::Rims(rims) = relation(one, other, scale()) else {
        panic!("rims expected between {one:?} and {other:?}");
    };
    crossings.iter().all(|crossing| {
        rims.iter().any(|rim| {
            let circle = rim.circle;
            circle
                .point(circle.parameter(crossing.point))
                .distance(crossing.point)
                <= within
        })
    })
}

#[test]
fn a_meridian_circle_crosses_a_coaxial_ball_where_the_rims_of_its_own_round_stand() {
    let other = Sphere::through(DVec3::ZERO, DVec3::Z, 7.0, 2.0);
    let great = Curve::Circle(Circle::about(other.middle(), DVec3::Y, 2.0));
    let two = found(&great, &ball(), 1e-12 * REACH);
    assert_eq!(tangents(&two), [false, false]);
    let k = 63f64.sqrt() / 4.0;
    for crossing in &two {
        assert!(
            (crossing.point.z - 7.25).abs() < 1e-12 * REACH,
            "{crossing:?}"
        );
        assert!(
            (crossing.point.x.abs() - k).abs() < 1e-12 * REACH,
            "{crossing:?}"
        );
        assert!(crossing.point.y.abs() < 1e-12 * REACH);
    }
    assert!(on_the_rims(
        &two,
        &ball(),
        &Surface::Sphere(other),
        1e-12 * REACH
    ));

    let tilted = Curve::Circle(Circle::about(
        other.middle(),
        DVec3::new(1.0, 2.0, 0.0),
        2.0,
    ));
    let two = found(&tilted, &ball(), 1e-12 * REACH);
    assert_eq!(two.len(), 2);
    assert!(on_the_rims(
        &two,
        &ball(),
        &Surface::Sphere(other),
        1e-12 * REACH
    ));
}

#[test]
fn a_meridian_circle_touching_a_coaxial_round_is_found_at_its_touch_once() {
    let eps = scale().eps();
    let above = Sphere::through(DVec3::ZERO, DVec3::Z, 10.0, 2.0);
    let great = Curve::Circle(Circle::about(above.middle(), DVec3::Y, 2.0));
    let once = found(&great, &ball(), eps);
    assert_eq!(tangents(&once), [true]);
    assert!((once[0].point - DVec3::new(0.0, 0.0, 8.0)).length() <= eps);

    let inside = Sphere::through(DVec3::ZERO, DVec3::Z, 10.0 - eps / 2.0, 2.0);
    let great = Curve::Circle(Circle::about(inside.middle(), DVec3::Y, 2.0));
    assert_eq!(tangents(&found(&great, &ball(), eps)), [false, false]);
    assert_eq!(tangents(&found_touching(&great, &ball(), eps)), [true]);

    let tube = Curve::Circle(Circle::about(DVec3::new(4.0, 0.0, 5.0), DVec3::Y, 1.0));
    let once = found(&tube, &ball(), eps);
    assert_eq!(tangents(&once), [true]);
    assert!((once[0].point - DVec3::new(3.0, 0.0, 5.0)).length() <= eps);
}

/// A spindle's tube circle reaches across the axis: it crosses a coaxial
/// ring twice on its own side, where the pair's rims stand, and twice on
/// the far side, on the ring's own lobe there but on the spindle's mirror
/// lobe, which no edge of the spindle runs along.
#[test]
fn a_tube_circle_reaching_across_the_axis_crosses_a_coaxial_ring_on_both_sides() {
    let spindle = Surface::Torus(Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(2.0, 1.0),
        3.0,
        DVec2::new(2.0, 4.0),
    ));
    let wide = Surface::Torus(Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(2.0, 3.5),
        2.0,
        DVec2::new(2.0, 5.5),
    ));
    let tube = Curve::Circle(Circle::about(DVec3::new(1.0, 0.0, 2.0), DVec3::Y, 3.0));
    let four = found(&tube, &wide, 1e-12 * REACH);
    assert_eq!(tangents(&four), [false; 4]);
    let (own, far): (Vec<Crossing>, Vec<Crossing>) = four
        .into_iter()
        .partition(|crossing| crossing.point.x > 0.0);
    assert_eq!((own.len(), far.len()), (2, 2));
    assert!(on_the_rims(&own, &spindle, &wide, 1e-12 * REACH));
    for crossing in &own {
        assert!(
            (crossing.point.x - 3.25).abs() < 1e-12 * REACH,
            "{crossing:?}"
        );
    }
    for crossing in &far {
        assert!(spindle.distance(crossing.point).abs() > 1.0, "{crossing:?}");
    }
}

#[test]
fn a_meridian_circle_of_a_ring_meets_a_coaxial_plane_cylinder_and_cone_where_their_rims_stand() {
    let tube = Curve::Circle(Circle::about(DVec3::new(4.0, 0.0, 2.0), DVec3::Y, 1.0));
    let tools = [
        Surface::Plane(Plane::through(DVec3::new(0.0, 0.0, 2.5), DVec3::Z).0),
        Surface::Cylinder(Cylinder::about(DVec3::ZERO, DVec3::Z, 4.5)),
        Surface::Cone(Cone::through(
            DVec3::ZERO,
            DVec3::Z,
            [DVec2::new(0.0, 4.0), DVec2::new(8.0, 5.0)],
        )),
    ];
    for tool in &tools {
        let two = found(&tube, tool, 1e-12 * REACH);
        assert_eq!(tangents(&two), [false, false], "{tool:?}");
        assert!(on_the_rims(&two, &ring(), tool, 1e-12 * REACH), "{tool:?}");
    }
}
