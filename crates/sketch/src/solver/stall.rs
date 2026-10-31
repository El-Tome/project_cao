//! When a sweep has stopped getting anywhere, or gets there too slowly to be
//! waited for.
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

/// Keeping more than this of its smallest error over a window, a sweep still
/// gaining is creeping: settling the way it does, it would want thousands of
/// corrections more, and where it is heading is better read off its last
/// passes (#592).
const CREEPING: f64 = 0.8;

/// The smallest error a sweep has had, now and at the last look, and how the
/// last look found it going.
pub(super) struct Stall {
    least: f64,
    least_before: f64,
    creeping: bool,
}

impl Stall {
    pub(super) fn new() -> Self {
        Self {
            least: f64::INFINITY,
            least_before: f64::INFINITY,
            creeping: false,
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
        self.creeping = !stalled && self.least > self.least_before * CREEPING;
        self.least_before = self.least;
        stalled
    }

    /// Whether the last look found the sweep still gaining, but by less than
    /// a fifth of its error.
    pub(super) fn creeping(&self) -> bool {
        self.creeping
    }
}

#[cfg(test)]
mod tests;
