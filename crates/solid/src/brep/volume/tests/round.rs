//! The 1-form a face of a sphere or a torus is read with, held to the flux
//! of the position integrated straight over the surface (#576).

use std::f64::consts::{FRAC_PI_2, TAU};

use super::super::round::Meridian;
use super::*;
use crate::brep::curve::Curve;
use crate::brep::surface::{Sphere, Torus};

/// A ball of radius 3 about `h = 5` on a slanted axis off the origin.
fn ball() -> Sphere {
    Sphere::through(
        DVec3::new(1.0, -2.0, 3.0),
        DVec3::new(0.0, 1.0, 1.0),
        5.0,
        3.0,
    )
}

/// A ring of tube radius 2 about a circle of radius 5 at `h = 4`, on the
/// same axis.
fn ring() -> Torus {
    Torus::through(
        DVec3::new(1.0, -2.0, 3.0),
        DVec3::new(0.0, 1.0, 1.0),
        DVec2::new(4.0, 5.0),
        2.0,
        DVec2::new(4.0, 7.0),
    )
}

/// `∬ f dθ dφ` over a box of the chart, by composite Gauss–Legendre.
fn over(f: impl Fn(DVec2) -> f64, [from, to]: [f64; 2], [low, high]: [f64; 2]) -> f64 {
    const PANELS: usize = 48;
    let nodes = || {
        GAUSS_LEGENDRE
            .iter()
            .flat_map(|&(node, weight)| [(-node, weight), (node, weight)])
    };
    let mut total = 0.0;
    for row in 0..PANELS {
        for column in 0..PANELS {
            let (theta, width) = (
                from + (to - from) * (row as f64 + 0.5) / PANELS as f64,
                (to - from) / PANELS as f64,
            );
            let (phi, height) = (
                low + (high - low) * (column as f64 + 0.5) / PANELS as f64,
                (high - low) / PANELS as f64,
            );
            for (s, u) in nodes() {
                for (t, v) in nodes() {
                    let at = DVec2::new(theta + s * width / 2.0, phi + t * height / 2.0);
                    total += u * v * f(at) * width * height / 4.0;
                }
            }
        }
    }
    total
}

#[test]
fn a_cap_s_flux_is_the_centre_s_share_of_its_vector_area_plus_the_radius_times_its_area() {
    let sphere = ball();
    let surface = Surface::Sphere(sphere);
    let meridian = Meridian::of(&surface).expect("a sphere is round");
    let r = sphere.radius;
    for phi in [-1.2, -0.3, 0.0, 0.7, 1.4] {
        let flux = meridian
            .straight(FRAC_PI_2, DVec2::new(0.0, phi), DVec2::new(TAU, phi))
            .expect("a latitude circle has a closed form");
        let (sin, cos) = f64::sin_cos(phi);
        let vector_area = sphere.axis * PI * r * r * cos * cos;
        let area = TAU * r * r * (1.0 - sin);
        assert_relative(flux, sphere.middle().dot(vector_area) + r * area);
    }
}

#[test]
fn a_zone_of_a_torus_sums_to_the_flux_integrated_over_its_surface() {
    let torus = ring();
    let surface = Surface::Torus(torus);
    let meridian = Meridian::of(&surface).expect("a torus is round");
    let (r, big) = (torus.radius, torus.center.y);
    for (low, high, anchor) in [(-0.8, 1.9, -2.5), (0.0, TAU, 0.3), (2.0, 4.0, 2.0)] {
        let flux = meridian
            .straight(anchor, DVec2::new(0.0, low), DVec2::new(TAU, low))
            .expect("closed")
            + meridian
                .straight(anchor, DVec2::new(TAU, high), DVec2::new(0.0, high))
                .expect("closed");
        let integral = over(
            |at| torus.point(at).dot(torus.normal(at)) * r * (big + r * at.y.cos()),
            [0.0, TAU],
            [low, high],
        );
        assert_relative(flux, integral);
    }
}

#[test]
fn a_latitude_arc_s_closed_form_is_its_quadrature() {
    let torus = ring();
    let surface = Surface::Torus(torus);
    let meridian = Meridian::of(&surface).expect("a torus is round");
    for (phi, from, to, anchor) in [(0.4, -1.0, 2.5, 0.0), (2.9, 0.3, 0.9, 1.0)] {
        let (sin, cos) = f64::sin_cos(phi);
        let circle = torus.circle(
            torus.center.x + torus.radius * sin,
            torus.center.y + torus.radius * cos,
        );
        let closed = meridian
            .straight(anchor, DVec2::new(from, phi), DVec2::new(to, phi))
            .expect("closed");
        let quadrature = along(&Curve::Circle(circle), from, to, &surface, anchor);
        assert_relative(closed, quadrature);
    }
    let sphere = ball();
    let surface = Surface::Sphere(sphere);
    let meridian = Meridian::of(&surface).expect("a sphere is round");
    let phi = -0.6;
    let (sin, cos) = f64::sin_cos(phi);
    let circle = sphere.circle(sphere.center + sphere.radius * sin, sphere.radius * cos);
    let closed = meridian
        .straight(0.0, DVec2::new(0.2, phi), DVec2::new(3.0, phi))
        .expect("closed");
    assert_relative(
        closed,
        along(&Curve::Circle(circle), 0.2, 3.0, &surface, 0.0),
    );
    assert_eq!(
        meridian.straight(0.0, DVec2::new(1.0, -0.2), DVec2::new(1.0, 0.9)),
        Some(0.0)
    );
}
