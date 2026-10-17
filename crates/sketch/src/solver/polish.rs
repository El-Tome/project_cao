//! The last stretch to the stop line, walked all at once.
//!
//! Projection corrects one equation at a time, and on a stiff drawing — a
//! slot's caps held tangent to its flats — the corrections start undoing each
//! other: the error keeps falling, but by so little per sweep that the stop
//! line would cost thousands of them. From there Newton's step is the right
//! tool: every equation is linearised about the shape as it stands and the
//! one correction that cancels them together is taken, which converges in a
//! few steps where projection crawls.

use glam::DVec2;

use crate::equation::Equation;
use crate::precision::SOLVED;
use crate::sketch::{PointId, Sketch};

/// How many Newton steps the polish may take. Near the solution each one
/// squares the error, so a polish that has not landed in this many is not
/// converging and is left where projection put it.
const STEPS: usize = 8;

/// How far off the drawing may stand, as a share of its size, and still be
/// worth polishing: past it the drawing is contradicted or far from shape,
/// and a Newton step there can leap somewhere worse.
const POLISHABLE: f64 = 1e-4;

/// The longest step any unknown may take, as a share of the drawing's size.
/// A projection leftover asks for far less; a longer step is the mark of a
/// flat or contradicted system, and is refused whole.
const LONGEST_STEP: f64 = 1e-4;

/// A share of the normal matrix's largest diagonal, added to the whole
/// diagonal so that a system flat in some direction still solves.
const DAMPING: f64 = 1e-12;

/// Up to this many equations, the polish is afforded on every solve of a
/// drag, which runs on each pointer move: its cost is the cube of the
/// equations where a sweep's is linear in them. A larger drawing is polished
/// once the hand lets go, when the drop solves again without it.
const AFFORDED_UNDER_THE_HAND: usize = 128;

impl Sketch {
    /// Polishes the drawing to the stop line after projection has stopped
    /// gaining, and says how far off the worst equation now stands. The
    /// drawing is never left worse than it was handed over.
    pub(super) fn polish(&mut self, millimeters_per_unit: f64, scale: f64) -> f64 {
        let mut worst = self.worst_error(millimeters_per_unit, scale);
        let held = (0..self.points().len()).any(|index| self.is_held_still(PointId(index)));
        if held && self.equation_count() > AFFORDED_UNDER_THE_HAND {
            return worst;
        }
        for _ in 0..STEPS {
            if worst < SOLVED || worst >= POLISHABLE {
                return worst;
            }
            let before: Vec<DVec2> = self.points().to_vec();
            let sizes: Vec<f64> = self.circles().iter().map(|circle| circle.radius).collect();
            if !self.newton_step(millimeters_per_unit, scale) {
                return worst;
            }
            let polished = self.worst_error(millimeters_per_unit, scale);
            // Anything but a clean improvement is reverted: `>=` would read a
            // NaN as an improvement, and a poisoned step must be the first
            // thing this net catches.
            if polished.partial_cmp(&worst) != Some(std::cmp::Ordering::Less) {
                for (index, place) in before.iter().enumerate() {
                    self.place_point(PointId(index), *place);
                }
                for (index, radius) in sizes.iter().enumerate() {
                    self.set_circle_radius(crate::sketch::CircleId(index), *radius);
                }
                return worst;
            }
            worst = polished;
        }
        worst
    }

    /// One Newton step over every equation at once: the smallest movement of
    /// the unknowns that cancels them all, as the normal equations give it.
    /// Each row is held the way a sweep holds it — a point laid on a curve
    /// follows the curve, and never the other way round — and a step longer
    /// than what a projection leftover can ask for is a leap off a flat
    /// system, refused whole.
    fn newton_step(&mut self, millimeters_per_unit: f64, scale: f64) -> bool {
        let pinned = self.pinned_points();
        let pulled = self.pulled_elsewhere(millimeters_per_unit, &pinned);
        let mut equations: Vec<Equation> = Vec::new();
        for index in 0..self.equation_count() {
            let written = equations.len();
            self.any_equation(index, millimeters_per_unit, &pinned, &mut equations);
            let held_alone = self.held_alone(index, &pinned, &pulled);
            for equation in &mut equations[written..] {
                if let Some(point) = held_alone {
                    equation.hold_to(point);
                }
                self.hold_sizes(equation);
            }
        }
        if equations.is_empty() {
            return false;
        }
        let normal = normal_matrix(&equations);
        let errors: Vec<f64> = equations.iter().map(|equation| -equation.error).collect();
        let Some(weights) = solved_by_elimination(normal, errors) else {
            return false;
        };

        let variables = self.variables();
        let mut correction = vec![0.0; variables];
        for (equation, weight) in equations.iter().zip(&weights) {
            for (column, value) in equation.gradient.iter().enumerate() {
                correction[column] += value * weight;
            }
        }
        let longest = correction
            .iter()
            .fold(0.0, |far: f64, step| far.max(step.abs()));
        if !longest.is_finite() || longest > scale * LONGEST_STEP {
            return false;
        }
        for point in 0..self.points().len() {
            let delta = DVec2::new(correction[point * 2], correction[point * 2 + 1]);
            if delta != DVec2::ZERO {
                self.translate_point(PointId(point), delta);
            }
        }
        for circle in 0..self.circles().len() {
            let delta = correction[self.points().len() * 2 + circle];
            if delta != 0.0 {
                self.grow_circle(crate::sketch::CircleId(circle), delta);
            }
        }
        true
    }
}

/// The normal matrix of the system: how much every pair of equations pulls on
/// the same unknowns, damped along its diagonal.
fn normal_matrix(equations: &[Equation]) -> Vec<Vec<f64>> {
    let count = equations.len();
    let mut normal = vec![vec![0.0; count]; count];
    for row in 0..count {
        for column in row..count {
            let shared: f64 = equations[row]
                .gradient
                .iter()
                .zip(&equations[column].gradient)
                .map(|(one, other)| one * other)
                .sum();
            normal[row][column] = shared;
            normal[column][row] = shared;
        }
    }
    let largest = (0..count).map(|at| normal[at][at]).fold(0.0, f64::max);
    for (at, row) in normal.iter_mut().enumerate() {
        row[at] += largest * DAMPING + f64::MIN_POSITIVE;
    }
    normal
}

/// Gaussian elimination with partial pivoting, or `None` where the system is
/// too flat to answer.
fn solved_by_elimination(mut matrix: Vec<Vec<f64>>, mut rhs: Vec<f64>) -> Option<Vec<f64>> {
    let count = rhs.len();
    for column in 0..count {
        let pivot = (column..count).max_by(|a, b| {
            matrix[*a][column]
                .abs()
                .total_cmp(&matrix[*b][column].abs())
        })?;
        if matrix[pivot][column].abs() < f64::MIN_POSITIVE {
            return None;
        }
        matrix.swap(column, pivot);
        rhs.swap(column, pivot);
        for row in (column + 1)..count {
            let factor = matrix[row][column] / matrix[column][column];
            if factor == 0.0 {
                continue;
            }
            let (settled, open) = matrix.split_at_mut(row);
            let lead = &settled[column];
            for (ahead, value) in open[0].iter_mut().enumerate().skip(column) {
                *value -= factor * lead[ahead];
            }
            rhs[row] -= factor * rhs[column];
        }
    }
    let mut solution = vec![0.0; count];
    for row in (0..count).rev() {
        let settled: f64 = ((row + 1)..count)
            .map(|column| matrix[row][column] * solution[column])
            .sum();
        solution[row] = (rhs[row] - settled) / matrix[row][row];
    }
    Some(solution)
}
