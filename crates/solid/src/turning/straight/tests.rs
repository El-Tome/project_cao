//! How a profile is laid square to its axis before the exact kernel turns
//! it, and when it is not straight enough to be (#533).

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::mesh::tests::fan;
use crate::profile::{Profile, Run};
use crate::sweep::{self, Loop};
use crate::turning::{Axis, is_whole};

const FLAT: Frame = Frame {
    origin: DVec3::ZERO,
    u: DVec3::X,
    v: DVec3::Y,
};

const V: Axis = Axis {
    origin: DVec2::ZERO,
    direction: DVec2::Y,
};

fn turn_of(contours: &[&Contour], resolution: f64) -> Turn {
    let points: Vec<DVec2> = contours
        .iter()
        .flat_map(|contour| contour.corners.iter().copied())
        .collect();
    let area = Profile {
        exact: None,
        sampled: Loop::straight(&points),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    Turn::of(V, TAU, resolution, &area)
}

fn laid(outline: &Contour, holes: &[Contour], resolution: f64) -> Option<Straight> {
    let contours: Vec<&Contour> = std::iter::once(outline).chain(holes).collect();
    Straight::of(outline, holes, FLAT, &turn_of(&contours, resolution), 0.0)
}

fn polygon(points: &[[f64; 2]]) -> Contour {
    Contour::straight(points.iter().map(|&[x, y]| DVec2::new(x, y)).collect())
}

fn at(straight: &Straight) -> Vec<Vec<[f64; 2]>> {
    straight
        .contours
        .iter()
        .map(|contour| contour.iter().map(|corner| corner.at.to_array()).collect())
        .collect()
}

fn runs(straight: &Straight) -> Vec<Vec<u32>> {
    straight
        .contours
        .iter()
        .map(|contour| contour.iter().map(|corner| corner.run).collect())
        .collect()
}

#[test]
fn a_corner_a_hair_across_the_axis_is_laid_on_it() {
    let square = polygon(&[[-1e-5, 0.0], [5.0, 0.0], [5.0, 2.0], [-1e-5, 2.0]]);
    let straight = laid(&square, &[], 0.0).expect("a square a hair across its axis");
    assert_eq!(straight.side, -1.0);
    assert_eq!(
        at(&straight),
        [[[0.0, 0.0], [0.0, 5.0], [2.0, 5.0], [2.0, 0.0]]]
    );
    assert_eq!(runs(&straight), [[0, 1, 2, 3]]);
    assert_eq!(straight.runs, 4);
    assert_eq!(straight.last_off_the_axis, Some(2));
}

#[test]
fn a_corner_a_hair_off_the_axis_on_its_own_side_is_laid_on_it() {
    let square = polygon(&[[0.0, 0.0], [5.0, 0.0], [5.0, 2.0], [1e-5, 2.0]]);
    let straight = laid(&square, &[], 0.0).expect("a square a hair off its axis");
    assert_eq!(
        at(&straight),
        [[[0.0, 0.0], [0.0, 5.0], [2.0, 5.0], [2.0, 0.0]]]
    );
    assert_eq!(straight.last_off_the_axis, Some(2));

    let left = polygon(&[[-5.0, 0.0], [-1e-5, 0.0], [-1e-5, 2.0], [-5.0, 2.0]]);
    let straight = laid(&left, &[], 0.0).expect("a square a hair off its axis, left of it");
    assert_eq!(straight.side, 1.0);
    assert_eq!(
        at(&straight),
        [[[0.0, 5.0], [0.0, 0.0], [2.0, 0.0], [2.0, 5.0]]]
    );
    assert_eq!(straight.last_off_the_axis, Some(3));
}

const RESOLUTION: f64 = 1e-4;

fn near(value: f64, expected: f64) -> bool {
    (value - expected).abs() < 1e-12
}

#[test]
fn a_run_a_hair_off_parallel_to_the_axis_is_laid_parallel() {
    let hair = 0.1 * RESOLUTION;
    let leaning = polygon(&[[1.0, 0.0], [5.0, 0.0], [5.0 + hair, 2.0], [1.0, 2.0]]);
    let straight = laid(&leaning, &[], RESOLUTION).expect("a run a hair off parallel");
    let corners = &straight.contours[0];
    assert_eq!(corners[1].at.y, corners[2].at.y);
    assert!(near(corners[1].at.y, 5.0 + hair / 2.0), "{corners:?}");
    assert_eq!((corners[0].at.y, corners[3].at.y), (1.0, 1.0));
}

#[test]
fn a_run_a_hair_off_square_to_the_axis_is_laid_square() {
    let hair = 0.1 * RESOLUTION;
    let leaning = polygon(&[[1.0, 0.0], [5.0, hair], [5.0, 2.0], [1.0, 2.0]]);
    let straight = laid(&leaning, &[], RESOLUTION).expect("a run a hair off square");
    let corners = &straight.contours[0];
    assert_eq!(corners[0].at.x, corners[1].at.x);
    assert!(near(corners[0].at.x, hair / 2.0), "{corners:?}");
    assert_eq!((corners[2].at.x, corners[3].at.x), (2.0, 2.0));
}

#[test]
fn a_run_leaning_ten_times_the_resolution_is_not_straight() {
    let lean = 10.0 * RESOLUTION;
    let leaning = polygon(&[[1.0, 0.0], [5.0, 0.0], [5.0 + lean, 2.0], [1.0, 2.0]]);
    assert_eq!(laid(&leaning, &[], RESOLUTION), None);
    let leaning = polygon(&[[1.0, 0.0], [5.0, lean], [5.0, 2.0], [1.0, 2.0]]);
    assert_eq!(laid(&leaning, &[], RESOLUTION), None);
}

#[test]
fn a_chamfer_wider_than_the_resolution_stays_a_slant() {
    let chamfered = |width: f64| {
        polygon(&[
            [1.0, 0.0],
            [5.0 - width, 0.0],
            [5.0, width],
            [5.0, 2.0],
            [1.0, 2.0],
        ])
    };
    assert_eq!(laid(&chamfered(2.0 * RESOLUTION), &[], RESOLUTION), None);
    let straight = laid(&chamfered(0.5 * RESOLUTION), &[], RESOLUTION)
        .expect("a chamfer within the resolution");
    assert_eq!(runs(&straight), [[0, 2, 3, 4]], "the chamfer names no face");
}

#[test]
fn an_arc_is_not_straight() {
    let square = polygon(&[[1.0, 0.0], [5.0, 0.0], [5.0, 2.0], [1.0, 2.0]]);
    let mut rounded = square.clone();
    rounded.runs[1] = Run::Round {
        center: DVec2::new(5.0, 1.0),
        turn: 1e-6,
    };
    assert_eq!(laid(&rounded, &[], RESOLUTION), None);
    let hole = Contour::circle(DVec2::new(3.0, 1.0), 0.5);
    assert_eq!(laid(&square, &[hole], RESOLUTION), None);
}

#[test]
fn two_radii_a_hair_apart_that_no_run_joins_are_laid_as_one() {
    let hair = 0.5 * RESOLUTION;
    let stepped = polygon(&[
        [0.0, 0.0],
        [5.0, 0.0],
        [5.0, 2.0],
        [3.0, 2.0],
        [3.0, 4.0],
        [5.0 + hair, 4.0],
        [5.0 + hair, 6.0],
        [0.0, 6.0],
    ]);
    let straight = laid(&stepped, &[], RESOLUTION).expect("a stepped profile");
    let corners = &straight.contours[0];
    let radius = corners[1].at.y;
    assert!(near(radius, 5.0 + hair / 2.0), "{corners:?}");
    for corner in [2, 5, 6] {
        assert_eq!(corners[corner].at.y, radius, "corner {corner}");
    }
    assert_eq!((corners[3].at.y, corners[4].at.y), (3.0, 3.0));
}

#[test]
fn a_run_shorter_than_the_resolution_names_no_face_and_keeps_its_number() {
    let stub = 0.5 * RESOLUTION;
    let stubbed = polygon(&[
        [1.0, 0.0],
        [5.0, 0.0],
        [5.0, 2.0],
        [5.0 - stub, 2.0],
        [1.0, 2.0],
    ]);
    let straight = laid(&stubbed, &[], RESOLUTION).expect("a profile with a stub");
    assert_eq!(runs(&straight), [[0, 1, 3, 4]]);
    assert_eq!(straight.runs, 5);
    assert_eq!(straight.last_off_the_axis, Some(4));
    let corners = &straight.contours[0];
    assert_eq!(corners[2].at, DVec2::new(2.0, corners[1].at.y));

    let square = polygon(&[[1.0, 0.0], [5.0, 0.0], [5.0, 2.0], [1.0, 2.0]]);
    let hole = polygon(&[
        [2.0, 0.5],
        [2.0, 1.5],
        [2.0 + stub, 1.5],
        [3.0, 1.5],
        [3.0, 0.5],
    ]);
    let straight = laid(&square, &[hole], RESOLUTION).expect("a hole with a stub");
    assert_eq!(runs(&straight), [vec![0, 1, 2, 3], vec![4, 6, 7, 8]]);
    assert_eq!(straight.runs, 9);
}

#[test]
fn a_chain_of_hairs_wider_than_twice_the_tolerance_is_not_straight() {
    let step = 0.9 * RESOLUTION;
    let stairs = |steps: usize| {
        let mut points = vec![[1.0, 0.0]];
        for index in 0..steps {
            let radius = 5.0 + step * index as f64;
            points.push([radius, index as f64]);
            points.push([radius, index as f64 + 1.0]);
        }
        points.push([1.0, steps as f64]);
        polygon(&points)
    };
    assert!(laid(&stairs(3), &[], RESOLUTION).is_some());
    assert_eq!(laid(&stairs(7), &[], RESOLUTION), None);
}

/// Each corner of the laid profile with the number of the run leaving it,
/// whichever corner each contour starts at.
fn corners(straight: &Straight) -> Vec<Vec<([f64; 2], u32)>> {
    straight
        .contours
        .iter()
        .map(|contour| {
            let mut corners: Vec<([f64; 2], u32)> = contour
                .iter()
                .map(|corner| (corner.at.to_array(), corner.run))
                .collect();
            corners.sort_by(|one, other| one.partial_cmp(other).expect("no NaN"));
            corners
        })
        .collect()
}

#[test]
fn a_hole_a_hair_inside_its_outline_opens_onto_it_where_laying_makes_them_one() {
    let gap = 0.5 * RESOLUTION;
    let square = polygon(&[[1.0, 0.0], [5.0, 0.0], [5.0, 2.0], [1.0, 2.0]]);
    let hole = polygon(&[[2.0, 0.5], [2.0, 1.5], [5.0 - gap, 1.5], [5.0 - gap, 0.5]]);
    let straight = laid(&square, &[hole], RESOLUTION).expect("a notch where the wall was");
    let far = straight.contours[0]
        .iter()
        .map(|corner| corner.at.y)
        .fold(0.0, f64::max);
    assert!(near(far, 5.0 - gap / 2.0), "{straight:?}");
    assert_eq!(
        corners(&straight),
        [vec![
            ([0.0, 1.0], 3),
            ([0.0, far], 0),
            ([0.5, 2.0], 7),
            ([0.5, far], 1),
            ([1.5, 2.0], 4),
            ([1.5, far], 5),
            ([2.0, 1.0], 2),
            ([2.0, far], 1),
        ]],
        "the wall and the hole's run along it name nothing",
    );
    assert_eq!(straight.runs, 8);
}

#[test]
fn a_slot_narrower_than_the_resolution_is_closed_by_laying() {
    let gap = 0.5 * RESOLUTION;
    let slotted = polygon(&[
        [1.0, 0.0],
        [5.0, 0.0],
        [5.0, 4.0],
        [3.0 + gap, 4.0],
        [3.0 + gap, 1.0],
        [3.0, 1.0],
        [3.0, 4.0],
        [1.0, 4.0],
    ]);
    let straight = laid(&slotted, &[], RESOLUTION).expect("the slot closed");
    let corners = corners(&straight);
    let at: Vec<[f64; 2]> = corners[0].iter().map(|(at, _)| *at).collect();
    let numbers: Vec<u32> = corners[0].iter().map(|(_, run)| *run).collect();
    assert_eq!(corners.len(), 1);
    assert_eq!(at[0], [0.0, 1.0]);
    assert_eq!(numbers, [7, 0, 6, 2, 1]);
    assert_eq!(at[3][0], 4.0);
    assert!(near(at[3][1], 3.0 + gap / 2.0), "{at:?}");
}

#[test]
fn a_hole_laid_onto_its_outline_at_a_corner_alone_is_not_straight() {
    let gap = 0.5 * RESOLUTION;
    let bent = polygon(&[
        [1.0, 0.0],
        [5.0, 0.0],
        [5.0, 1.0],
        [3.0, 1.0],
        [3.0, 2.0],
        [1.0, 2.0],
    ]);
    let into_the_bend = |off: f64| {
        polygon(&[
            [2.0, 0.5],
            [2.0, 1.0 - off],
            [3.0 - off, 1.0 - off],
            [3.0 - off, 0.5],
        ])
    };
    assert_eq!(laid(&bent, &[into_the_bend(gap)], RESOLUTION), None);
    assert!(
        laid(&bent, &[into_the_bend(0.0)], RESOLUTION).is_some(),
        "a contact the drawing has is the kernel's to judge",
    );
}

#[test]
fn the_tolerance_is_never_finer_than_twenty_of_the_kernel_s() {
    let leaning = |lean: f64| polygon(&[[1.0, 0.0], [5.0, 0.0], [5.0 + lean, 2.0], [1.0, 2.0]]);
    let eps = Scale::of(10.0).eps();
    assert!(laid(&leaning(10.0 * eps), &[], 0.0).is_some());
    assert_eq!(laid(&leaning(40.0 * eps), &[], 0.0), None);

    let profile = leaning(1e-3);
    let turn = turn_of(&[&profile], 0.0);
    assert_eq!(Straight::of(&profile, &[], FLAT, &turn, 0.0), None);
    assert!(
        Straight::of(&profile, &[], FLAT, &turn, 1e6).is_some(),
        "laid within the kernel's tolerance over the part it is turned against",
    );
    let far = Frame {
        origin: DVec3::new(0.0, 0.0, 1e6),
        ..FLAT
    };
    assert!(
        Straight::of(&profile, &[], far, &turn, 0.0).is_some(),
        "laid within the kernel's tolerance over where it stands",
    );
}

/// The numbers the profile names turned by `angle`, laid straight, and the
/// count of the flats' revolution of it.
fn counted(outline: &[DVec2], holes: &[Vec<DVec2>], angle: f64) -> (u32, usize) {
    let exact_holes: Vec<Contour> = holes
        .iter()
        .map(|hole| Contour::straight(hole.clone()))
        .collect();
    let mut contours = vec![Contour::straight(outline.to_vec())];
    contours.extend(exact_holes.iter().cloned());
    let turn = Turn {
        angle,
        ..turn_of(&contours.iter().collect::<Vec<_>>(), 0.0)
    };
    let straight = Straight::of(&contours[0], &exact_holes, FLAT, &turn, 0.0)
        .expect("a profile of straight runs");
    let hole_loops: Vec<Loop<'_>> = holes.iter().map(|hole| Loop::straight(hole)).collect();
    let flats = sweep::revolution(
        Loop::straight(outline),
        &hole_loops,
        &fan(outline),
        |point| FLAT.at(point),
        V.origin,
        V.direction,
        angle,
    )
    .expect("a profile on one side of its axis");
    (straight.numbers(is_whole(angle)), flats.faces_end())
}

fn points(corners: &[[f64; 2]]) -> Vec<DVec2> {
    corners.iter().map(|&[x, y]| DVec2::new(x, y)).collect()
}

#[test]
fn a_full_turn_ending_on_its_axis_counts_its_numbers_as_the_flats_do() {
    let on_the_axis = points(&[[0.0, 0.0], [5.0, 0.0], [5.0, 2.0], [0.0, 2.0]]);
    assert_eq!(counted(&on_the_axis, &[], TAU), (3, 3));
    assert_eq!(counted(&on_the_axis, &[], FRAC_PI_2), (6, 6));
}

#[test]
fn the_count_of_a_turned_profile_is_the_flats_faces_end() {
    let on_the_axis = points(&[[0.0, 0.0], [5.0, 0.0], [5.0, 2.0], [0.0, 2.0]]);
    let off_the_axis = points(&[[1.0, 0.0], [5.0, 0.0], [5.0, 2.0], [1.0, 2.0]]);
    let shaft = points(&[
        [0.0, 0.0],
        [5.0, 0.0],
        [5.0, 2.0],
        [3.0, 2.0],
        [3.0, 4.0],
        [0.0, 4.0],
    ]);
    let hole = points(&[[2.0, 0.5], [3.0, 0.5], [3.0, 1.5], [2.0, 1.5]]);
    let walked = |corners: &[DVec2], from: usize| {
        let mut corners = corners.to_vec();
        corners.rotate_left(from);
        corners
    };
    for angle in [TAU, -TAU, FRAC_PI_2, -3.0 * FRAC_PI_2, PI] {
        for outline in [&on_the_axis, &off_the_axis, &shaft] {
            for from in 0..outline.len() {
                let outline = walked(outline, from);
                let (numbers, faces_end) = counted(&outline, &[], angle);
                assert_eq!(
                    numbers as usize, faces_end,
                    "{outline:?} turned {angle} rad"
                );
            }
        }
        for outline in [&on_the_axis, &off_the_axis] {
            for from in 0..hole.len() {
                let holes = [walked(&hole, from)];
                let (numbers, faces_end) = counted(outline, &holes, angle);
                assert_eq!(
                    numbers as usize, faces_end,
                    "{outline:?} holed by {holes:?} turned {angle} rad",
                );
            }
        }
    }
}
