//! What sketch · drawing_on.rs is held to.
//!
//! Closes #561.
//! - on the second trait of a chain, the field shows the corner angle with the
//!   trait before, signed by the side the mouse is on —
//!   `the_angle_read_is_the_corner_signed_by_the_side_the_trait_leaves_on`
//! - 90 typed there draws a square corner on the side of the mouse —
//!   `ninety_typed_draws_a_square_corner_on_the_side_of_the_cursor`; the angle
//!   it lays at the corner is held where it is laid, in
//!   `crates/app/src/screens/viewport/input/angle_arm/tests.rs`
//! - started on the end of the only trait ending at a point, the angle is read
//!   against that trait — `a_trait_started_on_the_end_of_a_lone_trait_reads_its_angle_against_it`
//! - started where several traits end, the angle is read against the
//!   horizontal — `a_trait_started_where_two_traits_end_reads_its_angle_against_the_horizontal`
//!
//! Decided with the owner, in the issue: the angle is the corner's, not the
//! turn — `the_angle_typed_is_the_corner_one_not_the_turn`; and a sign typed is
//! left to the mouse — `a_sign_typed_on_a_chained_trait_is_left_to_the_cursor`.

use glam::DVec2;

use crate::aim::{ChainAnchor, LockedInput};
use crate::plane::WorkPlane;
use crate::sketch::{SegmentId, Sketch};

const TOLERANCE: f64 = 1e-9;

/// A first trait running east from the origin to (40, 0), the chain carrying
/// on from its end.
fn a_chain_of_one() -> (Sketch, SegmentId, ChainAnchor) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::ZERO);
    let corner = sketch.add_point(DVec2::new(40.0, 0.0));
    let first = sketch.add_segment(start, corner);
    (sketch, first, ChainAnchor::Point(corner))
}

fn angle(degrees: f64) -> LockedInput {
    LockedInput {
        first: None,
        second: Some(degrees),
    }
}

fn runs(sketch: &Sketch, anchor: ChainAnchor, position: DVec2) -> DVec2 {
    (position - sketch.anchor_position(anchor).expect("the anchor is there")).normalize()
}

#[test]
fn the_angle_read_is_the_corner_signed_by_the_side_the_trait_leaves_on() {
    let (sketch, first, anchor) = a_chain_of_one();
    let corner = DVec2::new(40.0, 0.0);

    for (towards, expected) in [
        (corner + DVec2::new(0.0, 30.0), 90.0),
        (corner + DVec2::new(0.0, -30.0), -90.0),
        (corner + DVec2::new(30.0, 30.0), 135.0),
        (corner + DVec2::new(-30.0, -30.0), -45.0),
    ] {
        let read = sketch
            .angle_as_typed(anchor, Some(first), towards)
            .expect("the anchor is there");
        assert!(
            (read - expected).abs() < TOLERANCE,
            "towards {towards} the field reads {read}°, not {expected}°",
        );
    }
}

#[test]
fn ninety_typed_draws_a_square_corner_on_the_side_of_the_cursor() {
    let (sketch, first, anchor) = a_chain_of_one();

    for (cursor, wanted) in [
        (DVec2::new(45.0, 30.0), DVec2::Y),
        (DVec2::new(45.0, -30.0), -DVec2::Y),
    ] {
        let aimed = sketch.aim(anchor, Some(first), cursor, angle(90.0), 1.0);

        assert!(
            runs(&sketch, anchor, aimed.position).distance(wanted) < TOLERANCE,
            "90 typed with the cursor at {cursor} runs towards {}",
            aimed.position,
        );
    }
}

#[test]
fn the_angle_typed_is_the_corner_one_not_the_turn() {
    let (sketch, first, anchor) = a_chain_of_one();
    let above = DVec2::new(45.0, 30.0);

    let straight_on = sketch.aim(anchor, Some(first), above, angle(180.0), 1.0);
    let sharp = sketch.aim(anchor, Some(first), above, angle(30.0), 1.0);

    assert!(
        runs(&sketch, anchor, straight_on.position).distance(DVec2::X) < TOLERANCE,
        "180 carries straight on: {}",
        straight_on.position,
    );
    let back_and_up = DVec2::from_angle(150.0_f64.to_radians());
    assert!(
        runs(&sketch, anchor, sharp.position).distance(back_and_up) < TOLERANCE,
        "30 is a sharp point turning back over the first trait: {}",
        sharp.position,
    );
}

#[test]
fn a_sign_typed_on_a_chained_trait_is_left_to_the_cursor() {
    let (sketch, first, anchor) = a_chain_of_one();

    let aimed = sketch.aim(
        anchor,
        Some(first),
        DVec2::new(45.0, 30.0),
        angle(-90.0),
        1.0,
    );

    assert!(
        runs(&sketch, anchor, aimed.position).distance(DVec2::Y) < TOLERANCE,
        "-90 typed with the cursor above still turns up: {}",
        aimed.position,
    );
}

#[test]
fn a_trait_started_on_the_end_of_a_lone_trait_reads_its_angle_against_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::ZERO);
    let end = sketch.add_point(DVec2::new(30.0, 30.0));
    let leaning = sketch.add_segment(start, end);
    let anchor = ChainAnchor::Point(end);

    assert_eq!(sketch.drawn_on_from(anchor, None), Some(leaning));
    let square = DVec2::new(30.0, 30.0) + DVec2::new(20.0, -20.0);
    let read = sketch
        .angle_as_typed(anchor, None, square)
        .expect("the anchor is there");
    assert!(
        (read.abs() - 90.0).abs() < TOLERANCE,
        "square to it reads {read}°"
    );
}

#[test]
fn a_trait_started_where_two_traits_end_reads_its_angle_against_the_horizontal() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::ZERO);
    let east = sketch.add_point(DVec2::new(40.0, 0.0));
    let north = sketch.add_point(DVec2::new(0.0, 40.0));
    sketch.add_segment(corner, east);
    sketch.add_segment(north, corner);
    let anchor = ChainAnchor::Point(corner);

    assert_eq!(sketch.drawn_on_from(anchor, None), None);
    let read = sketch
        .angle_as_typed(anchor, None, DVec2::new(-20.0, -20.0))
        .expect("the anchor is there");
    assert!(
        (read + 135.0).abs() < TOLERANCE,
        "against the horizontal it reads {read}°"
    );
    let aimed = sketch.aim(anchor, None, DVec2::new(5.0, 30.0), angle(30.0), 1.0);
    assert!(
        runs(&sketch, anchor, aimed.position).distance(DVec2::from_angle(30.0_f64.to_radians()))
            < TOLERANCE,
        "30 typed runs at 30° off the horizontal: {}",
        aimed.position,
    );
}

#[test]
fn a_point_with_no_trait_ending_on_it_is_drawn_on_from_nothing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let alone = sketch.add_point(DVec2::new(5.0, 5.0));

    assert_eq!(sketch.drawn_on_from(ChainAnchor::Point(alone), None), None);
    assert_eq!(
        sketch.drawn_on_from(ChainAnchor::Pending(DVec2::ZERO), None),
        None
    );
}
