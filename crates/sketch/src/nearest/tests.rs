//! What of a drawing the cursor finds nearest it.

use glam::DVec2;

use crate::annotation::AnnotationMetrics;
use crate::constraints::DimensionTarget;
use crate::plane::WorkPlane;
use crate::sketch::Sketch;

#[test]
fn the_nearest_segment_is_found_along_its_body() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::ZERO);
    let b = sketch.add_point(DVec2::new(100.0, 0.0));
    let segment = sketch.add_segment(a, b);

    assert_eq!(
        sketch.nearest_segment(DVec2::new(50.0, 2.0), 5.0),
        Some(segment)
    );
    assert_eq!(sketch.nearest_segment(DVec2::new(50.0, 40.0), 5.0), None);
    assert_eq!(sketch.nearest_segment(DVec2::new(150.0, 0.0), 5.0), None);
}

#[test]
fn a_circle_is_found_by_its_outline() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::ZERO);
    let circle = sketch.add_circle(center, 10.0);

    assert_eq!(
        sketch.nearest_circle(DVec2::new(10.2, 0.0), 1.0),
        Some(circle)
    );
    assert_eq!(
        sketch.nearest_circle(DVec2::ZERO, 1.0),
        None,
        "not the middle"
    );
    assert_eq!(sketch.nearest_circle(DVec2::new(30.0, 0.0), 1.0), None);
}

#[test]
fn nearest_dimension_no_longer_needs_anchors_from_the_caller() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(40.0, 0.0));
    let segment = sketch.add_segment(start, end);
    let target = DimensionTarget::Length(segment);
    sketch.set_dimension(target, 40.0, false);

    let metrics = AnnotationMetrics {
        offset_pixels: 22.0,
        arrow_pixels: 8.0,
        arc_pixels: 34.0,
        pixel: 1.0,
        nudge: DVec2::ZERO,
    };
    let written_at = sketch.place(target, metrics).unwrap().text_at;

    assert_eq!(
        sketch.nearest_dimension(written_at, 1.0, metrics),
        Some(target),
        "the sketch works out where its own dimension is written, with no \
         anchors passed in from outside"
    );
    assert_eq!(
        sketch.nearest_dimension(DVec2::new(1000.0, 1000.0), 1.0, metrics),
        None,
    );
}
