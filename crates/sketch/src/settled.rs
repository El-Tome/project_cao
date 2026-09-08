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
mod tests {
    use glam::DVec2;

    use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
    use crate::plane::WorkPlane;
    use crate::sketch::{CircleId, Element, PointId, SegmentId, Sketch};

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

    /// Random enough to reach the edits a scripted drawing never makes, and
    /// repeatable so a failure can be looked at again.
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        fn range(&mut self, n: usize) -> usize {
            (self.next() % n as u64) as usize
        }
        fn coordinate(&mut self) -> f64 {
            (self.next() % 20_000) as f64 / 100.0 - 100.0
        }
    }

    fn live_segment(sketch: &Sketch, rng: &mut Rng) -> Option<SegmentId> {
        let live: Vec<SegmentId> = sketch.live_segments().map(|(id, _)| id).collect();
        (!live.is_empty()).then(|| live[rng.range(live.len())])
    }

    /// Hung off the origin and heavily measured, because a drawing that holds
    /// nothing has the same verdict whatever is done to it, and would let this
    /// test pass while reading nothing at all.
    #[test]
    fn a_drawing_edited_every_way_never_answers_from_a_reading_it_has_outlived() {
        for seed in 1..12u64 {
            let mut rng = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).max(1));
            let mut sketch = Sketch::new(WorkPlane::XY);
            let mut verdicts: Vec<Vec<bool>> = Vec::new();

            for step in 0..90 {
                match rng.range(9) {
                    0 | 1 => {
                        let onto = PointId(rng.range(sketch.points().len()));
                        let from = sketch.point(onto);
                        let step = 10.0 + rng.range(90) as f64;
                        let next = sketch.add_point(match rng.range(4) {
                            0 => from + DVec2::new(step, 0.0),
                            1 => from + DVec2::new(0.0, step),
                            _ => DVec2::new(rng.coordinate(), rng.coordinate()),
                        });
                        sketch.add_segment(onto, next);
                    }
                    2 | 3 => {
                        if let Some(segment) = live_segment(&sketch, &mut rng) {
                            let value = 10.0 + rng.range(90) as f64;
                            sketch.set_dimension(DimensionTarget::Length(segment), value, false);
                        }
                    }
                    7 => match rng.range(2) {
                        0 => {
                            let start = DVec2::new(rng.coordinate(), rng.coordinate());
                            let (a, b) = (
                                sketch.add_point(start),
                                sketch.add_point(start + DVec2::new(30.0, 0.0)),
                            );
                            sketch.add_segment(a, b);
                        }
                        _ => {
                            let live: Vec<CircleId> =
                                sketch.live_circles().map(|(id, _)| id).collect();
                            if !live.is_empty() {
                                let circle = live[rng.range(live.len())];
                                sketch.set_dimension(
                                    DimensionTarget::Radius(circle),
                                    5.0 + rng.range(40) as f64,
                                    false,
                                );
                            }
                        }
                    },
                    4 => {
                        if let Some(segment) = live_segment(&sketch, &mut rng) {
                            let axis = match rng.range(2) {
                                0 => SketchAxis::U,
                                _ => SketchAxis::V,
                            };
                            let angle = match rng.range(2) {
                                0 => (rng.range(4) * 90) as f64,
                                _ => rng.range(360) as f64,
                            };
                            sketch.set_dimension(
                                DimensionTarget::AxisAngle { segment, axis },
                                angle,
                                rng.range(6) == 0,
                            );
                        }
                    }
                    5 => {
                        let (first, second) = (
                            live_segment(&sketch, &mut rng),
                            live_segment(&sketch, &mut rng),
                        );
                        if let (Some(first), Some(second)) = (first, second)
                            && first != second
                        {
                            sketch.add_constraint(match rng.range(3) {
                                0 => Constraint::Perpendicular { first, second },
                                1 => Constraint::Parallel { first, second },
                                _ => Constraint::Equal { first, second },
                            });
                        }
                    }
                    6 => match rng.range(6) {
                        0 => {
                            let center = PointId(rng.range(sketch.points().len()));
                            sketch.add_circle(center, 5.0 + rng.range(40) as f64);
                        }
                        1 => {
                            if let Some(segment) = live_segment(&sketch, &mut rng) {
                                sketch.erase(Element::Segment(segment));
                            }
                        }
                        2 => {
                            if let Some(segment) = live_segment(&sketch, &mut rng) {
                                sketch.erase_dimension(DimensionTarget::Length(segment));
                            }
                        }
                        3 => {
                            let live: Vec<CircleId> =
                                sketch.live_circles().map(|(id, _)| id).collect();
                            if !live.is_empty() {
                                sketch.erase(Element::Circle(live[rng.range(live.len())]));
                            }
                        }
                        4 => {
                            let point = PointId(1 + rng.range(sketch.points().len() - 1));
                            sketch.erase(Element::Point(point));
                        }
                        _ => {
                            let points = sketch.points().len();
                            let (start, end) = (rng.range(points), rng.range(points));
                            if start != end {
                                sketch.add_segment(PointId(start), PointId(end));
                            }
                        }
                    },
                    _ => {
                        let point = PointId(rng.range(sketch.points().len()));
                        sketch.move_point(point, DVec2::new(rng.coordinate(), rng.coordinate()));
                    }
                }

                let remembered = sketch.settled_points(1.0);
                sketch.forget_what_is_settled();

                assert_eq!(
                    remembered,
                    sketch.settled_points(1.0),
                    "seed {seed}, edit {step}: the verdict came from a drawing that no longer is",
                );
                verdicts.push(remembered);
            }

            assert!(
                verdicts
                    .iter()
                    .any(|verdict| verdict[1..].iter().any(|held| *held)),
                "seed {seed} never settled a point, so it read nothing worth remembering",
            );
            assert!(
                verdicts.windows(2).any(|pair| pair[0] != pair[1]),
                "seed {seed} gave one verdict throughout, so no edit was ever seen",
            );
        }
    }
}
