//! A listing on an elliptic wall (#571), raised by the kernel and read by
//! soundness's own formulas: the wall's by Newton's steps along the
//! eccentric angle, its rims' by the conic's.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::{Mislisted, listed};
use crate::brep::{Body, Conic, Curve, EllipticCylinder, Listing, Surface};
use crate::profile::{Contour, Frame, Run};

/// An ellipse of half-axes 7 and 3 about `(3, −1)`, turned 0.4 off X,
/// raised 10, and how far it reaches.
fn elliptic_prism() -> (Listing, f64) {
    let first = DVec2::from_angle(0.4) * 7.0;
    let center = DVec2::new(3.0, -1.0);
    let ellipse = Contour {
        corners: vec![center + first],
        runs: vec![Run::Oval {
            center,
            first,
            second: 3.0,
            turn: TAU,
        }],
    };
    let frame = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    let body = Body::raised(&ellipse, &[], frame, DVec3::Z * 10.0).expect("an ellipse raises");
    (body.listing(), body.scale().reach())
}

fn wall_of(listing: &Listing) -> (usize, EllipticCylinder) {
    listing
        .faces
        .iter()
        .enumerate()
        .find_map(|(rank, face)| match face.surface {
            Surface::Elliptic(wall) => Some((rank, wall)),
            Surface::Plane(_)
            | Surface::Cylinder(_)
            | Surface::Cone(_)
            | Surface::Sphere(_)
            | Surface::Torus(_) => None,
        })
        .expect("the prism stands on an elliptic wall")
}

#[test]
fn an_elliptic_prism_raised_by_the_kernel_keeps_every_rule_of_a_listing() {
    let (listing, reach) = elliptic_prism();
    assert_eq!(listed(&listing, reach), Ok(()));
}

#[test]
fn a_rim_a_hair_off_its_elliptic_wall_is_found_off_it() {
    let slid = |hair: f64| {
        let (mut listing, reach) = elliptic_prism();
        let (_, wall) = wall_of(&listing);
        let Curve::Conic(rim) = listing.edges[0].curve else {
            panic!("the rim is a conic");
        };
        listing.edges[0].curve = Curve::Conic(Conic {
            anchor: rim.anchor + wall.u * hair,
            ..rim
        });
        listed(&listing, reach)
    };
    let (listing, _) = elliptic_prism();
    let (face, _) = wall_of(&listing);
    let found = slid(1e-6);
    assert!(
        matches!(found, Err(Mislisted::OffFace { edge: 0, face: off, distance, .. }) if off == face && distance > 1e-8),
        "{found:?}"
    );
    assert_eq!(slid(1e-11), Ok(()));
}

#[test]
fn an_elliptic_wall_a_hair_wider_than_its_rims_is_found_off_them() {
    let widened = |hair: f64| {
        let (mut listing, reach) = elliptic_prism();
        let (face, wall) = wall_of(&listing);
        listing.faces[face].surface = Surface::Elliptic(EllipticCylinder {
            major: wall.major + hair,
            ..wall
        });
        listed(&listing, reach)
    };
    let found = widened(1e-6);
    assert!(
        matches!(found, Err(Mislisted::OffFace { distance, .. }) if distance > 1e-8),
        "{found:?}"
    );
    assert_eq!(widened(1e-11), Ok(()));
}
