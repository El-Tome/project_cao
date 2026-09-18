//! What the walk of a drawing encloses.

use super::*;
use crate::plane::WorkPlane;

fn rectangle(sketch: &mut Sketch, min: DVec2, max: DVec2) {
    let corners = [
        sketch.add_point(min),
        sketch.add_point(DVec2::new(max.x, min.y)),
        sketch.add_point(max),
        sketch.add_point(DVec2::new(min.x, max.y)),
    ];
    for index in 0..4 {
        sketch.add_segment(corners[index], corners[(index + 1) % 4]);
    }
}

#[test]
fn an_open_shape_encloses_nothing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::ZERO);
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let c = sketch.add_point(DVec2::new(10.0, 10.0));
    sketch.add_segment(a, b);
    sketch.add_segment(b, c);
    assert!(sketch.regions().is_empty());
}

#[test]
fn a_closed_contour_is_one_region() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));
    let regions = sketch.regions();
    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0].depth, 0);
    let area: f64 = regions[0]
        .triangles
        .iter()
        .map(|[a, b, c]| (b - a).perp_dot(c - a).abs() * 0.5)
        .sum();
    assert!((area - 40.0).abs() < 1e-3, "area {area}");
}

#[test]
fn a_shape_inside_another_is_one_level_deeper() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(20.0, 20.0));
    rectangle(&mut sketch, DVec2::new(5.0, 5.0), DVec2::new(10.0, 10.0));
    let regions = sketch.regions();
    assert_eq!(regions.len(), 2);
    assert_eq!(regions[0].depth, 0);
    assert_eq!(regions[1].depth, 1);
}

#[test]
fn two_shapes_sharing_a_side_are_two_regions() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::ZERO);
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let c = sketch.add_point(DVec2::new(10.0, 10.0));
    let d = sketch.add_point(DVec2::new(0.0, 10.0));
    let e = sketch.add_point(DVec2::new(20.0, 0.0));
    let f = sketch.add_point(DVec2::new(20.0, 10.0));
    for (from, to) in [(a, b), (b, c), (c, d), (d, a), (b, e), (e, f), (f, c)] {
        sketch.add_segment(from, to);
    }
    let regions = sketch.regions();
    assert_eq!(regions.len(), 2);
    assert!(regions.iter().all(|region| region.depth == 0));
}

fn area(triangles: &[[DVec2; 3]]) -> f64 {
    triangles
        .iter()
        .map(|[a, b, c]| (b - a).perp_dot(c - a).abs() * 0.5)
        .sum()
}

/// Two circles one inside the other are a tube, not a rod: the face keeps
/// the middle hollow.
#[test]
fn a_shape_inside_another_is_a_hole_in_its_face() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(20.0, 20.0));
    rectangle(&mut sketch, DVec2::new(5.0, 5.0), DVec2::new(15.0, 15.0));
    let regions = sketch.regions();

    assert_eq!(regions[0].holes.len(), 1, "the outer contour is pierced");
    assert!(regions[1].holes.is_empty());

    let ring = area(&regions[0].face_triangles());
    assert!((ring - 300.0).abs() < 1e-2, "area of the ring: {ring}");
    assert!(
        (area(&regions[0].triangles) - 400.0).abs() < 1e-2,
        "the solid fill ignores the hole"
    );

    assert!(regions[0].contains(DVec2::new(2.0, 2.0)));
    assert!(
        !regions[0].contains(DVec2::new(10.0, 10.0)),
        "the hole is empty"
    );
    assert!(regions[1].contains(DVec2::new(10.0, 10.0)));
}

/// Matter inside a hole is matter again, and belongs to its own face.
#[test]
fn a_shape_inside_a_hole_is_not_a_hole_of_the_outer_one() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(30.0, 30.0));
    rectangle(&mut sketch, DVec2::new(5.0, 5.0), DVec2::new(25.0, 25.0));
    rectangle(&mut sketch, DVec2::new(10.0, 10.0), DVec2::new(20.0, 20.0));
    let regions = sketch.regions();

    assert_eq!(regions[0].holes.len(), 1);
    assert_eq!(regions[1].holes.len(), 1);
    assert!(regions[2].holes.is_empty());
    assert!((area(&regions[2].face_triangles()) - 100.0).abs() < 1e-2);
}

/// Le cas cité : deux cercles concentriques font un tube.
#[test]
fn two_circles_make_a_tube() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::new(10.0, 10.0));
    sketch.add_circle(center, 8.0);
    sketch.add_circle(center, 5.0);

    let regions = sketch.regions();
    assert_eq!(regions.len(), 2);
    let ring = area(&regions[0].face_triangles());
    let expected = std::f64::consts::PI * (8.0f64.powi(2) - 5.0f64.powi(2));
    assert!(
        (ring - expected).abs() / expected < 0.02,
        "ring {ring}, expected ~{expected}"
    );
    assert!(!regions[0].contains(DVec2::new(10.0, 10.0)));
}

#[test]
fn a_circle_encloses_its_disc() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::new(3.0, 3.0));
    sketch.add_circle(center, 2.0);
    let regions = sketch.regions();
    assert_eq!(regions.len(), 1);
    assert!(encloses(&regions[0].outline, DVec2::new(3.0, 3.0)));
}
