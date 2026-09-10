use cao_sketch::{Element, Sketch, WorkPlane};
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
    let fourth = sketch.add_segment(d, a);
    sketch.set_construction(Element::Segment(fourth), true);

    assert!(sketch.regions().is_empty());
}

#[test]
fn a_construction_circle_encloses_no_area() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::ZERO);
    let circle = sketch.add_circle(center, 5.0);
    sketch.set_construction(Element::Circle(circle), true);

    assert!(sketch.regions().is_empty());
}

#[test]
fn flagging_a_segment_construction_is_read_back_off_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::ZERO);
    let b = sketch.add_point(DVec2::X);
    let id = sketch.add_segment(a, b);

    sketch.set_construction(Element::Segment(id), true);

    assert!(sketch.segments()[id.0].construction);
}

#[test]
fn flagging_a_circle_construction_is_read_back_off_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::ZERO);
    let id = sketch.add_circle(center, 5.0);

    sketch.set_construction(Element::Circle(id), true);

    assert!(sketch.circles()[id.0].construction);
}
