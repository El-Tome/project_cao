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
pub(crate) fn rigidify(
    positions: &[DVec2],
    moves: &mut [DVec2],
    blocks: &[Block],
    owner: &[Option<usize>],
    pinned: &[bool],
) {
    for (index, block) in blocks.iter().enumerate() {
        let hinge = block
            .points
            .iter()
            .find(|point| pinned[point.0] || owner[point.0] != Some(index))
            .copied();
        let (center, carried) = match hinge {
            Some(point) => (positions[point.0], moves[point.0]),
            None => (
                block
                    .points
                    .iter()
                    .map(|point| positions[point.0])
                    .sum::<DVec2>()
                    / block.points.len() as f64,
                block
                    .points
                    .iter()
                    .map(|point| moves[point.0])
                    .sum::<DVec2>()
                    / block.points.len() as f64,
            ),
        };

        let (mut torque, mut spread) = (0.0, 0.0);
        for point in &block.points {
            let arm = positions[point.0] - center;
            torque += arm.perp_dot(moves[point.0] - carried);
            spread += arm.length_squared();
        }
        let turn = if spread > 1e-12 { torque / spread } else { 0.0 };
        // A tangent, not an angle: laid on the perpendicular it stretches every arm.
        let spin = DVec2::from_angle(turn.atan());
        for point in &block.points {
            if pinned[point.0] || owner[point.0] != Some(index) {
                continue;
            }
            let arm = positions[point.0] - center;
            moves[point.0] = carried + spin.rotate(arm) - arm;
        }
    }
}
