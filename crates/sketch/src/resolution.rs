//! How finely the drawing holds its rules.

use crate::sketch::Sketch;
use crate::solver::TOLERANCE;

impl Sketch {
    /// The distance below which the drawing cannot tell two places apart.
    ///
    /// The solver counts a rule as kept once it holds within this much, so a
    /// trait given "Parallèle" may still lean by it, and one laid on an axis
    /// may still stand that far off. Whatever reads the drawing as exact has to
    /// forgive at least this, or it reads the solver's leftovers as geometry.
    pub fn resolution(&self) -> f64 {
        TOLERANCE * self.characteristic_size()
    }
}

#[cfg(test)]
mod tests;
