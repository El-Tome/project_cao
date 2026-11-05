//! Rays through spheres and tori (#576): a ball crossed twice, a ring four
//! times, a tangent ray and a ray through a pole doubtful. The box round each
//! such face, which a place far from it misses (#625).
//!
//! Closes #625.
//! - a face on a sphere or a torus that covers part of its surface is boxed
//!   by its own extent — the extremes of its edges and the points within it
//!   where a coordinate turns — in `boxed`, `bounds`, `reach` and the ray's
//!   `Boxes`; a face that covers the whole surface still spans the whole box —
//!   `a_face_covering_part_of_its_sphere_or_torus_is_missed_where_only_its_whole_surface_reaches`,
//!   `a_face_covering_its_whole_sphere_or_torus_is_met_anywhere_in_the_whole_box`;
//!   `boxed`, `bounds` and `reach` in `topology/tests/round.rs`
//! - every point of such a face lies in its box: sampled over a quarter
//!   torus, a hemisphere, a torus face holding the top of its tube, a sphere
//!   face holding a pole —
//!   `every_point_of_a_face_covering_part_of_its_sphere_or_torus_is_met_by_its_ray_box`;
//!   `boxed` in `topology/tests/round.rs`
//!
//! The faces are `topology/tests/round.rs`'s, and so is why the guard holds
//! faces turned about a slanted axis besides the four the issue names.

use glam::{BVec3, DVec3};

use super::{EPS, assert_windings};
use crate::brep::Declined;
use crate::brep::ray::Boxes;
use crate::brep::topology::tests::round::{
    partial_rounds, points_of, round_face, sampled_rounds, whole_rounds,
};
use crate::brep::turned::tests::round::{ball, ring, turned};

#[test]
fn every_point_of_a_face_covering_part_of_its_sphere_or_torus_is_met_by_its_ray_box() {
    for round in sampled_rounds() {
        let what = round.what;
        let boxes = Boxes::of(&round.body, 0.0);
        let sampled = points_of(&round.body, round.face);
        assert!(
            sampled.len() >= 100,
            "{what}: {} points of the face sampled",
            sampled.len()
        );
        for point in sampled.iter().chain(&round.named) {
            assert!(
                !boxes.misses(round.face, *point),
                "{what}: {point} of the face is missed by its box"
            );
        }
        assert!(
            boxes.misses(round.face, DVec3::splat(100.0)),
            "{what}: a place far past its whole surface is missed"
        );
    }
}

#[test]
fn a_face_covering_part_of_its_sphere_or_torus_is_missed_where_only_its_whole_surface_reaches() {
    let met: Vec<String> = partial_rounds()
        .iter()
        .filter(|partial| !Boxes::of(&partial.body, 0.0).misses(partial.face, partial.beyond))
        .map(|partial| {
            format!(
                "{}: {} is met, past its own points {} .. {} though within its whole surface {} .. {}",
                partial.what,
                partial.beyond,
                partial.own[0],
                partial.own[1],
                partial.whole[0],
                partial.whole[1]
            )
        })
        .collect();
    assert!(met.is_empty(), "{}", met.join("\n"));
}

#[test]
fn a_face_covering_its_whole_sphere_or_torus_is_met_anywhere_in_the_whole_box() {
    for (what, body, [low, high]) in whole_rounds() {
        let face = round_face(&body);
        let boxes = Boxes::of(&body, 0.0);
        for corner in 0..8 {
            let pick = |axis: usize| (corner >> axis) & 1 == 1;
            let at = DVec3::select(BVec3::new(pick(0), pick(1), pick(2)), high, low);
            assert!(!boxes.misses(face, at), "{what}: the corner {at} is missed");
        }
        let middle = (low + high) / 2.0;
        let past = (high - low).max_element() / 10.0;
        for axis in DVec3::AXES {
            for at in [
                middle + axis * (axis.dot(high - middle) + past),
                middle - axis * (axis.dot(middle - low) + past),
            ] {
                assert!(boxes.misses(face, at), "{what}: {at}, past its box, is met");
            }
        }
    }
}

#[test]
fn a_ball_is_wound_once_inside_and_nought_outside_and_a_line_crosses_it_twice() {
    let body = turned(&ball(), 360.0);
    assert_windings(
        &body,
        &[
            ([0.0, 0.0, 0.0], 1),
            ([3.0, 3.0, 0.0], 1),
            ([0.0, 4.9, 0.0], 1),
            ([5.1, 0.0, 0.0], 0),
            ([0.0, 0.0, -6.0], 0),
            ([3.0, 3.0, 3.0], 0),
        ],
    );
    let crossed = body
        .crossings_along(DVec3::new(-10.0, 1.0, 0.5), DVec3::X, EPS)
        .expect("a line through the ball crosses it");
    let half = (25.0f64 - 1.25).sqrt();
    assert_eq!(crossed.len(), 2, "{crossed:?}");
    assert_eq!([crossed[0].1, crossed[1].1], [1, -1]);
    for ((at, _), expected) in crossed.iter().zip([10.0 - half, 10.0 + half]) {
        assert!(
            (at - expected).abs() < 1e-12 * 40.0,
            "{at} against {expected}"
        );
    }
}

#[test]
fn a_ring_is_crossed_four_times_and_wound_once_in_its_tube_and_nought_in_its_hole() {
    let body = turned(&ring(), 360.0);
    assert_windings(
        &body,
        &[
            ([6.0, 3.0, 0.0], 1),
            ([0.0, 3.0, 6.0], 1),
            ([7.5, 3.0, 0.0], 1),
            ([0.0, 3.0, 0.0], 0),
            ([6.0, 3.0, 6.0], 0),
            ([6.0, 6.0, 0.0], 0),
        ],
    );
    let crossed = body
        .crossings_along(DVec3::new(-10.0, 3.0, 0.0), DVec3::X, EPS)
        .expect("a line through the ring crosses it");
    assert_eq!(crossed.len(), 4, "{crossed:?}");
    assert_eq!(
        crossed.iter().map(|(_, step)| *step).collect::<Vec<i32>>(),
        [1, -1, 1, -1]
    );
    for ((at, _), expected) in crossed.iter().zip([2.0, 6.0, 14.0, 18.0]) {
        assert!((at - expected).abs() < 1e-9, "{at} against {expected}");
    }
}

#[test]
fn a_ray_touching_a_ball_or_passing_through_its_pole_is_doubtful() {
    let body = turned(&ball(), 360.0);
    assert_eq!(
        body.crossings_along(DVec3::new(-10.0, 0.0, 5.0), DVec3::X, EPS),
        Err(Declined::Tie),
        "a tangent ray"
    );
    assert_eq!(
        body.crossings_along(DVec3::new(0.0, -10.0, 0.0), DVec3::Y, EPS),
        Err(Declined::Tie),
        "a ray along the axis through both poles"
    );
    assert_eq!(body.winding(DVec3::new(0.0, 4.999, 0.0), EPS), Ok(1));
}

#[test]
fn a_ring_turned_part_way_or_backwards_is_crossed_where_its_tube_is_and_nowhere_else() {
    let part = turned(&ring(), 150.0);
    let (sin, cos) = 75f64.to_radians().sin_cos();
    assert_windings(
        &part,
        &[
            ([6.0 * cos, 3.0, -6.0 * sin], 1),
            ([6.0 * cos, 3.0, 6.0 * sin], 0),
            ([-6.0, 3.0, 0.0], 0),
            ([9.0 * cos, 3.0, -9.0 * sin], 0),
        ],
    );
    let x = 6.0 * cos;
    let crossed = part
        .crossings_along(DVec3::new(x, 3.0, -20.0), DVec3::Z, EPS)
        .expect("a line along the axis's direction through the tube");
    assert_eq!(crossed.len(), 2, "{crossed:?}");
    assert_eq!([crossed[0].1, crossed[1].1], [1, -1]);
    for ((at, _), rho) in crossed.iter().zip([8.0f64, 4.0]) {
        let expected = 20.0 - (rho * rho - x * x).sqrt();
        assert!((at - expected).abs() < 1e-9, "{at} against {expected}");
    }

    let backwards = turned(&ring(), -360.0);
    assert_windings(&backwards, &[([6.0, 3.0, 0.0], 1), ([0.0, 3.0, 0.0], 0)]);
    let crossed = backwards
        .crossings_along(DVec3::new(-10.0, 3.0, 0.0), DVec3::X, EPS)
        .expect("a line through the ring crosses it");
    assert_eq!(
        crossed.iter().map(|(_, step)| *step).collect::<Vec<i32>>(),
        [1, -1, 1, -1]
    );
}
