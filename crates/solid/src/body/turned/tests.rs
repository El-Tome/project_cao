//! A profile the flats turn is first laid on its axis where the drawing can
//! only have meant it there (#487, #488).

use std::f64::consts::{FRAC_PI_2, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::Body;
use crate::brep::Declined;
use crate::mesh::tests::fan;
use crate::soundness::{closed, uncrossed};
use crate::sweep::Loop;
use crate::turning::Axis;

const FLAT: Frame = Frame {
    origin: DVec3::ZERO,
    u: DVec3::X,
    v: DVec3::Y,
};

const V: Axis = Axis {
    origin: DVec2::ZERO,
    direction: DVec2::Y,
};

fn sampled<'a>(
    points: &'a [DVec2],
    curves: &'a [Option<usize>],
    triangles: &'a [[DVec2; 3]],
) -> Profile<'a> {
    Profile {
        exact: None,
        sampled: Loop { points, curves },
        sampled_holes: Vec::new(),
        triangles,
    }
}

fn square(left: f64) -> Vec<DVec2> {
    vec![
        DVec2::new(left, 0.0),
        DVec2::new(1.0, 0.0),
        DVec2::new(1.0, 1.0),
        DVec2::new(left, 1.0),
    ]
}

#[test]
fn a_square_a_hair_across_its_axis_turns_on_the_flats_into_the_square_on_it() {
    let on_it = square(0.0);
    let on_it_triangles = fan(&on_it);
    let drawn_on_it = sampled(&on_it, &[], &on_it_triangles);
    for hair in [-1e-4, -1e-6, 1e-6, 1e-4] {
        let off = square(hair);
        let triangles = fan(&off);
        let profile = sampled(&off, &[], &triangles);
        for angle in [TAU, -TAU, FRAC_PI_2, 2.5] {
            let turn = Turn::of(V, angle, 0.0, &profile);
            let turned = turned_flats(&profile, FLAT, &turn).expect("a square turned");
            let expected = turned_flats(&drawn_on_it, FLAT, &turn).expect("a square turned");
            assert_eq!(turned, expected, "a hair of {hair} turned {angle} rad");
            assert_eq!(closed(&turned.triangles()), Ok(()), "turned {angle} rad");
            assert_eq!(uncrossed(&turned.triangles()), Ok(()), "turned {angle} rad");
        }
    }
}

#[test]
fn a_profile_with_an_arc_a_hair_off_its_axis_comes_out_closed() {
    let hair = 1e-6;
    let mut points = vec![
        DVec2::new(hair, 0.0),
        DVec2::new(2.0, 0.0),
        DVec2::new(2.0, 1.0),
    ];
    let mut curves = vec![None, None];
    for step in 1..=8 {
        let angle = FRAC_PI_2 * step as f64 / 8.0;
        points.push(DVec2::new(1.0 + angle.cos(), 1.0 + angle.sin()));
        curves.push(Some(0));
    }
    points.push(DVec2::new(hair, 2.0));
    curves.extend([None, None]);
    let triangles = fan(&points);
    let profile = sampled(&points, &curves, &triangles);
    for angle in [TAU, FRAC_PI_2] {
        let turn = Turn::of(V, angle, 0.0, &profile);
        let turned = turned_flats(&profile, FLAT, &turn).expect("a rounded profile turned");
        assert_eq!(closed(&turned.triangles()), Ok(()), "turned {angle} rad");
        assert_eq!(uncrossed(&turned.triangles()), Ok(()), "turned {angle} rad");
    }
}

#[test]
fn a_profile_across_its_axis_past_the_band_is_declined() {
    for (left, declined) in [(-1.1e-3, true), (-0.9e-3, false)] {
        let across = square(left);
        let triangles = fan(&across);
        let profile = sampled(&across, &[], &triangles);
        let turn = Turn::of(V, TAU, 0.0, &profile);
        let turned = Body::default().tool_turned(&profile, FLAT, &turn);
        if declined {
            assert_eq!(turned, Err(Declined::Profile), "{left} across the axis");
        } else {
            let turned = turned.expect("a square within the band of its axis");
            assert_eq!(
                closed(&turned.triangles()),
                Ok(()),
                "{left} across the axis"
            );
        }
    }
}
