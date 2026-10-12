//! Where a curve crosses an elliptic wall (#571): a line, by its quadratic,
//! its touch decided once on the gap to the wall's tangent parallel to it;
//! the conic a plane cuts the wall along, through the line its plane shares
//! with the third surface.

use super::*;
use crate::brep::conic::{Conic, Round};
use crate::brep::surface::EllipticCylinder;

/// An ellipse of half-axes 7 and 3 about `(1, −2)`, its major half-axis
/// turned 30° off X, raised along Z.
fn oval() -> EllipticCylinder {
    let turned = 30f64.to_radians();
    EllipticCylinder::about(
        DVec3::new(1.0, -2.0, 0.0),
        DVec3::Z,
        DVec3::new(turned.cos(), turned.sin(), 0.0) * 7.0,
        3.0,
    )
}

/// The line through `point` along `direction`, standing `off` beyond the
/// wall's tangent parallel to it at the angle `theta`, across the axis.
fn tangent_to(theta: f64, off: f64, rise: f64) -> Curve {
    let wall = oval();
    let foot = wall.point(glam::DVec2::new(theta, 4.0));
    let running = wall.u * (-wall.major * theta.sin()) + wall.v * (wall.minor * theta.cos());
    Curve::Line(Line::through(
        foot + wall.normal(theta) * off,
        running.normalize() + DVec3::Z * rise,
    ))
}

#[test]
fn a_line_across_an_elliptic_wall_crosses_it_twice_on_both() {
    let wall = Surface::Elliptic(oval());
    for direction in [
        DVec3::X,
        DVec3::new(0.3, 1.0, 0.0),
        DVec3::new(1.0, -0.4, 0.8),
    ] {
        let line = Curve::Line(Line::through(DVec3::new(2.0, -1.5, 3.0), direction));
        let crossings = found(&line, &wall, 1e-12 * REACH);
        assert_eq!(tangents(&crossings), [false, false], "along {direction}");
    }
}

#[test]
fn a_line_passing_within_the_tolerance_outside_an_elliptic_wall_touches_it_once_where_it_passes_nearest()
 {
    let eps = scale().eps();
    let wall = Surface::Elliptic(oval());
    for theta in [0.0, 0.7, 2.0, -2.6] {
        let line = tangent_to(theta, eps / 2.0, 0.2);
        let crossings = found(&line, &wall, eps);
        assert_eq!(tangents(&crossings), [true], "at {theta}");
        let foot = oval().point(glam::DVec2::new(theta, 0.0));
        let flat = |point: DVec3| DVec3::new(point.x, point.y, 0.0);
        let touch = flat(crossings[0].point);
        assert!(touch.distance(flat(foot)) < 1e-6, "{touch} for {foot}");
        assert!(found(&tangent_to(theta, 2.0 * eps, 0.2), &wall, eps).is_empty());
    }
}

#[test]
fn a_line_a_hair_inside_an_elliptic_wall_crosses_it_twice_unless_a_surface_it_lies_on_touches_it() {
    let eps = scale().eps();
    let wall = Surface::Elliptic(oval());
    let line = tangent_to(1.1, -eps / 2.0, 0.0);
    assert_eq!(tangents(&found(&line, &wall, eps)), [false, false]);
    assert_eq!(tangents(&found_touching(&line, &wall, eps)), [true]);
}

#[test]
fn a_line_along_an_elliptic_wall_s_axis_lies_on_it_or_never_meets_it() {
    let wall = oval();
    let on = Curve::Line(Line::through(
        wall.point(glam::DVec2::new(0.4, 0.0)),
        DVec3::Z,
    ));
    let beside = Curve::Line(Line::through(
        wall.point(glam::DVec2::new(0.4, 0.0)) + wall.normal(0.4),
        DVec3::Z,
    ));
    let surface = Surface::Elliptic(wall);
    assert_eq!(crossings(&on, &surface, scale()), Crossings::Along);
    assert_eq!(
        crossings(&beside, &surface, scale()),
        Crossings::At(Vec::new())
    );
}

#[test]
fn the_conic_on_an_elliptic_wall_crosses_a_plane_where_the_line_they_share_crosses_the_wall() {
    let wall = oval();
    let slant = Plane::through(DVec3::new(0.0, 0.0, 6.0), DVec3::new(0.3, 0.2, 1.0)).0;
    let [conic] = Conic::cut(&slant, &Round::Elliptic(wall), scale())[..] else {
        panic!("one conic");
    };
    let side = plane(DVec3::new(3.0, 0.0, 0.0), DVec3::new(1.0, 0.3, 0.1));
    let met = found(&Curve::Conic(conic), &side, 1e-12 * REACH);
    assert_eq!(tangents(&met), [false, false]);
    for crossing in &met {
        assert!(slant.distance(crossing.point).abs() <= 1e-12 * REACH);
        assert!(wall.distance(crossing.point).abs() <= 1e-12 * REACH);
    }
    let Relation::Line(shared) = relation(&Surface::Plane(slant), &side, scale()) else {
        panic!("the two planes cross along a line");
    };
    let Crossings::At(through) = crossings(&Curve::Line(shared), &Surface::Elliptic(wall), scale())
    else {
        panic!("the line crosses the wall");
    };
    let mut bits: Vec<[u64; 3]> = through
        .iter()
        .map(|crossing| crossing.point.to_array().map(f64::to_bits))
        .collect();
    let mut reached: Vec<[u64; 3]> = met
        .iter()
        .map(|crossing| crossing.point.to_array().map(f64::to_bits))
        .collect();
    bits.sort();
    reached.sort();
    assert_eq!(reached, bits, "one corner, whichever curve reaches it");
}

#[test]
fn the_conic_on_an_elliptic_wall_lies_along_its_own_wall() {
    let wall = oval();
    let slant = Plane::through(DVec3::new(0.0, 0.0, 6.0), DVec3::new(0.3, 0.2, 1.0)).0;
    let [conic] = Conic::cut(&slant, &Round::Elliptic(wall), scale())[..] else {
        panic!("one conic");
    };
    assert_eq!(
        crossings(&Curve::Conic(conic), &Surface::Elliptic(wall), scale()),
        Crossings::Along
    );
}
