//! The parts of a drawing that keep their shape while the rest of it settles.
//!
//! A correction is written for loose points, one coordinate at a time. Here it
//! is read as a movement of whole shapes: each one is carried and turned, never
//! bent.

use glam::DVec2;

use crate::sketch::{PointId, Sketch};

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

impl Sketch {
    /// Puts every untouched block back to the shape it had, in the place the
    /// solve moved it to.
    ///
    /// The blocks are welded in order: what cannot move first, then each block
    /// hinged on the point it shares with those already in place. A block is
    /// only carried and turned, never bent. Returns how far the furthest point
    /// had to be put back, which is how the loop knows it has settled.
    pub(crate) fn weld(&mut self, before: &[DVec2], blocks: &[Block]) -> f64 {
        let mut placed = vec![false; self.points().len()];
        let pinned = self.pinned_points();
        let mut worst = 0.0f64;

        for Block { points, .. } in blocks {
            let hinge = points
                .iter()
                .find(|point| placed[point.0] || pinned[point.0])
                .copied();
            let (was, is) = match hinge {
                Some(point) => (before[point.0], self.point(point)),
                None => (
                    points.iter().map(|point| before[point.0]).sum::<DVec2>() / points.len() as f64,
                    points.iter().map(|point| self.point(*point)).sum::<DVec2>()
                        / points.len() as f64,
                ),
            };

            // The turn that best lines the shape up with where the solve put
            // it: nothing else about the block is allowed to change.
            let (mut across, mut along) = (0.0, 0.0);
            for point in points {
                let then = before[point.0] - was;
                let now = self.point(*point) - is;
                across += then.perp_dot(now);
                along += then.dot(now);
            }
            let turn = DVec2::from_angle(across.atan2(along));

            for point in points {
                if pinned[point.0] {
                    placed[point.0] = true;
                    continue;
                }
                let landing = is + turn.rotate(before[point.0] - was);
                worst = worst.max(landing.distance(self.point(*point)));
                self.place_point(*point, landing);
                placed[point.0] = true;
            }
        }
        worst
    }
}

#[cfg(test)]
mod tests;
