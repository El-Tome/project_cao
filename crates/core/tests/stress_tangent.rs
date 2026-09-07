//! A tangent circle put through every change the interface can ask of it.
//!
//! Not a proof that any particular value is right — a check that none of them
//! sends a coordinate to infinity or takes the application down, which is what
//! a solver fed contradictory values is apt to do.

use cao_core::{Operation, PartDocument, PointRef};
use cao_sketch::{Constraint, DimensionTarget, PointId, SegmentId, SketchAxis, WorkPlane};
use glam::DVec2;

/// Random enough to shuffle the order of the changes, and repeatable so a
/// failure can be looked at again.
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
    fn unit(&mut self) -> f64 {
        (self.next() % 100_000) as f64 / 100_000.0
    }
}

#[test]
fn a_tangent_circle_hammered_with_values_never_goes_to_pieces() {
    for seed in 1..16u64 {
        let mut rng = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).max(1));
        let mut doc = PartDocument::new("stress");
        doc.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
        });
        let corners = [
            DVec2::new(-60.0, -40.0),
            DVec2::new(70.0, -35.0),
            DVec2::new(5.0, 60.0),
        ];
        for pair in [(0, 1), (1, 2), (2, 0)] {
            doc.apply(Operation::AddSegment {
                sketch: 0,
                start: PointRef::New(corners[pair.0]),
                end: PointRef::New(corners[pair.1]),
            });
        }
        doc.apply(Operation::AddCircle {
            sketch: 0,
            center: PointRef::New(DVec2::new(2.0, -5.0)),
            radius: 25.0,
            rim: Vec::new(),
        });
        let circle = cao_sketch::CircleId(0);
        let tangents = 2 + rng.range(2);
        for line in 0..tangents {
            doc.apply(Operation::Constrain {
                sketch: 0,
                constraint: Constraint::Tangent {
                    circle,
                    segment: SegmentId(line),
                    at: None,
                },
            });
        }
        let center = doc.sketches()[0].circle(circle).center;

        for _ in 0..40 {
            let sketch = &doc.sketches()[0];
            let points = sketch.points().len();
            let segments = sketch.segments().len();
            match rng.range(5) {
                0 => {
                    doc.apply(Operation::SetDimension {
                        sketch: 0,
                        target: DimensionTarget::PointToSegment {
                            point: center,
                            segment: SegmentId(rng.range(segments)),
                        },
                        value: rng.unit() * 120.0,
                        placement: None,
                    });
                }
                1 => {
                    doc.apply(Operation::SetDimension {
                        sketch: 0,
                        target: DimensionTarget::Diameter(circle),
                        value: 1.0 + rng.unit() * 150.0,
                        placement: None,
                    });
                }
                2 => {
                    doc.apply(Operation::MovePoint {
                        sketch: 0,
                        point: PointId(rng.range(points)),
                        position: DVec2::new(
                            (rng.unit() - 0.5) * 200.0,
                            (rng.unit() - 0.5) * 200.0,
                        ),
                    });
                }
                3 => {
                    doc.apply(Operation::SetDimension {
                        sketch: 0,
                        target: DimensionTarget::Length(SegmentId(rng.range(segments))),
                        value: 1.0 + rng.unit() * 200.0,
                        placement: None,
                    });
                }
                _ => {
                    doc.apply(Operation::SetDimension {
                        sketch: 0,
                        target: DimensionTarget::AxisAngle {
                            segment: SegmentId(rng.range(segments)),
                            axis: SketchAxis::U,
                        },
                        value: rng.unit() * 180.0,
                        placement: None,
                    });
                }
            }

            let sketch = &doc.sketches()[0];
            for (rank, point) in sketch.points().iter().enumerate() {
                assert!(
                    point.is_finite(),
                    "seed {seed}: le point {rank} est parti a {point}"
                );
            }
            for round in sketch.circles() {
                assert!(round.radius.is_finite(), "seed {seed}: rayon");
            }
            let _ = sketch.settled_points(doc.scale());
            let _ = sketch.freedom(doc.scale());
            let _ = sketch.regions();
            for target in [
                DimensionTarget::Diameter(circle),
                DimensionTarget::PointToSegment {
                    point: center,
                    segment: SegmentId(0),
                },
            ] {
                let _ = sketch.would_be_redundant(target, doc.scale());
                let _ = doc.measured(0, target);
            }
        }
    }
}
