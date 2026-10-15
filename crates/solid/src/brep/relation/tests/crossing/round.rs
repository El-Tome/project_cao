//! Where a line or a circle crosses a sphere or a torus (#576).

use glam::{DVec2, DVec3};

use super::{found, found_touching, tangents};
use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::relation::tests::{REACH, scale};
use crate::brep::relation::{Crossings, crossings};
use crate::brep::surface::{Sphere, Surface, Torus};

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
fn a_circle_a_round_holds_lies_along_it_and_any_other_crosses_it_nowhere() {
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
    let leaning = Curve::Circle(Circle::about(DVec3::new(4.0, 0.0, 2.0), DVec3::X, 1.0));
    assert_eq!(
        crossings(&leaning, &ring(), scale()),
        Crossings::At(Vec::new())
    );
}
