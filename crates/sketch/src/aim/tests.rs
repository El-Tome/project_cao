//! What sketch · aim.rs is held to.
//!
//! Closes #560.
//! - an angle typed for a trait drawn from nowhere is the angle drawn, wherever
//!   the mouse is: -90 goes down with the mouse above the start —
//!   `a_locked_angle_is_the_angle_drawn_wherever_the_cursor_is`
//! - the mouse still gives the length when none is typed —
//!   `a_locked_angle_takes_its_length_from_the_cursor_on_either_side`
//! - a symmetric line drawn at a typed angle runs at that angle —
//!   `a_symmetric_trait_runs_at_the_angle_typed_wherever_the_cursor_is`
//! - the reading laid with the trait measures what was typed — no test: held
//!   where the reading is laid, in
//!   `crates/app/src/screens/viewport/input/angle_arm/tests.rs`

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
fn a_locked_angle_is_the_angle_drawn_wherever_the_cursor_is() {
    let sketch = Sketch::new(WorkPlane::XY);
    for (typed, cursor) in [
        (-90.0, DVec2::new(0.0, 100.0)),
        (-90.0, DVec2::new(0.0, -100.0)),
        (30.0, DVec2::from_angle(210.0_f64.to_radians()) * 100.0),
        (30.0, DVec2::from_angle(30.0_f64.to_radians()) * 100.0),
    ] {
        let locked = LockedInput {
            first: None,
            second: Some(typed),
        };

        let aimed = sketch.aim(ChainAnchor::Pending(DVec2::ZERO), None, cursor, locked, 1.0);

        let wanted = DVec2::from_angle(f64::to_radians(typed));
        assert!(
            aimed.position.normalize().distance(wanted) < TOLERANCE,
            "{typed}° typed with the cursor at {cursor} runs towards {}",
            aimed.position,
        );
    }
}

#[test]
fn a_locked_angle_takes_its_length_from_the_cursor_on_either_side() {
    let sketch = Sketch::new(WorkPlane::XY);
    let locked = LockedInput {
        first: None,
        second: Some(-90.0),
    };

    for cursor in [DVec2::new(5.0, -60.0), DVec2::new(5.0, 60.0)] {
        let aimed = sketch.aim(ChainAnchor::Pending(DVec2::ZERO), None, cursor, locked, 1.0);

        assert!(
            aimed.position.distance(DVec2::new(0.0, -60.0)) < TOLERANCE,
            "the cursor at {cursor} reaches 60 along the line, and the trait ends at {}",
            aimed.position,
        );
    }
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
fn a_symmetric_trait_runs_at_the_angle_typed_wherever_the_cursor_is() {
    let sketch = Sketch::new(WorkPlane::XY);
    let middle = DVec2::new(10.0, 10.0);
    let locked = LockedInput {
        first: None,
        second: Some(30.0),
    };
    let thirty = DVec2::from_angle(30.0_f64.to_radians());

    let (start, end) = sketch.symmetric_ends(middle, middle - thirty * 50.0, locked, 1.0);

    assert!(
        (end - middle).normalize().distance(thirty) < TOLERANCE,
        "the end runs from the middle towards {}, not at 30°",
        end - middle,
    );
    assert!(
        (start - middle).normalize().distance(-thirty) < TOLERANCE,
        "the start mirrors it: {}",
        start - middle,
    );
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
