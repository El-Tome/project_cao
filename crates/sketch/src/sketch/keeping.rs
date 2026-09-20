//! Everything a settling may move, kept whole and given back whole.
//!
//! A gesture the drawing cannot take is refused rather than left half-way: the
//! shape goes back to what it was, which asks for somewhere to have kept it.
//! Points and circles are the whole of it — a trait, an arc and a rule are
//! drawn from points, and a circle is the one shape with a size of its own.

use glam::DVec2;

use super::{Circle, Sketch};

impl Sketch {
    pub(crate) fn shapes_now(&self) -> (Vec<DVec2>, Vec<Circle>) {
        (self.points.clone(), self.circles.clone())
    }

    pub(crate) fn give_back(&mut self, kept: (Vec<DVec2>, Vec<Circle>)) {
        (self.points, self.circles) = kept;
    }
}
