//! When a sweep has stopped getting anywhere.
//!
//! A drawing asked for what it cannot have — a point held beyond its shape's
//! reach — stops gaining after a few dozen corrections, and would otherwise
//! spend every one it is allowed for nothing. A drag asks that of the solver
//! many times a frame while the hand is past the shape's reach; that is where
//! the time went. A solve's rounds are watched the same way.

use super::TOLERANCE;

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

/// How much of the error a round must clear to be worth another one.
const WORTH_ANOTHER_ROUND: f64 = 0.9;

/// How a solve's rounds are clearing its error.
pub(super) struct Rounds {
    before: f64,
    left: usize,
}

impl Rounds {
    pub(super) fn new(rounds: usize) -> Self {
        Self {
            before: f64::INFINITY,
            left: rounds,
        }
    }

    /// Whether another round is worth beginning, `error` left after this one.
    /// It must clear a tenth of what the last one left, and rounds clearing at
    /// that pace must bring it under the tolerance in the rounds that remain:
    /// a drawing asked for what it cannot have — a point held beyond the reach
    /// of the values around it — clears just enough each round to earn the
    /// next, and a drag asks that of it at every try (#592). A solve that would
    /// get there is never cut short.
    pub(super) fn worth_another(&mut self, error: f64) -> bool {
        self.left = self.left.saturating_sub(1);
        let before = std::mem::replace(&mut self.before, error);
        if !before.is_finite() {
            return true;
        }
        let pace = error / before;
        pace <= WORTH_ANOTHER_ROUND && error * pace.powi(self.left as i32) <= TOLERANCE
    }
}

#[cfg(test)]
mod tests;
