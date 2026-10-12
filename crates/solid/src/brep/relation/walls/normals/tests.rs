use glam::DVec2;

use super::*;
use crate::soundness::Random;

/// How far, as a share of the ellipse's size, the normal at a foot may miss
/// the point it was drawn through.
const ROUNDING: f64 = 1e-12;

fn on(major: f64, minor: f64, theta: f64) -> DVec2 {
    DVec2::new(major * theta.cos(), minor * theta.sin())
}

/// How far the normal at the foot `theta` passes from `q`.
fn missed(major: f64, minor: f64, q: DVec2, theta: f64) -> f64 {
    let foot = on(major, minor, theta);
    let normal = DVec2::new(minor * theta.cos(), major * theta.sin()).normalize();
    (q - foot).perp_dot(normal).abs()
}

/// Whether `q` stands strictly inside the evolute of the ellipse, where four
/// normals pass through it.
fn inside_the_evolute(major: f64, minor: f64, q: DVec2) -> bool {
    let span = (major - minor) * (major + minor);
    (major * q.x).abs().powf(2.0 / 3.0) + (minor * q.y).abs().powf(2.0 / 3.0) < span.powf(2.0 / 3.0)
}

fn held(major: f64, minor: f64, q: DVec2) -> Vec<f64> {
    let found = feet(major, minor, q);
    for &theta in &found {
        assert!(
            missed(major, minor, q, theta) <= ROUNDING * major,
            "the normal at {theta} misses {q} by {}",
            missed(major, minor, q, theta)
        );
        assert!(theta > -PI && theta <= PI);
    }
    assert!(found.windows(2).all(|pair| pair[0] < pair[1]), "{found:?}");
    found
}

#[test]
fn a_point_outside_the_evolute_has_two_normals_and_one_inside_it_four() {
    let mut random = Random::seeded(571);
    for _ in 0..2000 {
        let major = random.between(0.5, 9.0);
        let minor = major * random.between(0.05, 0.98);
        let q = DVec2::new(
            random.between(-2.0, 2.0) * major,
            random.between(-2.0, 2.0) * major,
        );
        let found = held(major, minor, q);
        let expected = if inside_the_evolute(major, minor, q) {
            4
        } else {
            2
        };
        assert_eq!(found.len(), expected, "{major} {minor} {q}: {found:?}");
    }
}

#[test]
fn the_distance_to_a_point_only_grows_or_only_falls_between_two_feet() {
    let mut random = Random::seeded(5571);
    for _ in 0..300 {
        let major = random.between(0.5, 9.0);
        let minor = major * random.between(0.05, 0.98);
        let q = DVec2::new(
            random.between(-1.0, 1.0) * major,
            random.between(-1.0, 1.0) * minor,
        );
        let found = held(major, minor, q);
        for (index, &from) in found.iter().enumerate() {
            let to = found.get(index + 1).copied().unwrap_or(found[0] + 2.0 * PI);
            let distances: Vec<f64> = (1..64)
                .map(|step| from + (to - from) * f64::from(step) / 64.0)
                .map(|theta| on(major, minor, theta).distance(q))
                .collect();
            let rising = distances.windows(2).all(|pair| pair[1] >= pair[0] - 1e-12);
            let falling = distances.windows(2).all(|pair| pair[1] <= pair[0] + 1e-12);
            assert!(
                rising || falling,
                "{major} {minor} {q} between {from} and {to}"
            );
        }
    }
}

#[test]
fn a_point_on_an_axis_has_the_vertices_on_that_axis_among_its_feet() {
    let (major, minor) = (5.0, 2.0);
    let near_the_centre = held(major, minor, DVec2::new(1.0, 0.0));
    assert_eq!(near_the_centre.len(), 4);
    assert!(near_the_centre.contains(&0.0) && near_the_centre.contains(&PI));
    let far_out = held(major, minor, DVec2::new(8.0, 0.0));
    assert_eq!(far_out, vec![0.0, PI]);
    let across = held(major, minor, DVec2::new(0.0, -1.0));
    assert_eq!(across.len(), 4);
    assert!(across.contains(&FRAC_PI_2) && across.contains(&-FRAC_PI_2));
    let beyond = held(major, minor, DVec2::new(0.0, 30.0));
    assert_eq!(beyond, vec![-FRAC_PI_2, FRAC_PI_2]);
    assert_eq!(
        held(major, minor, DVec2::ZERO),
        vec![-FRAC_PI_2, 0.0, FRAC_PI_2, PI]
    );
}

#[test]
fn a_point_a_hair_off_an_axis_has_its_feet_where_the_axis_has_them_but_for_the_hair() {
    let (major, minor) = (5.0, 2.0);
    for (q, hair) in [
        (DVec2::new(1.0, 0.0), DVec2::new(0.0, 1e-15)),
        (DVec2::new(1.0, 0.0), DVec2::new(0.0, -1e-11)),
        (DVec2::new(0.0, -1.0), DVec2::new(1e-15, 0.0)),
        (DVec2::new(0.0, -1.0), DVec2::new(-1e-11, 0.0)),
    ] {
        let exact = held(major, minor, q);
        let off = held(major, minor, q + hair);
        assert_eq!(exact.len(), off.len(), "{q} {hair}: {exact:?} {off:?}");
        for (one, other) in exact.iter().zip(&off) {
            let apart = (one - other).abs();
            let apart = apart.min(2.0 * PI - apart);
            assert!(apart <= 1e-8, "{q} {hair}: {exact:?} {off:?}");
        }
    }
}

#[test]
fn a_circle_has_two_feet_along_the_line_through_its_centre() {
    let found = held(3.0, 3.0, DVec2::new(1.0, 1.0));
    assert_eq!(found.len(), 2);
    assert!((found[1] - std::f64::consts::FRAC_PI_4).abs() <= 1e-15);
}
