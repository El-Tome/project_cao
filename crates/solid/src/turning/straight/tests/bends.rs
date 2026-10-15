//! How a profile's arcs are laid against its axis (#576), and when they
//! leave the profile to the flats.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::DVec2;

use super::{FLAT, RESOLUTION, V, laid};
use crate::profile::{Contour, Run};
use crate::turning::{Bend, ON_THE_AXIS, Straight, Turn};

/// A contour drawn in the profile's `(x, y)`, the axis along Y and the
/// profile on its right: `x` is the distance from the axis, `y` the height
/// along it.
fn drawn(corners: &[[f64; 2]], runs: Vec<Run>) -> Contour {
    Contour {
        corners: corners.iter().map(|&[x, y]| DVec2::new(x, y)).collect(),
        runs,
    }
}

fn round(center: [f64; 2], turn: f64) -> Run {
    Run::Round {
        center: DVec2::from(center),
        turn,
    }
}

/// A shaft of radius 4 along the axis up to `y = 10`, ending in a ball: the
/// arc from `(4, 10)` to the axis at `(0, 14)` about `(0, 10)`, drawn with
/// `hair` on the shaft's side and the centre.
fn ball_ended_shaft(hair: f64) -> Contour {
    drawn(
        &[[0.0, 0.0], [4.0, 0.0], [4.0 + hair, 10.0], [0.0, 14.0]],
        vec![
            Run::Straight,
            Run::Straight,
            round([hair / 2.0, 10.0 - hair], FRAC_PI_2),
            Run::Straight,
        ],
    )
}

/// The contour laid with the band read off `furthest`, the profile's
/// furthest point from the axis, which an arc's bulge may be.
fn laid_about(contour: &Contour, furthest: f64) -> Option<Straight> {
    let turn = Turn {
        axis: V,
        angle: TAU,
        resolution: RESOLUTION,
        on_the_axis: ON_THE_AXIS * furthest,
    };
    Straight::of(contour, &[], FLAT, &turn, 0.0)
}

fn bends(straight: &Straight) -> Vec<Vec<Option<Bend>>> {
    straight
        .contours
        .iter()
        .map(|contour| contour.iter().map(|corner| corner.bend).collect())
        .collect()
}

#[test]
fn a_ball_end_is_laid_tangent_to_its_shaft_to_the_bit() {
    let hair = 0.5 * RESOLUTION;
    let straight = laid(&ball_ended_shaft(hair), &[], RESOLUTION).expect("a ball-ended shaft");
    let corners = &straight.contours[0];
    let shaft = corners[2];
    let bend = shaft.bend.expect("the arc leaves the shaft's end");
    assert_eq!(
        bend.center,
        DVec2::new(10.0, 0.0),
        "a sphere, its centre on the axis"
    );
    assert!((bend.turn + FRAC_PI_2).abs() < 1e-15, "{}", bend.turn);
    let radius = (shaft.at - bend.center).length();
    assert_eq!(
        (shaft.at.y - bend.center.y).abs(),
        radius,
        "tangent to the shaft"
    );
    assert_eq!(
        corners[1].at.y, shaft.at.y,
        "the shaft's two corners on one level"
    );
    let tip = corners[3];
    assert_eq!(tip.at.y, 0.0);
    assert_eq!(
        tip.at.x,
        bend.center.x + radius,
        "the tip laid on the circle"
    );
    assert_eq!(bends(&straight)[0][..2], [None, None]);
    assert_eq!(straight.last_off_the_axis, Some(2));
    assert_eq!(straight.runs, 4);
}

#[test]
fn a_centre_a_hair_inside_the_band_is_a_sphere_and_a_hair_outside_a_torus() {
    let turn = super::turn_of(&[&ball_ended_shaft(0.0)], RESOLUTION);
    let band = turn.on_the_axis;
    for (off, sphere) in [(0.9 * band, true), (1.1 * band, false)] {
        let radius = 4.0 - off;
        let tip = 10.0 + (radius * radius - off * off).sqrt();
        let contour = drawn(
            &[[0.0, 0.0], [4.0, 0.0], [4.0, 10.0], [0.0, tip]],
            vec![
                Run::Straight,
                Run::Straight,
                round([off, 10.0], (tip - 10.0).atan2(-off)),
                Run::Straight,
            ],
        );
        let straight = laid(&contour, &[], RESOLUTION).expect("a ball-ended shaft");
        let bend = straight.contours[0][2].bend.expect("the arc");
        assert_eq!(bend.center.y == 0.0, sphere, "{off}");
        if !sphere {
            assert_eq!(bend.center.y, off);
        }
    }
}

#[test]
fn a_semicircle_with_both_ends_on_the_axis_is_laid_as_two_corners_and_a_bend() {
    let hair = 0.5 * RESOLUTION;
    let ball = drawn(
        &[[hair, -5.0], [0.0, 5.0]],
        vec![round([0.0, 0.0], PI), Run::Straight],
    );
    let straight = laid_about(&ball, 5.0).expect("a ball");
    let [south, north] = straight.contours[0][..] else {
        panic!("two corners: {straight:?}");
    };
    assert_eq!(
        (south.at, north.at),
        (DVec2::new(-5.0, 0.0), DVec2::new(5.0, 0.0))
    );
    let bend = south.bend.expect("the arc leaves the south pole");
    assert_eq!(bend.center, DVec2::ZERO);
    assert!(
        (bend.turn + PI).abs() < 1e-15,
        "clockwise in (h, r): {}",
        bend.turn
    );
    assert_eq!(north.bend, None);
    assert_eq!(straight.last_off_the_axis, Some(0));
}

#[test]
fn a_whole_circle_off_the_axis_is_laid_as_a_ring_of_two_halves() {
    let ring = Contour::circle(DVec2::new(10.0, 3.0), 2.0);
    let straight = laid(&ring, &[], RESOLUTION).expect("a ring");
    let corners = &straight.contours[0];
    assert_eq!(corners.len(), 2);
    for corner in corners {
        let bend = corner.bend.expect("each half is an arc");
        assert_eq!(bend.center, DVec2::new(3.0, 10.0));
        assert!((bend.turn + PI).abs() < 1e-15, "{}", bend.turn);
    }
    let one = Contour {
        corners: vec![DVec2::new(12.0, 3.0)],
        runs: vec![round([10.0, 3.0], TAU)],
    };
    let straight = laid(&one, &[], RESOLUTION).expect("a ring of one run");
    let [corner] = straight.contours[0][..] else {
        panic!("one corner: {straight:?}");
    };
    assert!((corner.bend.expect("a whole turn").turn + TAU).abs() < 1e-15);
}

#[test]
fn a_hole_bowed_keeps_its_bend_and_an_arc_of_ellipse_leaves_the_profile_to_the_flats() {
    let plate = drawn(
        &[[1.0, 0.0], [8.0, 0.0], [8.0, 10.0], [1.0, 10.0]],
        vec![Run::Straight; 4],
    );
    let hole = drawn(
        &[[3.0, 3.0], [5.0, 3.0], [5.0, 7.0], [3.0, 7.0]],
        vec![
            Run::Straight,
            Run::Straight,
            round([4.0, 7.0], -PI),
            Run::Straight,
        ],
    );
    let straight =
        laid(&plate, std::slice::from_ref(&hole), RESOLUTION).expect("a plate with a bowed hole");
    assert_eq!(bends(&straight)[0], [None; 4]);
    let bowed = bends(&straight)[1].clone();
    assert_eq!(bowed[..2], [None, None]);
    let bend = bowed[2].expect("the hole's top is an arc");
    assert_eq!(bend.center, DVec2::new(7.0, 4.0));
    assert!(
        (bend.turn - PI).abs() < 1e-15,
        "anticlockwise in (h, r): {}",
        bend.turn
    );

    let oval = Contour {
        runs: vec![
            Run::Straight,
            Run::Straight,
            Run::Oval {
                center: DVec2::new(4.0, 7.0),
                first: DVec2::X,
                second: 2.0,
                turn: -PI,
            },
            Run::Straight,
        ],
        ..hole
    };
    assert_eq!(laid(&plate, &[oval], RESOLUTION), None);
}

#[test]
fn an_arc_crossing_a_level_or_a_corner_within_the_tolerance_of_one_goes_to_the_flats() {
    let stepped = drawn(
        &[
            [0.0, 0.0],
            [6.0, 0.0],
            [6.0, 4.0],
            [2.0, 4.0],
            [2.0, 6.0],
            [0.0, 8.0],
        ],
        vec![
            Run::Straight,
            Run::Straight,
            Run::Straight,
            Run::Straight,
            round([0.0, 6.0], FRAC_PI_2),
            Run::Straight,
        ],
    );
    let kept = laid(&stepped, &[], RESOLUTION).expect("a stepped shaft with a ball end");
    assert!(kept.contours[0][4].bend.is_some());

    let under = drawn(
        &[
            [0.0, 0.0],
            [6.0, 0.0],
            [6.0, 7.0],
            [2.0, 7.0],
            [2.0, 6.0],
            [0.0, 8.0],
        ],
        vec![
            Run::Straight,
            Run::Straight,
            Run::Straight,
            Run::Straight,
            round([0.0, 6.0], FRAC_PI_2),
            Run::Straight,
        ],
    );
    assert!(
        laid(&under, &[], RESOLUTION).is_some(),
        "the arc passes under the step's level without crossing its run"
    );
    let crossing = drawn(
        &[
            [0.0, 0.0],
            [6.0, 0.0],
            [6.0, 9.0],
            [1.5, 9.0],
            [1.5, 6.0],
            [2.0, 6.0],
            [0.0, 8.0],
        ],
        vec![
            Run::Straight,
            Run::Straight,
            Run::Straight,
            Run::Straight,
            Run::Straight,
            round([0.0, 6.0], FRAC_PI_2),
            Run::Straight,
        ],
    );
    assert_eq!(
        laid(&crossing, &[], RESOLUTION),
        None,
        "the arc crosses a run"
    );

    let hole = drawn(
        &[[2.0, 1.0], [3.0, 1.0], [3.0, 2.0], [2.0, 2.0]],
        vec![Run::Straight; 4],
    );
    let shaft = ball_ended_shaft(0.0);
    assert!(laid(&shaft, std::slice::from_ref(&hole), RESOLUTION).is_some());
    let near = 12f64.sqrt() - 0.5 * RESOLUTION;
    let under = drawn(
        &[[3.0, 11.5], [3.0, 12.0], [near, 12.0], [near, 11.5]],
        vec![Run::Straight; 4],
    );
    let gap = (near * near + 4.0).sqrt() - 4.0;
    assert!(gap.abs() < RESOLUTION && gap < 0.0, "{gap}");
    assert_eq!(laid(&shaft, &[under], RESOLUTION), None);
}

#[test]
fn an_arc_reaching_the_axis_between_its_ends_or_tangent_to_it_goes_to_the_flats() {
    let bulb = drawn(
        &[[0.0, 0.0], [4.0, 0.0], [4.0, 10.0], [0.0, 14.0]],
        vec![
            Run::Straight,
            Run::Straight,
            round([0.0, 10.0], 3.0 * FRAC_PI_2),
            Run::Straight,
        ],
    );
    assert_eq!(
        laid(&bulb, &[], RESOLUTION),
        None,
        "the arc crosses the axis"
    );

    let horn = drawn(
        &[[0.0, 0.0], [4.0, 0.0], [4.0, 10.0], [0.0, 12.0]],
        vec![
            Run::Straight,
            Run::Straight,
            round([2.0, 10.0], PI),
            Run::Straight,
        ],
    );
    assert_eq!(
        laid(&horn, &[], RESOLUTION),
        None,
        "the arc is tangent to the axis at its end"
    );

    let off = drawn(
        &[
            [0.0, 0.0],
            [4.0, 0.0],
            [4.0, 10.0],
            [0.0, 14.0 + 3.0 * RESOLUTION],
        ],
        vec![
            Run::Straight,
            Run::Straight,
            round([0.0, 10.0], FRAC_PI_2),
            Run::Straight,
        ],
    );
    assert_eq!(
        laid(&off, &[], RESOLUTION),
        None,
        "the end is off the circle"
    );
}

#[test]
fn a_straight_profile_is_laid_as_it_was_before_arcs() {
    let shaft = drawn(
        &[[0.0, 0.0], [4.0, 0.0], [4.0, 10.0], [0.0, 10.0]],
        vec![Run::Straight; 4],
    );
    let straight = laid(&shaft, &[], RESOLUTION).expect("a shaft");
    assert!(
        straight.contours[0]
            .iter()
            .all(|corner| corner.bend.is_none())
    );
}
