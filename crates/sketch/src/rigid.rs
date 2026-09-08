//! The parts of a drawing that keep their shape while the rest of it settles.
//!
//! A correction is written for loose points, one coordinate at a time. Here it
//! is read as a movement of whole shapes: each one is carried and turned, never
//! bent.

use glam::DVec2;

use crate::sketch::PointId;

/// Which block answers for each point: the first one holding it. A point two
/// blocks share is the hinge between them, and only its own block moves it.
pub(crate) fn ownership(blocks: &[Block], points: usize) -> Vec<Option<usize>> {
    let mut owner = vec![None; points];
    for (index, block) in blocks.iter().enumerate() {
        for point in &block.points {
            owner[point.0].get_or_insert(index);
        }
    }
    owner
}

/// A part of a drawing that keeps its shape while the rest of it settles.
pub(crate) struct Block {
    pub(crate) points: Vec<PointId>,
    /// Whether it holds a point that cannot move, in which case it is turned
    /// about that point rather than carried along.
    pub(crate) anchored: bool,
}

/// Takes a correction meant for loose points and makes it a movement of
/// whole blocks: each one is carried and turned, never bent.
///
/// A block hinged on a point another block already answers for turns about
/// that point and follows it, which is what makes a shape swing round a
/// corner instead of stretching away from it. Doing this to every step,
/// rather than tidying up afterwards, is what makes the drawing settle: a
/// correction spread over the points and then straightened out again is a
/// correction mostly thrown away.
///
/// Two hinges leave the block nothing to decide: where they go says how far it
/// turns as well as where it lands, and the best it can do is the rigid
/// movement that comes closest to both. One hinge fixes the landing and leaves
/// the turn to what the block's own points are asking for; none leaves both.
pub(crate) fn rigidify(
    positions: &[DVec2],
    moves: &mut [DVec2],
    blocks: &[Block],
    owner: &[Option<usize>],
    pinned: &[bool],
    scale: f64,
) {
    for (index, block) in blocks.iter().enumerate() {
        let held = |point: &PointId| pinned[point.0] || owner[point.0] != Some(index);
        let hinges = block.points.iter().filter(|point| held(point)).count();

        let followed = |point: &PointId| hinges == 0 || held(point);
        let share = match hinges {
            0 => block.points.len(),
            _ => hinges,
        } as f64;
        let mut center = DVec2::ZERO;
        let mut carried = DVec2::ZERO;
        for point in block.points.iter().filter(|point| followed(point)) {
            center += positions[point.0];
            carried += moves[point.0];
        }
        let (center, carried) = (center / share, carried / share);

        let (mut torque, mut spread) = (0.0, 0.0);
        for point in &block.points {
            if hinges > 1 && !held(point) {
                continue;
            }
            let arm = positions[point.0] - center;
            torque += arm.perp_dot(moves[point.0] - carried);
            spread += arm.length_squared();
        }
        // Arms, so a size squared: below a millionth of the drawing there is
        // nothing to turn about, whatever the units are called.
        let flat = scale * scale * 1e-12;
        let turn = if spread > flat { torque / spread } else { 0.0 };
        // A tangent, not an angle: laid on the perpendicular it stretches every arm.
        let spin = DVec2::from_angle(turn.atan());
        for point in &block.points {
            if held(point) {
                continue;
            }
            let arm = positions[point.0] - center;
            moves[point.0] = carried + spin.rotate(arm) - arm;
        }
    }
}

#[cfg(test)]
mod tests {
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
                (landed(one).distance(landed(other)) - positions[one].distance(positions[other]))
                    .abs()
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
}
