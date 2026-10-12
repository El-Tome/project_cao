//! A step the kernel builds though it reads it unstably (#552): a plane of
//! one operand leaning off square to, or along, the axis of a wall of the
//! other, by more than decision 11 lays exactly and less than a thousandth
//! of a radian, their faces near enough to meet. Built, and noted.

use glam::DQuat;

use super::*;
use crate::brep::{Lean, Unstable};

/// The body turned by `degrees` about `axis`, about the middle of its box.
fn leaning(body: Body, axis: DVec3, degrees: f64) -> Body {
    let (low, high) = body.bounds().expect("a body has a box");
    let turn = DQuat::from_axis_angle(axis, degrees.to_radians());
    body.rotated((low + high) / 2.0, turn, DVec3::ZERO, &[], 0.0)
        .expect("a small turn is made")
}

/// A shaft of radius five about Z, from 0 to 30.
fn shaft() -> Body {
    standing([0.0, 0.0], 5.0, 0.0, 30.0)
}

fn noted(unstable: Option<Unstable>, lean: Lean, degrees: f64) {
    let Some(unstable) = unstable else {
        panic!("a lean of {degrees}° is noted");
    };
    assert_eq!(unstable.lean, lean);
    assert!(
        (unstable.degrees - degrees).abs() <= 1e-6 * degrees,
        "the lean noted is {}°, against {degrees}°",
        unstable.degrees,
    );
}

#[test]
fn a_cut_leaning_a_ten_thousandth_of_a_degree_off_square_to_a_shaft_is_built_and_noted_unstable() {
    let tool = leaning(
        block([-10.0, -10.0, 20.0], [10.0, 10.0, 40.0]),
        DVec3::X,
        1e-4,
    );

    let cut = shaft().cut_by(&tool).expect("the kernel builds it");

    noted(cut.unstable(), Lean::Square, 1e-4);
}

#[test]
fn a_side_leaning_a_ten_thousandth_of_a_degree_off_along_a_bore_is_built_and_noted_unstable() {
    let bored = block([-20.0, -20.0, 0.0], [20.0, 20.0, 30.0])
        .cut_by(&shaft())
        .expect("the block is bored");
    let tool = leaning(
        block([2.0, -30.0, -10.0], [30.0, 30.0, 40.0]),
        DVec3::Y,
        1e-4,
    );

    let cut = bored.cut_by(&tool).expect("the kernel builds it");

    noted(cut.unstable(), Lean::Along, 1e-4);
}

#[test]
fn a_lean_decision_11_lays_exactly_is_not_noted() {
    let tool = leaning(
        block([-10.0, -10.0, 20.0], [10.0, 10.0, 40.0]),
        DVec3::X,
        1e-7,
    );

    let cut = shaft().cut_by(&tool).expect("the kernel builds it");

    assert_eq!(cut.unstable(), None);
}

#[test]
fn a_plane_at_a_real_slant_is_not_noted() {
    let tool = leaning(
        block([-10.0, -10.0, 20.0], [10.0, 10.0, 40.0]),
        DVec3::X,
        30.0,
    );

    let cut = shaft().cut_by(&tool).expect("the kernel builds it");

    assert_eq!(cut.unstable(), None);
}

#[test]
fn a_lean_whose_faces_stand_clear_of_the_wall_is_not_noted() {
    let along = DVec3::new(1.0, 1.0, 0.0).normalize();
    let across = DVec3::new(-1.0, 1.0, 0.0).normalize();
    let frame = |origin: DVec3| Frame {
        origin,
        u: DVec3::Z,
        v: across,
    };
    let disc = Contour {
        corners: vec![DVec2::X * 5.0],
        runs: vec![Run::Round {
            center: DVec2::ZERO,
            turn: TAU,
        }],
    };
    let slanted_shaft = Body::raised(
        &disc,
        &[],
        frame(DVec3::ZERO),
        -frame(DVec3::ZERO).normal() * 30.0,
    )
    .expect("a shaft along a diagonal raises");
    let square = Contour::rectangle(DVec2::splat(-10.0), DVec2::splat(10.0));
    let beyond = Body::raised(&square, &[], frame(along * 31.0), along * 10.0)
        .expect("a block beyond its end raises");
    let tool = leaning(beyond, DVec3::Z, 1e-4);

    let cut = slanted_shaft.cut_by(&tool).expect("the kernel builds it");

    assert_eq!(cut.unstable(), None);
}

#[test]
fn the_least_lean_of_a_step_is_the_one_noted() {
    let tool = leaning(
        leaning(
            block([2.0, -30.0, 20.0], [30.0, 30.0, 50.0]),
            DVec3::Y,
            1e-4,
        ),
        DVec3::X,
        3e-4,
    );

    let cut = shaft().cut_by(&tool).expect("the kernel builds it");

    noted(cut.unstable(), Lean::Along, 1e-4);
}

#[test]
fn a_body_cut_again_after_an_unstable_step_carries_no_note_of_it() {
    let tool = leaning(
        block([-10.0, -10.0, 20.0], [10.0, 10.0, 40.0]),
        DVec3::X,
        1e-4,
    );
    let cut = shaft().cut_by(&tool).expect("the kernel builds it");
    assert!(cut.unstable().is_some(), "the first cut is noted");

    let again = cut
        .cut_by(&block([-10.0, 2.0, -10.0], [10.0, 10.0, 10.0]))
        .expect("the kernel cuts it again");

    assert_eq!(again.unstable(), None);
}
