//! Whole spheres and tori in a listing (#576): one face, no loop, every
//! pole of the surface held.

use glam::{DVec2, DVec3};

use std::f64::consts::TAU;

use super::*;
use crate::brep::{Circle, Cylinder, ListedEdge, Plane, Sphere, Torus};

const REACH: f64 = 10.0;

fn ball(poles: Vec<usize>) -> Listing {
    let sphere = Sphere::through(DVec3::new(1.0, 2.0, 3.0), DVec3::Z, 0.0, 4.0);
    Listing {
        faces: vec![ListedFace {
            surface: Surface::Sphere(sphere),
            outward: true,
            loops: Vec::new(),
            poles,
        }],
        edges: Vec::new(),
        vertices: vec![
            DVec3::new(1.0, 2.0, -1.0),
            DVec3::new(1.0, 2.0, 7.0),
            DVec3::new(5.0, 2.0, 3.0),
        ],
    }
}

fn ring() -> Listing {
    let torus = Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(1.0, 5.0),
        2.0,
        DVec2::new(1.0, 7.0),
    );
    Listing {
        faces: vec![ListedFace {
            surface: Surface::Torus(torus),
            outward: true,
            loops: Vec::new(),
            poles: Vec::new(),
        }],
        edges: Vec::new(),
        vertices: Vec::new(),
    }
}

#[test]
fn a_ball_is_one_face_with_no_loop_holding_its_two_poles() {
    assert_eq!(listed(&ball(vec![0, 1]), REACH), Ok(()));
    assert_eq!(listed(&ring(), REACH), Ok(()));
}

#[test]
fn a_sphere_face_holding_one_pole_or_none_is_unbounded() {
    for poles in [Vec::new(), vec![0]] {
        assert_eq!(
            listed(&ball(poles), REACH),
            Err(Mislisted::Unbounded { face: 0 })
        );
    }
}

#[test]
fn a_pole_a_sphere_face_holds_on_its_equator_is_found_away_from_both_poles() {
    let listing = ball(vec![0, 2]);
    assert_eq!(
        listed(&listing, REACH),
        Err(Mislisted::ApexAway {
            face: 0,
            vertex: 2,
            distance: 32f64.sqrt(),
        })
    );
}

/// A cap of the ball of radius 5 about the origin, cut at `z = -3`: the
/// sphere's face above the circle, holding the pole, and the disc below.
fn cap(forward: bool, poles: Vec<usize>) -> Listing {
    let sphere = Sphere::through(DVec3::ZERO, DVec3::Z, 0.0, 5.0);
    let rim = Circle::on(&Cylinder::about(DVec3::ZERO, DVec3::Z, 4.0), -3.0);
    let (disc, _) = Plane::through(DVec3::new(0.0, 0.0, -3.0), DVec3::Z);
    with_sides(Listing {
        faces: vec![
            ListedFace {
                surface: Surface::Sphere(sphere),
                outward: true,
                loops: vec![vec![(0, forward)]],
                poles,
            },
            ListedFace {
                surface: Surface::Plane(disc),
                outward: false,
                loops: vec![vec![(0, !forward)]],
                poles: Vec::new(),
            },
        ],
        edges: vec![ListedEdge {
            curve: Curve::Circle(rim),
            from: 0.0,
            to: TAU,
            ends: None,
            sides: Vec::new(),
        }],
        vertices: vec![DVec3::new(0.0, 0.0, 5.0)],
    })
}

#[test]
fn a_cap_holding_its_pole_is_sound_and_one_wound_the_wrong_way_or_holding_none_is_found() {
    assert_eq!(listed(&cap(true, vec![0]), REACH), Ok(()));
    assert_eq!(
        listed(&cap(true, Vec::new()), REACH),
        Err(Mislisted::Pointed { face: 0 })
    );
    assert_eq!(
        listed(&cap(false, vec![0]), REACH),
        Err(Mislisted::Backwards { face: 0, lap: 0 })
    );
}

/// A ring of tube radius 2 about a circle of radius 6 at `z = 3`, turned a
/// quarter from the plane `y = 0` to the plane `x = 0`: the torus's face
/// between two whole meridian circles, and a disc on each end.
fn quarter_ring() -> Listing {
    let torus = Torus::through(
        DVec3::ZERO,
        DVec3::Z,
        DVec2::new(3.0, 6.0),
        2.0,
        DVec2::new(3.0, 8.0),
    );
    let opening = Circle::about(DVec3::new(6.0, 0.0, 3.0), DVec3::Y, 2.0);
    let closing = Circle::about(DVec3::new(0.0, 6.0, 3.0), DVec3::X, 2.0);
    let meridian = |circle: Circle| ListedEdge {
        curve: Curve::Circle(circle),
        from: 0.0,
        to: TAU,
        ends: None,
        sides: Vec::new(),
    };
    with_sides(Listing {
        faces: vec![
            ListedFace {
                surface: Surface::Torus(torus),
                outward: true,
                loops: vec![vec![(0, true)], vec![(1, false)]],
                poles: Vec::new(),
            },
            ListedFace {
                surface: Surface::Plane(Plane::through(DVec3::ZERO, DVec3::Y).0),
                outward: false,
                loops: vec![vec![(0, false)]],
                poles: Vec::new(),
            },
            ListedFace {
                surface: Surface::Plane(Plane::through(DVec3::ZERO, DVec3::X).0),
                outward: true,
                loops: vec![vec![(1, true)]],
                poles: Vec::new(),
            },
        ],
        edges: vec![meridian(opening), meridian(closing)],
        vertices: Vec::new(),
    })
}

#[test]
fn an_edge_a_hair_off_its_torus_is_found_off_it_by_the_listing_s_own_formula() {
    let mut listing = quarter_ring();
    assert_eq!(listed(&listing, REACH), Ok(()));
    let Curve::Circle(circle) = &mut listing.edges[0].curve else {
        panic!("a meridian circle");
    };
    circle.radius += 1e-6;
    assert!(matches!(
        listed(&listing, REACH),
        Err(Mislisted::OffFace { edge: 0, .. })
    ));
}
