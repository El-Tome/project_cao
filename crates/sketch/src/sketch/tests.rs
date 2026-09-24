//! What a drawing is held to, by subject, with the fixtures they share.

mod finding;
mod joining_and_erasing;
mod rules;
mod settling;
mod values;
mod what_is_left_to_pin;

pub(crate) use super::*;
pub(crate) use crate::annotation::AnnotationMetrics;
pub(crate) use crate::constraints::SketchAxis;

/// The smallest drawing there is: two traits leaving the same corner.
fn corner() -> (Sketch, SegmentId, SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let pivot = sketch.add_point(DVec2::new(10.0, 10.0));
    let a = sketch.add_point(DVec2::new(60.0, 14.0));
    let b = sketch.add_point(DVec2::new(16.0, 55.0));
    let first = sketch.add_segment(pivot, a);
    let second = sketch.add_segment(pivot, b);
    (sketch, first, second)
}

fn direction(sketch: &Sketch, segment: SegmentId) -> DVec2 {
    let (start, end) = sketch.endpoints(segment);
    (end - start).normalize()
}

/// A right-angled triangle hung off the sketch origin.
fn triangle() -> (Sketch, [SegmentId; 3]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = Sketch::ORIGIN;
    let right = sketch.add_point(DVec2::new(40.0, 0.0));
    let top = sketch.add_point(DVec2::new(0.0, 30.0));
    let base = sketch.add_segment(corner, right);
    let side = sketch.add_segment(corner, top);
    let hypotenuse = sketch.add_segment(right, top);
    (sketch, [base, side, hypotenuse])
}
