//! A surface with no hole in it, T-junctions and edges two solids share
//! included, and the kernel's own solids held to it.
//!
//! Closes #448.
//! - closed, every stretch of edge bordered by exactly two faces, T-junctions
//!   included, caught out by a solid broken on purpose —
//!   `a_cube_missing_a_triangle_is_open_along_an_edge_of_the_hole`,
//!   `a_triangle_laid_twice_is_open_two_against_one`,
//!   `a_top_cut_into_pieces_that_meet_at_a_t_is_still_closed`,
//!   `two_cubes_touching_along_one_edge_are_closed_with_two_faces_each_way_on_it`

use std::f64::consts::{FRAC_PI_2, TAU};
use std::time::{Duration, Instant};

use glam::{DVec2, DVec3};

use super::*;
use crate::mesh::Mesh;
use crate::mesh::tests::{box_of, fan as fan_in_the_plane};
use crate::soundness::tests::{cube, unit_cube};
use crate::sweep::{Loop, prism, revolution};

/// A flat face cut into triangles by a fan from its first corner, as the
/// kernel's own meshes are.
fn fan(corners: &[DVec3]) -> Vec<Triangle> {
    (1..corners.len() - 1)
        .map(|index| [corners[0], corners[index], corners[index + 1]])
        .collect()
}

/// The unit cube with its top face taken off, and something else to put in
/// its place.
fn unit_cube_topped_with(top: Vec<Triangle>) -> Vec<Triangle> {
    let mut solid = unit_cube();
    solid.drain(2..4);
    solid.extend(top);
    solid
}

fn top(x: f64, y: f64) -> DVec3 {
    DVec3::new(x, y, 1.0)
}

#[test]
fn nothing_at_all_has_no_hole() {
    assert_eq!(closed(&[]), Ok(()));
}

#[test]
fn the_unit_cube_is_closed() {
    assert_eq!(closed(&unit_cube()), Ok(()));
}

#[test]
fn a_cube_missing_a_triangle_is_open_along_an_edge_of_the_hole() {
    let mut holed = unit_cube();
    let [a, b, c] = holed.remove(0);

    let Err(Flaw::Open {
        from,
        to,
        one_way,
        other_way,
    }) = closed(&holed)
    else {
        panic!("{:?}", closed(&holed));
    };
    assert!(
        [[b, a], [c, b], [a, c]].contains(&[from, to]),
        "{from} -> {to}"
    );
    assert_eq!((one_way, other_way), (1, 0));
}

#[test]
fn a_cube_with_a_triangle_turned_round_is_open_two_against_none() {
    let mut turned = unit_cube();
    let [a, b, c] = turned[7];
    turned[7] = [a, c, b];

    let Err(Flaw::Open {
        from,
        to,
        one_way,
        other_way,
    }) = closed(&turned)
    else {
        panic!("{:?}", closed(&turned));
    };
    assert!(
        [[a, c], [c, b], [b, a]].contains(&[from, to]),
        "{from} -> {to}"
    );
    assert_eq!((one_way, other_way), (2, 0));
}

#[test]
fn a_triangle_laid_twice_is_open_two_against_one() {
    let mut doubled = unit_cube();
    let [a, b, c] = doubled[4];
    doubled.push(doubled[4]);

    let Err(Flaw::Open {
        from,
        to,
        one_way,
        other_way,
    }) = closed(&doubled)
    else {
        panic!("{:?}", closed(&doubled));
    };
    assert!(
        [[a, b], [b, c], [c, a]].contains(&[from, to]),
        "{from} -> {to}"
    );
    assert_eq!((one_way, other_way), (2, 1));
}

#[test]
fn two_cubes_touching_along_one_edge_are_closed_with_two_faces_each_way_on_it() {
    let mut touching = unit_cube();
    touching.extend(cube(DVec3::new(1.0, 1.0, 0.0), DVec3::new(2.0, 2.0, 1.0)));
    assert_eq!(closed(&touching), Ok(()));
}

/// The left half of the top is one piece; the right half is two, whose shared
/// corner lies in the middle of the left piece's edge. The pieces also put a
/// corner in the middle of the top edge of two of the walls.
#[test]
fn a_top_cut_into_pieces_that_meet_at_a_t_is_still_closed() {
    let pieces = [
        vec![top(0.0, 0.0), top(0.5, 0.0), top(0.5, 1.0), top(0.0, 1.0)],
        vec![top(0.5, 0.0), top(1.0, 0.0), top(1.0, 0.5), top(0.5, 0.5)],
        vec![top(0.5, 0.5), top(1.0, 0.5), top(1.0, 1.0), top(0.5, 1.0)],
    ];
    let cut = unit_cube_topped_with(pieces.iter().flat_map(|piece| fan(piece)).collect());
    assert_eq!(closed(&cut), Ok(()));
}

#[test]
fn a_corner_that_stops_short_of_its_neighbours_edge_leaves_a_hole() {
    let short = top(0.5 - 1e-3, 0.5);
    let pieces = [
        vec![top(0.0, 0.0), top(0.5, 0.0), top(0.5, 1.0), top(0.0, 1.0)],
        vec![top(0.5, 0.0), top(1.0, 0.0), top(1.0, 0.5), short],
        vec![short, top(1.0, 0.5), top(1.0, 1.0), top(0.5, 1.0)],
    ];
    let gapped = unit_cube_topped_with(pieces.iter().flat_map(|piece| fan(piece)).collect());
    assert!(
        matches!(closed(&gapped), Err(Flaw::Open { .. })),
        "{:?}",
        closed(&gapped)
    );
}

/// The top written with a corner in the middle of its first edge: the fan
/// from the first corner makes a triangle with no area, which runs along that
/// edge both ways and cancels out.
#[test]
fn a_face_with_a_corner_in_line_with_its_neighbours_is_still_closed() {
    let top_with_a_corner_in_line = fan(&[
        top(0.0, 0.0),
        top(0.5, 0.0),
        top(1.0, 0.0),
        top(1.0, 1.0),
        top(0.0, 1.0),
    ]);
    let [a, b, c] = top_with_a_corner_in_line[0];
    assert_eq!((b - a).cross(c - a), DVec3::ZERO, "a triangle with no area");
    assert_eq!(
        closed(&unit_cube_topped_with(top_with_a_corner_in_line)),
        Ok(())
    );
}

/// The corner is more than `near` from the end of the edge, so it is not that
/// end, and less than `near` off the edge, so it lies on it — though its
/// shadow on the edge falls closer to the end than `near`.
#[test]
fn a_corner_a_hair_off_an_edge_close_to_its_end_still_splits_it() {
    let near = NEAR * reach(&unit_cube());
    let (a, b) = (top(0.0, 0.0), top(1.0, 0.0));
    let beside = a + DVec3::new(0.5 * near, 0.9 * near, 0.0);
    assert!(a.distance(beside) > near, "not welded to the end");

    let split = unit_cube_topped_with(
        [
            fan(&[a, beside, top(0.0, 1.0)]),
            fan(&[beside, b, top(1.0, 1.0), top(0.0, 1.0)]),
        ]
        .concat(),
    );
    assert_eq!(closed(&split), Ok(()));
}

#[test]
fn corners_a_hair_apart_are_the_same_corner() {
    let mut nudged = unit_cube();
    nudged[0][0] += DVec3::new(1e-13, -1e-13, 1e-13);
    nudged[5][2].y += 1e-13;
    assert_eq!(closed(&nudged), Ok(()));
}

#[test]
fn corners_a_millionth_apart_are_not_the_same_corner() {
    let mut nudged = unit_cube();
    nudged[0][0].x += 1e-6;
    assert!(
        matches!(closed(&nudged), Err(Flaw::Open { .. })),
        "{:?}",
        closed(&nudged)
    );
}

#[test]
fn two_cubes_sharing_part_of_an_edge_are_closed() {
    let mut touching = unit_cube();
    touching.extend(cube(DVec3::new(1.0, 1.0, 0.5), DVec3::new(2.0, 2.0, 1.5)));
    assert_eq!(closed(&touching), Ok(()));
}

/// A profile in the plane of the drawing, turned about its vertical axis.
fn turned(outline: &[DVec2], turn: f64) -> Mesh {
    let triangles: Vec<[DVec2; 3]> = (1..outline.len() - 1)
        .map(|index| [outline[0], outline[index], outline[index + 1]])
        .collect();
    revolution(
        Loop::straight(outline),
        &[],
        &triangles,
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec2::ZERO,
        DVec2::Y,
        turn,
    )
    .expect("a profile on one side of the axis")
}

fn rectangle(low: DVec2, high: DVec2) -> Vec<DVec2> {
    vec![
        low,
        DVec2::new(high.x, low.y),
        high,
        DVec2::new(low.x, high.y),
    ]
}

fn ring(centre: DVec2, radius: f64, points: usize) -> Vec<DVec2> {
    (0..points)
        .map(|index| {
            let angle = TAU * index as f64 / points as f64;
            centre + DVec2::from_angle(angle) * radius
        })
        .collect()
}

#[test]
fn two_boxes_added_across_each_other_by_the_kernel_are_closed() {
    let joined =
        box_of(10.0, 4.0, DVec3::ZERO).union(&box_of(10.0, 4.0, DVec3::new(5.0, 5.0, 2.0)));
    assert_eq!(closed(&joined.triangles()), Ok(()));
}

#[test]
fn a_pocket_cut_by_the_kernel_is_closed() {
    let block = box_of(10.0, 10.0, DVec3::ZERO);
    let pocket = block.difference(&box_of(4.0, 4.0, DVec3::new(3.0, 3.0, 6.0)));
    assert_eq!(closed(&pocket.triangles()), Ok(()));
}

#[test]
fn two_pockets_cut_by_the_kernel_on_faces_that_land_on_each_other_are_closed() {
    let block = box_of(20.0, 10.0, DVec3::ZERO);
    let once = block.difference(&box_of(4.0, 10.0, DVec3::new(2.0, 2.0, 0.0)));
    let twice = once.difference(&box_of(8.0, 10.0, DVec3::new(0.0, 8.0, 0.0)));
    assert_eq!(closed(&twice.triangles()), Ok(()));
}

#[test]
fn a_ring_turned_right_round_by_the_kernel_is_closed() {
    let ring = turned(&rectangle(DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0)), TAU);
    assert_eq!(closed(&ring.triangles()), Ok(()));
}

#[test]
fn a_quarter_turn_capped_at_both_ends_by_the_kernel_is_closed() {
    let quarter = turned(
        &rectangle(DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0)),
        FRAC_PI_2,
    );
    assert_eq!(closed(&quarter.triangles()), Ok(()));
}

/// Every step of the sweep brings a triangle to the same point on the axis,
/// and the wall lying along the axis sweeps nothing at all.
#[test]
fn a_cylinder_turned_about_its_own_side_by_the_kernel_is_closed() {
    let cylinder = turned(&rectangle(DVec2::ZERO, DVec2::new(3.0, 2.0)), TAU);
    assert_eq!(closed(&cylinder.triangles()), Ok(()));
}

#[test]
fn a_ring_cut_into_by_the_kernel_is_closed() {
    let ring = turned(&rectangle(DVec2::new(3.0, 0.0), DVec2::new(9.0, 6.0)), TAU);
    let cut = ring.difference(&box_of(3.0, 30.0, DVec3::new(4.0, 1.0, -15.0)));
    assert_eq!(closed(&cut.triangles()), Ok(()));
}

/// Forty thousand triangles, judged in tens of milliseconds. Looking along
/// every edge for every corner takes seconds here instead.
#[test]
fn a_finely_turned_torus_is_judged_in_well_under_a_second() {
    let torus = turned(&ring(DVec2::new(6.0, 0.0), 2.0, 320), TAU).triangles();
    assert!(torus.len() >= 40_000, "{}", torus.len());

    let started = Instant::now();
    assert_eq!(closed(&torus), Ok(()));
    assert!(
        started.elapsed() < Duration::from_secs(1),
        "{:?}",
        started.elapsed()
    );
}

/// The torus alone is spread evenly; a small cube far off stretches the box
/// around the whole to a hundred thousand times its size, and a search sized
/// on that box finds every corner of the torus in the same few places.
#[test]
fn a_torus_with_a_small_cube_far_off_is_judged_in_well_under_a_second() {
    let mut soup = turned(&ring(DVec2::new(6.0, 0.0), 2.0, 320), TAU).triangles();
    soup.extend(cube(DVec3::splat(1e5), DVec3::splat(1e5 + 1.0)));

    let started = Instant::now();
    assert_eq!(closed(&soup), Ok(()));
    assert!(
        started.elapsed() < Duration::from_secs(1),
        "{:?}",
        started.elapsed()
    );
}

/// Each cap is a fan of triangles from one corner, so most edges cross the
/// whole polygon, past no corner but their own two.
#[test]
fn a_prism_on_a_polygon_of_twenty_thousand_sides_is_judged_in_well_under_a_second() {
    let outline = ring(DVec2::ZERO, 10.0, 20_000);
    let solid = prism(
        Loop::straight(&outline),
        &[],
        &fan_in_the_plane(&outline),
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec3::Z * 20.0,
    )
    .triangles();
    assert!(solid.len() > 79_000, "{}", solid.len());

    let started = Instant::now();
    assert_eq!(closed(&solid), Ok(()));
    assert!(
        started.elapsed() < Duration::from_secs(1),
        "{:?}",
        started.elapsed()
    );
}

#[test]
fn a_corner_that_is_not_a_number_leaves_a_hole() {
    let mut lost = unit_cube();
    lost[0][1] = DVec3::new(f64::NAN, 0.0, 0.0);
    lost[3][2] = DVec3::new(0.0, f64::INFINITY, 0.0);
    assert!(
        matches!(closed(&lost), Err(Flaw::Open { .. })),
        "{:?}",
        closed(&lost)
    );
}

/// Corners so far out that the length of an edge between them is more than a
/// float holds. The answer does not matter as much as getting one.
#[test]
fn a_cube_too_large_to_measure_is_still_judged() {
    let huge = cube(DVec3::splat(-1e200), DVec3::splat(1e200));
    assert_eq!(closed(&huge), Ok(()));
}
