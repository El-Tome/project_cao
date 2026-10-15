//! One pass of a sweep: every row of the system corrected in turn, the points
//! it speaks of moved along its gradient, and the sizes of the circles with
//! them — and, once a pass is done, whether the sweep is.

use glam::DVec2;

use crate::equation::Equation;
use crate::rigid::{Block, ownership, rigidify};
use crate::sketch::{CircleId, PointId, Sketch};

use super::heading::Heading;
use super::stall::Stall;
use super::{SETTLED_ALREADY, TOLERANCE};

/// What a sweep holds from one pass to the next: the points that stay, those
/// something else than a rule holding them pulls on, the shapes carried whole
/// and which of them each point belongs to, the buffers a correction is
/// written into — the same size on every turn, so the sweep owns them rather
/// than asking for them again — and how its passes have been going.
pub(super) struct Sweep<'a> {
    pinned: Vec<bool>,
    pulled: Vec<bool>,
    blocks: &'a [Block],
    owner: Vec<Option<usize>>,
    moves: Vec<DVec2>,
    entry: Vec<Equation>,
    stall: Stall,
    heading: Heading,
}

impl Sketch {
    /// What a sweep of the drawing as it stands holds, `blocks` carried whole.
    pub(super) fn sweep<'a>(&self, blocks: &'a [Block], millimeters_per_unit: f64) -> Sweep<'a> {
        let pinned = self.pinned_points();
        let pulled = self.pulled_elsewhere(millimeters_per_unit, &pinned);
        Sweep {
            pinned,
            pulled,
            blocks,
            owner: ownership(blocks, self.points().len()),
            moves: vec![DVec2::ZERO; self.points().len()],
            entry: Vec::new(),
            stall: Stall::new(),
            heading: Heading::new(),
        }
    }

    /// Whether the sweep is done after its pass `iteration` left `worst`:
    /// polished, or no longer gaining. Otherwise a sweep found creeping is
    /// taken where its passes are heading.
    pub(super) fn swept(
        &mut self,
        sweep: &mut Sweep,
        iteration: usize,
        worst: f64,
        scale: f64,
    ) -> bool {
        // Held, it is polished while it gains: the graph welds finer (#545).
        if worst < TOLERANCE * SETTLED_ALREADY || sweep.stall.stalled(iteration, worst) {
            return true;
        }
        let creeping = sweep.stall.creeping();
        if sweep.blocks.is_empty()
            && sweep
                .heading
                .passed(self, worst, creeping, &sweep.pinned, scale)
        {
            // The drawing put back is further from settled than the mixing
            // had brought it: the least error the watch remembers is no
            // longer the drawing's, and would stop its passes at once.
            sweep.stall = Stall::new();
        }
        false
    }

    /// One pass: every row corrected in turn, just far enough to cancel its
    /// error. Says the worst error a row had when its turn came.
    pub(super) fn pass(&mut self, sweep: &mut Sweep, millimeters_per_unit: f64, scale: f64) -> f64 {
        sweep.heading.starting(self);
        let mut worst: f64 = 0.0;
        let mut entry = std::mem::take(&mut sweep.entry);
        for index in 0..self.equation_count() {
            let held_alone = self.held_alone(index, &sweep.pinned, &sweep.pulled);
            self.any_equation(index, millimeters_per_unit, &sweep.pinned, &mut entry);
            for equation in entry.iter_mut() {
                let off = equation.off_by(scale);
                worst = worst.max(off);
                if off < TOLERANCE * SETTLED_ALREADY {
                    continue;
                }
                // A point laid on a curve follows it: the step this row
                // takes moves the point, and leaves the curve alone.
                if let Some(point) = held_alone {
                    equation.hold_to(point);
                }
                self.hold_sizes(equation);

                let norm = equation.norm_squared();
                if norm < equation.flat_below(scale) {
                    continue;
                }
                self.correct(equation, -equation.error / norm, sweep, scale);
            }
            // The gradients are handed back once a row is corrected: the next
            // row wants a buffer of the very same size.
            for equation in entry.drain(..) {
                equation.recycle();
            }
        }
        sweep.entry = entry;
        worst
    }

    /// Moves the drawing `step` along the gradient of `equation`: pinned
    /// points stay, and the shapes the sweep carries are carried whole.
    ///
    /// With no shape carried, a row moves only the points it speaks of — a
    /// handful, where the drawing may hold hundreds — and the others are not
    /// walked at all. Correcting row after row is where a sweep spends its
    /// time.
    fn correct(&mut self, equation: &Equation, step: f64, sweep: &mut Sweep, scale: f64) {
        let points = self.points().len();
        if sweep.blocks.is_empty() {
            for (point, held) in sweep.pinned.iter().enumerate() {
                let (x, y) = (
                    equation.gradient[point * 2],
                    equation.gradient[point * 2 + 1],
                );
                if !held && (x != 0.0 || y != 0.0) {
                    self.translate_point(PointId(point), DVec2::new(x * step, y * step));
                }
            }
        } else {
            for (point, delta) in sweep.moves.iter_mut().enumerate() {
                *delta = match sweep.pinned[point] {
                    true => DVec2::ZERO,
                    false => DVec2::new(
                        equation.gradient[point * 2] * step,
                        equation.gradient[point * 2 + 1] * step,
                    ),
                };
            }
            rigidify(
                self.points(),
                &mut sweep.moves,
                sweep.blocks,
                &sweep.owner,
                &sweep.pinned,
                scale,
            );
            for (point, delta) in sweep.moves.iter().copied().enumerate() {
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
