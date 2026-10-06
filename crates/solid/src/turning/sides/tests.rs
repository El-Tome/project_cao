//! Which sides of an area across its axis a turn needs (#533).

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::DVec2;

use super::super::{Axis, ON_THE_AXIS, Turn};
use crate::profile::{Contour, Profile};
use crate::sweep::Loop;

const V: Axis = Axis {
    origin: DVec2::ZERO,
    direction: DVec2::Y,
};

fn turn(angle: f64, reach: f64) -> Turn {
    Turn {
        axis: V,
        angle,
        resolution: 1e-5 * reach,
        on_the_axis: ON_THE_AXIS * reach,
    }
}

fn corners(low: DVec2, high: DVec2) -> Vec<DVec2> {
    Contour::rectangle(low, high).corners
}

fn side<'a>(points: &'a [DVec2], curves: &'a [Option<usize>]) -> Profile<'a> {
    Profile {
        exact: None,
        sampled: Loop { points, curves },
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

fn straight(points: &[DVec2]) -> Profile<'_> {
    side(points, &[])
}

/// The two sides of a circle about `center` cut along V, the left one first,
/// each arc sampled into `steps` segments and closed along the axis.
fn round_sides(center: DVec2, radius: f64, steps: usize) -> [(Vec<DVec2>, Vec<Option<usize>>); 2] {
    let top = (-center.x / radius).acos();
    let arc = |from: f64, to: f64| {
        let points: Vec<DVec2> = (0..=steps)
            .map(|step| {
                let angle = from + (to - from) * step as f64 / steps as f64;
                center + radius * DVec2::new(angle.cos(), angle.sin())
            })
            .map(|point| DVec2::new(if point.x.abs() < 1e-12 { 0.0 } else { point.x }, point.y))
            .collect();
        let mut curves = vec![Some(0); steps];
        curves.push(None);
        (points, curves)
    };
    [arc(top, TAU - top), arc(-top, top)]
}

#[test]
fn turned_whole_a_side_whose_mirror_lies_within_the_other_is_left_out() {
    let left = corners(DVec2::new(-4.0, 0.0), DVec2::new(0.0, 2.0));
    let right = corners(DVec2::new(0.0, 0.0), DVec2::new(5.0, 2.0));
    let whole = turn(TAU, 5.0);

    assert_eq!(
        whole.sides_needed(&[straight(&left), straight(&right)]),
        vec![1]
    );
    assert_eq!(
        whole.sides_needed(&[straight(&right), straight(&left)]),
        vec![0]
    );
    assert_eq!(
        turn(-TAU, 5.0).sides_needed(&[straight(&left), straight(&right)]),
        vec![1],
        "a whole turn backwards",
    );
}

#[test]
fn turned_part_way_every_side_is_needed() {
    let left = corners(DVec2::new(-4.0, 0.0), DVec2::new(0.0, 2.0));
    let right = corners(DVec2::new(0.0, 0.0), DVec2::new(5.0, 2.0));
    for angle in [FRAC_PI_2, -FRAC_PI_2, PI, TAU - 2e-3] {
        assert_eq!(
            turn(angle, 5.0).sides_needed(&[straight(&left), straight(&right)]),
            vec![0, 1],
            "a turn of {angle} rad sweeps each side apart",
        );
    }
}

#[test]
fn sides_reaching_past_each_other_are_both_needed() {
    let taller = corners(DVec2::new(-4.0, 0.0), DVec2::new(0.0, 3.0));
    let wider = corners(DVec2::new(0.0, 0.0), DVec2::new(5.0, 2.0));
    assert_eq!(
        turn(TAU, 5.0).sides_needed(&[straight(&taller), straight(&wider)]),
        vec![0, 1],
    );

    let a_hair_wider = corners(DVec2::new(-5.0001, 0.0), DVec2::new(0.0, 2.0));
    let a_hair_taller = corners(DVec2::new(0.0, 0.0), DVec2::new(5.0, 2.0001));
    assert_eq!(
        turn(TAU, 5.0).sides_needed(&[straight(&a_hair_wider), straight(&a_hair_taller)]),
        vec![0, 1],
        "a straight side reaching a hair past the other is still turned",
    );
}

#[test]
fn of_two_sides_mirroring_each_other_one_is_turned() {
    let left = corners(DVec2::new(-5.0, 0.0), DVec2::new(0.0, 2.0));
    let right = corners(DVec2::new(0.0, 0.0), DVec2::new(5.0, 2.0));
    assert_eq!(
        turn(TAU, 5.0).sides_needed(&[straight(&left), straight(&right)]),
        vec![0],
    );
}

#[test]
fn a_holed_side_is_turned_with_the_other() {
    let left = corners(DVec2::new(-4.0, 0.0), DVec2::new(0.0, 2.0));
    let right = corners(DVec2::new(0.0, 0.0), DVec2::new(5.0, 2.0));
    let hole = corners(DVec2::new(1.0, 0.5), DVec2::new(2.0, 1.5));
    let mut holed = straight(&right);
    holed.sampled_holes.push(Loop::straight(&hole));
    assert_eq!(
        turn(TAU, 5.0).sides_needed(&[straight(&left), holed]),
        vec![0, 1],
        "the left side turned whole fills the ring the hole leaves",
    );
}

#[test]
fn turned_whole_the_smaller_side_of_a_circle_across_its_axis_is_left_out() {
    let [left, right] = round_sides(DVec2::new(5.0, 16.0), 7.0, 32);
    assert_eq!(
        turn(TAU, 12.0).sides_needed(&[side(&left.0, &left.1), side(&right.0, &right.1)]),
        vec![1],
    );
}

#[test]
fn turned_whole_a_circle_centred_on_its_axis_is_turned_from_one_side() {
    let [left, right] = round_sides(DVec2::new(0.0, 16.0), 7.0, 32);
    assert_eq!(
        turn(TAU, 7.0).sides_needed(&[side(&left.0, &left.1), side(&right.0, &right.1)]),
        vec![0],
    );
}

#[test]
fn a_circle_across_its_axis_turned_part_way_is_turned_on_both_sides() {
    let [left, right] = round_sides(DVec2::new(5.0, 16.0), 7.0, 32);
    assert_eq!(
        turn(PI, 12.0).sides_needed(&[side(&left.0, &left.1), side(&right.0, &right.1)]),
        vec![0, 1],
    );
}
