//! Which points of a drawing can no longer move.
//!
//! A drawing is rarely all-or-nothing: one contour can be nailed down while
//! another is still floating beside it, and saying so per point is what tells
//! the user what is left to do.

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
    pub fn settled_points(&self, millimeters_per_unit: f64) -> Vec<bool> {
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
mod tests {
    use glam::DVec2;

    use crate::constraints::{DimensionTarget, SketchAxis};
    use crate::plane::WorkPlane;
    use crate::sketch::Sketch;

    /// One contour nailed down beside another still floating: the drawing has
    /// to say so per element, not give one verdict for the whole.
    #[test]
    fn part_of_a_drawing_can_be_settled_while_the_rest_floats() {
        let mut sketch = Sketch::new(WorkPlane::XY);

        let held = sketch.add_point(DVec2::new(20.0, 0.0));
        let fixed = sketch.add_segment(Sketch::ORIGIN, held);
        sketch.set_dimension(DimensionTarget::Length(fixed), 20.0, false);
        sketch.set_dimension(
            DimensionTarget::AxisAngle {
                segment: fixed,
                axis: SketchAxis::U,
            },
            0.0,
            false,
        );

        let loose_a = sketch.add_point(DVec2::new(50.0, 50.0));
        let loose_b = sketch.add_point(DVec2::new(70.0, 50.0));
        sketch.add_segment(loose_a, loose_b);

        let settled = sketch.settled_points(1.0);

        assert!(settled[Sketch::ORIGIN.0], "the origin never moves");
        assert!(settled[held.0], "held by a length and a direction");
        assert!(!settled[loose_a.0], "nothing holds this one");
        assert!(!settled[loose_b.0]);
        assert!(!sketch.is_fully_constrained(1.0));
    }

    /// A length from the origin is enough on its own: the drawing keeps the
    /// direction it was drawn in, so the far end has nowhere left to go.
    #[test]
    fn a_length_from_the_origin_settles_its_end() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let end = sketch.add_point(DVec2::new(20.0, 0.0));
        let segment = sketch.add_segment(Sketch::ORIGIN, end);
        sketch.set_dimension(DimensionTarget::Length(segment), 20.0, false);

        assert!(sketch.settled_points(1.0)[end.0]);
    }

    /// A shape drawn beside another must not stop it from being settled: each
    /// group of joined geometry keeps the direction it was drawn in on its own.
    #[test]
    fn a_loose_shape_beside_a_measured_one_leaves_it_settled() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corner = sketch.add_point(DVec2::new(80.0, 0.0));
        let side = sketch.add_segment(Sketch::ORIGIN, corner);
        sketch.set_dimension(DimensionTarget::Length(side), 80.0, false);

        let loose_a = sketch.add_point(DVec2::new(200.0, 200.0));
        let loose_b = sketch.add_point(DVec2::new(260.0, 200.0));
        sketch.add_segment(loose_a, loose_b);

        let settled = sketch.settled_points(1.0);
        assert!(
            settled[corner.0],
            "held by a length and the way it was drawn"
        );
        assert!(!settled[loose_a.0]);
    }

    /// A length that hangs off nothing fixed still swings freely.
    #[test]
    fn a_length_away_from_the_origin_leaves_its_end_free() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let anchor = sketch.add_point(DVec2::new(30.0, 30.0));
        let end = sketch.add_point(DVec2::new(50.0, 30.0));
        let segment = sketch.add_segment(anchor, end);
        sketch.set_dimension(DimensionTarget::Length(segment), 20.0, false);

        assert!(!sketch.settled_points(1.0)[end.0]);
    }
}
