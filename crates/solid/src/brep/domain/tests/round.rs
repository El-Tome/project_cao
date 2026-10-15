//! The floors of a sphere's chart (#576): which poles a face holds, read by
//! parity off its traces, and the domain closed along them for parity.

use std::f64::consts::{FRAC_PI_2, TAU};

use glam::{DVec2, DVec3};

use crate::brep::domain::crossing::heights;
use crate::brep::domain::floor::{closed_round, holds, poles_of};
use crate::brep::surface::{Sphere, Surface};
use crate::brep::trace::Trace;

fn ball() -> Surface {
    Surface::Sphere(Sphere::through(DVec3::ZERO, DVec3::Z, 0.0, 2.0))
}

/// A whole latitude circle at `phi`, run with the angle growing when
/// `forward`: the face on its left lies above it then.
fn latitude(phi: f64, forward: bool) -> Vec<Trace> {
    let (from, to) = if forward { (0.0, TAU) } else { (TAU, 0.0) };
    vec![Trace::Segment {
        from: DVec2::new(from, phi),
        to: DVec2::new(to, phi),
    }]
}

/// How many times a ray up the meridian from `at` crosses the closed loops.
fn crossed(closed: &[Vec<Trace>], at: DVec2) -> usize {
    heights(closed, at.x, true)
        .into_iter()
        .filter(|height| *height > at.y)
        .count()
}

#[test]
fn a_face_with_no_loop_holds_both_poles_and_a_ray_up_from_inside_crosses_one_floor() {
    let poles = poles_of(&ball(), 1e-9);
    assert_eq!(poles.len(), 2);
    assert!(!poles[0].high && poles[1].high);
    assert!((poles[0].at + FRAC_PI_2).abs() < 1e-15 && (poles[1].at - FRAC_PI_2).abs() < 1e-15);
    assert!(poles[0].point.distance(DVec3::new(0.0, 0.0, -2.0)) < 1e-15);
    assert!(poles.iter().all(|pole| holds(&[], pole)));
    let closed = closed_round(&poles, Vec::new());
    assert_eq!(closed.len(), 2);
    for at in [
        DVec2::new(0.7, 0.3),
        DVec2::new(-2.0, -1.4),
        DVec2::new(3.0, 1.5),
    ] {
        assert_eq!(crossed(&closed, at), 1, "{at}");
    }
}

#[test]
fn a_cap_holds_the_pole_it_lies_towards() {
    let poles = poles_of(&ball(), 1e-9);
    let cap = vec![latitude(0.4, true)];
    assert!(!holds(&cap, &poles[0]) && holds(&cap, &poles[1]));
    let bowl = vec![latitude(0.4, false)];
    assert!(holds(&bowl, &poles[0]) && !holds(&bowl, &poles[1]));

    let closed = closed_round(&poles, cap);
    assert_eq!(closed.len(), 2);
    assert_eq!(crossed(&closed, DVec2::new(1.0, 1.0)) % 2, 1);
    assert_eq!(crossed(&closed, DVec2::new(1.0, 0.0)) % 2, 0);
    let closed = closed_round(&poles, bowl);
    assert_eq!(crossed(&closed, DVec2::new(1.0, -1.0)) % 2, 1);
    assert_eq!(crossed(&closed, DVec2::new(1.0, 1.0)) % 2, 0);
}

#[test]
fn a_zone_holds_no_pole_and_a_lune_through_both_is_closed_along_them() {
    let poles = poles_of(&ball(), 1e-9);
    let caps = vec![latitude(0.8, true), latitude(-0.5, false)];
    assert!(poles.iter().all(|pole| holds(&caps, pole)));
    let zone = vec![latitude(0.8, false), latitude(-0.5, true)];
    assert!(poles.iter().all(|pole| !holds(&zone, pole)));
    let closed = closed_round(&poles, zone);
    assert_eq!(closed.len(), 2);
    assert_eq!(crossed(&closed, DVec2::new(2.0, 0.1)) % 2, 1);
    assert_eq!(crossed(&closed, DVec2::new(2.0, 1.2)) % 2, 0);
    assert_eq!(crossed(&closed, DVec2::new(2.0, -1.2)) % 2, 0);

    let lune = vec![vec![
        Trace::Segment {
            from: DVec2::new(1.0, -FRAC_PI_2),
            to: DVec2::new(1.0, FRAC_PI_2),
        },
        Trace::Segment {
            from: DVec2::new(0.0, FRAC_PI_2),
            to: DVec2::new(0.0, -FRAC_PI_2),
        },
    ]];
    assert!(poles.iter().all(|pole| !holds(&lune, pole)));
    let closed = closed_round(&poles, lune);
    assert_eq!(closed.len(), 1);
    assert_eq!(closed[0].len(), 4);
    assert_eq!(crossed(&closed, DVec2::new(0.5, 0.0)) % 2, 1);
    assert_eq!(crossed(&closed, DVec2::new(3.0, 0.0)) % 2, 0);
}

#[test]
fn a_face_holding_a_pole_beside_a_hole_that_winds_round_nothing_holds_it_still() {
    let poles = poles_of(&ball(), 1e-9);
    let hole = vec![vec![
        Trace::Segment {
            from: DVec2::new(1.0, 0.2),
            to: DVec2::new(1.0, 0.6),
        },
        Trace::Segment {
            from: DVec2::new(1.0, 0.6),
            to: DVec2::new(2.0, 0.6),
        },
        Trace::Segment {
            from: DVec2::new(2.0, 0.6),
            to: DVec2::new(2.0, 0.2),
        },
        Trace::Segment {
            from: DVec2::new(2.0, 0.2),
            to: DVec2::new(1.0, 0.2),
        },
    ]];
    assert!(poles.iter().all(|pole| holds(&hole, pole)));
    let patch: Vec<Vec<Trace>> = hole
        .iter()
        .map(|lap| lap.iter().rev().map(Trace::reversed).collect())
        .collect();
    assert!(poles.iter().all(|pole| !holds(&patch, pole)));
}
