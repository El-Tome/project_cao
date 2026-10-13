//! A listing holding the curve two square walls meet along, an elliptic one
//! among them (#571), raised and cut by the kernel and read by soundness's
//! own formula for it.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::super::meet;
use super::{Mislisted, listed};
use crate::brep::{Body, Curve, EllipticCylinder, Listing, Meet, Wall};
use crate::profile::{Contour, Frame, Run};

/// A whole ellipse or circle about `center`, `first` its first half-axis.
fn section(center: DVec2, first: DVec2, second: f64) -> Contour {
    let run = if first.length() == second {
        Run::Round { center, turn: TAU }
    } else {
        Run::Oval {
            center,
            first,
            second,
            turn: TAU,
        }
    };
    Contour {
        corners: vec![center + first],
        runs: vec![run],
    }
}

/// An elliptic boss on Z, half-axes 7 and 4 turned 0.4 off X, 10 tall,
/// drilled along X from `x = -10` to `end` by `hole`, drawn in `(y, z)`.
fn drilled(hole: Contour, end: f64) -> (Listing, f64) {
    let ground = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    let boss = section(DVec2::ZERO, DVec2::from_angle(0.4) * 7.0, 4.0);
    let boss = Body::raised(&boss, &[], ground, DVec3::Z * 10.0).expect("the boss raises");
    let side = Frame {
        origin: DVec3::X * -10.0,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let bit = Body::raised(&hole, &[], side, DVec3::X * (end + 10.0)).expect("the bit raises");
    let body = boss.cut_by(&bit).expect("the hole is drilled");
    (body.listing(), body.scale().reach())
}

fn holes() -> Vec<Contour> {
    vec![
        section(DVec2::new(1.0, 5.0), DVec2::X * 2.0, 2.0),
        section(DVec2::new(0.5, 5.0), DVec2::from_angle(-0.7) * 2.5, 1.2),
    ]
}

fn meets(listing: &Listing) -> Vec<(usize, Meet)> {
    listing
        .edges
        .iter()
        .enumerate()
        .filter_map(|(rank, edge)| match edge.curve {
            Curve::Meet(meet) => Some((rank, meet)),
            _ => None,
        })
        .collect()
}

#[test]
fn an_elliptic_boss_drilled_from_the_side_keeps_every_rule_of_a_listing() {
    for hole in holes() {
        for end in [10.0, 0.0, 5.0] {
            let (listing, reach) = drilled(hole.clone(), end);
            assert!(!meets(&listing).is_empty());
            assert_eq!(listed(&listing, reach), Ok(()), "{end}");
        }
    }
}

#[test]
fn the_listing_reads_a_point_of_an_elliptic_meet_where_the_kernel_puts_it() {
    for hole in holes() {
        let (listing, reach) = drilled(hole, 10.0);
        for (_, meet) in meets(&listing) {
            let period = meet.period().expect("a component closes on itself");
            for step in 0..=200 {
                let at = period * f64::from(step) / 200.0;
                let gap = meet::point(&meet, at).distance(meet.point(at));
                assert!(gap <= 1e-12 * reach, "{meet:?} at {at}: {gap}");
            }
        }
    }
}

#[test]
fn an_elliptic_meet_whose_wall_slid_a_hair_is_found_off_the_faces_beside_it() {
    let slid = |hair: f64| {
        let (mut listing, reach) = drilled(holes()[1].clone(), 0.0);
        let (rank, meet) = meets(&listing)[0];
        let Wall::Elliptic(oval) = meet.second else {
            panic!("the hole is elliptic")
        };
        let moved = EllipticCylinder {
            origin: oval.origin + oval.v * hair,
            ..oval
        };
        listing.edges[rank].curve = Curve::Meet(Meet {
            second: Wall::Elliptic(moved),
            ..meet
        });
        listed(&listing, reach)
    };
    let found = slid(1e-6);
    assert!(
        matches!(
            found,
            Err(Mislisted::OffFace { .. } | Mislisted::EndAway { .. })
        ),
        "{found:?}"
    );
    assert_eq!(slid(1e-12), Ok(()));
}
