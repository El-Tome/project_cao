//! What sketch · settled.rs is held to.

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
                        let live: Vec<CircleId> = sketch.live_circles().map(|(id, _)| id).collect();
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
                        let live: Vec<CircleId> = sketch.live_circles().map(|(id, _)| id).collect();
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
