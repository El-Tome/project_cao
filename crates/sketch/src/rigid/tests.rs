//! What sketch · rigid.rs is held to.

use super::*;

/// A square, and a quarter of it turned about its lower left corner.
fn square() -> [DVec2; 4] {
    square_of(10.0)
}

fn square_of(side: f64) -> [DVec2; 4] {
    [
        DVec2::new(0.0, 0.0),
        DVec2::new(side, 0.0),
        DVec2::new(side, side),
        DVec2::new(0.0, side),
    ]
}

fn block_of(points: usize) -> Block {
    Block {
        points: (0..points).map(PointId).collect(),
        anchored: false,
    }
}

#[test]
fn a_block_held_by_two_hinges_turns_as_far_as_both_of_them_ask() {
    let positions = square();
    let spin = DVec2::from_angle(0.2_f64.atan());
    let turned = |point: DVec2| spin.rotate(point - positions[0]) + positions[0];

    // The two hinges belong to a block that has already been moved: one of
    // them stays where it is, the other is carried a fifth of its own arm.
    let mut moves = vec![DVec2::ZERO; 4];
    moves[1] = turned(positions[1]) - positions[1];
    let owner = vec![Some(1), Some(1), Some(0), Some(0)];

    rigidify(
        &positions,
        &mut moves,
        &[block_of(4)],
        &owner,
        &[false; 4],
        10.0,
    );

    for rank in [2, 3] {
        let landed = positions[rank] + moves[rank];
        let asked = turned(positions[rank]);
        assert!(
            landed.distance(asked) < 0.1,
            "corner {rank} landed at {landed} instead of {asked}, where \
             the two hinges together put it: the second one is read for \
             its pull and not for where it goes. The tenth of a unit \
             allowed here is the turn being taken as a tangent of itself, \
             which a sweep closes as it repeats.",
        );
    }
}

#[test]
fn a_block_held_by_one_hinge_follows_it_and_keeps_its_shape() {
    let positions = square();
    let mut moves = vec![DVec2::ZERO; 4];
    moves[0] = DVec2::new(3.0, 0.0);
    let owner = vec![Some(1), Some(0), Some(0), Some(0)];

    rigidify(
        &positions,
        &mut moves,
        &[block_of(4)],
        &owner,
        &[false; 4],
        10.0,
    );

    assert_eq!(
        moves[0],
        DVec2::new(3.0, 0.0),
        "the hinge is moved by the block that owns it, not by this one",
    );
    let landed = |rank: usize| positions[rank] + moves[rank];
    for (one, other) in [(1, 2), (2, 3), (1, 3)] {
        assert!(
            (landed(one).distance(landed(other)) - positions[one].distance(positions[other])).abs()
                < 1e-9,
            "the side from {one} to {other} was stretched",
        );
    }
}

#[test]
fn a_block_turns_whatever_the_units_of_the_drawing_are() {
    let side = 1e-6;
    let positions = square_of(side);
    let spin = DVec2::from_angle(0.2_f64.atan());
    let turned = |point: DVec2| spin.rotate(point - positions[0]) + positions[0];

    let mut moves = vec![DVec2::ZERO; 4];
    moves[1] = turned(positions[1]) - positions[1];
    let owner = vec![Some(1), Some(1), Some(0), Some(0)];

    rigidify(
        &positions,
        &mut moves,
        &[block_of(4)],
        &owner,
        &[false; 4],
        side,
    );

    for rank in [2, 3] {
        let landed = positions[rank] + moves[rank];
        let asked = turned(positions[rank]);
        assert!(
            landed.distance(asked) < side / 100.0,
            "corner {rank} landed at {landed} instead of {asked}: a drawing \
             this small has its arms read as no arms at all, and the block \
             is carried without being turned",
        );
    }
}
