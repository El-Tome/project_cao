//! What the refinement of a piece of sphere on its central chart decides on
//! its own (#800), on a sphere seen along Z, the outline handed to it as the
//! sweep would: where a triangle stands nearest the centre — at the foot of
//! the centre on its plane, or on one of its sides —, how far the chord of a
//! side on the boundary lets it stand off the sphere, where a triangle cut
//! inside is cut, that a fine tolerance gets every point it needs, and that a
//! cut which can go no further, or a triangle turning clockwise, gives the
//! face up.
//!
//! What `cargo mutants` changes in the chart and its refinement, run against
//! these tests, the rest of `cao_solid --lib` and the corner test file, is
//! caught (#828) — the refusal at `FRONT` by
//! `the_sphere_face_of_a_sector_whose_corners_stand_eighty_eight_degrees_from_its_middle_is_left_open_rather_than_laid_out_where_its_chart_runs_off`
//! in `tessellation/tests/rounds.rs` — but for these, which change nothing
//! drawn:
//!
//! - `reach`, `>` → `>=`: the other branch serves a side whose two ends are
//!   one point. The NaN it then gives is dropped by the `max` of the allowance
//!   and the `min` of `nearest`, which would never have picked the radius the
//!   branch gives over the other sides either.
//! - `most`, `/ 2.0` turned to `* 2.0`, a side of the triangle summed rather
//!   than subtracted, and the two sums of the last line turned to products:
//!   each makes the budget larger. `BUDGET * area` turned to a
//!   sum makes it smaller, but still over a point for each area of the radius
//!   by the tolerance, where a refinement takes about a third — measured on
//!   an ellipse like this file's, at `1e-3`, `5e-4` and `2e-4` of the
//!   radius. Either way unreachable: only a refinement that would never end
//!   meets the budget, and it is given up all the same — which takes a
//!   boundary chord the sampling let sag past the tolerance.
//! - `beyond`, `>` → `>=`: differs only where the triple product equals its
//!   margin exactly. Each `*` of the margin turned to `/`: scales the margin by
//!   the triangle's lengths, which moves only how near a tie a side is left
//!   uncut; the sign read anywhere else is the same. Either way the side left
//!   bounds two triangles that turn, refined within the tolerance all the
//!   same.
//! - `unplace` → `()`: a cut fails only where the middle of a side between two
//!   triangles turning counterclockwise, or of one triangle, rounds past a
//!   corner a hair from that side. The point then left behind lies in no
//!   triangle: the triangles drawn are the same.
//! - `Chart::of`, `<` → `<=` at `FRONT`: differs only for a sample whose
//!   cosine from the axis is exactly `FRONT`.
//! - `Ends::of`, the first `||` of the sphere's arm turned to `&&`: a latitude
//!   or a meridian of the sphere lies on it, which `carries` reads on its own.
//!   The three are named for whoever reads the match, kept on purpose by
//!   #826's review.
//!
//! Closes #828.
//! - `cargo mutants --in-diff` on the lines above, run against `-p cao_solid
//!   --lib` and the corner test file, leaves no missed mutant without a written
//!   reason (equivalent or unreachable) in the test module's doc —
//!   `a_chord_comes_nearest_the_centre_at_its_middle_whichever_way_it_is_asked`,
//!   `a_triangle_cut_inside_is_cut_at_its_middle_on_the_chart`,
//!   `a_triangle_turning_clockwise_on_the_chart_gives_the_face_up`,
//!   `an_ellipse_on_a_large_sphere_refined_finely_takes_every_point_its_tolerance_needs`,
//!   and the reasons above
//! - No production behaviour changes — no test: the branch touches test files
//!   only

use std::collections::{BTreeMap, BTreeSet};

use glam::{DVec2, DVec3};

use super::super::Chart;
use super::{Mesh, refined};

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

#[test]
fn a_triangle_cut_inside_is_cut_at_its_middle_on_the_chart() {
    let chart = chart();
    let points = [
        DVec2::new(0.4, 0.05),
        DVec2::new(-0.05, 0.31),
        DVec2::new(-0.05, -0.21),
    ];
    let places = places(&chart, &points);

    let (added, _) = refined(
        &chart,
        (&points, &places),
        &[[0, 1], [1, 2], [0, 2]],
        vec![[0, 1, 2]],
        0.01,
    )
    .expect("the triangle is cut");

    let middle = (points[0] + points[1] + points[2]) / 3.0;
    assert!(
        added
            .first()
            .is_some_and(|first| first.distance(middle) < 1e-12),
        "a triangle with its three sides on the boundary is cut first at its middle, \
         {middle:?}, rather than at the chart's origin: {added:?}"
    );
}

#[test]
fn a_triangle_turning_clockwise_on_the_chart_gives_the_face_up() {
    let chart = chart();
    let points = [
        DVec2::new(0.1, 0.0),
        DVec2::new(-0.05, 0.08),
        DVec2::new(-0.05, -0.08),
    ];
    let places = places(&chart, &points);

    let given_up = refined(
        &chart,
        (&points, &places),
        &[[0, 1], [1, 2], [0, 2]],
        vec![[0, 2, 1]],
        0.5,
    );

    assert_eq!(
        given_up, None,
        "[0, 2, 1] turns clockwise, seen from outside the sphere: it would be drawn inside out"
    );
}

#[test]
fn a_chord_comes_nearest_the_centre_at_its_middle_whichever_way_it_is_asked() {
    let chart = chart();
    let points = [DVec2::new(-0.3, 0.1), DVec2::new(0.4, -0.2)];
    let places = places(&chart, &points);
    let mesh = Mesh {
        chart: &chart,
        points: points.to_vec(),
        places: places.clone(),
        triangles: Vec::new(),
        sides: BTreeMap::new(),
        bounds: BTreeSet::new(),
    };
    let middle = ((places[0] + places[1]) / 2.0).length();

    for [one, other] in [[0, 1], [1, 0]] {
        let reach = mesh.reach(one, other);
        assert!(
            (reach - middle).abs() < 1e-12,
            "from {one} to {other}, the chord comes {reach} from the centre, {middle} at its middle"
        );
    }
}

#[test]
fn an_ellipse_on_a_large_sphere_refined_finely_takes_every_point_its_tolerance_needs() {
    let chart = Chart {
        radius: 10_000.0,
        ..chart()
    };
    let count = 40;
    let points: Vec<DVec2> = (0..count)
        .map(|rank| {
            let angle = 0.3 + std::f64::consts::TAU * rank as f64 / count as f64;
            DVec2::new(0.5 * angle.cos(), 0.35 * angle.sin())
        })
        .collect();
    let places = places(&chart, &points);
    let round_it: Vec<[usize; 2]> = (0..count).map(|rank| [rank, (rank + 1) % count]).collect();
    let fan: Vec<[usize; 3]> = (1..count - 1).map(|rank| [0, rank, rank + 1]).collect();
    let tolerance = 10.0;
    assert!(
        (0..count).all(|rank| {
            let middle = (places[rank] + places[(rank + 1) % count]) / 2.0;
            chart.radius - middle.length() < tolerance
        }),
        "every chord of the boundary sags within the tolerance, as the sampling holds it"
    );

    let refined = refined(&chart, (&points, &places), &round_it, fan, tolerance);

    assert!(
        refined
            .as_ref()
            .is_some_and(|(added, _)| added.len() > 64 + count),
        "the ellipse is refined, with more points inside than its outline holds and than a \
         piece too small to need any is allowed: {:?}",
        refined.map(|(added, _)| added.len())
    );
}
