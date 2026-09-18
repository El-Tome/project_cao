//! Which points of a drawing can no longer move.
//!
//! A drawing is rarely all-or-nothing: one contour can be nailed down while
//! another is still floating beside it, and saying so per point is what tells
//! the user what is left to do.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::constraints::Freedom;
use crate::independence::{null_space, rank};
use crate::sketch::{PointId, Sketch};

impl Sketch {
    /// Coordinates that no constraint holds, worked out from the rank of the
    /// system: how many independent things the dimensions actually say.
    ///
    /// Points out of play are taken out of the count outright, since neither of
    /// their coordinates can move.
    pub fn freedom(&self, millimeters_per_unit: f64) -> Freedom {
        // Two coordinates per point, plus one size per circle. A **Fixe** is
        // not out of play: it holds a point while the drawing settles but
        // anchors it to nothing, and a figure it alone holds could be anywhere.
        let loose = (0..self.points().len())
            .filter(|index| !self.out_of_play(PointId(*index)))
            .count();
        let unknowns = loose * 2 + self.live_circles().count();
        let held = rank(&self.anchored_system(millimeters_per_unit)).min(unknowns);

        Freedom {
            degrees_of_freedom: unknowns - held,
        }
    }

    pub fn is_fully_constrained(&self, millimeters_per_unit: f64) -> bool {
        self.points().len() > 1 && self.freedom(millimeters_per_unit).fully_constrained()
    }

    /// Which points can no longer move at all.
    ///
    /// Read once per state of the drawing rather than once per frame: the
    /// painting asks for this on every image it draws, and the answer is a
    /// cubic pass over the whole sketch.
    pub fn settled_points(&self, millimeters_per_unit: f64) -> Vec<bool> {
        let drawing = self.print_of_what_is_read(millimeters_per_unit);
        if let Some(verdict) = self.settled_read_from(drawing) {
            return verdict;
        }

        let verdict = self.read_settled_points(millimeters_per_unit);
        self.remember_settled(drawing, verdict.clone());
        verdict
    }

    /// Everything the verdict answers to, in one number.
    ///
    /// The whole drawing, positions included, and not only its shapes: a trait
    /// lying square to the sketch is what lets its group keep the direction it
    /// was drawn in without being told, so moving one point can settle another.
    fn print_of_what_is_read(&self, millimeters_per_unit: f64) -> u64 {
        let mut print = DefaultHasher::new();
        millimeters_per_unit.to_bits().hash(&mut print);
        for point in self.points() {
            point.x.to_bits().hash(&mut print);
            point.y.to_bits().hash(&mut print);
        }
        for index in 0..self.points().len() {
            self.out_of_play(PointId(index)).hash(&mut print);
        }
        for (id, segment) in self.live_segments() {
            (id, segment.start, segment.end).hash(&mut print);
        }
        // Erased circles among them: a dead radius is still a column of the
        // system, and the count of columns is what the reading is shaped on.
        self.circles().len().hash(&mut print);
        for (id, circle) in self.live_circles() {
            (id, circle.center).hash(&mut print);
            circle.radius.to_bits().hash(&mut print);
        }
        for dimension in self.dimensions() {
            (dimension.target, dimension.driven).hash(&mut print);
            dimension.value.to_bits().hash(&mut print);
        }
        self.constraints().hash(&mut print);
        print.finish()
    }

    fn read_settled_points(&self, millimeters_per_unit: f64) -> Vec<bool> {
        let variables = self.variables();
        let pinned: Vec<bool> = (0..self.points().len())
            .map(|index| self.out_of_play(PointId(index)))
            .collect();
        let system = self.anchored_system(millimeters_per_unit);
        let free = null_space(&system, &pinned, variables);

        (0..self.points().len())
            .map(|index| {
                if pinned[index] {
                    return true;
                }
                // Settled means no way to move survives at this point.
                free.iter().all(|direction| {
                    direction[index * 2].abs() < 1e-3 && direction[index * 2 + 1].abs() < 1e-3
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests;
