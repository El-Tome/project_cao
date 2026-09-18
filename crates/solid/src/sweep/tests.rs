//! What a solid raised from a drawing holds.

use glam::{DVec2, DVec3};

use super::{prism, revolution};
use crate::mesh::tests::{box_of, fan, square, volume};

fn profile(min: DVec2, max: DVec2) -> Vec<DVec2> {
    vec![min, DVec2::new(max.x, min.y), max, DVec2::new(min.x, max.y)]
}

/// Pappus: sweeping an area right round gives its area times the distance
/// travelled by its centre.
#[test]
fn a_full_turn_gives_pappus_volume() {
    let outline = profile(DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0));
    let solid = revolution(
        &outline,
        &[],
        &fan(&outline),
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec2::ZERO,
        DVec2::Y,
        std::f64::consts::TAU,
    )
    .expect("a profile on one side of the axis");

    let expected = std::f64::consts::TAU * 4.0 * 4.0;
    let made = volume(&solid);
    assert!(
        (made - expected).abs() / expected < 0.01,
        "{made} / {expected}"
    );
}

/// A part turn is capped at both ends, and holds the matching share.
#[test]
fn a_quarter_turn_holds_a_quarter_of_the_volume() {
    let outline = profile(DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0));
    let solid = revolution(
        &outline,
        &[],
        &fan(&outline),
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec2::ZERO,
        DVec2::Y,
        std::f64::consts::FRAC_PI_2,
    )
    .expect("a quarter turn");

    let expected = std::f64::consts::FRAC_PI_2 * 4.0 * 4.0;
    let made = volume(&solid);
    assert!(
        (made - expected).abs() / expected < 0.02,
        "{made} / {expected}"
    );
}

/// Turning the other way must not turn the solid inside out.
#[test]
fn turning_backwards_still_faces_outwards() {
    let outline = profile(DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0));
    let solid = revolution(
        &outline,
        &[],
        &fan(&outline),
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec2::ZERO,
        DVec2::Y,
        -std::f64::consts::FRAC_PI_2,
    )
    .expect("a quarter turn the other way");
    assert!(volume(&solid) > 0.0, "{}", volume(&solid));
}

/// A profile lying across the axis would sweep through itself.
#[test]
fn a_profile_across_the_axis_is_refused() {
    let outline = profile(DVec2::new(-2.0, 0.0), DVec2::new(5.0, 2.0));
    assert!(
        revolution(
            &outline,
            &[],
            &fan(&outline),
            |point| DVec3::new(point.x, point.y, 0.0),
            DVec2::ZERO,
            DVec2::Y,
            std::f64::consts::TAU,
        )
        .is_none()
    );
}

#[test]
fn a_prism_holds_the_volume_of_its_face() {
    let solid = box_of(10.0, 4.0, DVec3::ZERO);
    assert!((volume(&solid) - 400.0).abs() < 1e-2, "{}", volume(&solid));
}

/// Extruding the other way must not turn the solid inside out.
#[test]
fn extruding_backwards_still_faces_outwards() {
    let outline = square(10.0);
    let solid = prism(
        &outline,
        &[],
        &fan(&outline),
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec3::NEG_Z * 4.0,
    );
    assert!((volume(&solid) - 400.0).abs() < 1e-2, "{}", volume(&solid));
}

/// A tube: the walls of the hole belong to the surface, and the middle is
/// not matter.
#[test]
fn a_hole_is_kept_hollow() {
    let outline = square(10.0);
    let hole = vec![
        DVec2::new(3.0, 3.0),
        DVec2::new(7.0, 3.0),
        DVec2::new(7.0, 7.0),
        DVec2::new(3.0, 7.0),
    ];
    // The face of the ring, cut by hand into four strips.
    let triangles = vec![
        [
            DVec2::new(0.0, 0.0),
            DVec2::new(10.0, 0.0),
            DVec2::new(7.0, 3.0),
        ],
        [
            DVec2::new(0.0, 0.0),
            DVec2::new(7.0, 3.0),
            DVec2::new(3.0, 3.0),
        ],
        [
            DVec2::new(10.0, 0.0),
            DVec2::new(10.0, 10.0),
            DVec2::new(7.0, 7.0),
        ],
        [
            DVec2::new(10.0, 0.0),
            DVec2::new(7.0, 7.0),
            DVec2::new(7.0, 3.0),
        ],
        [
            DVec2::new(10.0, 10.0),
            DVec2::new(0.0, 10.0),
            DVec2::new(3.0, 7.0),
        ],
        [
            DVec2::new(10.0, 10.0),
            DVec2::new(3.0, 7.0),
            DVec2::new(7.0, 7.0),
        ],
        [
            DVec2::new(0.0, 10.0),
            DVec2::new(0.0, 0.0),
            DVec2::new(3.0, 3.0),
        ],
        [
            DVec2::new(0.0, 10.0),
            DVec2::new(3.0, 3.0),
            DVec2::new(3.0, 7.0),
        ],
    ];

    let solid = prism(
        &outline,
        std::slice::from_ref(&hole),
        &triangles,
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec3::Z * 2.0,
    );

    // (100 - 16) × 2
    assert!((volume(&solid) - 168.0).abs() < 1e-2, "{}", volume(&solid));
}
