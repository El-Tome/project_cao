//! How a profile is read for a turn: whole or not, on which side of its axis,
//! and how close to the axis counts as on it, as the flats have it (#533).

use std::f64::consts::{FRAC_PI_2, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::mesh::tests::fan;
use crate::profile::{Contour, Frame};
use crate::sweep::{self, Loop};

const FLAT: Frame = Frame {
    origin: DVec3::ZERO,
    u: DVec3::X,
    v: DVec3::Y,
};

const V: Axis = Axis {
    origin: DVec2::ZERO,
    direction: DVec2::Y,
};

fn corners(low: DVec2, high: DVec2) -> Vec<DVec2> {
    Contour::rectangle(low, high).corners
}

fn sampled(points: &[DVec2]) -> Profile<'_> {
    Profile {
        exact: None,
        sampled: Loop::straight(points),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

fn drawn_only_exactly(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

fn flats_faces_end(points: &[DVec2], angle: f64) -> usize {
    sweep::revolution(
        Loop::straight(points),
        &[],
        &fan(points),
        |point| FLAT.at(point),
        V.origin,
        V.direction,
        angle,
    )
    .expect("a profile on one side of its axis")
    .faces_end()
}

fn lie(points: &[DVec2], angle: f64) -> Lie {
    let area = sampled(points);
    Turn::of(V, angle, 0.0, &area).lie(&area)
}

#[test]
fn a_turn_within_a_thousandth_of_a_radian_of_a_whole_turn_is_whole_as_the_flats_have_it() {
    let ring = corners(DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0));
    for (angle, whole) in [
        (TAU, true),
        (-TAU, true),
        (TAU - 5e-4, true),
        (-(TAU - 5e-4), true),
        (TAU + 5e-4, true),
        (TAU - 2e-3, false),
        (-(TAU - 2e-3), false),
        (FRAC_PI_2, false),
    ] {
        assert_eq!(is_whole(angle), whole, "a turn of {angle} rad");
        let area = sampled(&ring);
        assert_eq!(Turn::of(V, angle, 0.0, &area).is_whole(), whole);
        let ends = if whole { 0 } else { 2 };
        assert_eq!(
            flats_faces_end(&ring, angle),
            ring.len() + ends,
            "the flats turn {angle} rad with {ends} ends",
        );
    }
}

#[test]
fn the_band_is_a_thousandth_of_the_area_s_furthest_point_from_its_axis() {
    let ring = corners(DVec2::new(1.0, 0.0), DVec2::new(5.0, 2.0));
    let turn = Turn::of(V, TAU, 2e-5, &sampled(&ring));
    assert_eq!(turn.on_the_axis, ON_THE_AXIS * 5.0);
    assert_eq!(turn.resolution, 2e-5);
    assert_eq!((turn.axis, turn.angle), (V, TAU));

    let across = Axis {
        origin: DVec2::new(0.0, 1.0),
        direction: DVec2::X * 3.0,
    };
    let exactly = drawn_only_exactly(Contour::rectangle(
        DVec2::new(1.0, -7.0),
        DVec2::new(5.0, 0.5),
    ));
    let turn = Turn::of(across, TAU, 0.0, &exactly);
    assert_eq!(
        turn.on_the_axis,
        ON_THE_AXIS * 8.0,
        "read off the exact corners when nothing was sampled",
    );
}

#[test]
fn an_area_on_one_side_of_its_axis_lies_on_that_side() {
    assert_eq!(
        lie(&corners(DVec2::new(1.0, 0.0), DVec2::new(5.0, 2.0)), TAU),
        Lie::Side(-1.0),
    );
    assert_eq!(
        lie(
            &corners(DVec2::new(-5.0, 0.0), DVec2::new(-1.0, 2.0)),
            -FRAC_PI_2
        ),
        Lie::Side(1.0),
    );
    assert_eq!(
        lie(&corners(DVec2::new(0.0, 0.0), DVec2::new(5.0, 2.0)), TAU),
        Lie::Side(-1.0),
        "a side lying on the axis",
    );
}

#[test]
fn an_area_a_hair_across_its_axis_lies_on_its_own_side() {
    let hair = corners(DVec2::new(-1e-5, 0.0), DVec2::new(5.0, 2.0));
    assert_eq!(lie(&hair, TAU), Lie::Side(-1.0));
    let mut holed = sampled(&hair);
    let hole = corners(DVec2::new(4.0, 1.0), DVec2::new(4.5, 1.5));
    holed.sampled_holes.push(Loop::straight(&hole));
    assert_eq!(Turn::of(V, TAU, 0.0, &holed).lie(&holed), Lie::Side(-1.0));
}

#[test]
fn an_area_truly_across_its_axis_lies_across() {
    assert_eq!(
        lie(&corners(DVec2::new(-1.0, 0.0), DVec2::new(5.0, 2.0)), TAU),
        Lie::Across,
    );
    let outline = corners(DVec2::new(1.0, 0.0), DVec2::new(5.0, 2.0));
    let hole = corners(DVec2::new(-2.0, 0.5), DVec2::new(-1.0, 1.5));
    let mut holed = sampled(&outline);
    holed.sampled_holes.push(Loop::straight(&hole));
    assert_eq!(
        Turn::of(V, TAU, 0.0, &holed).lie(&holed),
        Lie::Across,
        "a hole is read as the outline is",
    );
}

#[test]
fn a_turn_shorter_than_a_ten_thousandth_of_a_radian_makes_nothing() {
    let ring = corners(DVec2::new(1.0, 0.0), DVec2::new(5.0, 2.0));
    assert_eq!(lie(&ring, 0.5e-4), Lie::Nothing);
    assert_eq!(lie(&ring, -0.5e-4), Lie::Nothing);
    assert_eq!(lie(&ring, 2e-4), Lie::Side(-1.0));
    assert_eq!(
        lie(&corners(DVec2::new(0.0, 0.0), DVec2::new(1e-7, 2.0)), TAU),
        Lie::Nothing,
        "an area too thin to stand off its axis",
    );
    let pointless = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::ZERO,
    };
    let area = sampled(&ring);
    assert_eq!(
        Turn::of(pointless, TAU, 0.0, &area).lie(&area),
        Lie::Nothing
    );
}

fn laid(corners: &[DVec2], last_off_the_axis: Option<u32>) -> Straight {
    Straight {
        side: -1.0,
        contours: vec![
            corners
                .iter()
                .zip(0..)
                .map(|(corner, run)| Corner {
                    at: DVec2::new(corner.y, corner.x),
                    run,
                })
                .collect(),
        ],
        runs: corners.len() as u32,
        last_off_the_axis,
    }
}

#[test]
fn a_straight_profile_counts_the_numbers_the_flats_turn_it_into() {
    let axis_last = [
        DVec2::new(0.0, 0.0),
        DVec2::new(5.0, 0.0),
        DVec2::new(5.0, 2.0),
        DVec2::new(0.0, 2.0),
    ];
    let axis_first = [axis_last[3], axis_last[0], axis_last[1], axis_last[2]];
    let off = corners(DVec2::new(1.0, 0.0), DVec2::new(5.0, 2.0));
    for (points, straight) in [
        (&axis_last[..], laid(&axis_last, Some(2))),
        (&axis_first[..], laid(&axis_first, Some(3))),
        (&off[..], laid(&off, Some(3))),
    ] {
        for angle in [TAU, -TAU, FRAC_PI_2, -3.0 * FRAC_PI_2] {
            assert_eq!(
                straight.numbers(is_whole(angle)) as usize,
                flats_faces_end(points, angle),
                "{points:?} turned {angle} rad",
            );
        }
    }
}
