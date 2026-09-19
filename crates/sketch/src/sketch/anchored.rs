//! The points of a drawing that something outside it holds still.
//!
//! A point dropped on a corner of the part is held there: neither coordinate
//! can move, the drawing reads it as settled, and what is left to pin is
//! reckoned without it. The part moves, and the point goes with it.
//!
//! This is not a rule of the drawing. It is not laid by hand, does not show
//! among the rules and cannot be deleted like one — it is worked out again at
//! every replay from what the part then is, and so is never saved. A rule the
//! user can delete would have to be deleted by hand every time a drawing came
//! to stand on its own; this one retires by itself.

use crate::constraints::Freedom;
use crate::length::LengthOutcome;

/// How far a rule may be from holding and still count as held. The solver
/// stops at this, so asking for less would call its own answers failures.
const HONOURED: f64 = 1e-5;

use super::{PointId, Sketch};

impl Sketch {
    /// Whether something outside the drawing holds this point still.
    pub fn is_anchored(&self, point: PointId) -> bool {
        self.anchored.contains(&point)
    }

    /// Whether the solver may move this point at all.
    ///
    /// The drawing's own origin never moves; a point under the cursor does
    /// not give while the gesture lasts; and a point the part holds stays
    /// where the part puts it. The solver has to read the same list the
    /// reckoning does, or a drawing settles one way and reports another.
    pub(crate) fn is_held_where_it_is(&self, point: PointId) -> bool {
        self.is_origin(point) || self.is_held_still(point) || self.is_anchored(point)
    }

    /// Holds a point where the part puts it.
    ///
    /// What is remembered about which points have settled answers to
    /// `out_of_play`, which this changes, so nothing has to be forgotten
    /// here.
    pub fn hold_on_the_part(&mut self, point: PointId) {
        self.anchored.insert(point);
    }

    /// Lets go of every such hold, which is what a drawing its own rules
    /// already determine does with all but the one at its origin.
    pub fn let_the_part_go(&mut self) {
        self.anchored.clear();
    }

    /// Whether the part holds any of this drawing's points at all.
    pub fn is_held_by_the_part(&self) -> bool {
        !self.anchored.is_empty()
    }

    /// How far the drawing is from honouring its own rules: the worst of
    /// them, against the size of the drawing so that it means the same thing
    /// whatever the part is measured in. Zero when every rule holds.
    ///
    /// The solver reads this as it works. Read from outside, it answers a
    /// question the outcome cannot: a system with nothing left free to move
    /// has nothing to solve and reports itself done, however badly its rules
    /// are broken — which is exactly a drawing two corners of the part hold
    /// apart.
    pub fn off_by(&self, millimeters_per_unit: f64) -> f64 {
        self.worst_error(millimeters_per_unit, self.characteristic_size())
    }

    /// What the drawing's own rules leave free, with nothing held from
    /// outside it.
    pub fn freedom_alone(&self, millimeters_per_unit: f64) -> Freedom {
        let mut alone = self.clone();
        alone.let_the_part_go();
        alone.freedom(millimeters_per_unit)
    }

    /// Settles the drawing, letting go of what the part holds when holding it
    /// is what stops the drawing's own rules from being true.
    ///
    /// A hold is a convenience, never an authority. Two of them and a value
    /// between them can ask for something impossible the moment the part
    /// changes shape, and a drawing that cannot settle is a drawing whose
    /// dimensions scatter and whose corners come apart. It gives instead, and
    /// keeps the shape it was given.
    ///
    /// It gives just as readily when the drawing needs no help: a drawing its
    /// own rules already determine is dragged about by a hold rather than
    /// held by it.
    ///
    /// Answers whether it let go, which is what the tree says out loud.
    pub fn settle_with_the_part(&mut self, millimeters_per_unit: f64) -> (LengthOutcome, bool) {
        if !self.is_held_by_the_part() {
            return (self.resolve(millimeters_per_unit), false);
        }
        if self.freedom_alone(millimeters_per_unit).fully_constrained() {
            self.let_the_part_go();
            return (self.resolve(millimeters_per_unit), true);
        }
        let settled = self.resolve(millimeters_per_unit);
        if self.off_by(millimeters_per_unit) <= HONOURED {
            return (settled, false);
        }
        self.let_the_part_go();
        (self.resolve(millimeters_per_unit), true)
    }
}

#[cfg(test)]
mod tests;
