//! When a sweep has stopped getting anywhere.
//!
//! A drawing asked for what it cannot have — a point held beyond its shape's
//! reach — stops gaining after a few dozen corrections, and would otherwise
//! spend every one it is allowed for nothing. A drag asks that of the solver
//! many times a frame while the hand is past the shape's reach; that is where
//! the time went.

/// How many corrections a sweep is watched over between two looks.
const WINDOW: usize = 25;
/// How much of its smallest error a sweep must clear in that time to go on.
/// One that is getting there, however slowly, clears far more than this; one
/// clearing less would not settle within the corrections it has left anyway.
const GAINING: f64 = 0.999;

/// The smallest error a sweep has had, now and at the last look.
pub(super) struct Stall {
    least: f64,
    least_before: f64,
}

impl Stall {
    pub(super) fn new() -> Self {
        Self {
            least: f64::INFINITY,
            least_before: f64::INFINITY,
        }
    }

    /// Whether the sweep, `iteration` corrections in with `worst` left, has
    /// stopped gaining.
    pub(super) fn stalled(&mut self, iteration: usize, worst: f64) -> bool {
        self.least = self.least.min(worst);
        if !iteration.is_multiple_of(WINDOW) {
            return false;
        }
        let stalled = self.least > self.least_before * GAINING;
        self.least_before = self.least;
        stalled
    }
}

#[cfg(test)]
mod tests;
