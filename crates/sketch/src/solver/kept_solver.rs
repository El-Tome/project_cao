//! The lines a drag keeps where they lie, as rows of the system.
//!
//! Each is linear: a direction kept is a trait whose far end stays on the line
//! through its near one, and a place kept is a point that stays on a line of
//! the plane. That is what lets the sweep lay them exactly, in one step each,
//! before any rule reads the shape they give it.

use crate::equation::Equation;
use crate::sketch::Sketch;
use crate::sketch::settling::Kept;

impl Sketch {
    /// Every line the drag under way keeps, appended to what the caller holds.
    pub(crate) fn kept_equations(&self, pinned: &[bool], into: &mut Vec<Equation>) {
        for at in 0..self.kept_lines().len() {
            self.kept_equation(at, pinned, into);
        }
    }

    pub(super) fn kept_equation(&self, at: usize, pinned: &[bool], into: &mut Vec<Equation>) {
        if let Some(line) = self.kept_lines().get(at) {
            into.extend(self.kept_row(line, pinned));
        }
    }

    /// One kept line as an equation, pinned points taking no share of it.
    pub(crate) fn kept_row(&self, line: &Kept, pinned: &[bool]) -> Option<Equation> {
        if line.to.0 >= self.points().len()
            || line.from.is_some_and(|from| from.0 >= self.points().len())
        {
            return None;
        }
        let from = line.from.map_or(glam::DVec2::ZERO, |from| self.point(from));
        let mut equation = Equation::new(self.variables());
        equation.error = (self.point(line.to) - from).dot(line.normal) - line.at;
        equation.add(line.to, line.normal);
        if let Some(from) = line.from {
            equation.add(from, -line.normal);
        }
        for (point, pinned) in pinned.iter().enumerate() {
            if *pinned {
                equation.gradient[point * 2] = 0.0;
                equation.gradient[point * 2 + 1] = 0.0;
            }
        }
        Some(equation)
    }
}
