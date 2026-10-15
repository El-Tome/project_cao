//! Whole spheres and tori in a listing (#576): one face, no loop, every
//! pole of the surface held.

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::{Sphere, Torus};

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
