//! What sketch · patterning/span.rs is held to.

use crate::plane::WorkPlane;

use super::*;

const TOLERANCE: f64 = 1e-9;

#[test]
fn a_lone_circle_stands_as_wide_as_it_is_across() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(1.0, 1.0));
    let round = Element::Circle(sketch.add_circle(centre, 3.0));

    let span = sketch.widest_span(&[round]).expect("a circle has a width");

    assert!(
        (span - 6.0).abs() <= TOLERANCE,
        "a circle of radius 3 spans {span}, wanted 6"
    );
}

#[test]
fn an_arc_stands_as_wide_as_the_circle_it_is_cut_from() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let start = sketch.add_point(DVec2::new(5.0, 0.0));
    let end = sketch.add_point(DVec2::new(0.0, 5.0));
    let curve = Element::Arc(sketch.add_arc(centre, start, end));

    let span = sketch.widest_span(&[curve]).expect("an arc has a width");

    assert!(
        (span - 10.0).abs() <= TOLERANCE,
        "a quarter arc of radius 5 spans {span}, wanted 10"
    );
}

#[test]
fn a_square_stands_as_wide_as_its_diagonal() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners: Vec<_> = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]
        .map(|(x, y)| sketch.add_point(DVec2::new(x, y)))
        .to_vec();
    let sides: Vec<Element> = (0..4)
        .map(|corner| {
            Element::Segment(sketch.add_segment(corners[corner], corners[(corner + 1) % 4]))
        })
        .collect();

    let span = sketch.widest_span(&sides).expect("a square has a width");

    assert!(
        (span - 32.0_f64.sqrt()).abs() <= TOLERANCE,
        "a square of side 4 spans {span}, wanted its diagonal: no direction it is later \
         stepped along can make that too small"
    );
}

#[test]
fn a_lone_point_has_no_width_to_open_a_step_on() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let alone = Element::Point(sketch.add_point(DVec2::new(2.0, 2.0)));

    assert_eq!(sketch.widest_span(&[alone]), None);
    assert_eq!(sketch.widest_span(&[]), None, "nothing held stands nowhere");
}
