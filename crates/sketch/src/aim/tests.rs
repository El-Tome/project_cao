//! What sketch · aim.rs is held to.

use super::*;
use crate::plane::WorkPlane;

const TOLERANCE: f64 = 1e-9;

fn chain_of_one(sketch: &mut Sketch) -> (SegmentId, DVec2) {
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let corner = sketch.add_point(DVec2::new(40.0, 0.0));
    (sketch.add_segment(start, corner), sketch.point(corner))
}

fn cursor_off_square(from: DVec2, degrees: f64) -> DVec2 {
    from + DVec2::from_angle((90.0 - degrees).to_radians()) * 100.0
}

#[test]
fn a_cursor_within_four_degrees_of_square_snaps_to_the_right_angle() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (previous, corner) = chain_of_one(&mut sketch);

    let aimed = sketch.aim(
        ChainAnchor::Pending(corner),
        Some(previous),
        cursor_off_square(corner, 3.0),
        LockedInput::default(),
        1.0,
    );

    assert!(
        (aimed.position.x - corner.x).abs() < TOLERANCE,
        "three degrees off square is square: expected x {}, got {}",
        corner.x,
        aimed.position.x,
    );
    assert_eq!(
        aimed.square_with,
        Some(previous),
        "the trait it was squared against is named, so a rule can be written"
    );
}

#[test]
fn a_cursor_eight_degrees_off_square_is_left_alone() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let (previous, corner) = chain_of_one(&mut sketch);
    let cursor = cursor_off_square(corner, 8.0);

    let aimed = sketch.aim(
        ChainAnchor::Pending(corner),
        Some(previous),
        cursor,
        LockedInput::default(),
        1.0,
    );

    assert!(
        aimed.position.distance(cursor) < TOLERANCE,
        "eight degrees off square is an angle meant as it was drawn: expected {cursor}, got {}",
        aimed.position,
    );
    assert_eq!(
        aimed.square_with, None,
        "nothing was squared up, so nothing is named"
    );
}

#[test]
fn a_locked_length_leaves_the_line_free_to_turn() {
    let sketch = Sketch::new(WorkPlane::XY);
    let from = DVec2::new(5.0, 5.0);
    let locked = LockedInput {
        first: Some(50.0),
        second: None,
    };

    let east = sketch.aim(
        ChainAnchor::Pending(from),
        None,
        from + DVec2::new(200.0, 0.0),
        locked,
        1.0,
    );
    let north = sketch.aim(
        ChainAnchor::Pending(from),
        None,
        from + DVec2::new(0.0, 200.0),
        locked,
        1.0,
    );

    for (aimed, way) in [(east, "east"), (north, "north")] {
        let reached = aimed.position.distance(from);
        assert!(
            (reached - 50.0).abs() < TOLERANCE,
            "a length typed holds however the cursor turns: {way} reached {reached}",
        );
    }
    assert!(
        east.position.distance(north.position) > 1.0,
        "the two still point different ways: {} and {}",
        east.position,
        north.position,
    );
}

#[test]
fn a_locked_angle_follows_the_side_the_cursor_is_on() {
    let sketch = Sketch::new(WorkPlane::XY);
    let from = DVec2::ZERO;
    let locked = LockedInput {
        first: None,
        second: Some(30.0),
    };
    let thirty = DVec2::from_angle(30.0_f64.to_radians());

    let pointing_up = sketch.aim(
        ChainAnchor::Pending(from),
        None,
        thirty * 100.0,
        locked,
        1.0,
    );
    let pointing_down = sketch.aim(
        ChainAnchor::Pending(from),
        None,
        thirty * -100.0,
        locked,
        1.0,
    );

    assert!(
        pointing_up.position.normalize().distance(thirty) < TOLERANCE,
        "pointing up the line, thirty degrees is the one above: {}",
        pointing_up.position,
    );
    assert!(
        pointing_down.position.normalize().distance(-thirty) < TOLERANCE,
        "pointing the other way, it is the thirty degrees the cursor is on: {}",
        pointing_down.position,
    );
}

#[test]
fn a_symmetric_trait_grows_equally_on_both_sides_of_its_middle() {
    let sketch = Sketch::new(WorkPlane::XY);
    let middle = DVec2::new(10.0, 0.0);

    let (start, end) = sketch.symmetric_ends(
        middle,
        middle + DVec2::new(30.0, 0.0),
        LockedInput::default(),
        1.0,
    );

    assert!(
        middle.distance((start + end) * 0.5) < TOLERANCE,
        "the middle should sit halfway between the two ends: got {start} and {end}",
    );
    assert!((end.x - 40.0).abs() < TOLERANCE, "end = {end}");
    assert!((start.x + 20.0).abs() < TOLERANCE, "start = {start}");
}

#[test]
fn a_length_typed_for_a_symmetric_trait_is_the_distance_to_the_edge_it_aims_at() {
    let sketch = Sketch::new(WorkPlane::XY);
    let middle = DVec2::ZERO;
    let locked = LockedInput {
        first: Some(50.0),
        second: None,
    };

    let (start, end) = sketch.symmetric_ends(middle, DVec2::new(200.0, 0.0), locked, 1.0);

    assert!(
        (middle.distance(end) - 50.0).abs() < TOLERANCE,
        "50 typed reaches the edge, the way the plain line tool reads its own anchor: got {}",
        middle.distance(end),
    );
    assert!(
        (start.distance(end) - 100.0).abs() < TOLERANCE,
        "the far side mirrors it, so the whole trait spans twice as much: got {}",
        start.distance(end),
    );
}

#[test]
fn a_side_typed_fixes_only_that_side_of_the_rectangle() {
    let start = DVec2::ZERO;
    let locked = LockedInput {
        first: Some(40.0),
        second: None,
    };

    let dragged_right = rectangle_corner(start, DVec2::new(120.0, -80.0), locked, 1.0);
    let dragged_left = rectangle_corner(start, DVec2::new(-120.0, -80.0), locked, 1.0);

    assert!(
        (dragged_right.x - 40.0).abs() < TOLERANCE && (dragged_right.y + 80.0).abs() < TOLERANCE,
        "the width typed holds and the height still follows the cursor: {dragged_right}",
    );
    assert!(
        (dragged_left.x + 40.0).abs() < TOLERANCE,
        "forty typed means forty the way the user is dragging: {dragged_left}",
    );
}
