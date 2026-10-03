//! What counts as a face passing through another or lying on it, what is only
//! two faces meeting, and the kernel's own solids held to it.
//!
//! Closes #448.
//! - no face crosses another, caught out by a solid broken on purpose —
//!   `two_overlapping_cubes_laid_together_cross`,
//!   `two_cubes_laid_together_face_against_face_lie_on_each_other`,
//!   `the_union_and_the_intersection_of_two_boxes_laid_together_cross`
//!
//! Closes #502.
//! - the gate passes on `main` on a machine slower than the CI, each test run
//!   alone as well as within the whole suite —
//!   `a_round_bar_of_five_thousand_sides_pushed_slantwise_is_judged_ahead_of_a_quadratic_pass`,
//!   `a_prism_whose_caps_are_fans_of_five_thousand_triangles_is_judged_ahead_of_a_quadratic_pass`
//! - every timed test still fails on a quadratic pass — no test: measured,
//!   each check against the pass over every pair it avoids, written down in
//!   #502
//! - the gate is no slower — no test: the bounds only widen
//! - no file of #498's branch is touched beyond these assertions and names —
//!   no test: read off `git diff --stat origin/main`

use std::f64::consts::TAU;
use std::time::{Duration, Instant};

use glam::{DVec2, DVec3};

use super::*;
use crate::mesh::Mesh;
use crate::mesh::tests::{self as kernel, box_of};
use crate::soundness::tests::{cube, unit_cube};
use crate::sweep::{Loop, prism, revolution};

fn at(x: f64, y: f64, z: f64) -> DVec3 {
    DVec3::new(x, y, z)
}

/// A face lying in z = 0, big enough for others to cross it well inside.
fn flat() -> Triangle {
    [DVec3::ZERO, at(4.0, 0.0, 0.0), at(0.0, 4.0, 0.0)]
}

/// Several solids laid in one list of triangles, nothing cut or removed.
fn soup(solids: &[Vec<Triangle>]) -> Vec<Triangle> {
    solids.concat()
}

/// A flat face cut into triangles by a fan from its first corner, as the
/// kernel's own meshes are.
fn fan(corners: &[DVec3]) -> Vec<Triangle> {
    (1..corners.len() - 1)
        .map(|index| [corners[0], corners[index], corners[index + 1]])
        .collect()
}

#[test]
fn nothing_at_all_crosses_nothing() {
    assert_eq!(uncrossed(&[]), Ok(()));
}

#[test]
fn the_unit_cube_is_uncrossed() {
    assert_eq!(uncrossed(&unit_cube()), Ok(()));
}

#[test]
fn two_triangles_passing_through_each_other_cross() {
    let upright = [at(1.0, 1.0, -1.0), at(2.0, 1.0, 1.0), at(1.0, 2.0, 1.0)];
    assert_eq!(
        uncrossed(&[flat(), upright]),
        Err(Flaw::Crossing {
            first: Box::new(flat()),
            second: Box::new(upright)
        })
    );
}

#[test]
fn a_triangle_poking_one_corner_through_a_face_crosses_it() {
    let poking = [at(1.0, 1.0, -0.5), at(2.0, 1.0, 1.0), at(1.0, 2.0, 1.0)];
    assert_eq!(
        uncrossed(&[flat(), poking]),
        Err(Flaw::Crossing {
            first: Box::new(flat()),
            second: Box::new(poking)
        })
    );
}

#[test]
fn a_triangle_resting_one_corner_on_a_face_is_uncrossed() {
    let resting = [at(1.0, 1.0, 0.0), at(2.0, 1.0, 1.0), at(1.0, 2.0, 1.0)];
    assert_eq!(uncrossed(&[flat(), resting]), Ok(()));
}

#[test]
fn a_corner_sunk_into_a_face_by_less_than_the_tolerance_rests_on_it() {
    let sunk = -0.5 * NEAR * reach(&[flat()]);
    let resting = [at(1.0, 1.0, sunk), at(2.0, 1.0, 1.0), at(1.0, 2.0, 1.0)];
    assert_eq!(uncrossed(&[flat(), resting]), Ok(()));
}

#[test]
fn a_nearly_flat_triangle_with_a_corner_sunk_by_less_than_the_tolerance_rests_on_the_face() {
    let sunk = -0.5 * NEAR * reach(&[flat()]);
    let lying = [at(1.0, 1.0, sunk), at(3.0, 1.0, 0.01), at(1.0, 2.5, 0.01)];
    assert_eq!(uncrossed(&[flat(), lying]), Ok(()));
}

#[test]
fn a_triangle_crossing_the_plane_of_a_face_beside_it_is_uncrossed() {
    let beside = [at(5.0, 5.0, -1.0), at(6.0, 5.0, 1.0), at(5.0, 6.0, 1.0)];
    assert_eq!(uncrossed(&[flat(), beside]), Ok(()));
}

#[test]
fn the_first_crossing_by_index_is_the_one_reported_wherever_it_lies() {
    let poking = [at(1.0, 1.0, -0.5), at(2.0, 1.0, 1.0), at(1.0, 2.0, 1.0)];
    let far = |triangle: Triangle| triangle.map(|corner| corner + at(100.0, 0.0, 0.0));
    let triangles = [far(flat()), far(poking), flat(), poking];
    assert_eq!(
        uncrossed(&triangles),
        Err(Flaw::Crossing {
            first: Box::new(far(flat())),
            second: Box::new(far(poking))
        })
    );
}

#[test]
fn two_overlapping_cubes_laid_together_cross() {
    let both = soup(&[unit_cube(), cube(DVec3::splat(0.5), DVec3::splat(1.5))]);
    assert!(matches!(uncrossed(&both), Err(Flaw::Crossing { .. })));
}

#[test]
fn two_cubes_apart_are_uncrossed() {
    let both = soup(&[unit_cube(), cube(DVec3::splat(2.0), DVec3::splat(3.0))]);
    assert_eq!(uncrossed(&both), Ok(()));
}

#[test]
fn two_cubes_meeting_along_one_edge_are_uncrossed() {
    let both = soup(&[unit_cube(), cube(at(1.0, 1.0, 0.0), at(2.0, 2.0, 1.0))]);
    assert_eq!(uncrossed(&both), Ok(()));
}

#[test]
fn two_cubes_laid_together_face_against_face_lie_on_each_other() {
    let both = soup(&[unit_cube(), cube(at(1.0, 0.0, 0.0), at(2.0, 1.0, 1.0))]);
    assert!(matches!(uncrossed(&both), Err(Flaw::Crossing { .. })));
}

#[test]
fn a_small_cube_laid_against_part_of_a_face_lies_on_it() {
    let both = soup(&[unit_cube(), cube(at(1.0, 0.25, 0.25), at(2.0, 0.75, 0.75))]);
    assert!(matches!(uncrossed(&both), Err(Flaw::Crossing { .. })));
}

#[test]
fn two_triangles_in_one_plane_facing_the_same_way_and_overlapping_lie_on_each_other() {
    let shifted = flat().map(|corner| corner + at(1.0, 1.0, 0.0));
    assert_eq!(
        uncrossed(&[flat(), shifted]),
        Err(Flaw::Crossing {
            first: Box::new(flat()),
            second: Box::new(shifted)
        })
    );
}

#[test]
fn a_triangle_laid_twice_lies_on_itself() {
    assert!(matches!(
        uncrossed(&[flat(), flat()]),
        Err(Flaw::Crossing { .. })
    ));
}

#[test]
fn two_triangles_in_one_plane_side_by_side_are_uncrossed() {
    let beside = [at(4.0, 0.0, 0.0), at(4.0, 4.0, 0.0), at(0.0, 4.0, 0.0)];
    assert_eq!(uncrossed(&[flat(), beside]), Ok(()));
}

#[test]
fn two_triangles_in_one_plane_overlapping_by_less_than_the_tolerance_are_uncrossed() {
    let over = 0.5 * NEAR * reach(&[flat()]);
    let beside = [at(4.0, 0.0, 0.0), at(4.0, 4.0, 0.0), at(-over, 4.0, 0.0)];
    assert_eq!(uncrossed(&[flat(), beside]), Ok(()));
}

#[test]
fn two_squares_in_one_plane_overlapping_by_a_little_more_than_the_tolerance_lie_on_each_other() {
    let over = 1.5 * NEAR * 4.0;
    let square = |from: f64, to: f64| {
        fan(&[
            at(from, 0.0, 0.0),
            at(to, 0.0, 0.0),
            at(to, 2.0, 0.0),
            at(from, 2.0, 0.0),
        ])
    };
    let both = soup(&[square(0.0, 2.0), square(2.0 - over, 4.0)]);
    assert!(matches!(uncrossed(&both), Err(Flaw::Crossing { .. })));
}

#[test]
fn two_thin_faces_side_by_side_further_apart_than_the_tolerance_are_uncrossed() {
    let one = [
        at(-8.681305311543667, -9.032204628152172, -0.02765179462952183),
        at(-5.968825217705918, -2.5146485842103674, -7.110300660316117),
        at(-7.325065298168186, -5.773426617717082, -3.568976250934534),
    ];
    let other = [
        at(-6.127084732371974, -2.8949150243449937, -6.697063889932684),
        at(-8.839564826209724, -9.412471068286798, 0.38558497575391126),
        at(-7.483324812834241, -6.153693057851708, -3.155739480551101),
    ];
    assert_eq!(uncrossed(&[one, other]), Ok(()));
    assert_eq!(uncrossed(&[other, one]), Ok(()));
}

/// The unit cube with its top cut in two halves along x = ½, and its front and
/// back each kept as one face with a corner partway along its top edge — fanned
/// from that edge's end, so that the first triangle of each is a sliver.
fn unit_cube_with_a_cut_top() -> Vec<Triangle> {
    let lies_in = |triangle: &Triangle, plane: fn(&DVec3) -> bool| triangle.iter().all(plane);
    let mut solid = unit_cube();
    solid.retain(|triangle| {
        !lies_in(triangle, |corner| corner.z == 1.0)
            && !lies_in(triangle, |corner| corner.y == 0.0)
            && !lies_in(triangle, |corner| corner.y == 1.0)
    });
    solid.extend(fan(&[
        at(0.0, 0.0, 1.0),
        at(0.5, 0.0, 1.0),
        at(0.5, 1.0, 1.0),
        at(0.0, 1.0, 1.0),
    ]));
    solid.extend(fan(&[
        at(0.5, 0.0, 1.0),
        at(1.0, 0.0, 1.0),
        at(1.0, 1.0, 1.0),
        at(0.5, 1.0, 1.0),
    ]));
    solid.extend(fan(&[
        at(1.0, 0.0, 1.0),
        at(0.5, 0.0, 1.0),
        at(0.0, 0.0, 1.0),
        at(0.0, 0.0, 0.0),
        at(1.0, 0.0, 0.0),
    ]));
    solid.extend(fan(&[
        at(0.0, 1.0, 1.0),
        at(0.5, 1.0, 1.0),
        at(1.0, 1.0, 1.0),
        at(1.0, 1.0, 0.0),
        at(0.0, 1.0, 0.0),
    ]));
    solid
}

#[test]
fn a_cube_whose_top_is_cut_in_two_leaving_corners_partway_along_its_sides_is_uncrossed() {
    let solid = unit_cube_with_a_cut_top();
    assert_eq!(solid.len(), 16, "two slivers among the fans");
    assert_eq!(uncrossed(&solid), Ok(()));
}

#[test]
fn a_triangle_with_no_area_crosses_nothing() {
    let needle = [at(1.0, 1.0, -1.0), at(1.0, 1.0, 0.0), at(1.0, 1.0, 1.0)];
    assert_eq!(uncrossed(&[flat(), needle]), Ok(()));
    assert_eq!(uncrossed(&[needle, flat()]), Ok(()));
}

#[test]
fn a_sliver_leaning_across_a_face_within_the_tolerance_crosses_nothing() {
    let (rise, off_line) = (10.0 * NEAR, 3.5 * NEAR);
    let height = |x: f64| rise - x * 2.0 * rise / 3.0;
    let sliver = [
        at(0.0, 1.0, height(0.0)),
        at(3.0, 1.0, height(3.0)),
        at(0.1, 1.0, height(0.1) - off_line),
    ];
    assert_eq!(uncrossed(&[flat(), sliver]), Ok(()));
}

/// Two triangles sharing an edge that lies inside `flat`, one rising from it
/// and the other sinking or rising too.
fn folded_on_flat(second_rises: bool) -> [Triangle; 2] {
    let (start, end) = (at(0.5, 1.0, 0.0), at(1.5, 1.0, 0.0));
    let height = if second_rises { 1.0 } else { -1.0 };
    [
        [start, end, at(1.0, 1.5, 1.0)],
        [end, start, at(1.0, 0.5, height)],
    ]
}

#[test]
fn a_surface_passing_through_a_face_along_edges_of_its_own_crosses_it() {
    let [rising, sinking] = folded_on_flat(false);
    assert_eq!(
        uncrossed(&[flat(), rising, sinking]),
        Err(Flaw::Crossing {
            first: Box::new(flat()),
            second: Box::new(rising)
        })
    );
}

#[test]
fn a_ridge_resting_on_a_face_is_uncrossed() {
    let [one, other] = folded_on_flat(true);
    assert_eq!(uncrossed(&[flat(), one, other]), Ok(()));
}

#[test]
fn a_face_standing_above_and_one_hanging_below_along_different_stretches_are_uncrossed() {
    let on_flat = |x: f64| at(x, 1.0, 0.0);
    let standing = [on_flat(0.5), on_flat(1.5), at(1.0, 1.5, 1.0)];
    let hanging = [on_flat(2.6), on_flat(1.8), at(2.2, 0.5, -1.0)];
    assert_eq!(uncrossed(&[flat(), standing, hanging]), Ok(()));
}

/// A strip of floor facing up, from `x = from` to `x = to`.
fn floor(from: f64, to: f64) -> Vec<Triangle> {
    fan(&[
        at(from, 0.0, 0.0),
        at(to, 0.0, 0.0),
        at(to, 1.0, 0.0),
        at(from, 1.0, 0.0),
    ])
}

/// A strip of the wall `x = 1` facing towards `+x`, from `z = from` to `z = to`.
fn wall(from: f64, to: f64) -> Vec<Triangle> {
    fan(&[
        at(1.0, 0.0, from),
        at(1.0, 1.0, from),
        at(1.0, 1.0, to),
        at(1.0, 0.0, to),
    ])
}

fn turned_over(triangles: Vec<Triangle>) -> Vec<Triangle> {
    triangles.into_iter().map(|[a, b, c]| [a, c, b]).collect()
}

#[test]
fn two_skins_cut_along_the_line_where_they_cross_and_all_kept_cross() {
    let crossed = soup(&[
        floor(0.0, 1.0),
        floor(1.0, 2.0),
        wall(0.0, 1.0),
        wall(-1.0, 0.0),
    ]);
    assert!(matches!(uncrossed(&crossed), Err(Flaw::Crossing { .. })));
}

#[test]
fn two_solids_touching_along_a_line_where_each_of_their_faces_ends_are_uncrossed() {
    let touching = soup(&[
        floor(1.0, 2.0),
        turned_over(wall(-1.0, 0.0)),
        turned_over(floor(0.0, 1.0)),
        wall(0.0, 1.0),
    ]);
    assert_eq!(uncrossed(&touching), Ok(()));
}

#[test]
fn the_union_and_the_intersection_of_two_boxes_laid_together_cross() {
    let (one, other) = (
        box_of(4.0, 4.0, DVec3::ZERO),
        box_of(4.0, 4.0, at(2.0, 1.0, 1.5)),
    );
    let both = soup(&[
        one.union(&other).triangles(),
        one.difference(&one.difference(&other)).triangles(),
    ]);
    assert!(matches!(uncrossed(&both), Err(Flaw::Crossing { .. })));
}

#[test]
fn what_each_of_two_boxes_keeps_outside_the_other_laid_together_is_uncrossed() {
    let (one, other) = (
        box_of(4.0, 4.0, DVec3::ZERO),
        box_of(4.0, 4.0, at(2.0, 1.0, 1.5)),
    );
    let both = soup(&[
        one.difference(&other).triangles(),
        other.difference(&one).triangles(),
    ]);
    assert_eq!(uncrossed(&both), Ok(()));
}

/// A round bar of `sides` flat walls, standing square on `from` and pushed as
/// far as `push`.
fn bar(sides: usize, radius: f64, from: DVec3, push: DVec3) -> Mesh {
    let outline: Vec<DVec2> = (0..sides)
        .map(|index| DVec2::from_angle(TAU * index as f64 / sides as f64) * radius)
        .collect();
    let across = push.any_orthonormal_vector();
    let up = push.normalize().cross(across);
    prism(
        Loop::straight(&outline),
        &[],
        &kernel::fan(&outline),
        |point| from + across * point.x + up * point.y,
        push,
    )
}

#[test]
fn two_round_bars_crossing_each_less_the_other_laid_together_are_uncrossed() {
    let upright = bar(48, 2.0, DVec3::ZERO, DVec3::Z * 4.0);
    let lying = bar(48, 1.0, at(-5.0, 0.0, 2.0), DVec3::X * 10.0);
    let both = soup(&[
        upright.difference(&lying).triangles(),
        lying.difference(&upright).triangles(),
    ]);
    assert_eq!(uncrossed(&both), Ok(()));
}

#[test]
fn the_union_and_the_intersection_of_two_round_bars_laid_together_cross() {
    let upright = bar(48, 2.0, DVec3::ZERO, DVec3::Z * 4.0);
    let lying = bar(48, 1.0, at(-5.0, 0.0, 2.0), DVec3::X * 10.0);
    let both = soup(&[
        upright.union(&lying).triangles(),
        upright.difference(&upright.difference(&lying)).triangles(),
    ]);
    assert!(matches!(uncrossed(&both), Err(Flaw::Crossing { .. })));
}

#[test]
fn the_union_of_two_overlapping_boxes_is_uncrossed() {
    let joined = box_of(10.0, 4.0, DVec3::ZERO).union(&box_of(10.0, 4.0, at(5.0, 5.0, 2.0)));
    assert_eq!(uncrossed(&joined.triangles()), Ok(()));
}

#[test]
fn two_boxes_of_one_height_joined_with_their_tops_in_one_plane_are_uncrossed() {
    let joined = box_of(10.0, 4.0, DVec3::ZERO).union(&box_of(10.0, 4.0, at(5.0, 5.0, 0.0)));
    assert_eq!(uncrossed(&joined.triangles()), Ok(()));
}

#[test]
fn two_boxes_joined_face_to_face_are_uncrossed() {
    let stacked = box_of(10.0, 4.0, DVec3::ZERO).union(&box_of(4.0, 4.0, at(3.0, 3.0, 4.0)));
    assert_eq!(uncrossed(&stacked.triangles()), Ok(()));
}

#[test]
fn a_box_with_a_pocket_cut_in_its_top_is_uncrossed() {
    let pocketed = box_of(10.0, 4.0, DVec3::ZERO).difference(&box_of(4.0, 2.0, at(3.0, 3.0, 3.0)));
    assert_eq!(uncrossed(&pocketed.triangles()), Ok(()));
}

#[test]
fn a_box_with_a_hole_right_through_it_is_uncrossed() {
    let holed = box_of(10.0, 4.0, DVec3::ZERO).difference(&box_of(4.0, 6.0, at(3.0, 3.0, -1.0)));
    assert_eq!(uncrossed(&holed.triangles()), Ok(()));
}

#[test]
fn boxes_far_from_the_origin_joined_and_cut_are_uncrossed() {
    let far = at(250.0, -120.0, 40.0);
    let solid = box_of(10.0, 4.0, far)
        .union(&box_of(10.0, 4.0, far + at(5.0, 5.0, 2.0)))
        .difference(&box_of(3.0, 10.0, far + at(4.0, 4.0, -1.0)));
    assert_eq!(uncrossed(&solid.triangles()), Ok(()));
}

#[test]
fn a_row_of_boxes_joined_one_after_another_is_uncrossed() {
    let row = (1..12).fold(box_of(4.0, 4.0, DVec3::ZERO), |row, step| {
        let step = step as f64;
        row.union(&box_of(
            4.0,
            4.0 + step / 2.0,
            at(step * 3.0, step % 3.0, 0.0),
        ))
    });
    assert_eq!(uncrossed(&row.triangles()), Ok(()));
}

/// A ring swept from a many-sided polygon standing beside the axis: every
/// wall a triangle leaning on its neighbours at a slight angle.
fn ring(sides: usize, turn: f64) -> Mesh {
    let outline: Vec<DVec2> = (0..sides)
        .map(|index| {
            let angle = std::f64::consts::TAU * index as f64 / sides as f64;
            DVec2::new(4.0, 0.0) + DVec2::from_angle(angle)
        })
        .collect();
    revolution(
        Loop::straight(&outline),
        &[],
        &kernel::fan(&outline),
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec2::ZERO,
        DVec2::Y,
        turn,
    )
    .expect("a profile beside the axis")
}

#[test]
fn a_ring_swept_a_quarter_turn_is_uncrossed() {
    let quarter = ring(12, std::f64::consts::FRAC_PI_2);
    assert_eq!(uncrossed(&quarter.triangles()), Ok(()));
}

#[test]
fn a_ring_cut_by_a_box_is_uncrossed() {
    let cut = ring(12, std::f64::consts::TAU).difference(&box_of(4.0, 4.0, at(2.0, -2.0, -2.0)));
    assert_eq!(uncrossed(&cut.triangles()), Ok(()));
}

#[test]
fn a_sphere_with_a_box_cut_out_of_it_is_uncrossed() {
    let half_disc: Vec<DVec2> = (0..=40)
        .map(|index| {
            let angle = std::f64::consts::PI * (index as f64 / 40.0 - 0.5);
            DVec2::from_angle(angle) * 3.0
        })
        .collect();
    let sphere = revolution(
        Loop::straight(&half_disc),
        &[],
        &kernel::fan(&half_disc),
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec2::ZERO,
        DVec2::Y,
        std::f64::consts::TAU,
    )
    .expect("a half disc beside the axis");
    let cut = sphere.difference(&box_of(4.0, 4.0, at(0.5, 0.5, 0.5)));
    assert_eq!(uncrossed(&cut.triangles()), Ok(()));
}

#[test]
fn a_ring_of_twenty_thousand_triangles_is_judged_in_well_under_a_second() {
    let triangles = ring(160, std::f64::consts::TAU).triangles();
    assert!(triangles.len() > 20_000, "{}", triangles.len());

    let started = Instant::now();
    assert_eq!(uncrossed(&triangles), Ok(()));
    assert!(
        started.elapsed() < Duration::from_secs(1),
        "{:?}",
        started.elapsed()
    );
}

/// A prism on a polygon of `sides` sides, its caps fanned from one corner the
/// way the kernel fans a sketch's area.
fn prism_on(sides: usize, radius: f64, push: DVec3) -> Vec<Triangle> {
    let outline: Vec<DVec2> = (0..sides)
        .map(|index| DVec2::from_angle(TAU * index as f64 / sides as f64) * radius)
        .collect();
    prism(
        Loop::straight(&outline),
        &[],
        &kernel::fan(&outline),
        |point| DVec3::new(point.x, point.y, 0.0),
        push,
    )
    .triangles()
}

#[test]
fn a_prism_whose_caps_are_fans_of_five_thousand_triangles_is_judged_ahead_of_a_quadratic_pass() {
    let solid = prism_on(5000, 10.0, DVec3::Z * 5.0);
    assert!(solid.len() > 19_990, "{}", solid.len());

    let started = Instant::now();
    assert_eq!(uncrossed(&solid), Ok(()));
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "{:?}",
        started.elapsed()
    );
}

#[test]
fn a_round_bar_of_five_thousand_sides_pushed_slantwise_is_judged_ahead_of_a_quadratic_pass() {
    let bar = prism_on(5000, 1.0, DVec3::splat(100.0));
    assert!(bar.len() > 19_990, "{}", bar.len());

    let started = Instant::now();
    assert_eq!(uncrossed(&bar), Ok(()));
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "{:?}",
        started.elapsed()
    );
}
