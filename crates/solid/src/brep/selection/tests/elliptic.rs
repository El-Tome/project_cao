//! Which side of each other a plane and an elliptic wall stand on (#571),
//! read off the relation of their pair: a plane touching the wall along a
//! ruling, a plane along its axis crossing it along two, and a plane apart
//! from it, read where they stand.

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::relation::{Relation, relation};
use crate::brep::surface::{EllipticCylinder, Plane};

const REACH: f64 = 20.0;

/// Far below the tolerance and far above rounding at this reach.
const ROUNDING: f64 = 1e-12;

fn wall() -> EllipticCylinder {
    EllipticCylinder::about(
        DVec3::new(1.0, -2.0, 0.0),
        DVec3::Z,
        DVec3::new(0.4f64.cos(), 0.4f64.sin(), 0.0) * 7.0,
        3.0,
    )
}

/// The plane along the wall's axis square to X, `offset` beyond its axis,
/// its normal along X.
fn side(offset: f64) -> Plane {
    Plane::through(wall().origin + DVec3::X * offset, DVec3::X).0
}

fn above(first: &Surface, second: &Surface, place: DVec3) -> Option<bool> {
    let scale = Scale::of(REACH);
    let related = relation(first, second, scale);
    wrapped::stands_above(first, second, &related, place, scale, ROUNDING)
}

#[test]
fn a_plane_touching_an_elliptic_wall_stands_outside_it_and_the_wall_on_its_axis_side() {
    let reach = wall().support(DVec3::X);
    for (offset, wall_above) in [(reach, false), (-reach, true)] {
        let [plane, round] = [Surface::Plane(side(offset)), Surface::Elliptic(wall())];
        assert!(matches!(
            relation(&plane, &round, Scale::of(REACH)),
            Relation::Tangent(_)
        ));
        let place = wall().origin + DVec3::new(offset, 0.0, 3.0);
        assert_eq!(above(&plane, &round, place), Some(wall_above), "{offset}");
        assert_eq!(above(&round, &plane, place), Some(true), "{offset}");
    }
}

#[test]
fn a_plane_crossing_an_elliptic_wall_stands_on_either_side_of_it_lobe_by_lobe() {
    let [plane, round] = [Surface::Plane(side(2.0)), Surface::Elliptic(wall())];
    assert!(matches!(
        relation(&plane, &round, Scale::of(REACH)),
        Relation::Lines(_)
    ));
    let theta = |point: DVec3| wall().point(DVec2::new(wall().parameters(point).x, 3.0));
    let beyond = theta(wall().origin + DVec3::X * 10.0);
    let behind = theta(wall().origin - DVec3::X * 10.0);
    assert_eq!(above(&plane, &round, beyond), Some(true));
    assert_eq!(above(&plane, &round, behind), Some(false));
    assert_eq!(above(&round, &plane, beyond), Some(false));
    assert_eq!(above(&round, &plane, behind), Some(true));
}

#[test]
fn a_plane_apart_from_an_elliptic_wall_reads_the_wall_on_the_side_it_stands() {
    let [plane, round] = [Surface::Plane(side(9.0)), Surface::Elliptic(wall())];
    assert_eq!(relation(&plane, &round, Scale::of(REACH)), Relation::Apart);
    let place = wall().origin + DVec3::new(9.0, 0.0, 3.0);
    assert_eq!(above(&plane, &round, place), Some(false));
    assert_eq!(above(&round, &plane, place), None);
}
