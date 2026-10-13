use std::f64::consts::{PI, TAU};

use glam::DVec3;

use crate::brep::conic::{Arm, Conic, On, Round};
use crate::brep::scale::Scale;
use crate::brep::surface::{EllipticCylinder, Plane};

const REACH: f64 = 20.0;

fn scale() -> Scale {
    Scale::of(REACH)
}

/// A tenth of the tolerance.
fn held() -> f64 {
    scale().eps() / 10.0
}

/// An ellipse of half-axes 7 and 3 about `(1, −2)`, its major half-axis
/// turned 30° off X.
fn oval() -> EllipticCylinder {
    let turned = 30f64.to_radians();
    EllipticCylinder::about(
        DVec3::new(1.0, -2.0, 0.0),
        DVec3::Z,
        DVec3::new(turned.cos(), turned.sin(), 0.0) * 7.0,
        3.0,
    )
}

fn plane(point: DVec3, normal: DVec3) -> Plane {
    Plane::through(point, normal).0
}

fn only(plane: &Plane) -> Conic {
    let conics = Conic::cut(plane, &Round::Elliptic(oval()), scale());
    let [conic] = conics[..] else {
        panic!("one conic: {conics:?}");
    };
    conic
}

/// Planes at a slant through the wall: leaning off square to the axis by
/// small and large angles, every way round.
fn slants() -> Vec<Plane> {
    let mut planes = Vec::new();
    for lean in [1e-3, 0.2, 0.7, 1.2, 1.5] {
        for way in [0.0, 1.0, 2.5, 4.0] {
            let across = DVec3::new(f64::cos(way), f64::sin(way), 0.0);
            let normal = DVec3::Z * f64::cos(lean) + across * f64::sin(lean);
            planes.push(plane(DVec3::new(1.5, -2.5, 3.0), normal));
        }
    }
    planes
}

#[test]
fn a_plane_square_to_an_elliptic_wall_cuts_its_section_whole_from_the_end_of_its_major_half_axis() {
    let wall = oval();
    let conic = only(&plane(DVec3::new(0.0, 0.0, 4.0), DVec3::Z));
    assert_eq!(conic.arm, Arm::Whole);
    assert!((conic.k - 1.0 / 9.0).abs() < 1e-15, "k = {}", conic.k);
    let ends = [1.0, -1.0].map(|side| DVec3::new(1.0, -2.0, 4.0) + wall.u * 7.0 * side);
    let nearest = if ends[0].length() <= ends[1].length() {
        ends[0]
    } else {
        ends[1]
    };
    assert!(conic.anchor.distance(nearest) < 1e-12, "{}", conic.anchor);
    let period = conic.period().expect("a whole ellipse closes");
    assert!((period - TAU * 3.0).abs() < 1e-12);
    for step in 0..=64 {
        let point = conic.point(period * f64::from(step) / 64.0);
        assert!((point.z - 4.0).abs() <= held() && wall.distance(point).abs() <= held());
    }
    assert!(conic.point(period).distance(conic.anchor) <= held());
}

#[test]
fn a_plane_at_a_slant_cuts_an_elliptic_wall_along_a_closed_conic_lying_on_both() {
    let wall = oval();
    for plane in slants() {
        let conic = only(&plane);
        let period = conic.period().expect("a slant cuts an ellipse");
        for step in 0..=96 {
            let point = conic.point(period * f64::from(step) / 96.0);
            let off = plane.distance(point).abs().max(wall.distance(point).abs());
            assert!(off <= held(), "{point} stands {off} off on {plane:?}");
        }
        assert!(conic.point(period).distance(conic.anchor) <= held());
    }
}

#[test]
fn the_conic_on_an_elliptic_wall_is_anchored_at_an_end_of_its_major_axis() {
    for plane in slants() {
        let conic = only(&plane);
        let period = conic.period().expect("a slant cuts an ellipse");
        let reach = |t: f64| conic.point(t).distance(conic.point(t + period / 2.0));
        let major = reach(0.0);
        for step in 1..64 {
            let across = reach(period * f64::from(step) / 128.0);
            assert!(
                across <= major + held(),
                "{across} across beats {major} on {plane:?}"
            );
        }
    }
}

#[test]
fn the_conic_on_an_elliptic_wall_runs_with_the_wall_s_own_angle() {
    let wall = oval();
    for plane in slants() {
        let conic = only(&plane);
        let start = wall.parameters(conic.anchor).x;
        let period = conic.period().expect("closed");
        for step in 0..=24 {
            let t = period * f64::from(step) / 24.0 - period / 2.0;
            let [seen, speed, bend] = conic.seen_on(On::Wall, t);
            assert!((seen.x - (start + conic.k.sqrt() * t)).abs() < 1e-12);
            assert!((speed.x - conic.k.sqrt()).abs() < 1e-15 && bend.x == 0.0);
            let point = conic.point(t);
            let apart = (wall.parameters(point).x - seen.x).rem_euclid(TAU);
            assert!(
                apart.min(TAU - apart) < 1e-9,
                "{t} seen at {} on {plane:?}",
                seen.x
            );
            assert!((seen.y - (point - wall.origin).dot(wall.axis)).abs() < 1e-12);
            assert!((speed.y - conic.derivative(t).dot(wall.axis)).abs() < 1e-12);
        }
    }
}

#[test]
fn the_conic_on_an_elliptic_wall_meets_each_ruling_where_the_plane_does() {
    let wall = oval();
    for plane in slants() {
        let conic = only(&plane);
        for step in 0..48 {
            let angle = TAU * f64::from(step) / 48.0;
            let (t, point) = conic.at_angle(angle);
            let own = wall.parameters(point).x;
            let apart = (own - angle).rem_euclid(TAU);
            assert!(
                apart.min(TAU - apart) <= 1e-12,
                "{point} stands at {own}, not {angle}"
            );
            assert!(plane.distance(point).abs() <= held());
            assert!(conic.point(t).distance(point) <= held(), "{t} on {plane:?}");
        }
    }
}

#[test]
fn a_conic_on_an_elliptic_wall_reads_back_the_parameter_of_its_points() {
    for plane in slants() {
        let conic = only(&plane);
        let period = conic.period().expect("closed");
        for step in 0..32 {
            let t = period * (f64::from(step) / 32.0 - 0.5) + 1e-3;
            let back = conic.parameter(conic.point(t));
            assert!(
                (back - t).abs() * conic.k.sqrt() < 1e-10,
                "{t} read as {back}"
            );
        }
    }
}

#[test]
fn a_conic_on_an_elliptic_wall_built_twice_has_the_same_bits() {
    for plane in slants() {
        let wall = Round::Elliptic(oval());
        assert_eq!(
            Conic::cut(&plane, &wall, scale()),
            Conic::cut(&plane, &wall, scale())
        );
    }
}

#[test]
fn a_plane_along_an_elliptic_wall_cuts_it_along_no_conic() {
    let wall = Round::Elliptic(oval());
    let along = plane(DVec3::new(2.0, 0.0, 0.0), DVec3::X);
    assert!(Conic::cut(&along, &wall, scale()).is_empty());
}

#[test]
fn a_plane_a_hair_off_along_an_elliptic_wall_is_cut_in_two_arms_held_to_the_tolerance() {
    let wall = oval();
    let normal = DVec3::new(1.0, 0.5, 0.0).normalize() * 1e-7f64.cos() + DVec3::Z * 1e-7f64.sin();
    let plane = plane(DVec3::new(2.0, -2.0, 0.0), normal);
    let conics = Conic::cut(&plane, &Round::Elliptic(wall), scale());
    assert_eq!(
        conics.iter().map(|conic| conic.arm).collect::<Vec<_>>(),
        [Arm::Split { side: 1 }, Arm::Split { side: -1 }]
    );
    for conic in &conics {
        assert!(
            (conic.anchor - wall.origin).dot(wall.axis).abs() <= held(),
            "{} is not on the wall's foot",
            conic.anchor
        );
        let mut read = 0;
        for step in -200..=200 {
            let t = f64::from(step) / 10.0;
            let point = conic.point(t);
            if point.abs().max_element() > REACH {
                continue;
            }
            let off = plane.distance(point).abs().max(wall.distance(point).abs());
            assert!(off <= held(), "{point} stands {off} off");
            assert!(conic.holds(point), "{point} is on its own arm");
            read += 1;
        }
        assert!(read > 20, "{read} points in the box");
    }
    let [one, other] = [&conics[0], &conics[1]];
    assert!(!one.holds(other.anchor) && !other.holds(one.anchor));
}

#[test]
fn an_elliptic_wall_s_conic_is_never_wider_than_its_major_half_axis() {
    let conic = only(&slants()[5]);
    assert_eq!(conic.widest(0.0, 1.0), oval().major);
}

#[test]
fn a_whole_turn_of_the_wall_is_one_period_of_its_conic() {
    let conic = only(&slants()[9]);
    let period = conic.period().expect("closed");
    let [start, _, _] = conic.seen_on(On::Wall, 0.0);
    let [end, _, _] = conic.seen_on(On::Wall, period);
    assert!((end.x - start.x - TAU).abs() < 1e-12);
    let half = conic.at_angle(start.x + PI);
    assert!((half.0 - period / 2.0).abs() < 1e-12, "{}", half.0);
}
