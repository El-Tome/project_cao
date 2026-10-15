//! One row of the system corrected: the points it speaks of moved along its
//! gradient, and the sizes of the circles with them.

use glam::DVec2;

use crate::equation::Equation;
use crate::rigid::{Block, ownership, rigidify};
use crate::sketch::{CircleId, PointId, Sketch};

/// The shapes a sweep carries whole, which of them each point belongs to, and
/// the buffer a correction's moves are written into — the same size on every
/// turn, so the sweep owns it rather than asking for it again.
pub(super) struct Carried<'a> {
    blocks: &'a [Block],
    owner: Vec<Option<usize>>,
    moves: Vec<DVec2>,
}

impl<'a> Carried<'a> {
    pub(super) fn new(blocks: &'a [Block], points: usize) -> Self {
        Self {
            blocks,
            owner: ownership(blocks, points),
            moves: vec![DVec2::ZERO; points],
        }
    }
}

impl Sketch {
    /// Moves the drawing `step` along the gradient of `equation`: pinned
    /// points stay, and the shapes `carried` are carried whole.
    ///
    /// With no shape carried, a row moves only the points it speaks of — a
    /// handful, where the drawing may hold hundreds — and the others are not
    /// walked at all. Correcting row after row is where a sweep spends its
    /// time.
    pub(super) fn correct(
        &mut self,
        equation: &Equation,
        step: f64,
        pinned: &[bool],
        carried: &mut Carried,
        scale: f64,
    ) {
        let points = self.points().len();
        if carried.blocks.is_empty() {
            for (point, held) in pinned.iter().enumerate() {
                let (x, y) = (
                    equation.gradient[point * 2],
                    equation.gradient[point * 2 + 1],
                );
                if !held && (x != 0.0 || y != 0.0) {
                    self.translate_point(PointId(point), DVec2::new(x * step, y * step));
                }
            }
        } else {
            for (point, delta) in carried.moves.iter_mut().enumerate() {
                *delta = match pinned[point] {
                    true => DVec2::ZERO,
                    false => DVec2::new(
                        equation.gradient[point * 2] * step,
                        equation.gradient[point * 2 + 1] * step,
                    ),
                };
            }
            let Carried {
                blocks,
                owner,
                moves,
            } = carried;
            rigidify(self.points(), moves, blocks, owner, pinned, scale);
            for (point, delta) in moves.iter().copied().enumerate() {
                self.translate_point(PointId(point), delta);
            }
        }
        // And the sizes of the circles, which are unknowns of the same system.
        for circle in 0..self.circles().len() {
            let delta = equation.gradient[points * 2 + circle] * step;
            if delta != 0.0 {
                self.grow_circle(CircleId(circle), delta);
            }
        }
    }
}
