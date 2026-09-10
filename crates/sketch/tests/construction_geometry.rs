use cao_sketch::{Sketch, WorkPlane};
use glam::DVec2;

#[test]
fn a_construction_side_leaves_the_shape_open() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::ZERO);
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let c = sketch.add_point(DVec2::new(10.0, 10.0));
    let d = sketch.add_point(DVec2::new(0.0, 10.0));
    sketch.add_segment(a, b);
    sketch.add_segment(b, c);
    sketch.add_segment(c, d);
    sketch.add_construction_segment(d, a);

    assert!(sketch.regions().is_empty());
}

#[test]
fn a_construction_circle_encloses_no_area() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::ZERO);
    sketch.add_construction_circle(center, 5.0);

    assert!(sketch.regions().is_empty());
}

#[test]
fn a_construction_segment_reads_as_such() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::ZERO);
    let b = sketch.add_point(DVec2::X);
    let id = sketch.add_construction_segment(a, b);

    assert!(sketch.segments()[id.0].construction);
}

#[test]
fn a_construction_circle_reads_as_such() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::ZERO);
    let id = sketch.add_construction_circle(center, 5.0);

    assert!(sketch.circles()[id.0].construction);
}
