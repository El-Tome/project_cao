//! What the refinement of a piece of sphere on its central chart decides on
//! its own (#800), on a unit sphere seen along Z, the outline handed to it as
//! the sweep would: where a triangle stands nearest the centre — at the foot
//! of the centre on its plane, or on one of its sides —, how far the chord of
//! a side on the boundary lets it stand off the sphere, and that a cut which
//! can go no further gives the face up.

use glam::{DVec2, DVec3};

use super::super::Chart;
use super::refined;

/// The chart of the unit sphere about the origin, ahead along Z.
fn chart() -> Chart {
    Chart {
        centre: DVec3::ZERO,
        radius: 1.0,
        ahead: DVec3::Z,
        x: DVec3::X,
        y: DVec3::Y,
    }
}

/// The points of the sphere the chart's `points` stand for, from its centre.
fn places(chart: &Chart, points: &[DVec2]) -> Vec<DVec3> {
    points.iter().map(|&at| chart.lifted(at)).collect()
}

/// How far the chord between two points of the unit sphere sags from it, at
/// its middle.
fn sag(one: DVec3, other: DVec3) -> f64 {
    1.0 - ((one + other) / 2.0).length()
}

/// The ranks of the quadrilateral's corners, round it.
const A: usize = 0;
const Y: usize = 1;
const B: usize = 2;
const X: usize = 3;

/// A quadrilateral of four boundary sides cut along its inner side `a`–`b`
/// into two triangles each obtuse, so that each stands nearest the centre on
/// its longest side: `[b, x, a]` obtuse at `x`, along the inner side, and
/// `[y, b, a]` obtuse at `b`, along the boundary side `a`–`y`. Its corner at
/// `b` turns back in, so that `a`–`b` is the only way to cut it. Each
/// triangle's longest side is the last of its three, from its third corner
/// round to its first.
const QUADRILATERAL: [DVec2; 4] = [
    DVec2::new(-0.3, 0.0),
    DVec2::new(0.9, -0.25),
    DVec2::new(0.3, 0.0),
    DVec2::new(0.0, 0.15),
];

/// The quadrilateral's boundary, and its two triangles.
const ROUND_IT: [[usize; 2]; 4] = [[A, Y], [Y, B], [B, X], [X, A]];
const CUT_ALONG_A_B: [[usize; 3]; 2] = [[B, X, A], [Y, B, A]];

#[test]
fn a_triangle_held_by_the_chord_of_its_boundary_side_is_left_as_it_is_however_far_that_chord_sags()
{
    let chart = chart();
    let places = places(&chart, &QUADRILATERAL);
    let tolerance = 0.05;
    assert!(
        sag(places[A], places[Y]) > 2.0 * tolerance && sag(places[A], places[B]) < tolerance,
        "the boundary chord a–y sags well past the tolerance, the inner side a–b within it"
    );

    let (added, triangles) = refined(
        &chart,
        (&QUADRILATERAL, &places),
        &ROUND_IT,
        CUT_ALONG_A_B.to_vec(),
        tolerance,
    )
    .expect("the quadrilateral is cut");

    assert!(
        added.is_empty(),
        "[y, b, a] stands nearest the centre on its boundary chord a–y, which the faces beyond \
         share, and [b, x, a] within the tolerance: no point is added, {added:?}"
    );
    assert_eq!(triangles, CUT_ALONG_A_B.to_vec());
}

#[test]
fn a_triangle_whose_middle_stands_further_off_than_its_three_boundary_sides_is_cut_inside() {
    let chart = chart();
    let points = [
        DVec2::new(0.3, 0.0),
        DVec2::new(-0.15, 0.26),
        DVec2::new(-0.15, -0.26),
    ];
    let places = places(&chart, &points);
    let tolerance = 0.01;
    let normal = (places[1] - places[0]).cross(places[2] - places[0]);
    let plane = 1.0 - normal.dot(places[0]) / normal.length();
    for side in 0..3 {
        let chord = sag(places[side], places[(side + 1) % 3]);
        assert!(
            tolerance < chord && chord < 0.8 * plane,
            "each side's chord sags past the tolerance, and the triangle's middle further"
        );
    }

    let (added, triangles) = refined(
        &chart,
        (&points, &places),
        &[[0, 1], [1, 2], [0, 2]],
        vec![[0, 1, 2]],
        tolerance,
    )
    .expect("the triangle is cut");

    assert!(
        !added.is_empty() && triangles.len() > 1,
        "the triangle stands nearest the centre at the foot of the centre on its plane, inside \
         it, further off than its sides: it is cut, {triangles:?}"
    );
}

#[test]
fn a_refinement_that_can_cut_nothing_more_gives_the_face_up_rather_than_going_round_for_ever() {
    let chart = chart();
    let points = [QUADRILATERAL[A], QUADRILATERAL[B], QUADRILATERAL[X]];
    let places = places(&chart, &points);
    let [a, b, x] = [0, 1, 2];

    let given_up = refined(
        &chart,
        (&points, &places),
        &[[b, x], [a, x]],
        vec![[b, x, a]],
        0.01,
    );

    assert_eq!(
        given_up, None,
        "[b, x, a] sags along a–b, which no other triangle shares and no segment bounds: it \
         cannot be cut there, and the face is given up"
    );
}
