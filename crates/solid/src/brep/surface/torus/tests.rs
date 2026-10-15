use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::Torus;
use crate::brep::curve::Circle;
use crate::brep::surface::Cylinder;

const CLOSE: f64 = 1e-12;

/// A ring of tube radius 2 about a circle of radius 5 at `h = 4`, on a
/// slanted axis, its chart centred on the crest.
fn ring() -> Torus {
    Torus::through(
        DVec3::new(1.0, -2.0, 3.0),
        DVec3::new(0.0, 1.0, 1.0),
        DVec2::new(4.0, 5.0),
        2.0,
        DVec2::new(4.0, 7.0),
    )
}

/// An apple: the outer part of the spindle torus whose meridian circle of
/// radius 2 is centred one from the axis, at `h = 4`, about Z.
fn apple() -> Torus {
    Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(4.0, 1.0),
        2.0,
        DVec2::new(4.0, 3.0),
    )
}

/// A lemon: the part of a spindle torus whose meridian circle of radius 2
/// is centred one across the axis, at `h = 4`, about Z.
fn lemon() -> Torus {
    Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(4.0, -1.0),
        2.0,
        DVec2::new(4.0, 1.0),
    )
}

#[test]
fn a_torus_shares_origin_and_reference_with_a_cylinder_of_its_axis() {
    let point = DVec3::new(2.0, -5.0, 1.0);
    let axis = DVec3::new(1.0, 2.0, -0.5);
    let torus = Torus::through(point, axis, DVec2::new(1.0, 4.0), 1.0, DVec2::new(1.0, 5.0));
    let cylinder = Cylinder::about(point, axis, 4.0);
    assert_eq!(
        [torus.origin, torus.axis, torus.u, torus.v],
        [cylinder.origin, cylinder.axis, cylinder.u, cylinder.v]
    );
    assert_eq!(torus.circle(3.0, 4.0), Circle::on(&cylinder, 3.0));
}

#[test]
fn a_torus_turned_from_one_arc_either_way_is_the_same_bits() {
    let point = DVec3::new(3.0, 7.0, -2.0);
    let axis = DVec3::new(0.3, -1.0, 0.2);
    let mirrored = |corner: DVec2| DVec2::new(-corner.x, corner.y);
    let (center, middle) = (DVec2::new(1.5, 2.0), DVec2::new(2.5, 3.0));
    let forwards = Torus::through(point, axis, center, 1.5, middle);
    let backwards = Torus::through(point, -axis, mirrored(center), 1.5, mirrored(middle));
    assert_eq!(forwards, backwards);
}

#[test]
fn the_chart_is_centred_on_the_arc_s_middle() {
    let torus = ring();
    assert_eq!(torus.seam, 0.0);
    let crest = torus.point(DVec2::new(0.3, 0.0));
    assert!(torus.parameters(crest).y.abs() < CLOSE);
    let inner = Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(4.0, 5.0),
        2.0,
        DVec2::new(4.0, 3.0),
    );
    assert!((inner.seam.abs() - PI).abs() < CLOSE);
    let window = inner.parameters(inner.point(DVec2::new(0.0, PI + 0.5))).y;
    assert!(
        window > inner.seam - PI && window <= inner.seam + PI,
        "{window} outside the window about {}",
        inner.seam
    );
    assert!((window - (PI + 0.5)).abs() < CLOSE, "{window}");
    let behind = inner.parameters(inner.point(DVec2::new(0.0, -0.5))).y;
    assert!((behind - (TAU - 0.5)).abs() < CLOSE, "{behind}");
}

#[test]
fn a_point_of_a_torus_reads_back_its_parameters() {
    for torus in [ring(), apple(), lemon()] {
        for at in [
            DVec2::new(0.3, 0.2),
            DVec2::new(-2.9, -1.0),
            DVec2::new(3.1, 0.9),
            DVec2::new(1.0, 0.0),
        ] {
            let point = torus.point(at);
            assert!(torus.distance(point).abs() < CLOSE, "{at}");
            let back = torus.parameters(point);
            assert!((back - at).length() < CLOSE, "{at} came back as {back}");
            assert!(torus.nearest(point).distance(point) < CLOSE);
        }
    }
}

#[test]
fn a_torus_s_parameters_turn_as_its_own_normal_the_poles_included() {
    let step = 1e-6;
    for (torus, poles) in [
        (ring(), [-PI + 0.3, PI - 0.3]),
        (apple(), [-2.0, 2.0]),
        (lemon(), [-1.0, 1.0]),
    ] {
        for at in [
            DVec2::new(0.4, 0.3),
            DVec2::new(-2.0, -0.7),
            DVec2::new(1.0, poles[0] + 1e-3),
            DVec2::new(1.0, poles[1] - 1e-3),
        ] {
            let here = torus.point(at);
            let round = torus.point(at + DVec2::new(step, 0.0)) - here;
            let along = torus.point(at + DVec2::new(0.0, step)) - here;
            let turned = round.cross(along).normalize();
            let normal = torus.normal(at);
            assert!((normal.length() - 1.0).abs() < CLOSE);
            assert!(
                turned.distance(normal) < 1e-5,
                "{turned} against {normal} at {at}"
            );
        }
    }
}

#[test]
fn the_poles_of_a_spindle_stand_where_its_meridian_circle_meets_the_axis() {
    assert!(ring().poles().is_empty());
    let apple = apple();
    let [low, high] = apple.poles()[..] else {
        panic!("an apple has two poles");
    };
    assert!((high - (-0.5f64).acos()).abs() < CLOSE && (low + high).abs() < CLOSE);
    for pole in [low, high] {
        let point = apple.point(DVec2::new(0.7, pole));
        let from = point - apple.origin;
        assert!(
            (from - apple.axis * from.dot(apple.axis)).length() < 1e-9,
            "{pole}"
        );
    }
    let lemon = lemon();
    let [low, high] = lemon.poles()[..] else {
        panic!("a lemon has two poles");
    };
    assert!((high - (0.5f64).acos()).abs() < CLOSE && (low + high).abs() < CLOSE);
    let horn = Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(0.0, 2.0),
        2.0,
        DVec2::new(0.0, 4.0),
    );
    let [one, other] = horn.poles()[..] else {
        panic!("a horn's one point is both ends of its lobe");
    };
    assert_eq!(one, other);
    let near = horn.point(DVec2::new(0.0, PI - 1e-9));
    let from = near - horn.origin;
    let rho = (from - horn.axis * from.dot(horn.axis)).length();
    assert!(
        (0.0..1e-15).contains(&rho),
        "{rho} a hair from the horn's point"
    );
}

#[test]
fn a_spindle_s_distance_is_read_on_the_own_lobe_alone() {
    let apple = apple();
    let on_the_mirror = DVec3::new(0.9, 0.0, 4.0);
    let lemon = (DVec2::new(4.0, 0.9) - DVec2::new(4.0, -1.0)).length() - 2.0;
    assert!(
        lemon.abs() < 0.1 + CLOSE,
        "{lemon}: the point stands by the mirror lobe"
    );
    let pole = apple.point(DVec2::new(0.0, (-0.5f64).acos()));
    let distance = apple.distance(on_the_mirror);
    assert!(
        (distance.abs() - on_the_mirror.distance(pole)).abs() < CLOSE,
        "{distance} against the pole {} away",
        on_the_mirror.distance(pole)
    );
    assert!(distance < 0.0, "{distance}: inside the apple");
    let nearest = apple.nearest(on_the_mirror);
    assert!((nearest.distance(on_the_mirror) - distance.abs()).abs() < CLOSE);

    let outside = DVec3::new(4.0, 0.0, 4.0);
    assert!((apple.distance(outside) - 1.0).abs() < CLOSE);
    let inside = DVec3::new(2.0, 0.0, 4.0);
    assert!((apple.distance(inside) + 1.0).abs() < CLOSE);
}

#[test]
fn a_latitude_circle_is_held_and_a_meridian_circle_is_told_from_it() {
    let eps = 1e-8;
    for torus in [ring(), apple()] {
        let phi = 0.4;
        let meridian = torus.point(DVec2::new(0.0, phi)) - torus.origin;
        let h = meridian.dot(torus.axis);
        let rho = (meridian - torus.axis * h).length();
        let latitude = torus.circle(h, rho);
        assert!(torus.holds(&latitude, eps));
        assert!(!torus.meridian(&latitude, eps));
        assert!(!torus.holds(&torus.circle(h, rho + 10.0 * eps), eps));

        let center =
            torus.origin + torus.axis * torus.center.x + torus.radial(0.3) * torus.center.y;
        let tube = Circle::about(center, torus.radial(0.3).cross(torus.axis), torus.radius);
        assert!(torus.meridian(&tube, eps));
        assert!(!torus.holds(&tube, eps));
        let off = Circle::about(center + torus.axis * 10.0 * eps, tube.axis, torus.radius);
        assert!(!torus.meridian(&off, eps));
        let leaning = Circle::about(center, torus.axis, torus.radius);
        assert!(!torus.meridian(&leaning, eps));
    }
}

#[test]
fn a_line_crosses_a_ring_four_times_and_passes_through_its_hole() {
    let torus = ring();
    let middle = torus.origin + torus.axis * torus.center.x;
    let direction = torus.radial(0.6);
    let origin = middle - direction * 10.0;
    let roots: Vec<f64> = torus
        .roots(origin, direction)
        .into_iter()
        .flatten()
        .collect();
    let expected = [3.0, 7.0, 13.0, 17.0];
    assert_eq!(roots.len(), 4, "{roots:?}");
    for (root, expected) in roots.iter().zip(expected) {
        assert!((root - expected).abs() < 1e-9, "{roots:?}");
        assert!(torus.distance(origin + direction * root).abs() < 1e-9);
    }
    assert_eq!(
        torus.roots(middle - torus.axis * 10.0, torus.axis),
        [None; 4]
    );
    let through = torus.roots(middle - torus.axis * 10.0 + direction * 5.0, torus.axis);
    let [Some(low), Some(high), None, None] = through else {
        panic!("a line along the axis through the tube crosses twice: {through:?}");
    };
    assert!((low - 8.0).abs() < 1e-9 && (high - 12.0).abs() < 1e-9);
}

#[test]
fn a_line_touching_a_ring_s_crest_is_found_once_and_one_passing_by_is_not() {
    let torus = ring();
    let middle = torus.origin + torus.axis * torus.center.x;
    let direction = torus.radial(FRAC_PI_2);
    for (away, count) in [(7.0, 1), (7.0 + 1e-9, 0), (7.0 - 1e-6, 2)] {
        let origin = middle + torus.radial(0.0) * away - direction * 10.0;
        let roots: Vec<f64> = torus
            .roots(origin, direction)
            .into_iter()
            .flatten()
            .collect();
        assert_eq!(roots.len(), count, "{away}: {roots:?}");
        for root in roots {
            assert!(torus.distance(origin + direction * root).abs() < 1e-8);
        }
    }
}

#[test]
fn a_spindle_s_mirror_lobe_is_never_crossed() {
    let apple = apple();
    let middle = DVec3::new(0.0, 0.0, 4.0);
    let roots: Vec<f64> = apple
        .roots(middle - DVec3::X * 10.0, DVec3::X)
        .into_iter()
        .flatten()
        .collect();
    assert_eq!(roots.len(), 2, "{roots:?}");
    assert!((roots[0] - 7.0).abs() < 1e-9 && (roots[1] - 13.0).abs() < 1e-9);
    let poles: Vec<f64> = apple
        .roots(middle - DVec3::Z * 10.0, DVec3::Z)
        .into_iter()
        .flatten()
        .collect();
    assert_eq!(poles.len(), 2, "{poles:?}");
    let reach = 3f64.sqrt();
    assert!((poles[0] - (10.0 - reach)).abs() < 1e-7 && (poles[1] - (10.0 + reach)).abs() < 1e-7);

    let lemon = lemon();
    let roots: Vec<f64> = lemon
        .roots(middle - DVec3::X * 10.0, DVec3::X)
        .into_iter()
        .flatten()
        .collect();
    assert_eq!(roots.len(), 2, "{roots:?}");
    assert!((roots[0] - 9.0).abs() < 1e-9 && (roots[1] - 11.0).abs() < 1e-9);
}

#[test]
fn a_torus_reaches_its_ring_and_tube_along_any_direction() {
    let torus = ring();
    let direction = DVec3::new(0.2, -0.7, 0.4).normalize();
    let [low, high] = torus.extent(direction);
    let mut furthest = f64::NEG_INFINITY;
    let mut nearest = f64::INFINITY;
    for step in 0..64 {
        for turn in 0..64 {
            let at = DVec2::new(TAU * step as f64 / 64.0, TAU * turn as f64 / 64.0);
            let along = torus.point(at).dot(direction);
            furthest = furthest.max(along);
            nearest = nearest.min(along);
        }
    }
    assert!(low <= nearest && furthest <= high);
    assert!(high - furthest < 0.5 && nearest - low < 0.5);
}
