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

use super::{PointId, Sketch};

impl Sketch {
    /// Whether something outside the drawing holds this point still.
    pub fn is_anchored(&self, point: PointId) -> bool {
        self.anchored.contains(&point)
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
}
