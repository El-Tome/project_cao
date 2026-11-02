//! How a profile's arcs are laid against its axis (#576), and when they
//! leave the profile unlaid, which the body declines (#632).

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::DVec2;

use super::{FLAT, RESOLUTION, V, laid};
use crate::profile::{Contour, Profile, Run};
use crate::sweep::Loop;
use crate::turning::{Axis, Bend, ON_THE_AXIS, Straight, Turn};

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

/// The shaft drawn the other way round, so that the arc leaves the pole:
/// from `(0, 14 + hair)` to the shaft's end at `(4, 10)` about `(0, 10)`,
/// the ball a `hair` larger than the shaft, or smaller when `hair` is
/// negative.
fn shaft_from_the_pole(hair: f64) -> Contour {
    drawn(
        &[[0.0, 14.0 + hair], [4.0, 10.0], [4.0, 0.0], [0.0, 0.0]],
        vec![
            round([0.0, 10.0], -FRAC_PI_2),
            Run::Straight,
            Run::Straight,
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
fn a_ball_end_a_hair_larger_or_smaller_than_its_shaft_is_laid_tangent_and_its_pole_slides() {
    for hair in [0.5 * RESOLUTION, -0.5 * RESOLUTION] {
        let straight =
            laid(&shaft_from_the_pole(hair), &[], RESOLUTION).expect("a ball-ended shaft");
        let corners = &straight.contours[0];
        assert_eq!(corners.len(), 4, "{hair}: {corners:?}");
        let (pole, shaft) = (corners[0], corners[1]);
        let bend = pole.bend.expect("the arc leaves the pole");
        assert_eq!(bend.center, DVec2::new(10.0, 0.0));
        assert_eq!(shaft.at, DVec2::new(10.0, 4.0), "the shaft's level stays");
        assert_eq!(
            pole.at,
            DVec2::new(14.0, 0.0),
            "{hair}: the pole slid onto the circle"
        );
        assert!((bend.turn - FRAC_PI_2).abs() < 1e-15, "{}", bend.turn);
    }
}

#[test]
fn a_step_a_hair_high_beside_a_ball_end_is_no_corner_of_its_own() {
    let hair = 0.5 * RESOLUTION;
    let stepped = drawn(
        &[
            [0.0, 0.0],
            [4.0, 0.0],
            [4.0, 10.0],
            [4.0 + hair, 10.0],
            [0.0, 14.0 + hair],
        ],
        vec![
            Run::Straight,
            Run::Straight,
            Run::Straight,
            round([0.0, 10.0], FRAC_PI_2),
            Run::Straight,
        ],
    );
    let straight = laid(&stepped, &[], RESOLUTION).expect("a stepped ball end");
    let corners = &straight.contours[0];
    assert_eq!(corners.len(), 4, "{corners:?}");
    let shaft = corners[2];
    let level = shaft.at.y;
    assert!(
        shaft.at.x == 10.0 && (level - 4.0).abs() <= hair,
        "{shaft:?}"
    );
    assert_eq!(
        corners[1].at.y, level,
        "the shaft's two corners on one level"
    );
    assert_eq!(shaft.run, 3, "the arc's run leaves the shaft's end");
    let bend = shaft.bend.expect("the arc");
    assert_eq!(bend.center, DVec2::new(10.0, 0.0));
    let tip = corners[3].at;
    assert!(
        tip.y == 0.0 && (tip.x - (10.0 + level)).abs() < 1e-12,
        "the tip on the circle: {tip}"
    );
}

#[test]
fn a_ball_end_whose_centre_and_end_stand_a_hair_off_the_axis_is_laid_with_its_pole_on_it() {
    let off = 0.5 * super::turn_of(&[&ball_ended_shaft(0.0)], RESOLUTION).on_the_axis;
    let contour = drawn(
        &[
            [0.0, 0.0],
            [4.0, 0.0],
            [4.0, 10.0],
            [off, 14.0 - off],
            [0.0, 14.0 - off],
        ],
        vec![
            Run::Straight,
            Run::Straight,
            round([off, 10.0], FRAC_PI_2),
            Run::Straight,
            Run::Straight,
        ],
    );
    let straight = laid(&contour, &[], RESOLUTION).expect("a ball-ended shaft");
    assert_eq!(
        super::at(&straight),
        [[[0.0, 0.0], [0.0, 4.0], [10.0, 4.0], [14.0, 0.0]]]
    );
    let bend = straight.contours[0][2].bend.expect("the arc");
    assert_eq!(bend.center, DVec2::new(10.0, 0.0));
    assert_eq!(straight.runs, 5);
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

/// #633's section, a ring from 30 to 90 off its axis with a groove in its
/// outer side whose bottom is a half round of 30 tangent to its two walls,
/// drawn in its own `(a, s)` — along the axis and across it — and placed
/// as the harness places it about the sketch's second axis run backwards and
/// leaning `degrees`: the corners and the centre come out as the drawing's
/// rounding leaves them.
fn grooved_ring_about_an_axis_leaning(degrees: f64) -> (Contour, Turn) {
    grooved_ring_drawn_leaning(degrees, degrees)
}

/// The same ring drawn leaning `drawn` degrees, and turned about the axis
/// leaning `axis` degrees: drawn square to the sketch's axis and turned
/// about a line a hair off it, as #632's pulley is in the application, the
/// groove's two ends stand a hair apart from the axis, and its centre a
/// hair off the middle of the walls once they are laid square.
fn grooved_ring_drawn_leaning(drawn: f64, axis: f64) -> (Contour, Turn) {
    let leaning = |degrees: f64| DVec2::from_angle(degrees.to_radians()).rotate(-DVec2::Y);
    let direction = leaning(drawn);
    let at = |a: f64, s: f64| direction * a + direction.perp() * s;
    let corners = [
        at(-90.0, 30.0),
        at(15.0, 30.0),
        at(15.0, 90.0),
        at(-15.0, 90.0),
        at(-15.0, 75.0),
        at(-75.0, 75.0),
        at(-75.0, 90.0),
        at(-90.0, 90.0),
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[4] = Run::Round {
        center: at(-45.0, 75.0),
        turn: -PI,
    };
    let contour = Contour {
        corners: corners.to_vec(),
        runs,
    };
    let area = Profile {
        exact: None,
        sampled: Loop::straight(&contour.corners),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let axis = Axis {
        origin: DVec2::ZERO,
        direction: leaning(axis),
    };
    let turn = Turn::of(axis, -TAU, 0.0, &area);
    (contour, turn)
}

#[test]
fn a_groove_ending_on_a_wall_tangent_to_it_is_laid_at_any_lean_its_end_at_the_tangent() {
    for degrees in [0.0, 1e-5, 1e-3, 0.01, 0.1, 1.0, 5.0, 15.0, 30.0] {
        let (groove, turn) = grooved_ring_about_an_axis_leaning(degrees);
        let straight = Straight::of(&groove, &[], FLAT, &turn, 0.0)
            .unwrap_or_else(|| panic!("leaning {degrees}°: laid"));
        let corners = &straight.contours[0];
        let arc = corners
            .iter()
            .position(|corner| corner.bend.is_some())
            .expect("the groove's bottom");
        let bend = corners[arc].bend.expect("an arc");
        let (start, end) = (corners[arc].at, corners[(arc + 1) % corners.len()].at);
        assert_eq!(
            [start.y, end.y],
            [bend.center.y; 2],
            "leaning {degrees}°: both ends at the tangent to their walls",
        );
        let radius = (start.x - bend.center.x).abs();
        assert!(
            ((end.x - bend.center.x).abs() - radius).abs() < 1e-12,
            "leaning {degrees}°: the walls {start} and {end} both tangent to {bend:?}",
        );
    }
}

#[test]
fn a_half_round_groove_drawn_square_to_its_sketch_about_an_axis_a_hair_off_it_ends_on_its_circle() {
    for degrees in [1e-7, 1e-6, 3e-6] {
        let (groove, turn) = grooved_ring_drawn_leaning(0.0, degrees);
        let straight = Straight::of(&groove, &[], FLAT, &turn, 0.0)
            .unwrap_or_else(|| panic!("about an axis leaning {degrees}°: laid"));
        let corners = &straight.contours[0];
        let arc = corners
            .iter()
            .position(|corner| corner.bend.is_some())
            .expect("the groove's bottom");
        let bend = corners[arc].bend.expect("an arc");
        let (start, end) = (corners[arc].at, corners[(arc + 1) % corners.len()].at);
        assert_eq!(
            [start.y, end.y],
            [bend.center.y; 2],
            "about an axis leaning {degrees}°: both ends at the tangent to their walls",
        );
        assert!(
            (start.distance(bend.center) - end.distance(bend.center)).abs() < 1e-12,
            "about an axis leaning {degrees}°: {start} and {end} both on the circle about {}",
            bend.center,
        );
    }
}

/// A tube from 30 to 40 off its axis and 50 long, its wall rounded whole at
/// its end, a half circle of 5 tangent to both its shafts, drawn square to
/// its sketch and turned about an axis leaning `degrees`.
fn tube_rounded_at_its_end_about_an_axis_leaning(degrees: f64) -> (Contour, Turn) {
    let at = |a: f64, s: f64| -DVec2::Y * a + DVec2::X * s;
    let corners = [at(0.0, 30.0), at(50.0, 30.0), at(50.0, 40.0), at(0.0, 40.0)];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[1] = Run::Round {
        center: at(50.0, 35.0),
        turn: -PI,
    };
    let contour = Contour {
        corners: corners.to_vec(),
        runs,
    };
    let area = Profile {
        exact: None,
        sampled: Loop::straight(&contour.corners),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let axis = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::from_angle(degrees.to_radians()).rotate(-DVec2::Y),
    };
    let turn = Turn::of(axis, -TAU, 0.0, &area);
    (contour, turn)
}

#[test]
fn a_tube_rounded_whole_at_its_end_about_an_axis_a_hair_off_its_sketch_s_ends_on_its_circle() {
    for degrees in [0.0, 1e-7, 1e-6] {
        let (tube, turn) = tube_rounded_at_its_end_about_an_axis_leaning(degrees);
        let straight = Straight::of(&tube, &[], FLAT, &turn, 0.0)
            .unwrap_or_else(|| panic!("about an axis leaning {degrees}°: laid"));
        let corners = &straight.contours[0];
        let arc = corners
            .iter()
            .position(|corner| corner.bend.is_some())
            .expect("the rounded end");
        let bend = corners[arc].bend.expect("an arc");
        let (start, end) = (corners[arc].at, corners[(arc + 1) % corners.len()].at);
        assert_eq!(
            [start.x, end.x],
            [bend.center.x; 2],
            "about an axis leaning {degrees}°: both ends at the tangent to their shafts",
        );
        assert!(
            (start.distance(bend.center) - end.distance(bend.center)).abs() < 1e-12,
            "about an axis leaning {degrees}°: {start} and {end} both on the circle about {}",
            bend.center,
        );
    }
}

#[test]
fn a_ball_drawn_a_hair_off_the_middle_of_its_poles_keeps_its_centre_as_laid() {
    let hair = 0.5 * RESOLUTION;
    let ball = drawn(
        &[[0.0, -5.0], [0.0, 5.0]],
        vec![round([0.0, hair], PI), Run::Straight],
    );
    let straight = laid_about(&ball, 5.0).expect("a ball");
    let bend = straight.contours[0][0]
        .bend
        .expect("the arc leaves the south pole");
    assert_eq!(
        bend.center,
        DVec2::new(hair, 0.0),
        "its poles land along the axis, where nothing is tangent"
    );
}
